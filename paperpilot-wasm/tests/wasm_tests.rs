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