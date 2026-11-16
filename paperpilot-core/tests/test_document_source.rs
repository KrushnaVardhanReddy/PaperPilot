use paperpilot_core::document_source::{
    DEFAULT_MMAP_THRESHOLD_BYTES, DocumentSource, PageBatchIterator,
};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_document_source_threshold_mmap_vs_read() {
    let mut tmp = NamedTempFile::new().expect("Failed to create temp file");
    // Generate deterministic content
    let pattern: Vec<u8> = (0..256).map(|b| b as u8).cycle().take(64 * 1024).collect();
    tmp.write_all(&pattern).expect("Failed to write pattern");
    tmp.flush().expect("Failed to flush");

    let std_bytes = std::fs::read(tmp.path()).expect("Failed to read via std::fs");

    // 1. Below threshold test
    let src_bytes = DocumentSource::open(tmp.path(), Some(128 * 1024)).expect("Failed open bytes");
    assert_eq!(src_bytes.as_ref(), std_bytes.as_slice());
    assert_eq!(src_bytes.len(), 64 * 1024);
    assert!(src_bytes.is_bytes());
    assert!(!src_bytes.is_mmap());

    // 2. Above threshold test (on native target)
    #[cfg(not(target_arch = "wasm32"))]
    {
        let src_mmap = DocumentSource::open(tmp.path(), Some(32 * 1024)).expect("Failed open mmap");
        assert_eq!(src_mmap.as_ref(), std_bytes.as_slice());
        assert_eq!(src_mmap.len(), 64 * 1024);
        assert!(src_mmap.is_mmap());
        assert!(!src_mmap.is_bytes());

        // Byte content equality across byte-by-byte dereference
        assert_eq!(&src_mmap[..], &std_bytes[..]);
        assert_eq!(&src_mmap[100..200], &std_bytes[100..200]);
    }
}

#[test]
fn test_document_source_default_threshold() {
    let mut tmp = NamedTempFile::new().expect("Failed to create temp file");
    let sample = b"%PDF-1.7\nSample test document header\n%%EOF";
    tmp.write_all(sample).expect("Failed to write");
    tmp.flush().expect("Failed to flush");

    let src = DocumentSource::open(tmp.path(), None).expect("Failed open");
    assert_eq!(src.as_ref(), sample);
    // Since file is < 10 MB, should be Bytes
    assert!(src.is_bytes());
    assert_eq!(DEFAULT_MMAP_THRESHOLD_BYTES, 10 * 1024 * 1024);
}

#[test]
fn test_page_batch_iterator_100_pages() {
    let batch_iter = PageBatchIterator::new(100, 25);
    let batches: Vec<_> = batch_iter.collect();

    assert_eq!(batches.len(), 4);
    assert_eq!(batches[0], 1..=25);
    assert_eq!(batches[1], 26..=50);
    assert_eq!(batches[2], 51..=75);
    assert_eq!(batches[3], 76..=100);

    // Verify all 100 pages are covered contiguously without gaps or overlaps
    let mut visited_pages = Vec::new();
    for batch in batches {
        for page in batch {
            visited_pages.push(page);
        }
    }
    let expected: Vec<usize> = (1..=100).collect();
    assert_eq!(visited_pages, expected);
}

#[test]
fn test_page_batch_iterator_various_splits() {
    // 73 pages in batches of 20 -> 20, 20, 20, 13 (4 batches)
    let batches: Vec<_> = PageBatchIterator::new(73, 20).collect();
    assert_eq!(batches.len(), 4);
    assert_eq!(batches[0], 1..=20);
    assert_eq!(batches[1], 21..=40);
    assert_eq!(batches[2], 41..=60);
    assert_eq!(batches[3], 61..=73);

    // Single page document
    let single: Vec<_> = PageBatchIterator::new(1, 10).collect();
    assert_eq!(single, vec![1..=1]);
}
