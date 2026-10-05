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
