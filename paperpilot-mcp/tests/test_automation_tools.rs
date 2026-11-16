#[path = "../../paperpilot-pdf/tests/support/workflow_fixture.rs"]
mod fixture;
use fixture::{PageKind::Text, pdf};
use paperpilot_mcp::server::PaperPilotMcpServer;
use paperpilot_pdf::operations::watch::WatcherManager;
use rmcp::model::{CallToolRequestParams, CallToolResponse, ContentBlock};
use serde_json::{Value, json};
use std::sync::Arc;

fn call(server: &PaperPilotMcpServer, name: &str, args: Value) -> Value {
    let mut request = CallToolRequestParams::default();
    request.name = name.to_string().into();
    request.arguments = args.as_object().cloned();
    let CallToolResponse::Complete(result) = server.execute_call_tool(request).unwrap() else {
        panic!("Expected complete response");
    };
    let ContentBlock::Text(text) = &result.content[0] else {
        panic!("Expected JSON text");
    };
    serde_json::from_str(&text.text).unwrap()
}

#[test]
fn injected_registry_owns_real_workers_and_removes_them_after_joining() {
    let registry = Arc::new(WatcherManager::default());
    let server = PaperPilotMcpServer::new(Some(registry.clone()));
    let root = tempfile::tempdir().unwrap();
    let started = call(
        &server,
        "watcher_start",
        json!({"dir":root.path(),"recipe":"preset-legal-discovery"}),
    );
    let id = started["watcher_id"].as_str().unwrap();
    assert_eq!(started["status"], "watching");
    assert_eq!(registry.list().len(), 1);
    assert_eq!(
        call(&server, "watcher_list", json!({}))["watchers"][0]["watcher_id"],
        id
    );
    assert_eq!(
        call(&server, "watcher_stop", json!({"watcher_id":id}))["success"],
        true
    );
    assert!(registry.list().is_empty());
    assert_eq!(
        call(&server, "watcher_list", json!({}))["watchers"],
        json!([])
    );
}

#[test]
fn recipe_run_preserves_real_pdf_pages_and_text() {
    let server = PaperPilotMcpServer::new(None);
    let root = tempfile::tempdir().unwrap();
    let input = root.path().join("input.pdf");
    let output = root.path().join("out.pdf");
    std::fs::write(&input, pdf(&[Text, Text])).unwrap();
    assert_eq!(
        call(
            &server,
            "recipe_run",
            json!({"input":input,"recipe":"preset-legal-discovery","output":output})
        )["success"],
        true
    );
    let doc = lopdf::Document::load(output).unwrap();
    assert_eq!(doc.get_pages().len(), 2);
    assert!(
        doc.extract_text(&[1, 2])
            .unwrap()
            .contains("Original invoice")
    );
}
