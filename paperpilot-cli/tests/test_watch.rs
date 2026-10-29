use std::process::Command;
use std::time::Duration;
use tempfile::tempdir;

fn create_mock_pdf(path: &std::path::Path) {
    let mut doc = lopdf::Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let mut catalog = lopdf::Dictionary::new();
    catalog.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
    catalog.set("Pages", lopdf::Object::Reference(pages_id));
    let catalog_id = doc.add_object(catalog);
    doc.trailer
        .set("Root", lopdf::Object::Reference(catalog_id));

    let mut pages = lopdf::Dictionary::new();
    pages.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
    pages.set("Kids", lopdf::Object::Array(vec![]));
    pages.set("Count", lopdf::Object::Integer(0));
    doc.objects
        .insert(pages_id, lopdf::Object::Dictionary(pages));
    doc.save(path).unwrap();
}

#[test]
fn test_watch_folder_daemon() {
    let watch_dir = tempdir().unwrap();
    let out_dir = tempdir().unwrap();

    // Spawn the daemon
    let mut child = Command::new(env!("CARGO_BIN_EXE_paperpilot"))
        .arg("watch")
        .arg("--directory")
        .arg(watch_dir.path())
        .arg("--output")
        .arg(out_dir.path())
        .arg("--recipe")
        .arg("preset-clean-scan")
        .arg("--settle-delay-ms")
        .arg("100")
        .spawn()
        .expect("Failed to start watch daemon");

    // Give it a moment to start
    std::thread::sleep(Duration::from_millis(500));

    let mock_pdf_path = watch_dir.path().join("test_input.pdf");
    let mut doc = lopdf::Document::with_version("1.4");
    let pages_id = doc.new_object_id();
    let catalog_id = doc.new_object_id();
    let page_id = doc.new_object_id();

    let mut page_dict = lopdf::Dictionary::new();
    page_dict.set("Type", lopdf::Object::Name(b"Page".to_vec()));
    page_dict.set("Parent", lopdf::Object::Reference(pages_id));
    page_dict.set("MediaBox", lopdf::Object::Array(vec![0.into(), 0.into(), 612.into(), 792.into()]));
    doc.objects.insert(page_id, lopdf::Object::Dictionary(page_dict));

    let mut pages_dict = lopdf::Dictionary::new();
    pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
    pages_dict.set("Kids", lopdf::Object::Array(vec![lopdf::Object::Reference(page_id)]));
    pages_dict.set("Count", lopdf::Object::Integer(1));
    doc.objects.insert(pages_id, lopdf::Object::Dictionary(pages_dict));

    let mut catalog_dict = lopdf::Dictionary::new();
    catalog_dict.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
    catalog_dict.set("Pages", lopdf::Object::Reference(pages_id));
    doc.objects.insert(catalog_id, lopdf::Object::Dictionary(catalog_dict));
    doc.trailer.set("Root", lopdf::Object::Reference(catalog_id));
    doc.save(&mock_pdf_path).unwrap();

    // Wait for debounce and processing
    std::thread::sleep(Duration::from_millis(3000));

    // Check output
    let expected_out_file = out_dir.path().join("test_input.pdf");
    assert!(
        expected_out_file.exists(),
        "Processed output file should exist"
    );

    // Kill the daemon
    child.kill().unwrap();
    child.wait().unwrap();
}
