#[path = "../../paperpilot-pdf/tests/support/workflow_fixture.rs"]
mod fixture;
use fixture::{PageKind::*, pdf};
use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::model::{CallToolRequestParams, CallToolResponse, ContentBlock};
use serde_json::{Value, json};
fn call(name: &str, args: Value) -> Value {
    let mut r = CallToolRequestParams::default();
    r.name = name.to_string().into();
    r.arguments = args.as_object().cloned();
    let CallToolResponse::Complete(result) =
        PaperPilotMcpServer::new(None).execute_call_tool(r).unwrap()
    else {
        panic!("Expected response")
    };
    let ContentBlock::Text(text) = &result.content[0] else {
        panic!("Expected JSON")
    };
    serde_json::from_str(&text.text).unwrap()
}
#[test]
fn mcp_runs_and_queries_the_same_durable_history_after_server_recreation() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("output.pdf");
    std::fs::write(&input, pdf(&[Text])).unwrap();
    let result = call(
        "recipe_run",
        json!({"input":input,"output":output,"recipe":"preset-legal-discovery"}),
    );
    let path = std::path::Path::new(result["receipt"]["receipt_path"].as_str().unwrap());
    let entries = call(
        "execution_receipt_list",
        json!({"directory":path.parent().unwrap()}),
    );
    assert_eq!(entries[0]["job_id"], result["receipt"]["job_id"]);
    let item = call(
        "execution_receipt_get",
        json!({"directory":path.parent().unwrap(),"receipt_id":path.file_name().unwrap().to_str().unwrap()}),
    );
    assert_eq!(item["success"], true);
    assert!(
        lopdf::Document::load(output)
            .unwrap()
            .extract_text(&[1])
            .unwrap()
            .contains("Original invoice")
    );
}
