use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::model::CallToolRequestParams;
use rmcp::model::CallToolResponse;
use serde_json::Value;
use std::path::PathBuf;

#[tokio::test]
async fn test_pdf_merge_e2e() {
    let _server = PaperPilotMcpServer::new();
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("paperpilot-pdf").join("tests")
        .join("fixtures");

    // Fallback if not found in root (instructions are explicitly asking for paperpilot-pdf/tests/fixtures)
    // Create symlink for tests/fixtures to paperpilot-pdf/tests/fixtures so both work.

    let input1 = fixtures_dir.join("simple.pdf");
    let input2 = fixtures_dir.join("multi_page.pdf");

    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("merged.pdf");

    let mut args = serde_json::Map::new();
    args.insert(
        "inputs".to_string(),
        Value::Array(vec![
            Value::String(input1.to_string_lossy().into_owned()),
            Value::String(input2.to_string_lossy().into_owned()),
        ]),
    );
    args.insert(
        "output".to_string(),
        Value::String(output_path.to_string_lossy().into_owned()),
    );

    let mut req = CallToolRequestParams::default();
    req.name = "pdf_merge".into();
    req.arguments = Some(args);

    let res = PaperPilotMcpServer::execute_call_tool(req).unwrap();

    if let CallToolResponse::Complete(result) = res {
        assert!(!result.is_error.unwrap_or(false), "Tool execution returned an error");
        let content_str = &result.content[0].as_text().unwrap().text;
        let json_res: serde_json::Value = serde_json::from_str(content_str).unwrap();
        assert_eq!(json_res.get("success").unwrap().as_bool(), Some(true));
    } else {
        panic!("Expected CallToolResponse::Complete");
    }

    assert!(output_path.exists());
}

#[tokio::test]
async fn test_pdf_extract_text_e2e() {
    let _server = PaperPilotMcpServer::new();
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("paperpilot-pdf").join("tests")
        .join("fixtures");

    let input = fixtures_dir.join("simple.pdf");
    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("out.txt");

    let mut args = serde_json::Map::new();
    args.insert(
        "input".to_string(),
        Value::String(input.to_string_lossy().into_owned()),
    );
    args.insert(
        "output".to_string(),
        Value::String(output_path.to_string_lossy().into_owned()),
    );

    let mut req = CallToolRequestParams::default();
    req.name = "pdf_extract_text".into();
    req.arguments = Some(args);

    let res = PaperPilotMcpServer::execute_call_tool(req).unwrap();

    if let CallToolResponse::Complete(result) = res {
        assert!(!result.is_error.unwrap_or(false), "Tool execution returned an error");
        let content_str = &result.content[0].as_text().unwrap().text;
        let json_res: serde_json::Value = serde_json::from_str(content_str).unwrap();
        assert_eq!(json_res.get("success").unwrap().as_bool(), Some(true));
    } else {
        panic!("Expected CallToolResponse::Complete");
    }

    assert!(output_path.exists());
}

#[tokio::test]
async fn test_pdf_metadata_e2e() {
    let _server = PaperPilotMcpServer::new();
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("paperpilot-pdf").join("tests")
        .join("fixtures");

    let input = fixtures_dir.join("simple.pdf");

    let mut args = serde_json::Map::new();
    args.insert(
        "input".to_string(),
        Value::String(input.to_string_lossy().into_owned()),
    );

    let mut req = CallToolRequestParams::default();
    req.name = "pdf_metadata".into();
    req.arguments = Some(args);

    let res = PaperPilotMcpServer::execute_call_tool(req).unwrap();

    if let CallToolResponse::Complete(result) = res {
        assert!(!result.is_error.unwrap_or(false), "Tool execution returned an error");
        let content_str = &result.content[0].as_text().unwrap().text;
        let json_res: serde_json::Value = serde_json::from_str(content_str).unwrap();
        assert_eq!(json_res.get("success").unwrap().as_bool(), Some(true));
    } else {
        panic!("Expected CallToolResponse::Complete");
    }
}
