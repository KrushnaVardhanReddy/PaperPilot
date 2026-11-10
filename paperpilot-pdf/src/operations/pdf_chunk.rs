use crate::document::LopdfDocument;
use paperpilot_core::document_source::DocumentSource;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

fn default_target_tokens() -> usize {
    500
}

fn default_overlap_tokens() -> usize {
    50
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChunkOptions {
    #[serde(default = "default_target_tokens")]
    pub target_tokens: usize, // default 500, min 50, max 4000
    #[serde(default = "default_overlap_tokens")]
    pub overlap_tokens: usize, // default 50, min 0, max 500
    #[serde(default = "default_true")]
    pub strip_boilerplate: bool, // default true
    #[serde(default = "default_true")]
    pub break_on_headings: bool, // default true
}

impl Default for ChunkOptions {
    fn default() -> Self {
        Self {
            target_tokens: 500,
            overlap_tokens: 50,
            strip_boilerplate: true,
            break_on_headings: true,
        }
    }
}

impl ChunkOptions {
    pub fn sanitize(&mut self) {
        if self.target_tokens < 50 {
            self.target_tokens = 50;
        } else if self.target_tokens > 4000 {
            self.target_tokens = 4000;
        }
        if self.overlap_tokens > 500 {
            self.overlap_tokens = 500;
        }
        if self.overlap_tokens >= self.target_tokens {
            self.overlap_tokens = self.target_tokens.saturating_sub(1);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentChunk {
    pub chunk_index: usize,
    pub page_start: u32,
    pub page_end: u32,
    pub estimated_tokens: usize,
    pub text: String,
    pub headings_context: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChunkResult {
    pub total_chunks: usize,
    pub total_tokens: usize,
    pub chunks: Vec<DocumentChunk>,
    pub execution_time_ms: f64,
}

/// Token estimation heuristic (~4 characters per token for English text/markdown).
pub fn estimate_tokens(text: &str) -> usize {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return 0;
    }
    let chars = trimmed.chars().count();
    (chars + 3) / 4
}

/// Split text into sentences based on punctuation (`. `, `! `, `? `, `.\n`, `!\n`, `?\n`).
pub fn split_into_sentences(text: &str) -> Vec<&str> {
    let mut sentences = Vec::new();
    let mut start = 0;
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let len = chars.len();

    let mut i = 0;
    while i < len {
        let (_byte_idx, c) = chars[i];
        if c == '.' || c == '!' || c == '?' {
            let is_end = if i + 1 < len {
                chars[i + 1].1.is_whitespace()
            } else {
                true
            };
            if is_end {
                let mut end_byte = if i + 1 < len {
                    chars[i + 1].0
                } else {
                    text.len()
                };
                let mut next_i = i + 1;
                while next_i < len
                    && (chars[next_i].1 == '"'
                        || chars[next_i].1 == '\''
                        || chars[next_i].1 == ')'
                        || chars[next_i].1 == ']')
                {
                    end_byte = if next_i + 1 < len {
                        chars[next_i + 1].0
                    } else {
                        text.len()
                    };
                    next_i += 1;
                }
                let s = &text[start..end_byte];
                if !s.trim().is_empty() {
                    sentences.push(s.trim());
                }
                while next_i < len && chars[next_i].1.is_whitespace() {
                    next_i += 1;
                }
                start = if next_i < len {
                    chars[next_i].0
                } else {
                    text.len()
                };
                i = next_i;
                continue;
            }
        }
        i += 1;
    }

    if start < text.len() {
        let rem = &text[start..];
        if !rem.trim().is_empty() {
            sentences.push(rem.trim());
        }
    }

    if sentences.is_empty() && !text.trim().is_empty() {
        sentences.push(text.trim());
    }

    sentences
}

/// Extract up to `overlap_tokens` from the tail of `prev_text` by rewinding sentences/words.
pub fn get_overlap_text(prev_text: &str, overlap_tokens: usize) -> Option<String> {
    if overlap_tokens == 0 || prev_text.trim().is_empty() {
        return None;
    }

    let sentences = split_into_sentences(prev_text);
    if sentences.is_empty() {
        return None;
    }

    let mut collected: Vec<&str> = Vec::new();
    let mut current_tokens = 0;

    for s in sentences.iter().rev() {
        let tok = estimate_tokens(s);
        if collected.is_empty() {
            if tok > overlap_tokens {
                let words: Vec<&str> = s.split_whitespace().collect();
                let mut word_collected = Vec::new();
                let mut w_tokens = 0;
                for w in words.iter().rev() {
                    let w_tok = estimate_tokens(w);
                    if w_tokens + w_tok <= overlap_tokens {
                        w_tokens += w_tok;
                        word_collected.push(*w);
                    } else {
                        break;
                    }
                }
                word_collected.reverse();
                if !word_collected.is_empty() {
                    return Some(word_collected.join(" "));
                }
                return None;
            } else {
                current_tokens += tok;
                collected.push(s);
            }
        } else if current_tokens + tok <= overlap_tokens {
            current_tokens += tok;
            collected.push(s);
        } else {
            break;
        }
    }

    if collected.is_empty() {
        None
    } else {
        collected.reverse();
        Some(collected.join(" "))
    }
}

/// Check if a line is a markdown table row (e.g. `| col1 | col2 |`).
fn is_table_row(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|') && trimmed.ends_with('|') && trimmed.matches('|').count() >= 2
}

/// Helper to update active heading hierarchy context.
fn update_headings_context(context: &mut Vec<String>, level: usize, title: &str) {
    if level == 0 {
        return;
    }
    let keep = level - 1;
    if context.len() > keep {
        context.truncate(keep);
    }
    context.push(title.trim().to_string());
}

/// Boilerplate suppression across consecutive pages (>= 3 pages).
fn strip_boilerplate_from_pages(pages: &mut Vec<(u32, String)>) {
    if pages.len() < 3 {
        return;
    }

    let mut headers: Vec<Option<String>> = Vec::with_capacity(pages.len());
    let mut footers: Vec<Option<String>> = Vec::with_capacity(pages.len());

    for (_, content) in pages.iter() {
        let lines: Vec<&str> = content.lines().collect();
        let first = lines
            .iter()
            .find(|l| !l.trim().is_empty())
            .map(|l| l.trim().to_string());
        let last = lines
            .iter()
            .rfind(|l| !l.trim().is_empty())
            .map(|l| l.trim().to_string());
        headers.push(first);
        footers.push(last);
    }

    let mut strip_header_indices = vec![false; pages.len()];
    let mut strip_footer_indices = vec![false; pages.len()];

    // 1. Exact identical header lines across >= 3 consecutive pages
    let mut i = 0;
    while i < pages.len() {
        if let Some(ref h) = headers[i] {
            let mut j = i + 1;
            while j < pages.len() && headers[j].as_ref() == Some(h) {
                j += 1;
            }
            if j - i >= 3 {
                for k in i..j {
                    strip_header_indices[k] = true;
                }
            }
            i = j;
        } else {
            i += 1;
        }
    }

    // 2. Page-number pattern headers across >= 3 consecutive pages
    let is_page_num = |s: &str| {
        let t = s.trim();
        let lower = t.to_lowercase();
        let stripped = lower.strip_prefix("page ").unwrap_or(&lower).trim();
        if !stripped.is_empty() && stripped.chars().all(|c| c.is_ascii_digit()) {
            return true;
        }
        if let Some((a, b)) = stripped.split_once(" of ").or_else(|| stripped.split_once('/')) {
            if !a.trim().is_empty()
                && !b.trim().is_empty()
                && a.trim().chars().all(|c| c.is_ascii_digit())
                && b.trim().chars().all(|c| c.is_ascii_digit())
            {
                return true;
            }
        }
        false
    };

    let mut i = 0;
    while i < pages.len() {
        if headers[i].as_ref().map(|s| is_page_num(s)).unwrap_or(false) {
            let mut j = i + 1;
            while j < pages.len() && headers[j].as_ref().map(|s| is_page_num(s)).unwrap_or(false) {
                j += 1;
            }
            if j - i >= 3 {
                for k in i..j {
                    strip_header_indices[k] = true;
                }
            }
            i = j;
        } else {
            i += 1;
        }
    }

    // 3. Exact identical footer lines across >= 3 consecutive pages
    let mut i = 0;
    while i < pages.len() {
        if let Some(ref f) = footers[i] {
            let mut j = i + 1;
            while j < pages.len() && footers[j].as_ref() == Some(f) {
                j += 1;
            }
            if j - i >= 3 {
                for k in i..j {
                    strip_footer_indices[k] = true;
                }
            }
            i = j;
        } else {
            i += 1;
        }
    }

    // 4. Page-number pattern footers across >= 3 consecutive pages
    let mut i = 0;
    while i < pages.len() {
        if footers[i].as_ref().map(|s| is_page_num(s)).unwrap_or(false) {
            let mut j = i + 1;
            while j < pages.len() && footers[j].as_ref().map(|s| is_page_num(s)).unwrap_or(false) {
                j += 1;
            }
            if j - i >= 3 {
                for k in i..j {
                    strip_footer_indices[k] = true;
                }
            }
            i = j;
        } else {
            i += 1;
        }
    }

    // Apply stripping
    for (idx, (_, content)) in pages.iter_mut().enumerate() {
        let strip_h = strip_header_indices[idx];
        let strip_f = strip_footer_indices[idx];
        if !strip_h && !strip_f {
            continue;
        }

        let lines: Vec<&str> = content.lines().collect();
        let first_idx = lines.iter().position(|l| !l.trim().is_empty());
        let last_idx = lines.iter().rposition(|l| !l.trim().is_empty());

        let mut filtered_lines = Vec::new();
        for (line_idx, line) in lines.iter().enumerate() {
            if strip_h && Some(line_idx) == first_idx {
                continue;
            }
            if strip_f && Some(line_idx) == last_idx {
                continue;
            }
            filtered_lines.push(*line);
        }
        *content = filtered_lines.join("\n");
    }
}

#[derive(Debug, Clone, PartialEq)]
enum BlockType {
    Heading { level: usize, title: String },
    Table,
    Paragraph,
}

#[derive(Debug, Clone)]
struct ParsedBlock {
    page: u32,
    block_type: BlockType,
    text: String,
    tokens: usize,
}

/// Parse page content into structured blocks (Headings, Tables, Paragraphs).
fn parse_page_blocks(page: u32, content: &str) -> Vec<ParsedBlock> {
    let mut blocks = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        if trimmed.is_empty() {
            i += 1;
            continue;
        }

        // 1. Table check
        if is_table_row(trimmed) {
            let mut table_lines = Vec::new();
            while i < lines.len() && is_table_row(lines[i].trim()) {
                table_lines.push(lines[i].trim());
                i += 1;
            }
            let text = table_lines.join("\n");
            let tokens = estimate_tokens(&text);
            blocks.push(ParsedBlock {
                page,
                block_type: BlockType::Table,
                text,
                tokens,
            });
            continue;
        }

        // 2. Heading check (# through ######)
        if trimmed.starts_with('#') {
            let hash_count = trimmed.chars().take_while(|&c| c == '#').count();
            if hash_count <= 6 && trimmed[hash_count..].starts_with(' ') {
                let title = trimmed[hash_count..].trim().to_string();
                let text = trimmed.to_string();
                let tokens = estimate_tokens(&text);
                blocks.push(ParsedBlock {
                    page,
                    block_type: BlockType::Heading {
                        level: hash_count,
                        title,
                    },
                    text,
                    tokens,
                });
                i += 1;
                continue;
            }
        }

        // 3. Regular paragraph lines until empty line, heading, or table
        let mut para_lines = Vec::new();
        while i < lines.len() {
            let cur = lines[i].trim();
            if cur.is_empty() || is_table_row(cur) || (cur.starts_with('#') && cur.chars().take_while(|&c| c == '#').count() <= 6 && cur[cur.chars().take_while(|&c| c == '#').count()..].starts_with(' ')) {
                break;
            }
            para_lines.push(lines[i]);
            i += 1;
        }

        if !para_lines.is_empty() {
            let text = para_lines.join("\n");
            let tokens = estimate_tokens(&text);
            blocks.push(ParsedBlock {
                page,
                block_type: BlockType::Paragraph,
                text,
                tokens,
            });
        }
    }

    blocks
}

/// Split a large table by rows while keeping headers on each sub-chunk.
fn split_table_to_chunks(table_text: &str, target_tokens: usize) -> Vec<String> {
    let lines: Vec<&str> = table_text.lines().collect();
    if lines.len() <= 2 {
        return vec![table_text.to_string()];
    }

    let header_lines = if lines.len() > 1 && lines[1].trim().contains("---") {
        vec![lines[0], lines[1]]
    } else {
        vec![lines[0]]
    };
    let data_rows = &lines[header_lines.len()..];

    let header_str = header_lines.join("\n");
    let header_tokens = estimate_tokens(&header_str);

    let mut result = Vec::new();
    let mut current_rows = Vec::new();
    let mut current_tokens = header_tokens;

    for row in data_rows {
        let row_tokens = estimate_tokens(row);
        if current_tokens + row_tokens > target_tokens && !current_rows.is_empty() {
            let mut chunk = header_lines.clone();
            chunk.extend(current_rows.iter().cloned());
            result.push(chunk.join("\n"));
            current_rows.clear();
            current_tokens = header_tokens;
        }
        current_rows.push(*row);
        current_tokens += row_tokens;
    }

    if !current_rows.is_empty() {
        let mut chunk = header_lines.clone();
        chunk.extend(current_rows.iter().cloned());
        result.push(chunk.join("\n"));
    }

    if result.is_empty() {
        vec![table_text.to_string()]
    } else {
        result
    }
}

/// Split a large paragraph into sentence/word chunks that fit within target_tokens.
fn split_paragraph_to_chunks(para_text: &str, target_tokens: usize) -> Vec<String> {
    let sentences = split_into_sentences(para_text);
    if sentences.is_empty() {
        return vec![para_text.to_string()];
    }

    let mut result = Vec::new();
    let mut current_sentences = Vec::new();
    let mut current_tokens = 0;

    for s in sentences {
        let s_tokens = estimate_tokens(s);
        if s_tokens > target_tokens {
            // Sentence itself exceeds target_tokens: split by words
            if !current_sentences.is_empty() {
                result.push(current_sentences.join(" "));
                current_sentences.clear();
                current_tokens = 0;
            }
            let words: Vec<&str> = s.split_whitespace().collect();
            let mut word_batch = Vec::new();
            let mut batch_tokens = 0;
            for w in words {
                let w_tokens = estimate_tokens(w);
                if batch_tokens + w_tokens > target_tokens && !word_batch.is_empty() {
                    result.push(word_batch.join(" "));
                    word_batch.clear();
                    batch_tokens = 0;
                }
                word_batch.push(w);
                batch_tokens += w_tokens;
            }
            if !word_batch.is_empty() {
                result.push(word_batch.join(" "));
            }
        } else {
            if current_tokens + s_tokens > target_tokens && !current_sentences.is_empty() {
                result.push(current_sentences.join(" "));
                current_sentences.clear();
                current_tokens = 0;
            }
            current_sentences.push(s);
            current_tokens += s_tokens;
        }
    }

    if !current_sentences.is_empty() {
        result.push(current_sentences.join(" "));
    }

    if result.is_empty() {
        vec![para_text.to_string()]
    } else {
        result
    }
}

/// Chunk a sequence of pre-extracted page markdown tuples `(page_num, markdown)`.
pub fn chunk_pages(mut pages: Vec<(u32, String)>, mut options: ChunkOptions) -> ChunkResult {
    let start_time = std::time::Instant::now();
    options.sanitize();

    if options.strip_boilerplate {
        strip_boilerplate_from_pages(&mut pages);
    }

    let mut all_blocks = Vec::new();
    for (page_num, text) in &pages {
        if text.trim().is_empty() {
            continue;
        }
        let blocks = parse_page_blocks(*page_num, text);
        all_blocks.extend(blocks);
    }

    if all_blocks.is_empty() {
        return ChunkResult {
            total_chunks: 0,
            total_tokens: 0,
            chunks: Vec::new(),
            execution_time_ms: start_time.elapsed().as_secs_f64() * 1000.0,
        };
    }

    let mut chunks = Vec::new();
    let mut current_items: Vec<(u32, String)> = Vec::new();
    let mut current_tokens: usize = 0;
    let mut current_headings: Vec<String> = Vec::new();
    let mut active_headings: Vec<String> = Vec::new();
    let mut previous_chunk_text: Option<String> = None;
    let mut previous_chunk_was_heading_break = false;

    let flush_current_chunk = |chunks: &mut Vec<DocumentChunk>,
                               items: &mut Vec<(u32, String)>,
                               tokens: &mut usize,
                               headings: &Vec<String>,
                               prev_text: &mut Option<String>,
                               was_heading: &mut bool,
                               is_heading_break: bool| {
        if items.is_empty() {
            return;
        }
        let text = items
            .iter()
            .map(|(_, t)| t.as_str())
            .collect::<Vec<_>>()
            .join("\n\n");
        let page_start = items.first().unwrap().0;
        let page_end = items.last().unwrap().0;
        let est = estimate_tokens(&text);
        let saved_text = text.clone();
        chunks.push(DocumentChunk {
            chunk_index: chunks.len(),
            page_start,
            page_end,
            estimated_tokens: est,
            text,
            headings_context: headings.clone(),
        });
        *prev_text = Some(saved_text);
        *was_heading = is_heading_break;
        items.clear();
        *tokens = 0;
    };

    for block in all_blocks {
        match &block.block_type {
            BlockType::Heading { level, title } => {
                let is_hard_break = options.break_on_headings && *level <= 2;
                if is_hard_break && !current_items.is_empty() {
                    flush_current_chunk(
                        &mut chunks,
                        &mut current_items,
                        &mut current_tokens,
                        &current_headings,
                        &mut previous_chunk_text,
                        &mut previous_chunk_was_heading_break,
                        true,
                    );
                }

                update_headings_context(&mut active_headings, *level, title);

                if current_items.is_empty() {
                    current_headings = active_headings.clone();
                    current_items.push((block.page, block.text.clone()));
                    current_tokens = block.tokens;
                } else if current_tokens + block.tokens <= options.target_tokens {
                    current_items.push((block.page, block.text.clone()));
                    current_tokens += block.tokens;
                } else {
                    flush_current_chunk(
                        &mut chunks,
                        &mut current_items,
                        &mut current_tokens,
                        &current_headings,
                        &mut previous_chunk_text,
                        &mut previous_chunk_was_heading_break,
                        false,
                    );
                    current_headings = active_headings.clone();
                    current_items.push((block.page, block.text.clone()));
                    current_tokens = block.tokens;
                }
            }
            BlockType::Table => {
                if current_items.is_empty() {
                    current_headings = active_headings.clone();
                    let mut overlap_text = None;
                    if !previous_chunk_was_heading_break && options.overlap_tokens > 0 {
                        if let Some(ref prev) = previous_chunk_text {
                            overlap_text = get_overlap_text(prev, options.overlap_tokens);
                        }
                    }

                    if block.tokens > options.target_tokens {
                        let sub_chunks =
                            split_table_to_chunks(&block.text, options.target_tokens);
                        for sub in sub_chunks {
                            let mut chunk_text = sub;
                            if let Some(ref ov) = overlap_text {
                                chunk_text = format!("{}\n\n{}", ov, chunk_text);
                                overlap_text = None;
                            }
                            let est = estimate_tokens(&chunk_text);
                            let saved_text = chunk_text.clone();
                            chunks.push(DocumentChunk {
                                chunk_index: chunks.len(),
                                page_start: block.page,
                                page_end: block.page,
                                estimated_tokens: est,
                                text: chunk_text,
                                headings_context: current_headings.clone(),
                            });
                            previous_chunk_text = Some(saved_text);
                            previous_chunk_was_heading_break = false;
                        }
                    } else {
                        if let Some(ov) = overlap_text {
                            current_items.push((block.page, ov));
                        }
                        current_items.push((block.page, block.text.clone()));
                        current_tokens =
                            current_items.iter().map(|(_, t)| estimate_tokens(t)).sum();
                    }
                } else if current_tokens + block.tokens <= options.target_tokens {
                    current_items.push((block.page, block.text.clone()));
                    current_tokens += block.tokens;
                } else {
                    flush_current_chunk(
                        &mut chunks,
                        &mut current_items,
                        &mut current_tokens,
                        &current_headings,
                        &mut previous_chunk_text,
                        &mut previous_chunk_was_heading_break,
                        false,
                    );

                    current_headings = active_headings.clone();
                    let mut overlap_text = None;
                    if !previous_chunk_was_heading_break && options.overlap_tokens > 0 {
                        if let Some(ref prev) = previous_chunk_text {
                            overlap_text = get_overlap_text(prev, options.overlap_tokens);
                        }
                    }

                    if block.tokens > options.target_tokens {
                        let sub_chunks =
                            split_table_to_chunks(&block.text, options.target_tokens);
                        for sub in sub_chunks {
                            let mut chunk_text = sub;
                            if let Some(ref ov) = overlap_text {
                                chunk_text = format!("{}\n\n{}", ov, chunk_text);
                                overlap_text = None;
                            }
                            let est = estimate_tokens(&chunk_text);
                            let saved_text = chunk_text.clone();
                            chunks.push(DocumentChunk {
                                chunk_index: chunks.len(),
                                page_start: block.page,
                                page_end: block.page,
                                estimated_tokens: est,
                                text: chunk_text,
                                headings_context: current_headings.clone(),
                            });
                            previous_chunk_text = Some(saved_text);
                            previous_chunk_was_heading_break = false;
                        }
                    } else {
                        if let Some(ov) = overlap_text {
                            current_items.push((block.page, ov));
                        }
                        current_items.push((block.page, block.text.clone()));
                        current_tokens =
                            current_items.iter().map(|(_, t)| estimate_tokens(t)).sum();
                    }
                }
            }
            BlockType::Paragraph => {
                if current_items.is_empty() {
                    current_headings = active_headings.clone();
                    let mut overlap_text = None;
                    if !previous_chunk_was_heading_break && options.overlap_tokens > 0 {
                        if let Some(ref prev) = previous_chunk_text {
                            overlap_text = get_overlap_text(prev, options.overlap_tokens);
                        }
                    }

                    if block.tokens > options.target_tokens {
                        let sub_chunks =
                            split_paragraph_to_chunks(&block.text, options.target_tokens);
                        for sub in sub_chunks {
                            let mut chunk_text = sub;
                            if let Some(ref ov) = overlap_text {
                                chunk_text = format!("{}\n\n{}", ov, chunk_text);
                                overlap_text = None;
                            }
                            let est = estimate_tokens(&chunk_text);
                            let saved_text = chunk_text.clone();
                            chunks.push(DocumentChunk {
                                chunk_index: chunks.len(),
                                page_start: block.page,
                                page_end: block.page,
                                estimated_tokens: est,
                                text: chunk_text,
                                headings_context: current_headings.clone(),
                            });
                            previous_chunk_text = Some(saved_text);
                            previous_chunk_was_heading_break = false;
                        }
                    } else {
                        if let Some(ov) = overlap_text {
                            current_items.push((block.page, ov));
                        }
                        current_items.push((block.page, block.text.clone()));
                        current_tokens =
                            current_items.iter().map(|(_, t)| estimate_tokens(t)).sum();
                    }
                } else if current_tokens + block.tokens <= options.target_tokens {
                    current_items.push((block.page, block.text.clone()));
                    current_tokens += block.tokens;
                } else {
                    flush_current_chunk(
                        &mut chunks,
                        &mut current_items,
                        &mut current_tokens,
                        &current_headings,
                        &mut previous_chunk_text,
                        &mut previous_chunk_was_heading_break,
                        false,
                    );

                    current_headings = active_headings.clone();
                    let mut overlap_text = None;
                    if !previous_chunk_was_heading_break && options.overlap_tokens > 0 {
                        if let Some(ref prev) = previous_chunk_text {
                            overlap_text = get_overlap_text(prev, options.overlap_tokens);
                        }
                    }

                    if block.tokens > options.target_tokens {
                        let sub_chunks =
                            split_paragraph_to_chunks(&block.text, options.target_tokens);
                        for sub in sub_chunks {
                            let mut chunk_text = sub;
                            if let Some(ref ov) = overlap_text {
                                chunk_text = format!("{}\n\n{}", ov, chunk_text);
                                overlap_text = None;
                            }
                            let est = estimate_tokens(&chunk_text);
                            let saved_text = chunk_text.clone();
                            chunks.push(DocumentChunk {
                                chunk_index: chunks.len(),
                                page_start: block.page,
                                page_end: block.page,
                                estimated_tokens: est,
                                text: chunk_text,
                                headings_context: current_headings.clone(),
                            });
                            previous_chunk_text = Some(saved_text);
                            previous_chunk_was_heading_break = false;
                        }
                    } else {
                        if let Some(ov) = overlap_text {
                            current_items.push((block.page, ov));
                        }
                        current_items.push((block.page, block.text.clone()));
                        current_tokens =
                            current_items.iter().map(|(_, t)| estimate_tokens(t)).sum();
                    }
                }
            }
        }
    }

    if !current_items.is_empty() {
        flush_current_chunk(
            &mut chunks,
            &mut current_items,
            &mut current_tokens,
            &current_headings,
            &mut previous_chunk_text,
            &mut previous_chunk_was_heading_break,
            false,
        );
    }

    let total_chunks = chunks.len();
    let total_tokens = chunks.iter().map(|c| c.estimated_tokens).sum();
    let execution_time_ms = start_time.elapsed().as_secs_f64() * 1000.0;

    ChunkResult {
        total_chunks,
        total_tokens,
        chunks,
        execution_time_ms,
    }
}

/// Convenience helper to chunk a single markdown string.
pub fn chunk_markdown(markdown: &str, options: ChunkOptions) -> ChunkResult {
    chunk_pages(vec![(1, markdown.to_string())], options)
}

/// Extract pages markdown from raw PDF bytes.
fn extract_pages_markdown(bytes: &[u8]) -> Result<Vec<(u32, String)>, PdfError> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }

    let mut pages = Vec::new();
    if let Ok(mut pdf) = pdf_oxide::api::Pdf::from_bytes(bytes.to_vec()) {
        if let Ok(page_count) = pdf.page_count() {
            for i in 0..page_count {
                if let Ok(md) = pdf.to_markdown(i) {
                    pages.push((i as u32 + 1, md));
                }
            }
        }
    }

    if pages.is_empty() {
        let doc = lopdf::Document::load_mem(bytes)
            .map_err(|e| PdfError::ParseError(format!("Failed to parse PDF document: {}", e)))?;
        let page_dict = doc.get_pages();
        let mut sorted: Vec<u32> = page_dict.keys().copied().collect();
        sorted.sort_unstable();
        for p in sorted {
            let text = doc.extract_text(&[p]).unwrap_or_default();
            pages.push((p, text));
        }
    }

    Ok(pages)
}

pub struct PdfChunkOperation {
    pub options: ChunkOptions,
    pub result: Arc<Mutex<Option<ChunkResult>>>,
}

impl Default for PdfChunkOperation {
    fn default() -> Self {
        Self::new(ChunkOptions::default())
    }
}

impl PdfChunkOperation {
    pub fn new(options: ChunkOptions) -> Self {
        Self {
            options,
            result: Arc::new(Mutex::new(None)),
        }
    }

    /// Execute semantic chunking directly on a [`DocumentSource`].
    pub fn chunk_source(
        source: &DocumentSource,
        options: ChunkOptions,
    ) -> Result<ChunkResult, PdfError> {
        Self::chunk_bytes(source.as_ref(), options)
    }

    /// Execute semantic chunking on raw PDF bytes.
    pub fn chunk_bytes(bytes: &[u8], options: ChunkOptions) -> Result<ChunkResult, PdfError> {
        if bytes.is_empty() {
            return Ok(ChunkResult {
                total_chunks: 0,
                total_tokens: 0,
                chunks: Vec::new(),
                execution_time_ms: 0.0,
            });
        }
        let pages = extract_pages_markdown(bytes)?;
        Ok(chunk_pages(pages, options))
    }

    /// Open a document and execute semantic chunking.
    pub fn chunk_path<P: AsRef<std::path::Path>>(
        path: P,
        options: ChunkOptions,
    ) -> Result<ChunkResult, PdfError> {
        let source = DocumentSource::open(path, None).map_err(PdfError::IoError)?;
        Self::chunk_source(&source, options)
    }
}

impl PdfOperation for PdfChunkOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let bytes = lopdf_doc.original_bytes.as_deref().unwrap_or(&[]);
        let result = Self::chunk_bytes(bytes, self.options.clone())?;
        if let Ok(mut lock) = self.result.lock() {
            *lock = Some(result);
        }
        Ok(())
    }
}

/// Convenience standalone function returning serialized JSON bytes.
pub fn pdf_chunk_bytes(bytes: &[u8], options: &ChunkOptions) -> Result<Vec<u8>, PdfError> {
    let result = PdfChunkOperation::chunk_bytes(bytes, options.clone())?;
    serde_json::to_vec_pretty(&result).map_err(|e| PdfError::Other(e.to_string()))
}

pub fn chunk_pdf_bytes(bytes: &[u8], options: ChunkOptions) -> Result<ChunkResult, PdfError> {
    PdfChunkOperation::chunk_bytes(bytes, options)
}

pub fn chunk_pdf_source(
    source: &DocumentSource,
    options: ChunkOptions,
) -> Result<ChunkResult, PdfError> {
    PdfChunkOperation::chunk_source(source, options)
}

pub fn chunk_pdf_path<P: AsRef<std::path::Path>>(
    path: P,
    options: ChunkOptions,
) -> Result<ChunkResult, PdfError> {
    PdfChunkOperation::chunk_path(path, options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_tokens() {
        assert_eq!(estimate_tokens(""), 0);
        assert_eq!(estimate_tokens("   "), 0);
        assert_eq!(estimate_tokens("hello"), 2); // 5 chars -> 8/4 = 2
        assert_eq!(estimate_tokens("The quick brown fox jumps over the lazy dog."), 11);
    }

    #[test]
    fn test_sentence_split() {
        let text = "Hello world! This is a test. Are you sure? Yes, absolutely.";
        let sentences = split_into_sentences(text);
        assert_eq!(sentences.len(), 4);
        assert_eq!(sentences[0], "Hello world!");
        assert_eq!(sentences[1], "This is a test.");
        assert_eq!(sentences[2], "Are you sure?");
        assert_eq!(sentences[3], "Yes, absolutely.");
    }

    #[test]
    fn test_overlap_extraction() {
        let text = "Sentence one. Sentence two. Sentence three.";
        let overlap = get_overlap_text(text, 10);
        assert!(overlap.is_some());
        let ov_str = overlap.unwrap();
        assert!(ov_str.contains("Sentence three."));
    }

    #[test]
    fn test_empty_document() {
        let options = ChunkOptions::default();
        let res = chunk_pages(vec![], options);
        assert_eq!(res.total_chunks, 0);
        assert_eq!(res.total_tokens, 0);
    }

    #[test]
    fn test_heading_preservation_and_break() {
        let md = "# Title 1\n\nSection 1 text here.\n\n# Title 2\n\nSection 2 text here.";
        let options = ChunkOptions {
            target_tokens: 500,
            overlap_tokens: 50,
            strip_boilerplate: true,
            break_on_headings: true,
        };
        let res = chunk_markdown(md, options);
        assert_eq!(res.total_chunks, 2);
        assert_eq!(res.chunks[0].headings_context, vec!["Title 1"]);
        assert_eq!(res.chunks[1].headings_context, vec!["Title 2"]);
    }

    #[test]
    fn test_table_preservation() {
        let md = "Some text before.\n\n| Col1 | Col2 |\n| --- | --- |\n| Val1 | Val2 |\n| Val3 | Val4 |\n\nSome text after.";
        let options = ChunkOptions {
            target_tokens: 500,
            overlap_tokens: 50,
            strip_boilerplate: true,
            break_on_headings: true,
        };
        let res = chunk_markdown(md, options);
        assert_eq!(res.total_chunks, 1);
        assert!(res.chunks[0].text.contains("| Col1 | Col2 |"));
        assert!(res.chunks[0].text.contains("| Val3 | Val4 |"));
    }

    #[test]
    fn test_boilerplate_suppression() {
        let pages = vec![
            (1, "Header Notice\n\nReal content on page 1.\n\nFooter 2026".to_string()),
            (2, "Header Notice\n\nReal content on page 2.\n\nFooter 2026".to_string()),
            (3, "Header Notice\n\nReal content on page 3.\n\nFooter 2026".to_string()),
        ];
        let options = ChunkOptions {
            target_tokens: 500,
            overlap_tokens: 50,
            strip_boilerplate: true,
            break_on_headings: true,
        };
        let res = chunk_pages(pages, options);
        for chunk in &res.chunks {
            assert!(!chunk.text.contains("Header Notice"));
            assert!(!chunk.text.contains("Footer 2026"));
            assert!(chunk.text.contains("Real content"));
        }
    }
}
