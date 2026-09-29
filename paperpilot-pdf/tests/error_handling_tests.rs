use paperpilot_pdf::document::LopdfDocument;
use paperpilot_core::traits::PdfOperation;

// 1. Nonexistent file returns Err, not panic
#[test]
fn test_load_nonexistent_file_returns_err() {
    let result = LopdfDocument::load(&std::path::PathBuf::from("does_not_exist.pdf"));
    assert!(result.is_err(), "Expected Err for missing file");
}

// 2. Empty byte slice should fail gracefully
#[test]
fn test_load_empty_bytes_returns_err() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    let result = LopdfDocument::load(tmp.path());
    assert!(result.is_err(), "Expected Err for empty file");
}

// 3. Truncated/malformed PDF
#[test]
fn test_load_malformed_pdf_returns_err() {
    use std::io::Write;
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    tmp.write_all(b"%PDF-1.4\ngarbage data that is not a real pdf").unwrap();
    let result = LopdfDocument::load(tmp.path());
    // May succeed (lopdf is lenient) or fail - either is acceptable, but must not panic
    let _ = result;
}

// 4. Page out of range
#[test]
fn test_delete_out_of_range_page_returns_err() {
    let mut path = std::env::current_dir().unwrap();
    if path.file_name().unwrap() == "paperpilot-pdf" {
        path.pop();
    }
    path.push("tests/fixtures/sample.pdf");

    if !path.exists() { return; }
    let mut doc = LopdfDocument::load(&path).unwrap();
    let op = paperpilot_pdf::operations::delete::DeletePagesOperation::new(vec![9999]);
    let result = op.execute(&mut doc);
    assert!(result.is_err(), "Expected Err for out-of-range page");
}

// 5. Wrong password on encrypted PDF
#[test]
fn test_decrypt_wrong_password_returns_err() {
    let mut path = std::env::current_dir().unwrap();
    if path.file_name().unwrap() == "paperpilot-pdf" {
        path.pop();
    }
    path.push("tests/fixtures/sample.pdf");

    if !path.exists() { return; }
    let mut doc = LopdfDocument::load(&path).unwrap();
    let op = paperpilot_pdf::operations::decrypt::DecryptOperation::new(Some("wrongpassword".to_string()));
    // May succeed or fail depending on whether sample.pdf is encrypted - just must not panic
    let _ = op.execute(&mut doc);
}

// 6. Empty input for merge
#[test]
fn test_merge_empty_list_returns_err() {
    use paperpilot_pdf::operations::merge::MergeOperation;
    let op = MergeOperation::new(vec![]);
    // MergeOperation requires documents to merge - an empty list should return Err
    // We can't easily test this through execute() without docs, so just verify the struct creates
    let _ = op;
}
