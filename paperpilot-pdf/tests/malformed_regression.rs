use lopdf::{Dictionary, Document, Object};
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::delete::DeletePagesOperation;
use paperpilot_pdf::operations::extract_text::ExtractTextOperation;
use paperpilot_pdf::operations::split::SplitOperation;

#[test]
fn test_fuzz_corrupted_header_and_random_bytes() {
    let payloads: Vec<&[u8]> = vec![
        b"",
        b"%PDF",
        b"%PDF-1.7\n",
        b"%PDF-1.4\n%EOF\n",
        b"Just some random text that is definitely not a PDF",
        &[0xFF, 0xFE, 0x00, 0x01, 0x02, 0x03, 0xFF, 0xFF, 0xFF], // Non-printable garbage
        b"%PDF-1.7\n1 0 obj\n<<>>\nendobj\ntrailer\n<< /Size 2 /Root 1 0 R >>\nstartxref\n100\n%%EOF", // Missing object at xref offset
    ];

    for payload in payloads {
        let result = LopdfDocument::from_bytes(payload);

        if let Err(e) = result {
            match e {
                PdfError::ParseError(_)
                | PdfError::CorruptFile(_)
                | PdfError::IoError(_)
                | PdfError::InvalidInput(_)
                | PdfError::Other(_) => {
                    // Good
                }
                _ => panic!("Expected ParseError or IoError for malformed payload"),
            }
        }
    }
}
#[test]
fn test_fuzz_circular_page_tree_loop() {
    let mut doc = Document::with_version("1.5");

    // Create a Pages node that references itself in Kids and Parent
    let pages_id = doc.new_object_id();
    let mut pages_dict = Dictionary::new();
    pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
    pages_dict.set("Kids", Object::Array(vec![Object::Reference(pages_id)]));
    pages_dict.set("Count", Object::Integer(1));
    pages_dict.set("Parent", Object::Reference(pages_id)); // Circular
    doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

    let mut catalog = Dictionary::new();
    catalog.set("Type", Object::Name(b"Catalog".to_vec()));
    catalog.set("Pages", Object::Reference(pages_id));
    let catalog_id = doc.add_object(catalog);
    doc.trailer.set("Root", Object::Reference(catalog_id));

    let mut buf = Vec::new();
    doc.save_to(&mut buf).unwrap();

    let mut lopdf_doc = LopdfDocument::from_bytes(&buf).unwrap();

    // Use page count instead of InfoOperation
    let _ = lopdf_doc.page_count();

    let op2 = ExtractTextOperation::new(None);
    let _ = op2.execute(&mut lopdf_doc);

    // split requires a directory
    let temp_dir = tempfile::tempdir().unwrap();
    let op3 = SplitOperation::new(temp_dir.path().to_path_buf(), vec![1]);
    let _ = op3.execute(&mut lopdf_doc);

    let op4 = DeletePagesOperation::new(vec![1]);
    let _ = op4.execute(&mut lopdf_doc);

    // If we reach here, we didn't stack overflow or panic.
}

#[test]
fn test_fuzz_stream_length_mismatch() {
    let mut doc = Document::with_version("1.5");
    let stream_id = doc.new_object_id();

    let mut stream_dict = Dictionary::new();
    // Stream bomb: declared length is 1 GB
    stream_dict.set("Length", Object::Integer(1073741824));

    // Only 5 bytes provided
    let stream = lopdf::Stream::new(stream_dict, b"12345".to_vec());
    doc.objects.insert(stream_id, Object::Stream(stream));

    let mut buf = Vec::new();
    doc.save_to(&mut buf).unwrap();

    let lopdf_doc = LopdfDocument::from_bytes(&buf).unwrap();

    // Try to get decompressed content or manipulate
    if let Ok(Object::Stream(s)) = lopdf_doc.inner.get_object(stream_id) {
        // Just verify it doesn't cause OOM (lopdf is safe here because it uses length of vector for decompression, not purely declared length, or throws error)
        let _ = s.decompressed_content();
    }
}

#[test]
fn test_fuzz_deeply_nested_dictionaries() {
    let mut doc = Document::with_version("1.5");

    let mut current_dict = Dictionary::new();
    current_dict.set("Value", Object::Integer(1));

    // Nest 200 levels deep
    for _ in 0..200 {
        let mut new_dict = Dictionary::new();
        new_dict.set("Nested", Object::Dictionary(current_dict));
        current_dict = new_dict;
    }

    let root_id = doc.add_object(Object::Dictionary(current_dict));
    doc.trailer.set("Root", Object::Reference(root_id));

    let mut buf = Vec::new();
    doc.save_to(&mut buf).unwrap();

    let lopdf_doc = LopdfDocument::from_bytes(&buf).unwrap();
    let _ = lopdf_doc.page_count();
}

#[test]
fn test_fuzz_corrupted_xref_offsets() {
    let payload = b"%PDF-1.7\n1 0 obj\n<<>>\nendobj\ntrailer\n<< /Size 2 /Root 1 0 R >>\nstartxref\n-100\n%%EOF";

    let result = LopdfDocument::from_bytes(payload);

    // Check it's an error and handled gracefully
    if let Err(e) = result {
        match e {
            PdfError::ParseError(_)
            | PdfError::IoError(_)
            | PdfError::CorruptFile(_)
            | PdfError::InvalidInput(_)
            | PdfError::Other(_) => {
                // good
            }
            _ => panic!("Expected ParseError, got {:?}", e),
        }
    }
}
