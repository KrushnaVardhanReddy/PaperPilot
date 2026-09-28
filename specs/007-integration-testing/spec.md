# Phase 1.4: Comprehensive Integration Testing

## Objective
Ensure 100% test coverage for all 29 implemented PDF operations using real-world PDF files (`tests/fixtures/simple.pdf` and `multi_page.pdf`). The tests must load real files, mutate them, write to a temp directory, and assert the output is valid.

## Architecture

Tests will be placed in `paperpilot-pdf/tests/fixture_tests.rs`.

Each test should follow this pattern:
```rust
#[test]
fn test_integration_merge() {
    let temp_dir = tempfile::tempdir().unwrap();
    let out_path = temp_dir.path().join("out.pdf");
    
    // 1. Load fixtures
    let mut doc1 = LopdfDocument::load("tests/fixtures/simple.pdf").unwrap();
    let mut doc2 = LopdfDocument::load("tests/fixtures/multi_page.pdf").unwrap();
    
    // 2. Execute operation
    let op = MergeOperation::new(vec![&mut doc2]);
    op.execute(&mut doc1).unwrap();
    
    // 3. Save
    doc1.save(&out_path).unwrap();
    
    // 4. Validate output
    let result = LopdfDocument::load(&out_path).unwrap();
    assert_eq!(result.inner.get_pages().len(), 4); // 1 page + 3 pages
}
```

## Batching
- **Batch 1 (Group A):** merge, split, extract, delete, reorder, rotate, crop, burst.
- **Batch 2 (Group B):** compress, repair, linearize, encrypt, decrypt, watermark, redact, metadata, signature, flatten, pdf-a, header_footer.
- **Batch 3 (Group C):** extract_text, extract_images, images_to_pdf, search, compare, bookmarks, render, ocr, bates.

**NOTE:** Stubs (like OCR, Render, PDF/A) should be tested to ensure they safely return `OperationResult::Err(PdfError::UnsupportedOperation(...))` rather than panicking.
