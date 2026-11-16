use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::model::{CallToolRequestParams, CallToolResponse, ContentBlock};
use serde_json::{Value, json};
use std::{fs, path::PathBuf};

fn template() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../apps/desktop/tests/fixtures/automation/form.pdf")
}
fn call(args: Value) -> (Value, bool) {
    let mut request = CallToolRequestParams::default();
    request.name = "batch_fill".into();
    request.arguments = args.as_object().cloned();
    let CallToolResponse::Complete(result) = PaperPilotMcpServer::new(None)
        .execute_call_tool(request)
        .unwrap()
    else {
        panic!("incomplete response")
    };
    let ContentBlock::Text(text) = &result.content[0] else {
        panic!("expected JSON")
    };
    (
        serde_json::from_str(&text.text).unwrap(),
        result.is_error.unwrap_or(false),
    )
}
#[test]
fn inline_rows_fill_real_form_and_return_partial_failure_receipt() {
    let dir = tempfile::tempdir().unwrap();
    let (r, error) = call(
        json!({"template":template(),"output_dir":dir.path(),"records":[{"customer":"Ada"},{"customer":{"nested":true}},{"customer":"Grace"}],"keep_editable":true}),
    );
    assert!(error);
    assert_eq!(r["succeeded"], 2);
    assert_eq!(r["failed"], 1);
    assert_eq!(r["data_source"], "inline_json");
    assert!(r["data"].is_null());
    for (index, expected) in [(0, "Ada"), (2, "Grace")] {
        let bytes = fs::read(r["results"][index]["output_path"].as_str().unwrap()).unwrap();
        let fields = paperpilot_pdf::operations::form_fill::read_fields(
            &lopdf::Document::load_mem(&bytes).unwrap(),
        )
        .unwrap();
        assert_eq!(fields["customer"], expected);
    }
    let saved: Value =
        serde_json::from_slice(&fs::read(r["receipt_path"].as_str().unwrap()).unwrap()).unwrap();
    assert_eq!(saved, r);
}
#[test]
fn csv_input_flattens_and_duplicate_calls_preserve_outputs() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path().join("rows.csv");
    fs::write(&data, "customer\nAda\n").unwrap();
    let out = dir.path().join("out");
    let (r, error) = call(json!({"template":template(),"output_dir":out,"data":data}));
    assert!(!error);
    assert_eq!(r["data_source"], "file");
    let path = r["results"][0]["output_path"].as_str().unwrap();
    let original = fs::read(path).unwrap();
    let doc = lopdf::Document::load_mem(&original).unwrap();
    assert!(!doc.catalog().unwrap().has(b"AcroForm"));
    let page = doc
        .get_dictionary(*doc.get_pages().get(&1).unwrap())
        .unwrap();
    let resources = page.get(b"Resources").unwrap().as_dict().unwrap();
    let xobjects = resources.get(b"XObject").unwrap().as_dict().unwrap();
    assert!(xobjects.iter().next().is_some());
    let content = doc.get_page_content(*doc.get_pages().get(&1).unwrap());
    assert!(String::from_utf8_lossy(&content).contains("Do"));
    let mut request = CallToolRequestParams::default();
    request.name = "batch_fill".into();
    request.arguments = json!({"template":template(),"output_dir":out,"data":data})
        .as_object()
        .cloned();
    assert!(
        PaperPilotMcpServer::new(None)
            .execute_call_tool(request)
            .is_err()
    );
    assert_eq!(fs::read(path).unwrap(), original);
}
#[test]
fn schema_and_handler_reject_ambiguous_sources_before_writing() {
    let server = PaperPilotMcpServer::new(None);
    assert!(
        server
            .execute_list_tools()
            .unwrap()
            .tools
            .iter()
            .any(|t| t.name == "batch_fill")
    );
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("out");
    for sources in [
        json!({}),
        json!({"data":"x.csv","records":[]}),
        json!({"records":{}}),
        json!({"records":[],"unknown":true}),
    ] {
        let mut args = json!({"template":template(),"output_dir":out});
        args.as_object_mut()
            .unwrap()
            .extend(sources.as_object().unwrap().clone());
        let mut req = CallToolRequestParams::default();
        req.name = "batch_fill".into();
        req.arguments = args.as_object().cloned();
        assert!(server.execute_call_tool(req).is_err());
        assert!(!out.exists());
    }
}
