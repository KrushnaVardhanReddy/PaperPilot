use lopdf::Document;
use paperpilot_core::document_source::DocumentSource;
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfExtractPatternsOptions {
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub patterns: HashMap<String, String>, // label -> regex string
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default = "default_context_len")]
    pub context_len: usize,
}

fn default_context_len() -> usize {
    30
}

impl Default for PdfExtractPatternsOptions {
    fn default() -> Self {
        Self {
            keywords: Vec::new(),
            patterns: HashMap::new(),
            case_sensitive: false,
            context_len: default_context_len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatternFinding {
    pub label: String,
    pub pattern_type: String, // "keyword" | "regex"
    pub value: String,
    pub page: u32,
    pub context_snippet: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rect: Option<[f64; 4]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatchedPattern {
    pub page: u32,
    pub text: String,
    pub rect: [f64; 4], // [x, y, width, height]
    pub line_text: String,
}

pub fn filter_occurrences<T>(items: Vec<T>, occurrence: &str) -> Vec<T> {
    if items.is_empty() {
        return items;
    }
    let occ = occurrence.trim().to_lowercase();
    match occ.as_str() {
        "all" | "" => items,
        "first" => vec![items.into_iter().next().unwrap()],
        "last" => vec![items.into_iter().last().unwrap()],
        other => {
            let indices: Vec<usize> = other
                .trim_matches(|c| c == '[' || c == ']')
                .split(',')
                .filter_map(|s| s.trim().parse::<usize>().ok())
                .collect();
            if indices.is_empty() {
                items
            } else {
                items
                    .into_iter()
                    .enumerate()
                    .filter(|(idx, _)| indices.contains(idx))
                    .map(|(_, item)| item)
                    .collect()
            }
        }
    }
}

fn compute_match_bbox(line: &pdf_oxide::layout::TextLine, byte_start: usize, byte_end: usize) -> [f64; 4] {
    let matched_text = &line.text[byte_start..byte_end];

    // Priority 1: Check if the match corresponds to complete words or consecutive words in line.words
    let mut matched_words_bbox: Vec<&pdf_oxide::layout::TextBlock> = Vec::new();
    let mut current_offset = 0;
    for w in &line.words {
        // Find position of this word in line.text starting from current_offset
        if let Some(pos) = line.text[current_offset..].find(&w.text) {
            let word_start = current_offset + pos;
            let word_end = word_start + w.text.len();
            current_offset = word_end;

            // Check if this word overlaps with [byte_start, byte_end]
            if word_start < byte_end && word_end > byte_start {
                matched_words_bbox.push(w);
            }
        }
    }

    if !matched_words_bbox.is_empty() {
        // If it's a sub-word match within a single word, refine using word's chars
        if matched_words_bbox.len() == 1 {
            let single_word = matched_words_bbox[0];
            let word_text = &single_word.text;
            if let Some(sub_start) = word_text.find(matched_text) {
                let sub_char_start = word_text[..sub_start].chars().count();
                let sub_char_len = matched_text.chars().count();
                let sub_char_end = sub_char_start + sub_char_len;
                if sub_char_start < single_word.chars.len() {
                    let chars_slice = &single_word.chars[sub_char_start..sub_char_end.min(single_word.chars.len())];
                    if !chars_slice.is_empty() {
                        let min_x = chars_slice.iter().map(|c| c.bbox.x as f64).fold(f64::INFINITY, f64::min);
                        let min_y = chars_slice.iter().map(|c| c.bbox.y as f64).fold(f64::INFINITY, f64::min);
                        let max_x = chars_slice.iter().map(|c| (c.bbox.x + c.bbox.width) as f64).fold(f64::NEG_INFINITY, f64::max);
                        let max_y = chars_slice.iter().map(|c| (c.bbox.y + c.bbox.height) as f64).fold(f64::NEG_INFINITY, f64::max);
                        let w = (max_x - min_x).max(1.0);
                        let h = (max_y - min_y).max(1.0);
                        if min_x.is_finite() && min_y.is_finite() {
                            return [min_x, min_y, w, h];
                        }
                    }
                }
            }
        }

        let min_x = matched_words_bbox.iter().map(|b| b.bbox.x as f64).fold(f64::INFINITY, f64::min);
        let min_y = matched_words_bbox.iter().map(|b| b.bbox.y as f64).fold(f64::INFINITY, f64::min);
        let max_x = matched_words_bbox.iter().map(|b| (b.bbox.x + b.bbox.width) as f64).fold(f64::NEG_INFINITY, f64::max);
        let max_y = matched_words_bbox.iter().map(|b| (b.bbox.y + b.bbox.height) as f64).fold(f64::NEG_INFINITY, f64::max);
        let w = (max_x - min_x).max(1.0);
        let h = (max_y - min_y).max(1.0);
        if min_x.is_finite() && min_y.is_finite() {
            return [min_x, min_y, w, h];
        }
    }

    let char_start = line.text[..byte_start].chars().count();
    let char_len = line.text[byte_start..byte_end].chars().count();
    let char_end = char_start + char_len;
    let total_chars = line.text.chars().count().max(1) as f64;
    let start_ratio = char_start as f64 / total_chars;
    let end_ratio = char_end as f64 / total_chars;
    let x = line.bbox.x as f64 + start_ratio * line.bbox.width as f64;
    let w = ((end_ratio - start_ratio) * line.bbox.width as f64).max(5.0);
    let y = line.bbox.y as f64;
    let h = (line.bbox.height as f64).max(10.0);
    [x, y, w, h]
}

pub fn find_text_matches(
    pdf_bytes: &[u8],
    search: Option<&str>,
    regex_pattern: Option<&str>,
    case_sensitive: bool,
    pages_filter: Option<&[u32]>,
) -> Vec<MatchedPattern> {
    let mut results = Vec::new();
    if pdf_bytes.is_empty() {
        return results;
    }
    let mut pdf = match pdf_oxide::api::Pdf::from_bytes(pdf_bytes.to_vec()) {
        Ok(p) => p,
        Err(_) => return results,
    };
    let page_count = match pdf.page_count() {
        Ok(c) => c,
        Err(_) => return results,
    };

    let compiled_regex = if let Some(re_str) = regex_pattern {
        if re_str.trim().is_empty() {
            None
        } else {
            regex::RegexBuilder::new(re_str)
                .case_insensitive(!case_sensitive)
                .build()
                .ok()
        }
    } else if let Some(kw) = search {
        if kw.trim().is_empty() {
            None
        } else {
            let escaped = regex::escape(kw);
            regex::RegexBuilder::new(&escaped)
                .case_insensitive(!case_sensitive)
                .build()
                .ok()
        }
    } else {
        None
    };

    let Some(regex) = compiled_regex else {
        return results;
    };

    for page_idx in 0..page_count {
        let page_num = (page_idx + 1) as u32;
        if let Some(target_pages) = pages_filter {
            if !target_pages.contains(&page_num) {
                continue;
            }
        }

        let lines = pdf.extract_text_lines(page_idx).unwrap_or_default();
        for line in &lines {
            if line.text.trim().is_empty() {
                continue;
            }
            for m in regex.find_iter(&line.text) {
                let matched_text = m.as_str().to_string();
                let match_start = m.start();
                let match_end = m.end();

                let rect = compute_match_bbox(line, match_start, match_end);
                results.push(MatchedPattern {
                    page: page_num,
                    text: matched_text,
                    rect,
                    line_text: line.text.clone(),
                });
            }
        }
    }

    results
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatternExtractionReport {
    pub total_matches: usize,
    pub matches_by_label: HashMap<String, usize>,
    pub findings: Vec<PatternFinding>,
    pub execution_time_ms: f64,
}

fn extract_context(text: &str, start: usize, end: usize, ctx_len: usize) -> String {
    let s = if start > ctx_len {
        let mut idx = start - ctx_len;
        while idx > 0 && !text.is_char_boundary(idx) {
            idx -= 1;
        }
        idx
    } else {
        0
    };
    let e = if end + ctx_len < text.len() {
        let mut idx = end + ctx_len;
        while idx < text.len() && !text.is_char_boundary(idx) {
            idx += 1;
        }
        idx
    } else {
        text.len()
    };
    text[s..e].replace('\n', " ")
}

pub fn extract_patterns(
    input_bytes: &[u8],
    options: &PdfExtractPatternsOptions,
) -> Result<PatternExtractionReport, PdfError> {
    let start_time = std::time::Instant::now();

    if input_bytes.is_empty() {
        return Err(PdfError::ParseError("Input document is empty".to_string()));
    }

    let doc = Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;

    // Precompile regexes
    let mut compiled_patterns = Vec::new();
    for (label, re_str) in &options.patterns {
        let regex = regex::RegexBuilder::new(re_str)
            .case_insensitive(!options.case_sensitive)
            .build()
            .map_err(|e| {
                PdfError::InvalidInput(format!(
                    "Invalid regex pattern for label '{}': {}",
                    label, e
                ))
            })?;
        compiled_patterns.push((label.clone(), regex));
    }

    let pages = doc.get_pages();
    let mut sorted_pages: Vec<u32> = pages.keys().copied().collect();
    sorted_pages.sort_unstable();

    let mut findings = Vec::new();
    let mut matches_by_label: HashMap<String, usize> = HashMap::new();

    for page_num in sorted_pages {
        let page_text = doc.extract_text(&[page_num]).unwrap_or_default();
        if page_text.trim().is_empty() {
            continue;
        }

        // 1. Match custom keywords (CRITICAL: find ALL occurrences on each page)
        for keyword in &options.keywords {
            if keyword.is_empty() {
                continue;
            }

            if options.case_sensitive {
                let kw_len = keyword.len();
                let mut search_start = 0;
                while search_start < page_text.len() {
                    if let Some(pos) = page_text[search_start..].find(keyword) {
                        let match_start = search_start + pos;
                        let match_end = match_start + kw_len;
                        let matched_str = &page_text[match_start..match_end];

                        findings.push(PatternFinding {
                            label: keyword.clone(),
                            pattern_type: "keyword".to_string(),
                            value: matched_str.to_string(),
                            page: page_num,
                            context_snippet: extract_context(
                                &page_text,
                                match_start,
                                match_end,
                                options.context_len,
                            ),
                            rect: None,
                        });
                        *matches_by_label.entry(keyword.clone()).or_insert(0) += 1;
                        search_start = match_end;
                    } else {
                        break;
                    }
                }
            } else {
                // Case-insensitive: construct escaped regex or search lowercased
                let escaped = regex::escape(keyword);
                if let Ok(kw_re) = regex::RegexBuilder::new(&escaped)
                    .case_insensitive(true)
                    .build()
                {
                    for m in kw_re.find_iter(&page_text) {
                        findings.push(PatternFinding {
                            label: keyword.clone(),
                            pattern_type: "keyword".to_string(),
                            value: m.as_str().to_string(),
                            page: page_num,
                            context_snippet: extract_context(
                                &page_text,
                                m.start(),
                                m.end(),
                                options.context_len,
                            ),
                            rect: None,
                        });
                        *matches_by_label.entry(keyword.clone()).or_insert(0) += 1;
                    }
                }
            }
        }

        // 2. Match custom regex patterns (CRITICAL: find ALL occurrences on each page)
        for (label, re) in &compiled_patterns {
            for m in re.find_iter(&page_text) {
                findings.push(PatternFinding {
                    label: label.clone(),
                    pattern_type: "regex".to_string(),
                    value: m.as_str().to_string(),
                    page: page_num,
                    context_snippet: extract_context(
                        &page_text,
                        m.start(),
                        m.end(),
                        options.context_len,
                    ),
                    rect: None,
                });
                *matches_by_label.entry(label.clone()).or_insert(0) += 1;
            }
        }
    }

    // Enrich findings with bounding coordinates from find_text_matches
    for finding in &mut findings {
        let matches = find_text_matches(
            input_bytes,
            Some(&finding.value),
            None,
            options.case_sensitive,
            Some(&[finding.page]),
        );
        if let Some(m) = matches.into_iter().next() {
            finding.rect = Some(m.rect);
        }
    }

    let duration = start_time.elapsed().as_secs_f64() * 1000.0;
    let total_matches = findings.len();

    Ok(PatternExtractionReport {
        total_matches,
        matches_by_label,
        findings,
        execution_time_ms: duration,
    })
}

pub fn pdf_extract_patterns_bytes(
    input_bytes: &[u8],
    options: &PdfExtractPatternsOptions,
) -> Result<Vec<u8>, PdfError> {
    let report = extract_patterns(input_bytes, options)?;
    serde_json::to_vec_pretty(&report).map_err(|e| PdfError::Other(e.to_string()))
}

/// Executes pattern extraction on a zero-copy [`DocumentSource`].
pub fn extract_patterns_source(
    source: &DocumentSource,
    options: &PdfExtractPatternsOptions,
) -> Result<PatternExtractionReport, PdfError> {
    extract_patterns(source.as_ref(), options)
}

/// Opens a document using memory-mapping if above threshold, and extracts patterns.
pub fn extract_patterns_path<P: AsRef<std::path::Path>>(
    path: P,
    options: &PdfExtractPatternsOptions,
) -> Result<PatternExtractionReport, PdfError> {
    let source = DocumentSource::open(path, None).map_err(PdfError::IoError)?;
    extract_patterns_source(&source, options)
}

/// Convenience helper to return serialized JSON bytes from a [`DocumentSource`].
pub fn pdf_extract_patterns_source(
    source: &DocumentSource,
    options: &PdfExtractPatternsOptions,
) -> Result<Vec<u8>, PdfError> {
    pdf_extract_patterns_bytes(source.as_ref(), options)
}

/// Convenience helper to return serialized JSON bytes directly from a file path using memory-mapping.
pub fn pdf_extract_patterns_path<P: AsRef<std::path::Path>>(
    path: P,
    options: &PdfExtractPatternsOptions,
) -> Result<Vec<u8>, PdfError> {
    let source = DocumentSource::open(path, None).map_err(PdfError::IoError)?;
    pdf_extract_patterns_source(&source, options)
}

#[derive(Default)]
pub struct PdfExtractPatternsOperation {
    pub options: PdfExtractPatternsOptions,
}

impl PdfExtractPatternsOperation {
    pub fn new(options: PdfExtractPatternsOptions) -> Self {
        Self { options }
    }
}

impl PdfOperation for PdfExtractPatternsOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> paperpilot_core::error::OperationResult<()> {
        Err(PdfError::UnsupportedOperation(
            "PdfExtractPatternsOperation is an extraction operation that outputs a JSON report, use pdf_extract_patterns_bytes or extract_patterns directly".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_extract_patterns_empty_document() {
        let options = PdfExtractPatternsOptions::default();
        let bytes = vec![];
        let result = pdf_extract_patterns_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_context_extraction_bounds() {
        let text = "Hello world from PaperPilot extraction engine!";
        // Match "world" (start: 6, end: 11)
        let ctx = extract_context(text, 6, 11, 10);
        assert!(ctx.contains("world"));
        assert!(ctx.contains("Hello"));
    }

    #[test]
    fn test_extract_patterns_with_real_pdf_fixture() {
        let pdf_path = std::path::Path::new("../tests/e2e_fixtures/real/text_pii.pdf");
        if !pdf_path.exists() {
            // Check current dir relative path if running from root
            let alt_path = std::path::Path::new("tests/e2e_fixtures/real/text_pii.pdf");
            if !alt_path.exists() {
                return;
            }
        }
        let file_path = if pdf_path.exists() { pdf_path } else { std::path::Path::new("tests/e2e_fixtures/real/text_pii.pdf") };
        let bytes = std::fs::read(file_path).expect("Failed to read fixture");

        let mut patterns = HashMap::new();
        patterns.insert("ssn".to_string(), r"\b\d{3}-\d{2}-\d{4}\b".to_string());
        patterns.insert("email".to_string(), r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}".to_string());

        let options = PdfExtractPatternsOptions {
            keywords: vec!["Alice".to_string(), "John".to_string()],
            patterns,
            case_sensitive: false,
            context_len: 25,
        };

        let report = extract_patterns(&bytes, &options).expect("Failed to extract patterns");
        assert!(report.total_matches > 0);
        assert!(report.matches_by_label.contains_key("ssn") || report.matches_by_label.contains_key("email"));
        for finding in &report.findings {
            assert!(finding.page >= 1);
            assert!(!finding.value.is_empty());
            assert!(!finding.context_snippet.is_empty());
        }

        // Test with DocumentSource
        let source = DocumentSource::from_bytes(bytes);
        let src_report = extract_patterns_source(&source, &options).expect("Failed via source");
        assert_eq!(src_report.total_matches, report.total_matches);
    }

    #[test]
    fn test_find_text_matches_and_filter_occurrences() {
        let pdf_path = std::path::Path::new("../ManaulTestingFixtures/pdf_redact/Input/sensitive_customer_records.pdf");
        let alt_path = std::path::Path::new("ManaulTestingFixtures/pdf_redact/Input/sensitive_customer_records.pdf");
        let p = if pdf_path.exists() { pdf_path } else { alt_path };
        if let Ok(bytes) = std::fs::read(p) {
            let matches = find_text_matches(&bytes, Some("123-45-6789"), None, false, None);
            assert!(!matches.is_empty());
            assert_eq!(matches[0].page, 1);
            assert!(matches[0].rect[2] > 0.0);
            assert!(matches[0].rect[3] > 0.0);

            let first = filter_occurrences(matches.clone(), "first");
            assert_eq!(first.len(), 1);

            let all = filter_occurrences(matches.clone(), "all");
            assert_eq!(all.len(), matches.len());
        }
    }
}
