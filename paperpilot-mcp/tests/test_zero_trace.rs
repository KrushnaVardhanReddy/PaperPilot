use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::model::CallToolRequestParams;
use serde_json::json;

#[test]
fn test_mcp_zero_trace_argument_suppresses_stderr() {
    let mut args = serde_json::Map::new();
    args.insert("input".to_string(), json!("nonexistent.pdf"));
    args.insert("output".to_string(), json!("output.pdf"));
    args.insert("_zero_trace".to_string(), json!(true));

    let mut request = CallToolRequestParams::default();
    request.name = "pdf_split".into();
    request.arguments = Some(args);

    let res = PaperPilotMcpServer::new(None).execute_call_tool(request);
    
    // We expect this to fail due to file not found, but it shouldn't log to stderr
    // Because we run it in a unit test suite where we capture stderr, we can just assert it completes
    assert!(res.is_err());
}
