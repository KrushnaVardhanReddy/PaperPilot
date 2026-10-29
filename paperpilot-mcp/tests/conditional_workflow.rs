#[path = "../../paperpilot-pdf/tests/support/workflow_fixture.rs"]
mod fixture;
use fixture::{PageKind::*, pdf};
use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::model::{CallToolRequestParams, CallToolResponse, ContentBlock};
use serde_json::{Value, json};
use std::fs;
use tempfile::tempdir;

fn request(args: Value) -> CallToolRequestParams {
    let mut r = CallToolRequestParams::default();
    r.name = "document_workflow".into();
    r.arguments = args.as_object().cloned();
    r
}

#[test]
fn mcp_executes_toml_recipe_and_reports_skipped_stage() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("in.pdf");
    let output = dir.path().join("out.pdf");
    fs::write(&input, pdf(&[Text])).unwrap();
    let recipe = "name='MCP'\n[[stage]]\nid='scan'\nop='ocr'\ncondition={if_scanned=true}\n[[stage]]\nid='rotate'\nop='rotate'";
    let result = PaperPilotMcpServer::new(None).execute_call_tool(request(
        json!({"input":input,"output":output,"recipe":recipe}),
    ))
    .unwrap();
    let CallToolResponse::Complete(result) = result else {
        panic!("Incomplete MCP response");
    };
    let ContentBlock::Text(text) = &result.content[0] else {
        panic!("Expected JSON text");
    };
    let response: Value = serde_json::from_str(&text.text).unwrap();
    assert_eq!(response["success"], true);
    assert_eq!(response["data"]["stages"][0]["status"], "skipped");
    let doc = lopdf::Document::load(output).unwrap();
    assert_eq!(doc.get_pages().len(), 1);
    assert!(doc.extract_text(&[1]).unwrap().contains("Original invoice"));
}

#[test]
fn mcp_failure_has_report_and_preserves_existing_output() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("in.pdf");
    let output = dir.path().join("out.pdf");
    fs::write(&input, pdf(&[Text])).unwrap();
    let original = pdf(&[Image]);
    fs::write(&output, &original).unwrap();
    let recipe = "name='MCP failure'\n[[stage]]\nid='bad'\nop='redact'";
    let error = PaperPilotMcpServer::new(None).execute_call_tool(request(
        json!({"input":input,"output":output,"recipe":recipe}),
    ))
    .unwrap_err();
    assert_eq!(error.data.unwrap()["stages"][0]["status"], "failed");
    assert_eq!(fs::read(output).unwrap(), original);
}
