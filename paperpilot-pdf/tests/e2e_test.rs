use lopdf::{Document, Object, Stream, dictionary};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;

// Import a few operations to test in sequence
use paperpilot_pdf::operations::delete::DeletePagesOperation;
use paperpilot_pdf::operations::extract_text::ExtractTextOperation;
use paperpilot_pdf::operations::rotate::RotatePagesOperation;
use paperpilot_pdf::operations::watermark::WatermarkOperation;

fn create_test_pdf(path: &std::path::Path) {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    });
    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! {
            "F1" => font_id,
        },
    });

    // Page 1
    let content1 = Stream::new(
        dictionary! {},
        b"BT /F1 12 Tf 10 10 Td (Page 1 Text) Tj ET".to_vec(),
    );
    let content1_id = doc.add_object(content1);
    let page1_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content1_id,
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    });

    // Page 2
    let content2 = Stream::new(
        dictionary! {},
        b"BT /F1 12 Tf 10 10 Td (Page 2 Text) Tj ET".to_vec(),
    );
    let content2_id = doc.add_object(content2);
    let page2_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content2_id,
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    });

    // Page 3
    let content3 = Stream::new(
        dictionary! {},
        b"BT /F1 12 Tf 10 10 Td (Page 3 Text) Tj ET".to_vec(),
    );
    let content3_id = doc.add_object(content3);
    let page3_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content3_id,
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    });

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![page1_id.into(), page2_id.into(), page3_id.into()],
        "Count" => 3,
    };

    doc.objects.insert(pages_id, Object::Dictionary(pages));
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });

    doc.trailer.set("Root", catalog_id);
    doc.compress();
    doc.save(path).unwrap();
}

#[test]
fn test_e2e_operations_pipeline() {
    let temp_dir = tempfile::tempdir().unwrap();
    let input_path = temp_dir.path().join("e2e_input.pdf");
    let output_path = temp_dir.path().join("e2e_output.pdf");

    // 1. Create a real PDF file on disk
    create_test_pdf(&input_path);

    // 2. Load it into our system
    let mut doc = LopdfDocument::load(&input_path).expect("Failed to load document");

    // 3. Operation 1: Extract Text (verify we can read it)
    let extract_op = ExtractTextOperation::new(None);
    extract_op
        .execute(&mut doc)
        .expect("Failed to extract text");

    let text_lock = extract_op.extracted_text.lock().unwrap();
    let text = text_lock.as_ref().unwrap();
    let joined_text = text.join("\n");
    assert!(
        joined_text.contains("Page 1 Text"),
        "Extracted text missing content: {}",
        joined_text
    );
    drop(text_lock);

    // 4. Operation 2: Delete Page 2
    let delete_op = DeletePagesOperation::new(vec![2]);
    delete_op
        .execute(&mut doc)
        .expect("Failed to delete page 2");

    // 5. Operation 3: Rotate the remaining pages by 90 degrees
    let rotate_op = RotatePagesOperation::new(90, None);
    rotate_op.execute(&mut doc).expect("Failed to rotate pages");

    // 6. Operation 4: Add Watermark
    let watermark_op = WatermarkOperation::new("CONFIDENTIAL".to_string());
    watermark_op
        .execute(&mut doc)
        .expect("Failed to add watermark");

    // 7. Save it
    doc.save(&output_path)
        .expect("Failed to save final document");

    // 8. Load it back and verify state
    let mut final_doc = LopdfDocument::load(&output_path).expect("Failed to load final document");
    let extract_op_final = ExtractTextOperation::new(None);
    extract_op_final
        .execute(&mut final_doc)
        .expect("Failed to extract final text");

    let text_lock_final = extract_op_final.extracted_text.lock().unwrap();
    let text_final = text_lock_final.as_ref().unwrap();
    let joined_text_final = text_final.join("\n");

    // Page 1 should still be there, Page 3 became Page 2, original Page 2 was deleted
    assert!(joined_text_final.contains("Page 1 Text"));
    assert!(joined_text_final.contains("Page 3 Text"));
    assert!(
        !joined_text_final.contains("Page 2 Text"),
        "Page 2 should have been deleted"
    );
}
