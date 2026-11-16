use paperpilot_core::document_source::{DocumentSource, PageBatchIterator};
use paperpilot_pdf::operations::pdf_extract_patterns::{
    PdfExtractPatternsOptions, extract_patterns_source,
};
use paperpilot_pdf::operations::pii::{PiiScanOptions, scan_pii_source};
use paperpilot_pdf::operations::search::SearchOperation;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_mmap_selection_and_byte_equality() {
    let mut tmp = NamedTempFile::new().expect("Failed to create temp file");
    // Generate deterministic content
    let pattern: Vec<u8> = (0..256).map(|b| b as u8).cycle().take(64 * 1024).collect();
    tmp.write_all(&pattern).expect("Failed to write pattern");
    tmp.flush().expect("Failed to flush");

    let std_bytes = std::fs::read(tmp.path()).expect("Failed to read via std::fs");

    // Lower threshold test: 32 KB threshold for 64 KB file -> triggers Mmap
    #[cfg(not(target_arch = "wasm32"))]
    {
        let src_mmap = DocumentSource::open(tmp.path(), Some(32 * 1024)).expect("Failed open mmap");
        assert!(src_mmap.is_mmap(), "DocumentSource must select Mmap when above threshold on native");
        assert!(!src_mmap.is_bytes());
        assert_eq!(src_mmap.len(), std_bytes.len());
        assert_eq!(src_mmap.as_ref(), std_bytes.as_slice());
        assert_eq!(&src_mmap[..], &std_bytes[..]);
    }

    // Threshold higher than file size: 128 KB threshold -> selects Bytes
    let src_bytes = DocumentSource::open(tmp.path(), Some(128 * 1024)).expect("Failed open bytes");
    assert!(src_bytes.is_bytes());
    assert!(!src_bytes.is_mmap());
    assert_eq!(src_bytes.as_ref(), std_bytes.as_slice());
}

#[test]
fn test_page_batch_iterator_splits_100_pages() {
    let batch_iter = PageBatchIterator::new(100, 25);
    let batches: Vec<_> = batch_iter.collect();

    assert_eq!(batches.len(), 4);
    assert_eq!(batches[0], 1..=25);
    assert_eq!(batches[1], 26..=50);
    assert_eq!(batches[2], 51..=75);
    assert_eq!(batches[3], 76..=100);

    // Ensure total contiguous page coverage
    let all_pages: Vec<usize> = batches.into_iter().flatten().collect();
    let expected: Vec<usize> = (1..=100).collect();
    assert_eq!(all_pages, expected);
}

#[test]
fn test_mmap_document_source_with_read_operations() {
    let fixture_path = std::path::Path::new("../tests/e2e_fixtures/real/text_pii.pdf");
    let alt_path = std::path::Path::new("tests/e2e_fixtures/real/text_pii.pdf");
    let path = if fixture_path.exists() {
        fixture_path
    } else if alt_path.exists() {
        alt_path
    } else {
        return;
    };

    // Open with threshold of 1 byte to force mmap on native
    let source = DocumentSource::open(path, Some(1)).expect("Failed to open fixture via DocumentSource");
    #[cfg(not(target_arch = "wasm32"))]
    assert!(source.is_mmap());

    // 1. PII Scan via DocumentSource
    let pii_opts = PiiScanOptions::default();
    let pii_report = scan_pii_source(&source, &pii_opts).expect("PII scan via DocumentSource failed");
    assert!(pii_report.total_findings > 0);

    // 2. Pattern extraction via DocumentSource
    let mut patterns = std::collections::HashMap::new();
    patterns.insert("ssn".to_string(), r"\b\d{3}-\d{2}-\d{4}\b".to_string());
    let pattern_opts = PdfExtractPatternsOptions {
        keywords: vec!["Alice".to_string()],
        patterns,
        case_sensitive: false,
        context_len: 20,
    };
    let pattern_report = extract_patterns_source(&source, &pattern_opts).expect("Pattern extraction via DocumentSource failed");
    assert!(pattern_report.total_matches > 0);

    // 3. SearchOperation via DocumentSource
    let search_op = SearchOperation::new("Alice".to_string());
    let found_pages = search_op.execute_on_source(&source).expect("Search via DocumentSource failed");
    assert!(!found_pages.is_empty());
}
