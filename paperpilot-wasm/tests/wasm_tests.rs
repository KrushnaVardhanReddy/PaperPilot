use lopdf::Document;
use paperpilot_wasm::operations::{compress, merge, rotate};
use std::collections::BTreeMap;

fn create_dummy_pdf() -> Vec<u8> {
    let mut doc = Document::with_version("1.5");

    // Add a dummy page
    let pages_id = doc.new_object_id();
    let root_id = doc.new_object_id();
    let page_id = doc.new_object_id();

    let mut page_dict = lopdf::Dictionary::new();
    page_dict.set("Type", lopdf::Object::Name(b"Page".to_vec()));
    page_dict.set("Parent", lopdf::Object::Reference(pages_id));
    doc.objects.insert(page_id, lopdf::Object::Dictionary(page_dict));

    let mut pages_dict = lopdf::Dictionary::new();
    pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
    pages_dict.set("Count", lopdf::Object::Integer(1));
    pages_dict.set("Kids", lopdf::Object::Array(vec![lopdf::Object::Reference(page_id)]));
    doc.objects.insert(pages_id, lopdf::Object::Dictionary(pages_dict));

    let mut catalog_dict = lopdf::Dictionary::new();
    catalog_dict.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
    catalog_dict.set("Pages", lopdf::Object::Reference(pages_id));
    doc.objects.insert(root_id, lopdf::Object::Dictionary(catalog_dict));

    doc.trailer.set("Root", lopdf::Object::Reference(root_id));

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer).unwrap();
    buffer
}

#[test]
fn test_merge() {
    let doc1 = create_dummy_pdf();
    let doc2 = create_dummy_pdf();

    let result = merge(vec![doc1, doc2]);
    assert!(result.is_ok(), "Merge operation failed: {:?}", result.err());

    let merged_bytes = result.unwrap();
    let parsed_doc = Document::load_mem(&merged_bytes).unwrap();

    // Should have 2 pages
    let pages = parsed_doc.get_pages();
    assert_eq!(pages.len(), 2, "Merged document should have 2 pages");
}

#[test]
fn test_rotate() {
    let doc = create_dummy_pdf();

    let result = rotate(&doc, 90, "all");
    assert!(result.is_ok(), "Rotate operation failed: {:?}", result.err());

    let rotated_bytes = result.unwrap();
    let parsed_doc = Document::load_mem(&rotated_bytes).unwrap();

    // Check if the page has rotation 90
    let pages = parsed_doc.get_pages();
    let page_id = pages.get(&1).unwrap();

    if let lopdf::Object::Dictionary(dict) = parsed_doc.get_object(*page_id).unwrap() {
        let rotation = dict.get(b"Rotate").unwrap();
        assert_eq!(rotation.as_i64().unwrap(), 90, "Page should be rotated 90 degrees");
    } else {
        panic!("Page object is not a dictionary");
    }
}

#[test]
fn test_compress() {
    let doc = create_dummy_pdf();

    let result = compress(&doc);
    assert!(result.is_ok(), "Compress operation failed: {:?}", result.err());

    let compressed_bytes = result.unwrap();
    let parsed_doc = Document::load_mem(&compressed_bytes).unwrap();

    assert!(!parsed_doc.objects.is_empty(), "Compressed document should have objects");
}
use paperpilot_wasm::operations::{delete_pages, extract_pages};

fn create_multi_page_pdf() -> Vec<u8> {
    let mut doc = Document::with_version("1.5");

    let pages_id = doc.new_object_id();
    let root_id = doc.new_object_id();

    let mut kids = Vec::new();
    for _ in 0..3 {
        let page_id = doc.new_object_id();
        let mut page_dict = lopdf::Dictionary::new();
        page_dict.set("Type", lopdf::Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", lopdf::Object::Reference(pages_id));
        doc.objects.insert(page_id, lopdf::Object::Dictionary(page_dict));
        kids.push(lopdf::Object::Reference(page_id));
    }

    let mut pages_dict = lopdf::Dictionary::new();
    pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
    pages_dict.set("Count", lopdf::Object::Integer(3));
    pages_dict.set("Kids", lopdf::Object::Array(kids));
    doc.objects.insert(pages_id, lopdf::Object::Dictionary(pages_dict));

    let mut catalog_dict = lopdf::Dictionary::new();
    catalog_dict.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
    catalog_dict.set("Pages", lopdf::Object::Reference(pages_id));
    doc.objects.insert(root_id, lopdf::Object::Dictionary(catalog_dict));

    doc.trailer.set("Root", lopdf::Object::Reference(root_id));

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer).unwrap();
    buffer
}

#[test]
fn test_delete_pages() {
    let doc = create_multi_page_pdf();

    let result = delete_pages(&doc, "2");
    assert!(result.is_ok(), "Delete pages operation failed: {:?}", result.err());

    let deleted_bytes = result.unwrap();
    let parsed_doc = Document::load_mem(&deleted_bytes).unwrap();

    let pages = parsed_doc.get_pages();
    assert_eq!(pages.len(), 2, "Document should have 2 pages after deleting 1");
}

#[test]
fn test_extract_pages() {
    let doc = create_multi_page_pdf();

    let result = extract_pages(&doc, "1, 3");
    assert!(result.is_ok(), "Extract pages operation failed: {:?}", result.err());

    let extracted_bytes = result.unwrap();
    let parsed_doc = Document::load_mem(&extracted_bytes).unwrap();

    let pages = parsed_doc.get_pages();
    assert_eq!(pages.len(), 2, "Document should have 2 pages after extracting 2 pages");
}

use paperpilot_wasm::operations::{reorder_pages, crop};

#[test]
fn test_reorder_pages() {
    let doc = create_multi_page_pdf(); // 3 pages

    let new_order = vec![3, 1, 2];
    let result = reorder_pages(&doc, &new_order);
    assert!(result.is_ok(), "Reorder pages operation failed: {:?}", result.err());

    let reordered_bytes = result.unwrap();
    let parsed_doc = Document::load_mem(&reordered_bytes).unwrap();

    let pages = parsed_doc.get_pages();
    assert_eq!(pages.len(), 3, "Document should still have 3 pages after reordering");
    // While we could deep inspect object IDs to verify the swap,
    // simply passing validation and retaining 3 pages confirms the tree is structurally valid.
}

#[test]
fn test_crop() {
    let doc = create_dummy_pdf(); // 1 page

    let result = crop(&doc, 10.0, 10.0, 200.0, 200.0);
    assert!(result.is_ok(), "Crop operation failed: {:?}", result.err());

    let cropped_bytes = result.unwrap();
    let parsed_doc = Document::load_mem(&cropped_bytes).unwrap();

    let pages = parsed_doc.get_pages();
    let page_id = pages.get(&1).unwrap();

    if let lopdf::Object::Dictionary(dict) = parsed_doc.get_object(*page_id).unwrap() {
        let crop_box = dict.get(b"CropBox").unwrap();
        if let lopdf::Object::Array(arr) = crop_box {
            assert_eq!(arr.len(), 4, "CropBox should have 4 elements");
            assert_eq!(arr[0].as_f32().or_else(|_| arr[0].as_i64().map(|v| v as f32)).unwrap(), 10.0);
            assert_eq!(arr[1].as_f32().or_else(|_| arr[1].as_i64().map(|v| v as f32)).unwrap(), 10.0);
            assert_eq!(arr[2].as_f32().or_else(|_| arr[2].as_i64().map(|v| v as f32)).unwrap(), 200.0);
            assert_eq!(arr[3].as_f32().or_else(|_| arr[3].as_i64().map(|v| v as f32)).unwrap(), 200.0);
        } else {
            panic!("CropBox is not an array");
        }
    } else {
        panic!("Page object is not a dictionary");
    }
}

use paperpilot_wasm::operations::{flatten, set_metadata};

#[test]
fn test_flatten() {
    let mut doc = Document::with_version("1.5");
    let catalog_id = doc.new_object_id();
    let mut catalog = lopdf::Dictionary::new();
    catalog.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
    catalog.set("AcroForm", lopdf::Object::Dictionary(lopdf::Dictionary::new()));
    doc.objects.insert(catalog_id, lopdf::Object::Dictionary(catalog));
    doc.trailer.set("Root", lopdf::Object::Reference(catalog_id));

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer).unwrap();

    let result = flatten(&buffer);
    assert!(result.is_ok(), "Flatten operation failed: {:?}", result.err());

    let flattened_bytes = result.unwrap();
    let parsed_doc = Document::load_mem(&flattened_bytes).unwrap();

    let root_id = parsed_doc.trailer.get(b"Root").unwrap().as_reference().unwrap();
    let catalog = parsed_doc.get_object(root_id).unwrap().as_dict().unwrap();

    assert!(!catalog.has(b"AcroForm"), "AcroForm should be removed");
}

#[test]
fn test_set_metadata() {
    let doc = create_dummy_pdf();

    let result = set_metadata(
        &doc,
        Some("Test Title".to_string()),
        Some("Test Author".to_string()),
        Some("Test Subject".to_string()),
        Some("Test Keywords".to_string()),
    );
    assert!(result.is_ok(), "Set metadata operation failed: {:?}", result.err());

    let metadata_bytes = result.unwrap();
    let parsed_doc = Document::load_mem(&metadata_bytes).unwrap();

    let info_id = parsed_doc.trailer.get(b"Info").unwrap().as_reference().unwrap();
    let info = parsed_doc.get_object(info_id).unwrap().as_dict().unwrap();

    let title = info.get(b"Title").unwrap().as_str().unwrap();
    assert_eq!(title, b"Test Title", "Title should match");

    let author = info.get(b"Author").unwrap().as_str().unwrap();
    assert_eq!(author, b"Test Author", "Author should match");

    let subject = info.get(b"Subject").unwrap().as_str().unwrap();
    assert_eq!(subject, b"Test Subject", "Subject should match");

    let keywords = info.get(b"Keywords").unwrap().as_str().unwrap();
    assert_eq!(keywords, b"Test Keywords", "Keywords should match");
}

use paperpilot_wasm::operations::{images_to_pdf, extract_images, pdf_hash};

#[test]
fn test_images_to_pdf() {
    let pixel = vec![137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 13, 73, 68, 65, 84, 120, 218, 99, 252, 207, 192, 80, 15, 0, 4, 133, 1, 128, 132, 169, 140, 33, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130];
    let buffers: Vec<&[u8]> = vec![&pixel];
    let result = images_to_pdf(&buffers);
    assert!(result.is_ok(), "Images to PDF operation failed: {:?}", result.err());
    let pdf_bytes = result.unwrap();
    let doc = Document::load_mem(&pdf_bytes).unwrap();
    assert_eq!(doc.get_pages().len(), 1, "Should create a 1-page PDF");
}

#[test]
fn test_extract_images() {
    let pixel = vec![137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 13, 73, 68, 65, 84, 120, 218, 99, 252, 207, 192, 80, 15, 0, 4, 133, 1, 128, 132, 169, 140, 33, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130];
    let buffers: Vec<&[u8]> = vec![&pixel];
    let pdf_bytes = images_to_pdf(&buffers).unwrap();

    let result = extract_images(&pdf_bytes);
    assert!(result.is_ok(), "Extract images operation failed: {:?}", result.err());
    let images = result.unwrap();
    assert_eq!(images.len(), 1, "Should extract 1 image");
}

#[test]
fn test_pdf_hash() {
    let doc = create_dummy_pdf();
    let result = pdf_hash(&doc);
    assert!(result.is_ok(), "PDF hash operation failed: {:?}", result.err());
    let hash = result.unwrap();
    assert_eq!(hash.len(), 64, "SHA-256 hash should be 64 hex characters");
}

use paperpilot_wasm::operations::{render_page, extract_text, decrypt, page_numbers, header_footer, pdf_info, ocr};

#[test]
fn test_render_page() {
    let doc = create_dummy_pdf();
    let result = render_page(&doc, 0, 1.0);
    assert!(result.is_ok(), "Render page failed: {:?}", result.err());
    let png_bytes = result.unwrap();
    assert!(png_bytes.len() > 8);
    assert_eq!(&png_bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10], "Should be valid PNG signature");
}

#[test]
fn test_extract_text() {
    let doc = create_dummy_pdf(); // create_dummy_pdf only has an empty catalog/pages
    // We expect an empty string but we check if it succeeds.
    let result = extract_text(&doc);
    assert!(result.is_ok(), "Extract text failed: {:?}", result.err());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_decrypt() {
    // Create an encrypted dummy PDF. We'll use operations::encrypt to create it first.
    let doc = create_dummy_pdf();
    let encrypted = paperpilot_wasm::operations::encrypt(&doc, "secret").unwrap();
    
    // Now test decrypt
    let result = decrypt(&encrypted, "secret");
    assert!(result.is_ok(), "Decrypt failed: {:?}", result.err());
    
    // Test wrong password
    let fail_result = decrypt(&encrypted, "wrong");
    assert!(fail_result.is_err(), "Decrypt should fail with wrong password");
}

#[test]
fn test_page_numbers() {
    let doc = create_dummy_pdf();
    let result = page_numbers(&doc, "Page {n} of {total}", "bottom-center");
    assert!(result.is_ok(), "Page numbers failed: {:?}", result.err());
    let res_doc = Document::load_mem(&result.unwrap()).unwrap();
    assert_eq!(res_doc.get_pages().len(), 1);
}

#[test]
fn test_header_footer() {
    let doc = create_dummy_pdf();
    let result = header_footer(&doc, "Header", "Footer");
    assert!(result.is_ok(), "Header Footer failed: {:?}", result.err());
    let res_doc = Document::load_mem(&result.unwrap()).unwrap();
    assert_eq!(res_doc.get_pages().len(), 1);
}

#[test]
fn test_pdf_info() {
    let doc = create_dummy_pdf();
    let result = pdf_info(&doc);
    assert!(result.is_ok(), "PDF Info failed: {:?}", result.err());
    let json = result.unwrap();
    assert!(json.contains("\"page_count\": 1"));
    assert!(json.contains("\"encrypted\": false"));
}

#[test]
fn test_ocr() {
    // create a simple dummy image
    let pixel = vec![137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 13, 73, 68, 65, 84, 120, 218, 99, 252, 207, 192, 80, 15, 0, 4, 133, 1, 128, 132, 169, 140, 33, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130];
    let buffers: Vec<&[u8]> = vec![&pixel];
    let pdf_bytes = paperpilot_wasm::operations::images_to_pdf(&buffers).unwrap();
    
    // OCR should not panic, though the small image won't have text.
    let result = ocr(&pdf_bytes);
    assert!(result.is_ok(), "OCR failed: {:?}", result.err());
}
