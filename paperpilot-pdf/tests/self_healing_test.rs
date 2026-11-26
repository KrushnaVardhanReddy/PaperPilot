#[macro_use]
extern crate lopdf;

use paperpilot_core::traits::PdfDocument;
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::RepairOperation;

fn make_valid_sample_pdf() -> Vec<u8> {
    let mut doc = lopdf::Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let content_id = doc.add_object(lopdf::Object::Stream(lopdf::Stream::new(
        lopdf::dictionary! {},
        b"BT /F1 24 Tf 100 700 Td (Resilient Self-Healing Test) Tj ET".to_vec(),
    )));
    let page_id = doc.add_object(lopdf::dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    });
    let pages = lopdf::dictionary! {
        "Type" => "Pages",
        "Kids" => vec![page_id.into()],
        "Count" => 1,
    };
    doc.set_object(pages_id, pages);
    let catalog_id = doc.add_object(lopdf::dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);

    let mut out = Vec::new();
    doc.save_to(&mut out).expect("Failed generating sample PDF");
    out
}

#[test]
fn test_self_healing_leading_junk_bytes() {
    let valid_pdf = make_valid_sample_pdf();
    
    // Inject 128 bytes of random binary noise / header prefix before `%PDF-`
    let mut corrupted = b"HTTP/1.1 200 OK\r\nContent-Type: application/pdf\r\nCustom-Header: 12345\r\n\r\n".to_vec();
    corrupted.extend_from_slice(&valid_pdf);

    // Verify LopdfDocument::from_bytes transparently self-heals and loads it cleanly
    let doc = LopdfDocument::from_bytes(&corrupted).expect("Self-healing should load PDF with leading noise");
    assert_eq!(doc.page_count().unwrap(), 1);
}

#[test]
fn test_self_healing_damaged_xref_and_missing_eof() {
    let valid_pdf = make_valid_sample_pdf();

    // Damage the xref table and truncate the EOF
    // Find "xref" and corrupt it
    let xref_pos = valid_pdf.windows(4).position(|w| w == b"xref").expect("Valid PDF should have xref");
    let mut corrupted = valid_pdf[..xref_pos].to_vec();
    // Append a corrupted trailer without valid xref table and omit %%EOF
    corrupted.extend_from_slice(b"trailer << /Invalid true >>\nstartxref\n999999\n");

    // Standard load definitely fails
    assert!(lopdf::Document::load_mem(&corrupted).is_err(), "Standard load should fail on damaged xref and startxref");

    // Verify resilient loading rescues the document
    let doc = LopdfDocument::from_bytes(&corrupted).expect("Self-healing should reconstruct objects from corrupted xref");
    assert_eq!(doc.page_count().unwrap(), 1);
}

#[test]
fn test_repair_bytes_idempotent_on_valid_pdf() {
    let valid_pdf = make_valid_sample_pdf();
    let repaired = RepairOperation::repair_bytes(&valid_pdf).expect("Repair on valid PDF should succeed");
    
    let doc = LopdfDocument::from_bytes(&repaired).expect("Repaired valid PDF should be loadable");
    assert_eq!(doc.page_count().unwrap(), 1);
}
