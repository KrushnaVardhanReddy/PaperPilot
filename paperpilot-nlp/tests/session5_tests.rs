use paperpilot_nlp::ocr::fallback::handle_ocr_fallback;
use paperpilot_nlp::search::workspace::workspace_search;
use paperpilot_nlp::pii::output::save_pii_report;
use std::fs;

#[test]
fn test_ocr_fallback() {
    let out_file = "test_fallback.txt";
    let fallback_val = handle_ocr_fallback(Some(out_file));

    assert!(fallback_val["success"].as_bool().unwrap());
    assert!(!fallback_val["model_configured"].as_bool().unwrap());
    assert_eq!(fallback_val["text"].as_str().unwrap(), "OCR fallback: text extraction completed without model weights.");

    let contents = fs::read_to_string(out_file).unwrap();
    assert_eq!(contents, "OCR fallback: text extraction completed without model weights.");
    fs::remove_file(out_file).unwrap();
}

#[test]
fn test_workspace_search_directory_recursion() {
    let dir = "tests/test_search_dir";
    fs::create_dir_all(dir).unwrap();
    fs::write(format!("{}/doc1.pdf", dir), "content").unwrap();
    fs::write(format!("{}/doc2.md", dir), "content").unwrap();

    let results = workspace_search(dir, "content").unwrap();
    assert_eq!(results.len(), 2);

    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_pii_output_flush() {
    let out_file = "test_pii.json";
    let report = r#"{"entities": []}"#;

    save_pii_report(out_file, report).unwrap();

    let meta = fs::metadata(out_file).unwrap();
    assert!(meta.len() > 0);

    fs::remove_file(out_file).unwrap();
}
