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
                });
                *matches_by_label.entry(label.clone()).or_insert(0) += 1;
            }
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
}
