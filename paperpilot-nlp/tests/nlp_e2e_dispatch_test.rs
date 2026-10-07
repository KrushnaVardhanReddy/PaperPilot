use lopdf::{Dictionary, Document as LopdfInnerDocument, Object};
use paperpilot_mcp::server::PaperPilotMcpServer;
use paperpilot_nlp::resolver::OfflineNlpResolver;
use paperpilot_nlp::traits::NlpResolver;
use rmcp::model::CallToolRequestParams;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

fn create_dummy_pdf(path: &PathBuf) {
    let mut inner = LopdfInnerDocument::with_version("1.5");
    let pages_id = inner.new_object_id();
    let mut page_dict = Dictionary::new();
    page_dict.set("Type", Object::Name(b"Page".to_vec()));
    page_dict.set("Parent", Object::Reference(pages_id));
    let page_id = inner.add_object(page_dict);

    let mut pages_dict = Dictionary::new();
    pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
    pages_dict.set("Kids", Object::Array(vec![Object::Reference(page_id)]));
    pages_dict.set("Count", Object::Integer(1));
    inner.set_object(pages_id, pages_dict);

    let mut catalog_dict = Dictionary::new();
    catalog_dict.set("Type", Object::Name(b"Catalog".to_vec()));
    catalog_dict.set("Pages", Object::Reference(pages_id));
    let catalog_id = inner.add_object(catalog_dict);

    inner.trailer.set("Root", Object::Reference(catalog_id));
    inner.save(path).unwrap();
}

struct TestCleaner {
    paths: Vec<PathBuf>,
}

impl Drop for TestCleaner {
    fn drop(&mut self) {
        for path in &self.paths {
            let _ = fs::remove_file(path);
        }
    }
}

#[test]
fn test_full_nlp_to_mcp_dispatch() {
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures");
    fs::create_dir_all(&fixtures_dir).unwrap();

    let input_pdf = fixtures_dir.join("e2e_test.pdf");
    let output_pdf = fixtures_dir.join("e2e_test_rotated.pdf");

    // Ensure cleanup happens even on panic
    let _cleaner = TestCleaner {
        paths: vec![input_pdf.clone(), output_pdf.clone()],
    };

    // Clean up if they already exist from a previous failed run
    let _ = fs::remove_file(&input_pdf);
    let _ = fs::remove_file(&output_pdf);

    create_dummy_pdf(&input_pdf);
    assert!(input_pdf.exists());

    let resolver = OfflineNlpResolver::new();
    // Prompt specifically requested: "Please rotate e2e_test.pdf by 90 degrees"
    let query = "Please rotate e2e_test.pdf by 90 degrees".to_string();
    let plan = resolver.resolve(&query).expect("Failed to resolve query");

    assert_eq!(plan.intent, paperpilot_nlp::intent::Intent::Rotate);
    assert_eq!(plan.angles, vec![90]);
    assert_eq!(plan.input_files.len(), 1);

    // Prepare MCP Payload using the actual path for execution
    let mut request = CallToolRequestParams::default();
    request.name = "pdf_rotate".into();
    request.arguments = Some(
        json!({
            "input": input_pdf.to_string_lossy().to_string(),
            "pages": "1",
            "angle": plan.angles[0] as f64,
            "output": output_pdf.to_string_lossy().to_string(),
        })
        .as_object()
        .unwrap()
        .clone(),
    );

    // Execute MCP dispatch (it is synchronous in the codebase)
    let res = PaperPilotMcpServer::execute_call_tool(request);
    assert!(res.is_ok(), "MCP execution failed: {:?}", res);

    // Verify output
    assert!(output_pdf.exists(), "Output PDF was not created");
}
