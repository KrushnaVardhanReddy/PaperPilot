use serde::{Deserialize, Serialize};

/// Structural elements of a document AST.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocElement {
    Heading { level: usize, text: String },
    Paragraph(String),
    ListItem { ordered: bool, text: String },
    CodeBlock { language: Option<String>, code: String },
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    Divider,
}

impl DocElement {
    /// Render this AST element to GitHub Flavored Markdown (GFM).
    pub fn to_markdown(&self) -> String {
        match self {
            DocElement::Heading { level, text } => {
                let lvl = (*level).clamp(1, 6);
                let hashes = "#".repeat(lvl);
                format!("{} {}", hashes, text.trim())
            }
            DocElement::Paragraph(p) => p.trim().to_string(),
            DocElement::ListItem { ordered, text } => {
                if *ordered {
                    format!("1. {}", text.trim())
                } else {
                    format!("- {}", text.trim())
                }
            }
            DocElement::CodeBlock { language, code } => {
                let lang = language.as_deref().unwrap_or("");
                format!("```{}\n{}\n```", lang, code.trim_end())
            }
            DocElement::Table { headers, rows } => {
                let mut out = String::new();
                if headers.is_empty() && rows.is_empty() {
                    return out;
                }

                let num_cols = if !headers.is_empty() {
                    headers.len()
                } else if let Some(first_row) = rows.first() {
                    first_row.len()
                } else {
                    0
                };

                if num_cols == 0 {
                    return out;
                }

                // Headers or fallback headers
                let effective_headers: Vec<String> = if headers.is_empty() {
                    (1..=num_cols).map(|i| format!("Column {}", i)).collect()
                } else {
                    let mut h = headers.clone();
                    while h.len() < num_cols {
                        h.push(format!("Column {}", h.len() + 1));
                    }
                    h
                };

                let clean_cell = |c: &str| c.replace('|', "\\|").replace('\n', " ").trim().to_string();

                out.push_str("| ");
                out.push_str(
                    &effective_headers
                        .iter()
                        .map(|h| clean_cell(h))
                        .collect::<Vec<_>>()
                        .join(" | "),
                );
                out.push_str(" |\n");

                out.push_str("| ");
                out.push_str(
                    &vec!["---"; num_cols].join(" | "),
                );
                out.push_str(" |\n");

                for row in rows {
                    let mut r = row.clone();
                    while r.len() < num_cols {
                        r.push(String::new());
                    }
                    out.push_str("| ");
                    out.push_str(
                        &r.iter()
                            .take(num_cols)
                            .map(|c| clean_cell(c))
                            .collect::<Vec<_>>()
                            .join(" | "),
                    );
                    out.push_str(" |\n");
                }
                out.trim_end().to_string()
            }
            DocElement::Divider => "---".to_string(),
        }
    }
}

/// A parsed document or page represented as a sequence of DocAST elements.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocAst {
    pub elements: Vec<DocElement>,
}

impl DocAst {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, element: DocElement) {
        self.elements.push(element);
    }

    /// Render AST elements into clean GitHub Flavored Markdown (GFM).
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        let mut in_list = false;

        for element in &self.elements {
            match element {
                DocElement::ListItem { .. } => {
                    if !in_list && !out.is_empty() && !out.ends_with("\n\n") {
                        if !out.ends_with('\n') {
                            out.push('\n');
                        }
                        out.push('\n');
                    }
                    out.push_str(&element.to_markdown());
                    out.push('\n');
                    in_list = true;
                }
                _ => {
                    if in_list {
                        out.push('\n');
                        in_list = false;
                    } else if !out.is_empty() && !out.ends_with("\n\n") {
                        if !out.ends_with('\n') {
                            out.push('\n');
                        }
                        out.push('\n');
                    }
                    out.push_str(&element.to_markdown());
                    out.push_str("\n\n");
                }
            }
        }

        out.trim().to_string()
    }
}

/// Layout-aware extractor that converts raw lines and layout structures into a `DocAst`.
pub struct LayoutExtractor;

impl LayoutExtractor {
    /// Parse multiple page text slices in parallel (via `rayon` on native targets) into [`DocAst`]s,
    /// preserving original page order.
    pub fn parse_pages_parallel<S: AsRef<str> + Sync>(pages: &[S]) -> Vec<DocAst> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            use rayon::prelude::*;
            pages
                .par_iter()
                .map(|page| Self::parse_text(page.as_ref()))
                .collect()
        }
        #[cfg(target_arch = "wasm32")]
        {
            pages
                .iter()
                .map(|page| Self::parse_text(page.as_ref()))
                .collect()
        }
    }

    /// Parse and render multiple page text slices in parallel (via `rayon` on native targets)
    /// into Markdown strings, preserving original page order.
    pub fn render_pages_markdown_parallel<S: AsRef<str> + Sync>(pages: &[S]) -> Vec<String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            use rayon::prelude::*;
            pages
                .par_iter()
                .map(|page| Self::parse_text(page.as_ref()).to_markdown())
                .collect()
        }
        #[cfg(target_arch = "wasm32")]
        {
            pages
                .iter()
                .map(|page| Self::parse_text(page.as_ref()).to_markdown())
                .collect()
        }
    }

    /// Parse raw text from a document or page into a `DocAst`.
    pub fn parse_text(text: &str) -> DocAst {
        // Normalize text that might contain embedded '\n', bullet markers, or merged numbered items
        let mut preprocessed = String::new();
        for orig_line in text.lines() {
            let mut line_buf = String::new();
            let chars: Vec<char> = orig_line.chars().collect();
            let mut idx = 0;
            while idx < chars.len() {
                let ch = chars[idx];
                if ch == '•' {
                    if !line_buf.trim().is_empty() {
                        preprocessed.push_str(&line_buf);
                        preprocessed.push('\n');
                        line_buf.clear();
                    }
                    line_buf.push('•');
                    idx += 1;
                } else if ch.is_ascii_digit() && idx + 1 < chars.len() && (chars[idx + 1] == '.' || chars[idx + 1] == ')') {
                    // Check if this digit is a new list item or section header preceded by text without newline
                    if !line_buf.trim().is_empty() && (line_buf.ends_with(' ') || line_buf.ends_with(|c: char| c.is_alphabetic())) {
                        preprocessed.push_str(&line_buf);
                        preprocessed.push('\n');
                        line_buf.clear();
                    }
                    line_buf.push(ch);
                    idx += 1;
                } else {
                    line_buf.push(ch);
                    idx += 1;
                }
            }
            if !line_buf.is_empty() {
                preprocessed.push_str(&line_buf);
                preprocessed.push('\n');
            }
        }

        // Additional heuristic: split numbered headings if glued to following sentences,
        // e.g. "1 Introduction This is an introduction paragraph."
        // or section like "1.1 Overview This is..."
        let mut normalized_text = String::new();
        for l in preprocessed.lines() {
            let trimmed = l.trim();
            if Self::parse_list_item(trimmed).is_none() {
                if let Some((section_len, _)) = Self::find_leading_numbered_section(trimmed) {
                    let rest = &trimmed[section_len..];
                    // Check if rest starts with a capital letter sentence after a space or has a period
                    let end_pos = rest.find(". ").or_else(|| {
                        // or e.g. "Introduction This is..." -> find word boundary where next word starts with uppercase
                        let words: Vec<&str> = rest.split_whitespace().collect();
                        if words.len() >= 2 {
                            // first word is title (e.g. Introduction), rest is body
                            rest.find(words[1]).map(|p| p.saturating_sub(1))
                        } else {
                            None
                        }
                    });
                    if let Some(pos) = end_pos {
                        let heading_cand = &trimmed[..section_len + pos];
                        let body_cand = &rest[pos..].trim_start_matches(|c| c == '.' || c == ' ');
                        if heading_cand.chars().count() < 60 && !body_cand.trim().is_empty() {
                            normalized_text.push_str(heading_cand);
                            normalized_text.push('\n');
                            normalized_text.push_str(body_cand);
                            normalized_text.push('\n');
                            continue;
                        }
                    }
                }
            }
            normalized_text.push_str(l);
            normalized_text.push('\n');
        }

        let mut ast = DocAst::new();
        let lines: Vec<&str> = normalized_text.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            if line.is_empty() {
                i += 1;
                continue;
            }

            // 1. Horizontal divider
            if line == "---" || line == "***" || line == "___" {
                ast.push(DocElement::Divider);
                i += 1;
                continue;
            }

            // 2. Fenced code block or indented code block
            if line.starts_with("```") {
                let lang_token = line.trim_start_matches('`').trim();
                let language = if lang_token.is_empty() {
                    None
                } else {
                    Some(lang_token.to_string())
                };
                let mut code_lines = Vec::new();
                i += 1;
                while i < lines.len() {
                    let next_line = lines[i];
                    if next_line.trim().starts_with("```") {
                        i += 1;
                        break;
                    }
                    code_lines.push(next_line);
                    i += 1;
                }
                ast.push(DocElement::CodeBlock {
                    language,
                    code: code_lines.join("\n"),
                });
                continue;
            }

            // 3. Markdown Table detection
            if Self::is_table_row(line) {
                let mut table_lines = Vec::new();
                while i < lines.len() && (Self::is_table_row(lines[i].trim()) || Self::is_table_delimiter(lines[i].trim())) {
                    table_lines.push(lines[i].trim());
                    i += 1;
                }
                if let Some(table_elem) = Self::parse_markdown_table(&table_lines) {
                    ast.push(table_elem);
                    continue;
                }
            }

            // 4. Tabular text detection (multiple columns separated by 2+ spaces or tabs)
            if Self::is_aligned_tabular_row(line) && i + 1 < lines.len() && Self::is_aligned_tabular_row(lines[i + 1].trim()) {
                let mut table_lines = Vec::new();
                while i < lines.len() && Self::is_aligned_tabular_row(lines[i].trim()) {
                    table_lines.push(lines[i].trim());
                    i += 1;
                }
                if let Some(table_elem) = Self::parse_aligned_table(&table_lines) {
                    ast.push(table_elem);
                    continue;
                }
            }

            // 5. Explicit Markdown headings (# Heading)
            if let Some((level, htext)) = Self::parse_explicit_heading(line) {
                ast.push(DocElement::Heading {
                    level,
                    text: htext,
                });
                i += 1;
                continue;
            }

            // 6. List Items (bullet: * , - , • ; ordered: 1. , 1) , [1] )
            if let Some((ordered, item_text)) = Self::parse_list_item(line) {
                ast.push(DocElement::ListItem {
                    ordered,
                    text: item_text,
                });
                i += 1;
                continue;
            }

            // 7. Heuristic Heading Detection
            // Short isolated line (< 80 chars, not ending in period/comma/colon/semicolon)
            // or all-caps short title, or numbered heading like "1.2 Introduction"
            if let Some((level, htext)) = Self::detect_heuristic_heading(line, lines.get(i.wrapping_sub(1)).copied(), lines.get(i + 1).copied()) {
                ast.push(DocElement::Heading {
                    level,
                    text: htext,
                });
                i += 1;
                continue;
            }

            // 8. Paragraph grouping
            let mut para_lines = vec![line];
            i += 1;
            while i < lines.len() {
                let next_line = lines[i].trim();
                if next_line.is_empty()
                    || next_line == "---"
                    || next_line == "***"
                    || next_line.starts_with("```")
                    || Self::is_table_row(next_line)
                    || Self::parse_explicit_heading(next_line).is_some()
                    || Self::parse_list_item(next_line).is_some()
                    || Self::detect_heuristic_heading(next_line, Some(para_lines.last().copied().unwrap_or("")), lines.get(i + 1).copied()).is_some()
                {
                    break;
                }
                para_lines.push(next_line);
                i += 1;
            }

            ast.push(DocElement::Paragraph(para_lines.join(" ")));
        }

        ast
    }

    fn parse_explicit_heading(line: &str) -> Option<(usize, String)> {
        if line.starts_with('#') {
            let hashes = line.chars().take_while(|c| *c == '#').count();
            if hashes <= 6 && line[hashes..].starts_with(' ') {
                return Some((hashes, line[hashes..].trim().to_string()));
            }
        }
        None
    }

    fn parse_list_item(line: &str) -> Option<(bool, String)> {
        // Bullet points: •, -, *
        if let Some(rest) = line.strip_prefix('•') {
            return Some((false, rest.trim().to_string()));
        }
        if (line.starts_with("- ") || line.starts_with("* ")) && line.len() > 2 {
            return Some((false, line[2..].trim().to_string()));
        }

        // Ordered list: "1. ", "1) "
        let digits: String = line.chars().take_while(|c| c.is_ascii_digit()).collect();
        if !digits.is_empty() && digits.len() <= 4 {
            let after_digits = &line[digits.len()..];
            if let Some(rest) = after_digits.strip_prefix(". ") {
                return Some((true, rest.trim().to_string()));
            }
            if let Some(rest) = after_digits.strip_prefix(") ") {
                return Some((true, rest.trim().to_string()));
            }
        }

        None
    }

    fn detect_heuristic_heading(line: &str, prev_line: Option<&str>, next_line: Option<&str>) -> Option<(usize, String)> {
        let trimmed = line.trim();
        let len = trimmed.chars().count();

        // Must be non-empty and reasonably short for a heading (< 80 chars)
        if len == 0 || len >= 80 {
            return None;
        }

        // Cannot end with trailing punctuation typical for normal body sentences
        if trimmed.ends_with('.') || trimmed.ends_with(',') || trimmed.ends_with(';') {
            return None;
        }

        // Section numbering patterns, e.g. "1 Introduction", "1.1 Architecture", "2.3.1 Data Flow"
        if let Some((level, title)) = Self::parse_section_numbered_heading(trimmed) {
            return Some((level, title));
        }

        let is_all_caps = len >= 3 && trimmed.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
        let has_isolated_spacing = prev_line.map_or(true, |l| l.trim().is_empty())
            && next_line.map_or(true, |l| l.trim().is_empty());

        // All caps line (< 80 chars) with letters
        let has_letters = trimmed.chars().any(|c| c.is_alphabetic());
        if is_all_caps && has_letters {
            // E.g. "TABLE OF CONTENTS", "ABSTRACT", "EXECUTIVE SUMMARY", "CHAPTER 1"
            if len <= 35 {
                return Some((1, trimmed.to_string()));
            } else {
                return Some((2, trimmed.to_string()));
            }
        }

        // Title Case or short isolated line surrounded by empty space
        if has_isolated_spacing && has_letters {
            // Title case heuristic: First letter uppercase, relatively short
            let first_char = trimmed.chars().next().unwrap();
            if first_char.is_uppercase() {
                if len <= 30 {
                    return Some((1, trimmed.to_string()));
                } else if len <= 50 {
                    return Some((2, trimmed.to_string()));
                } else {
                    return Some((3, trimmed.to_string()));
                }
            }
        }

        None
    }

    fn find_leading_numbered_section(line: &str) -> Option<(usize, usize)> {
        // Find if line starts with e.g. "1 " or "1.1 "
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() == 2 {
            let num_part = parts[0].trim_end_matches('.');
            let segments: Vec<&str> = num_part.split('.').collect();
            if segments.iter().all(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())) {
                let level = segments.len().clamp(1, 4);
                return Some((parts[0].len() + 1, level));
            }
        }
        None
    }

    fn parse_section_numbered_heading(line: &str) -> Option<(usize, String)> {
        // Match "1 Introduction", "1.1 Background", "1.1.1 Implementation"
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() == 2 {
            let num_part = parts[0].trim_end_matches('.');
            let title = parts[1].trim();

            if !title.is_empty() {
                let segments: Vec<&str> = num_part.split('.').collect();
                if segments.iter().all(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())) {
                    let level = segments.len().clamp(1, 4);
                    return Some((level, line.to_string()));
                }
            }
        }
        None
    }

    fn is_table_row(line: &str) -> bool {
        line.starts_with('|') && line.ends_with('|') && line.len() >= 3
    }

    fn is_table_delimiter(line: &str) -> bool {
        line.starts_with('|')
            && line.ends_with('|')
            && line.chars().all(|c| c == '|' || c == '-' || c == ':' || c == ' ')
    }

    fn parse_markdown_table(lines: &[&str]) -> Option<DocElement> {
        if lines.is_empty() {
            return None;
        }

        let split_row = |l: &str| -> Vec<String> {
            let trimmed = l.trim().trim_start_matches('|').trim_end_matches('|');
            trimmed.split('|').map(|s| s.trim().to_string()).collect()
        };

        let mut data_rows = Vec::new();
        let mut headers = Vec::new();

        let mut start_idx = 0;
        if lines.len() >= 2 && Self::is_table_delimiter(lines[1]) {
            headers = split_row(lines[0]);
            start_idx = 2;
        }

        for &l in &lines[start_idx..] {
            if !Self::is_table_delimiter(l) {
                data_rows.push(split_row(l));
            }
        }

        Some(DocElement::Table {
            headers,
            rows: data_rows,
        })
    }

    fn is_aligned_tabular_row(line: &str) -> bool {
        let cols: Vec<&str> = line.split("  ").filter(|s| !s.trim().is_empty()).collect();
        cols.len() >= 2 && cols.len() <= 10
    }

    fn parse_aligned_table(lines: &[&str]) -> Option<DocElement> {
        if lines.len() < 2 {
            return None;
        }

        let split_row = |l: &str| -> Vec<String> {
            l.split("  ")
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        };

        let headers = split_row(lines[0]);
        let mut rows = Vec::new();

        for &l in &lines[1..] {
            let row = split_row(l);
            if !row.is_empty() {
                rows.push(row);
            }
        }

        if headers.is_empty() || rows.is_empty() {
            return None;
        }

        Some(DocElement::Table { headers, rows })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_element_markdown_rendering() {
        let h1 = DocElement::Heading {
            level: 1,
            text: "Main Title".to_string(),
        };
        assert_eq!(h1.to_markdown(), "# Main Title");

        let h2 = DocElement::Heading {
            level: 2,
            text: "Subtitle".to_string(),
        };
        assert_eq!(h2.to_markdown(), "## Subtitle");

        let p = DocElement::Paragraph("This is a paragraph text.".to_string());
        assert_eq!(p.to_markdown(), "This is a paragraph text.");

        let item1 = DocElement::ListItem {
            ordered: false,
            text: "Item A".to_string(),
        };
        assert_eq!(item1.to_markdown(), "- Item A");

        let item2 = DocElement::ListItem {
            ordered: true,
            text: "Item 1".to_string(),
        };
        assert_eq!(item2.to_markdown(), "1. Item 1");

        let code = DocElement::CodeBlock {
            language: Some("rust".to_string()),
            code: "fn main() {\n    println!(\"hello\");\n}".to_string(),
        };
        assert_eq!(
            code.to_markdown(),
            "```rust\nfn main() {\n    println!(\"hello\");\n}\n```"
        );

        let table = DocElement::Table {
            headers: vec!["Col A".to_string(), "Col B".to_string()],
            rows: vec![
                vec!["Val 1".to_string(), "Val 2".to_string()],
                vec!["Val 3".to_string(), "Val 4".to_string()],
            ],
        };
        assert_eq!(
            table.to_markdown(),
            "| Col A | Col B |\n| --- | --- |\n| Val 1 | Val 2 |\n| Val 3 | Val 4 |"
        );

        let div = DocElement::Divider;
        assert_eq!(div.to_markdown(), "---");
    }

    #[test]
    fn test_layout_extractor_headings_and_lists() {
        let text = r#"
1 Introduction
This is the intro paragraph.

1.1 Architecture
Architecture overview text with some details.

• First bullet item
• Second bullet item
* Star bullet item
- Dash bullet item

1. Numbered item 1
2. Numbered item 2

2 Conclusion
Summary here.
"#;
        let ast = LayoutExtractor::parse_text(text);
        let md = ast.to_markdown();

        assert!(md.contains("# 1 Introduction"));
        assert!(md.contains("## 1.1 Architecture"));
        assert!(md.contains("- First bullet item"));
        assert!(md.contains("- Second bullet item"));
        assert!(md.contains("- Star bullet item"));
        assert!(md.contains("- Dash bullet item"));
        assert!(md.contains("1. Numbered item 1"));
        assert!(md.contains("1. Numbered item 2"));
        assert!(md.contains("# 2 Conclusion"));
    }

    #[test]
    fn test_layout_extractor_tables() {
        let pipe_table = r#"
| Name | Age | City |
| --- | --- | --- |
| Alice | 30 | New York |
| Bob | 25 | London |
"#;
        let ast1 = LayoutExtractor::parse_text(pipe_table);
        let md1 = ast1.to_markdown();
        assert!(md1.contains("| Name | Age | City |"));
        assert!(md1.contains("| Alice | 30 | New York |"));

        let space_table = r#"
Product    Price    Quantity
Apples     $1.50    10
Oranges    $2.00    5
"#;
        let ast2 = LayoutExtractor::parse_text(space_table);
        let md2 = ast2.to_markdown();
        assert!(md2.contains("| Product | Price | Quantity |"));
        assert!(md2.contains("| Apples | $1.50 | 10 |"));
    }

    #[test]
    fn test_layout_extractor_all_caps_heading() {
        let text = r#"
TABLE OF CONTENTS

Some paragraph under contents.

EXECUTIVE SUMMARY

Details of executive summary.
"#;
        let ast = LayoutExtractor::parse_text(text);
        let md = ast.to_markdown();
        assert!(md.contains("# TABLE OF CONTENTS"));
        assert!(md.contains("# EXECUTIVE SUMMARY"));
    }
}
