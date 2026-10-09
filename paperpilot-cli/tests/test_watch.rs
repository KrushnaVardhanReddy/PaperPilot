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
    doc.trailer.set("Root", lopdf::Object::Reference(catalog_id));

    let mut pages = lopdf::Dictionary::new();
    pages.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
    pages.set("Kids", lopdf::Object::Array(vec![]));
    pages.set("Count", lopdf::Object::Integer(0));
    doc.objects.insert(pages_id, lopdf::Object::Dictionary(pages));
    doc.save(path).unwrap();
}

#[test]
fn test_watch_folder_daemon() {
    let watch_dir = tempdir().unwrap();
    let out_dir = tempdir().unwrap();

    // Spawn the daemon
    let mut child = Command::new(env!("CARGO_BIN_EXE_paperpilot"))
        .arg("watch")
        .arg(watch_dir.path())
        .arg("--output")
        .arg(out_dir.path())
        .arg("--operation")
        .arg("compress")
        .arg("--settle-delay-ms")
        .arg("100")
        .spawn()
        .expect("Failed to start watch daemon");

    // Give it a moment to start
    std::thread::sleep(Duration::from_millis(500));

    let mock_pdf_path = watch_dir.path().join("test_input.pdf");
    create_mock_pdf(&mock_pdf_path);

    // Wait for debounce and processing
    std::thread::sleep(Duration::from_millis(1500));

    // Check output
    let expected_out_file = out_dir.path().join("test_input.pdf");
    assert!(expected_out_file.exists(), "Processed output file should exist");

    // Kill the daemon
    child.kill().unwrap();
    child.wait().unwrap();
}
