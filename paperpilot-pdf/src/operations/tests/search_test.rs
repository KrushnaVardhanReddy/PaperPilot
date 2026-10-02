use paperpilot_core::traits::PdfOperation;
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::search::SearchOperation;

#[test]
fn test_search_operation_no_match() {
    let mut doc = LopdfDocument::load(&std::path::PathBuf::from("../tests/e2e_fixtures/search_test.pdf")).unwrap();
    let op = SearchOperation::new("NonExistentString12345".to_string());
    assert!(op.execute(&mut doc).is_ok());

    let pages = op.match_pages.lock().unwrap();
    assert!(pages.is_empty());
}
