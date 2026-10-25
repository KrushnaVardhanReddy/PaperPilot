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

fn check_zip_magic(path: &str) {
    let contents = fs::read(path).expect("Failed to read output file");
    assert!(contents.len() > 4, "Output file too small to check magic");
    assert_eq!(
        &contents[0..4],
        &[0x50, 0x4B, 0x03, 0x04],
        "File does not start with ZIP PK header"
    );
}

#[test]
fn test_wave4_tools_mcp_execution() {
    let dir = tempdir().unwrap();
    let workspace_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    let sample_pdf = workspace_root
        .join("fixtures/Get_Started_With_Smallpdf.pdf")
        .to_string_lossy()
        .to_string();

    let sample_xlsx = workspace_root
        .join("tests/e2e_fixtures/real/test.xlsx")
        .to_string_lossy()
        .to_string();

    // 1. pdf_to_docx
    let docx_out = dir.path().join("output.docx").to_string_lossy().to_string();
    let docx_res = call_tool(
        "pdf_to_docx",
        json!({
            "input": sample_pdf,
            "output": docx_out
        }),
    );
    assert_eq!(docx_res["success"], true, "pdf_to_docx should succeed");
    assert!(fs::metadata(&docx_out).is_ok(), "docx output exists");
    let docx_meta = fs::metadata(&docx_out).unwrap();
    assert!(docx_meta.len() > 0, "docx output length > 0");
    check_zip_magic(&docx_out);
    println!(
        "✓ pdf_to_docx output: {} ({} bytes)",
        docx_res["message"],
        docx_meta.len()
    );

    // 2. pdf_to_xlsx
    let xlsx_out = dir.path().join("output.xlsx").to_string_lossy().to_string();
    let xlsx_res = call_tool(
        "pdf_to_xlsx",
        json!({
            "input": sample_pdf,
            "output": xlsx_out
        }),
    );
    assert_eq!(xlsx_res["success"], true, "pdf_to_xlsx should succeed");
    assert!(fs::metadata(&xlsx_out).is_ok(), "xlsx output exists");
    let xlsx_meta = fs::metadata(&xlsx_out).unwrap();
    assert!(xlsx_meta.len() > 0, "xlsx output length > 0");
    check_zip_magic(&xlsx_out);
    println!(
        "✓ pdf_to_xlsx output: {} ({} bytes)",
        xlsx_res["message"],
        xlsx_meta.len()
    );

    // 3. pdf_to_pptx
    let pptx_out = dir.path().join("output.pptx").to_string_lossy().to_string();
    let pptx_res = call_tool(
        "pdf_to_pptx",
        json!({
            "input": sample_pdf,
            "output": pptx_out
        }),
    );
    assert_eq!(pptx_res["success"], true, "pdf_to_pptx should succeed");
    assert!(fs::metadata(&pptx_out).is_ok(), "pptx output exists");
    let pptx_meta = fs::metadata(&pptx_out).unwrap();
    assert!(pptx_meta.len() > 0, "pptx output length > 0");
    check_zip_magic(&pptx_out);
    println!(
        "✓ pdf_to_pptx output: {} ({} bytes)",
        pptx_res["message"],
        pptx_meta.len()
    );

    // 4. office_to_markdown
    let md_out = dir.path().join("output.md").to_string_lossy().to_string();
    let md_res = call_tool(
        "office_to_markdown",
        json!({
            "input": sample_xlsx,
            "output": md_out
        }),
    );
    assert_eq!(md_res["success"], true, "office_to_markdown should succeed");
    assert!(fs::metadata(&md_out).is_ok(), "md output exists");
    let md_meta = fs::metadata(&md_out).unwrap();
    assert!(md_meta.len() > 0, "md output length > 0");
    let md_contents = fs::read_to_string(&md_out).unwrap();
    assert!(
        md_contents.contains('|') || md_contents.contains("---"),
        "Markdown output should contain table delimiters"
    );
    println!(
        "✓ office_to_markdown output: {} ({} bytes)",
        md_res["message"],
        md_meta.len()
    );

    // 5. office_extract_text
    let txt_out = dir.path().join("output.txt").to_string_lossy().to_string();
    let txt_res = call_tool(
        "office_extract_text",
        json!({
            "input": sample_xlsx,
            "output": txt_out
        }),
    );
    assert_eq!(
        txt_res["success"], true,
        "office_extract_text should succeed"
    );
    assert!(fs::metadata(&txt_out).is_ok(), "txt output exists");
    let txt_meta = fs::metadata(&txt_out).unwrap();
    assert!(txt_meta.len() > 0, "txt output length > 0");
    let txt_contents = fs::read_to_string(&txt_out).unwrap();
    assert!(
        !txt_contents.is_empty(),
        "Extracted text should not be empty"
    );
    println!(
        "✓ office_extract_text output: {} ({} bytes)",
        txt_res["message"],
        txt_meta.len()
    );

    // 6. office_modernize_legacy
    let modern_out = dir
        .path()
        .join("output_modern.xlsx")
        .to_string_lossy()
        .to_string();
    let modern_res = call_tool(
        "office_modernize_legacy",
        json!({
            "input": sample_xlsx,
            "output": modern_out
        }),
    );
    assert_eq!(
        modern_res["success"], true,
        "office_modernize_legacy should succeed"
    );
    assert!(fs::metadata(&modern_out).is_ok(), "modern output exists");
    let modern_meta = fs::metadata(&modern_out).unwrap();
    assert!(modern_meta.len() > 0, "modern output length > 0");
    check_zip_magic(&modern_out);
    println!(
        "✓ office_modernize_legacy output: {} ({} bytes)",
        modern_res["message"],
        modern_meta.len()
    );
}
