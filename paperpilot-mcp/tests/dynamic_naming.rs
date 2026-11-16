#[path = "../../paperpilot-pdf/tests/support/workflow_fixture.rs"]
mod fixture;
use fixture::{PageKind::Text, pdf};
use paperpilot_mcp::server::PaperPilotMcpServer;
use paperpilot_pdf::operations::watch::WatcherManager;
use rmcp::model::{CallToolRequestParams, CallToolResponse, ContentBlock};
use serde_json::{Value, json};
use std::{
    fs,
    sync::Arc,
    time::{Duration, Instant},
};
fn request(name: &str, args: Value) -> CallToolRequestParams {
    let mut r = CallToolRequestParams::default();
    r.name = name.to_string().into();
    r.arguments = args.as_object().cloned();
    r
}
fn call(server: &PaperPilotMcpServer, name: &str, args: Value) -> Value {
    let CallToolResponse::Complete(result) = server.execute_call_tool(request(name, args)).unwrap()
    else {
        panic!("incomplete")
    };
    let ContentBlock::Text(text) = &result.content[0] else {
        panic!("expected JSON")
    };
    serde_json::from_str(&text.text).unwrap()
}
#[test]
fn mcp_recipe_and_document_workflow_return_named_file_and_receipt() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("source.pdf");
    fs::write(&input, pdf(&[Text])).unwrap();
    let server = PaperPilotMcpServer::new(None);
    let r = call(
        &server,
        "recipe_run",
        json!({"input":input,"output":dir.path().join("one"),"recipe":"preset-legal-discovery","name_template":"{input_stem}-{index:04}.pdf"}),
    );
    assert_eq!(r["output"], json!(dir.path().join("one").join("source-0001.pdf")));
    assert!(std::path::Path::new(r["receipt"]["receipt_path"].as_str().unwrap()).exists());
    let r = call(
        &server,
        "document_workflow",
        json!({"input":input,"output":dir.path().join("two"),"preset":"preset-legal-discovery","name_template":"{input_name}"}),
    );
    let path = r["output_path"].as_str().unwrap();
    assert!(
        lopdf::Document::load(path)
            .unwrap()
            .extract_text(&[1])
            .unwrap()
            .contains("Original invoice")
    );
    assert_eq!(r["data"]["output"], json!(path));
    assert!(server.execute_call_tool(request("recipe_run",json!({"input":input,"recipe":"preset-legal-discovery","name_template":"{input_name}"}))).is_err());
}
#[test]
fn mcp_watcher_uses_naming_and_rejects_invalid_patterns_and_types() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("source.pdf"), pdf(&[Text])).unwrap();
    let manager = Arc::new(WatcherManager::default());
    let server = PaperPilotMcpServer::new(Some(manager.clone()));
    for pattern in [json!("../escape.pdf"), json!(42)] {
        assert!(server.execute_call_tool(request("watcher_start",json!({"dir":dir.path(),"recipe":"preset-legal-discovery","name_template":pattern}))).is_err());
        assert!(manager.list().is_empty());
    }
    let r = call(
        &server,
        "watcher_start",
        json!({"dir":dir.path(),"recipe":"preset-legal-discovery","settle_delay_ms":50,"name_template":"{input_stem}-{index:04}.pdf"}),
    );
    let id = r["watcher_id"].as_str().unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while manager.list()[0].succeeded == 0 {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(20));
    }
    call(&server, "watcher_stop", json!({"watcher_id":id}));
    assert!(lopdf::Document::load(dir.path().join("processed/source-0001.pdf")).is_ok());
}
