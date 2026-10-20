use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::model::{CallToolRequestParams, ContentBlock};
use serde_json::json;
use std::fs;
use tempfile::tempdir;

fn call_tool(name: &str, args: serde_json::Value) -> serde_json::Value {
    let mut params = CallToolRequestParams::default();
    params.name = name.to_string().into();
    if let serde_json::Value::Object(map) = args {
        params.arguments = Some(map);
    }
    let res = PaperPilotMcpServer::execute_call_tool(params).expect("MCP tool execution failed");
    if let rmcp::model::CallToolResponse::Complete(result) = res {
        if let Some(ContentBlock::Text(text_content)) = result.content.first() {
            return serde_json::from_str(&text_content.text).expect("Valid JSON response");
        }
    }
    panic!("Unexpected response format");
}

#[test]
fn test_wave2_tools_mcp_execution() {
    let dir = tempdir().unwrap();
    let workspace_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let sample_pdf = workspace_root
        .join("fixtures/Get_Started_With_Smallpdf.pdf")
        .to_string_lossy()
        .to_string();

    // 1. pdf_search
    let search_res = call_tool(
        "pdf_search",
        json!({
            "input": sample_pdf,
            "query": "Smallpdf"
        }),
    );
    assert_eq!(search_res["success"], true, "pdf_search should succeed");
    println!("✓ pdf_search output: {}", search_res["message"]);

    // 2. pdf_remove_blank
    let no_blank_out = dir.path().join("no_blank.pdf").to_string_lossy().to_string();
    let blank_res = call_tool(
        "pdf_remove_blank",
        json!({
            "input": sample_pdf,
            "output": no_blank_out,
            "sensitivity": 95
        }),
    );
    assert_eq!(blank_res["success"], true, "pdf_remove_blank should succeed");
    assert!(fs::metadata(&no_blank_out).is_ok(), "no_blank output exists");
    let no_blank_meta = fs::metadata(&no_blank_out).unwrap();
    assert!(no_blank_meta.len() > 0);
    println!("✓ pdf_remove_blank output: {} ({} bytes)", blank_res["message"], no_blank_meta.len());

    // 3. pdf_page_numbers
    let numbered_out = dir.path().join("numbered.pdf").to_string_lossy().to_string();
    let num_res = call_tool(
        "pdf_page_numbers",
        json!({
            "input": sample_pdf,
            "output": numbered_out,
            "position": "bottom-right",
            "format": "Page {n} of {total}"
        }),
    );
    assert_eq!(num_res["success"], true, "pdf_page_numbers should succeed");
    assert!(fs::metadata(&numbered_out).is_ok(), "numbered output exists");
    let num_meta = fs::metadata(&numbered_out).unwrap();
    assert!(num_meta.len() > 0);
    println!("✓ pdf_page_numbers output: {} ({} bytes)", num_res["message"], num_meta.len());

    // 4. pdf_burst
    let burst_dir = dir.path().join("burst_out");
    fs::create_dir_all(&burst_dir).unwrap();
    let burst_res = call_tool(
        "pdf_burst",
        json!({
            "input": sample_pdf,
            "output_dir": burst_dir.to_string_lossy().to_string()
        }),
    );
    assert_eq!(burst_res["success"], true, "pdf_burst should succeed");
    let burst_files: Vec<_> = fs::read_dir(&burst_dir).unwrap().collect();
    assert!(!burst_files.is_empty(), "burst_out directory must contain split pages");
    println!("✓ pdf_burst output: {} ({} individual pages created)", burst_res["message"], burst_files.len());

    // 5. pdf_extract_images
    let ext_dir = dir.path().join("ext_images");
    fs::create_dir_all(&ext_dir).unwrap();
    let ext_res = call_tool(
        "pdf_extract_images",
        json!({
            "input": sample_pdf,
            "output_dir": ext_dir.to_string_lossy().to_string()
        }),
    );
    assert_eq!(ext_res["success"], true, "pdf_extract_images should succeed");
    println!("✓ pdf_extract_images output: {}", ext_res["message"]);

    // 6. pdf_images_to_pdf
    let img_pdf_out = dir.path().join("images.pdf").to_string_lossy().to_string();
    let sample_png = workspace_root
        .join("tests/e2e_fixtures/img1.png")
        .to_string_lossy()
        .to_string();
    let img_res = call_tool(
        "pdf_images_to_pdf",
        json!({
            "inputs": [sample_png],
            "output": img_pdf_out
        }),
    );
    assert_eq!(img_res["success"], true, "pdf_images_to_pdf should succeed");
    assert!(fs::metadata(&img_pdf_out).is_ok(), "images.pdf output exists");
    let img_pdf_meta = fs::metadata(&img_pdf_out).unwrap();
    assert!(img_pdf_meta.len() > 0);
    println!("✓ pdf_images_to_pdf output: {} ({} bytes)", img_res["message"], img_pdf_meta.len());
}
