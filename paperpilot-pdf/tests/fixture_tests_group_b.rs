
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;

// Import operations
use paperpilot_pdf::operations::compress::CompressOperation;
use paperpilot_pdf::operations::decrypt::DecryptOperation;
use paperpilot_pdf::operations::encrypt::EncryptOperation;
use paperpilot_pdf::operations::flatten::FlattenOperation;
use paperpilot_pdf::operations::header_footer::HeaderFooterOperation;
use paperpilot_pdf::operations::linearize::LinearizeOperation;
use paperpilot_pdf::operations::metadata::MetadataOperation;
use paperpilot_pdf::operations::pdf_a::PdfAConversionOperation;
use paperpilot_pdf::operations::redact::{BoundingBox, RedactOperation};
use paperpilot_pdf::operations::repair::RepairOperation;
use paperpilot_pdf::operations::signature::SignatureOperation;
use paperpilot_pdf::operations::watermark::WatermarkOperation;

use std::path::PathBuf;
use tempfile::tempdir;

fn get_simple_pdf_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    if path.ends_with("paperpilot-pdf") {
        path.pop();
    }
    path.join("tests/fixtures/simple.pdf")
}

#[test]
fn test_compress() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("compressed.pdf");

    let mut doc = LopdfDocument::load(&get_simple_pdf_path()).expect("Failed to load simple.pdf");
    let op = CompressOperation::new(None);

    assert!(op.execute(&mut doc).is_ok());

    assert!(doc.save(&output_path).is_ok());
    assert!(output_path.exists());

    // We can also verify it loads again
    let loaded = LopdfDocument::load(&output_path).unwrap();
    assert_eq!(loaded.page_count().unwrap(), 1);
}

#[test]
fn test_repair() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("repaired.pdf");

    let mut doc = LopdfDocument::load(&get_simple_pdf_path()).expect("Failed to load simple.pdf");
    let op = RepairOperation::new();

    assert!(op.execute(&mut doc).is_ok());
    assert!(doc.save(&output_path).is_ok());
    assert!(output_path.exists());
}

#[test]
fn test_linearize() {
    let mut doc = LopdfDocument::load(&get_simple_pdf_path()).expect("Failed to load simple.pdf");
    let op = LinearizeOperation::new();

    let res = op.execute(&mut doc);
    assert!(res.is_ok());
}

#[test]
fn test_encrypt() {
    let mut doc = LopdfDocument::load(&get_simple_pdf_path()).expect("Failed to load simple.pdf");
    let mut op = EncryptOperation::new();
    op.user_password = Some("test".to_string());
    op.owner_password = Some("test".to_string());

    let res = op.execute(&mut doc);
    assert!(res.is_ok());
    assert!(doc.inner.trailer.has(b"Encrypt"));
}

#[test]
fn test_decrypt() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("decrypted.pdf");

    let mut doc = LopdfDocument::load(&get_simple_pdf_path()).expect("Failed to load simple.pdf");

    // Encrypt the test document first
    let mut encrypt_op = EncryptOperation::new();
    encrypt_op.user_password = Some("test".to_string());
    encrypt_op.owner_password = Some("test".to_string());
    assert!(encrypt_op.execute(&mut doc).is_ok());

    assert!(doc.inner.trailer.has(b"Encrypt"));

    // Now decrypt it
    let decrypt_op = DecryptOperation::new(Some("test".to_string()));
    assert!(decrypt_op.execute(&mut doc).is_ok());
    assert!(!doc.inner.trailer.has(b"Encrypt"));

    assert!(doc.save(&output_path).is_ok());
}

#[test]
fn test_watermark() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("watermarked.pdf");

    let mut doc = LopdfDocument::load(&get_simple_pdf_path()).expect("Failed to load simple.pdf");
    let op = WatermarkOperation::new("CONFIDENTIAL".to_string());

    assert!(op.execute(&mut doc).is_ok());
    assert!(doc.save(&output_path).is_ok());
    assert!(output_path.exists());

    // Check if the saved file actually contains the new stream size
    let orig_len = std::fs::metadata(get_simple_pdf_path()).unwrap().len();
    let new_len = std::fs::metadata(&output_path).unwrap().len();
    assert!(new_len > orig_len); // adding a watermark should increase the file size
}

#[test]
fn test_redact() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("redacted.pdf");

    let mut doc = LopdfDocument::load(&get_simple_pdf_path()).expect("Failed to load simple.pdf");
    let op = RedactOperation::new(
        1,
        BoundingBox {
            x: 10.0,
            y: 10.0,
            width: 100.0,
            height: 100.0,
        },
    );

    assert!(op.execute(&mut doc).is_ok());
    assert!(doc.save(&output_path).is_ok());
}

#[test]
fn test_metadata() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("metadata.pdf");

    let mut doc = LopdfDocument::load(&get_simple_pdf_path()).expect("Failed to load simple.pdf");
    let mut op = MetadataOperation::new();
    op.title = Some("New Title".to_string());
    op.author = Some("John Doe".to_string());

    assert!(op.execute(&mut doc).is_ok());
    assert!(doc.save(&output_path).is_ok());

    // Reload and check metadata
    let loaded = LopdfDocument::load(&output_path).unwrap();
    let trailer_info_ref = loaded
        .inner
        .trailer
        .get(b"Info")
        .unwrap()
        .as_reference()
        .unwrap();
    let info_dict = loaded
        .inner
        .get_object(trailer_info_ref)
        .unwrap()
        .as_dict()
        .unwrap();

    assert_eq!(
        info_dict.get(b"Title").unwrap().as_str().unwrap(),
        b"New Title"
    );
    assert_eq!(
        info_dict.get(b"Author").unwrap().as_str().unwrap(),
        b"John Doe"
    );
}

#[test]
fn test_signature() {
    let mut doc = LopdfDocument::load(&get_simple_pdf_path()).expect("Failed to load simple.pdf");
    let op = SignatureOperation::new();

    let res = op.execute(&mut doc);
    assert!(res.is_ok());
}

#[test]
fn test_flatten() {
    let mut doc = LopdfDocument::load(&get_simple_pdf_path()).expect("Failed to load simple.pdf");
    let op = FlattenOperation::new();

    let res = op.execute(&mut doc);
    assert!(res.is_ok());
}

#[test]
fn test_pdf_a() {
    let mut doc = LopdfDocument::load(&get_simple_pdf_path()).expect("Failed to load simple.pdf");
    let op = PdfAConversionOperation::new();

    let res = op.execute(&mut doc);
    assert!(res.is_ok());
}

#[test]
fn test_header_footer() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("header_footer.pdf");

    let mut doc = LopdfDocument::load(&get_simple_pdf_path()).expect("Failed to load simple.pdf");
    let op = HeaderFooterOperation::new("Header Text".to_string(), "header".to_string());

    assert!(op.execute(&mut doc).is_ok());
    assert!(doc.save(&output_path).is_ok());
    assert!(output_path.exists());

    // File size should increase due to added font/content stream
    let orig_len = std::fs::metadata(get_simple_pdf_path()).unwrap().len();
    let new_len = std::fs::metadata(&output_path).unwrap().len();
    assert!(new_len > orig_len);
}

fn get_multi_page_pdf_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    if path.ends_with("paperpilot-pdf") {
        path.pop();
    }
    path.join("tests/fixtures/multi_page.pdf")
}

#[test]
fn test_watermark_multipage() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("watermarked_multi.pdf");

    let mut doc =
        LopdfDocument::load(&get_multi_page_pdf_path()).expect("Failed to load multi_page.pdf");
    let initial_pages = doc.page_count().unwrap();
    assert!(initial_pages > 1, "Expected multi-page PDF");

    let op = WatermarkOperation::new("CONFIDENTIAL".to_string());
    assert!(op.execute(&mut doc).is_ok());
    assert!(doc.save(&output_path).is_ok());

    let loaded = LopdfDocument::load(&output_path).unwrap();
    assert_eq!(loaded.page_count().unwrap(), initial_pages);
}

#[test]
fn test_redact_multipage() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("redacted_multi.pdf");

    let mut doc =
        LopdfDocument::load(&get_multi_page_pdf_path()).expect("Failed to load multi_page.pdf");

    // Redact on page 2
    let op = RedactOperation::new(
        2,
        BoundingBox {
            x: 10.0,
            y: 10.0,
            width: 100.0,
            height: 100.0,
        },
    );
    assert!(op.execute(&mut doc).is_ok());
    assert!(doc.save(&output_path).is_ok());
}
