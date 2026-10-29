use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::model::{CallToolRequestParams, ContentBlock};
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

pub struct TestExecutionResult {
    pub tool_id: String,
    pub interface: String,
    pub latency_ms: f64,
    pub output_size: usize,
    pub passed: bool,
    pub error_notes: String,
}

fn call_mcp_tool(name: &str, args: serde_json::Value) -> std::result::Result<(bool, serde_json::Value, f64, String), Box<dyn std::error::Error>> {
    let mut params = CallToolRequestParams::default();
    params.name = name.to_string().into();
    if let serde_json::Value::Object(map) = args {
        params.arguments = Some(map);
    }

    let start = Instant::now();
    let server = PaperPilotMcpServer::new(None);
    let res = server.execute_call_tool(params);
    let latency = start.elapsed().as_secs_f64() * 1000.0;

    match res {
        Ok(rmcp::model::CallToolResponse::Complete(result)) => {
             if let Some(ContentBlock::Text(text_content)) = result.content.first() {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text_content.text) {
                    let success = parsed["success"].as_bool().unwrap_or(false);
                    return Ok((success, parsed, latency, String::new()));
                }
             }
             Ok((false, json!({}), latency, "Failed to parse content block text".into()))
        },
        Err(e) => Ok((false, json!({}), latency, format!("{:?}", e))),
        _ => Ok((false, json!({}), latency, "Unknown error".into()))
    }
}

fn run_cli(args: &[&str]) -> std::result::Result<(bool, String, f64, String), Box<dyn std::error::Error>> {
    let candidates = [
        PathBuf::from("target/debug/paperpilot"),
        PathBuf::from("target/release/paperpilot"),
        PathBuf::from("target/debug/paperpilot-cli"),
    ];
    let mut bin_path = candidates[0].clone();
    for p in &candidates {
        if p.exists() {
            bin_path = p.clone();
            break;
        }
    }

    let start = Instant::now();
    let output = Command::new(&bin_path).args(args).output()?;
    let latency = start.elapsed().as_secs_f64() * 1000.0;

    let success = output.status.success();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if success {
        Ok((true, stdout, latency, String::new()))
    } else {
        Ok((false, stdout, latency, stderr.replace("\n", " ")))
    }
}

pub fn get_workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

#[tokio::test]
async fn test_wave8_analysis_intelligence_penta_interface() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let workspace_root = get_workspace_root();
    let out_dir = tempfile::tempdir()?;
    let mut results: Vec<TestExecutionResult> = Vec::new();

    let sample_pdf = workspace_root.join("tests/e2e_fixtures/single_page.pdf").to_string_lossy().to_string();
    let multi_page_pdf = workspace_root.join("fixtures/Get_Started_With_Smallpdf.pdf").to_string_lossy().to_string();
    let ocr_pdf = workspace_root.join("tests/e2e_fixtures/real/sample_photo_doc.pdf").to_string_lossy().to_string();

    let run_tool_test = |tool_id: &str, cli_args: &[&str], mcp_payload: serde_json::Value, check_output_file: Option<PathBuf>, results: &mut Vec<TestExecutionResult>| {
        println!("Testing tool: {}...", tool_id);

        // CLI
        let (success, out, latency, error) = run_cli(cli_args).unwrap_or((false, String::new(), 0.0, "Command failed".into()));
        let mut passed = success;
        let mut size = 0;
        let mut err_msg = error;
        if passed {
             if let Some(ref path) = check_output_file {
                 if path.exists() {
                     size = fs::metadata(path).unwrap().len() as usize;
                     passed = size > 0;
                 } else {
                     passed = false;
                     err_msg = "Output file not generated".into();
                 }
             } else {
                 size = out.len();
                 passed = size > 0;
             }
        }
        results.push(TestExecutionResult {
            tool_id: tool_id.to_string(),
            interface: "CLI".into(),
            latency_ms: latency,
            output_size: size,
            passed,
            error_notes: err_msg,
        });

        // MCP
        let (success, resp, latency, error) = call_mcp_tool(tool_id, mcp_payload).unwrap_or((false, json!({}), 0.0, "MCP failed".into()));
        let mut passed = success;
        let mut size = 0;
        let mut err_msg = error;
        if passed {
             if let Some(ref path) = check_output_file {
                 if path.exists() {
                     size = fs::metadata(path).unwrap().len() as usize;
                     passed = size > 0;
                 } else {
                     passed = false;
                     err_msg = "Output file not generated".into();
                 }
             } else {
                 size = resp.to_string().len();
                 passed = size > 0;
             }
        }
        results.push(TestExecutionResult {
            tool_id: tool_id.to_string(),
            interface: "MCP".into(),
            latency_ms: latency,
            output_size: size,
            passed,
            error_notes: err_msg,
        });

        // API, WASM, Desktop - Not tested in MCP package layer explicitly due to constraints, recording as omitted.
        results.push(TestExecutionResult {
            tool_id: tool_id.to_string(),
            interface: "API".into(),
            latency_ms: 0.0,
            output_size: 0,
            passed: false,
            error_notes: "Not tested in Rust MCP".into(),
        });
        results.push(TestExecutionResult {
            tool_id: tool_id.to_string(),
            interface: "WASM".into(),
            latency_ms: 0.0,
            output_size: 0,
            passed: false,
            error_notes: "Not tested in Rust MCP".into(),
        });
        results.push(TestExecutionResult {
            tool_id: tool_id.to_string(),
            interface: "Desktop".into(),
            latency_ms: 0.0,
            output_size: 0,
            passed: false,
            error_notes: "Not tested in Rust MCP".into(),
        });
    };


    // 1. pdf_extract_text
    run_tool_test(
        "pdf_extract_text",
        &["extract-text", "--input", &sample_pdf, "--json"],
        json!({"input": sample_pdf}),
        None,
        &mut results
    );

    // 2. pdf_ocr
    let ocr_cli_out = out_dir.path().join("ocr_cli.pdf");
    let ocr_mcp_out = out_dir.path().join("ocr_mcp.pdf");
    run_tool_test(
        "pdf_ocr",
        &["ocr", "--input", &ocr_pdf, "--output", ocr_cli_out.to_str().unwrap(), "--json"],
        json!({"input": ocr_pdf, "output": ocr_mcp_out.to_str().unwrap()}),
        Some(ocr_cli_out.clone()),
        &mut results
    );

    // 3. pdf_extract_tables
    let tables_cli_out = out_dir.path().join("tables_cli.csv");
    let tables_mcp_out = out_dir.path().join("tables_mcp.json");
    run_tool_test(
        "pdf_extract_tables",
        &["extract-tables", "--input", &sample_pdf, "--output", tables_cli_out.to_str().unwrap(), "--format", "csv", "--pages", "all", "--json"],
        json!({"input": sample_pdf, "output": tables_mcp_out.to_str().unwrap(), "format": "json", "pages": "all"}),
        Some(tables_cli_out.clone()),
        &mut results
    );

    // 4. document_pii_scan
    let pii_cli_out = out_dir.path().join("pii_cli.json");
    let pii_mcp_out = out_dir.path().join("pii_mcp.json");
    run_tool_test(
        "document_pii_scan",
        &["pii-scan", "--input", &sample_pdf, "--output", pii_cli_out.to_str().unwrap(), "--json"],
        json!({"input": sample_pdf, "output": pii_mcp_out.to_str().unwrap()}),
        Some(pii_cli_out.clone()),
        &mut results
    );

    // 5. pdf_bookmarks
    run_tool_test(
        "pdf_bookmarks",
        &["bookmarks", "--input", &multi_page_pdf, "--json"],
        json!({"input": multi_page_pdf}),
        None,
        &mut results
    );

    // 6. pdf_bates
    let bates_cli_out = out_dir.path().join("bates_cli.pdf");
    let bates_mcp_out = out_dir.path().join("bates_mcp.pdf");
    run_tool_test(
        "pdf_bates",
        &["bates", "--input", &multi_page_pdf, "--output", bates_cli_out.to_str().unwrap(), "--prefix", "BATES", "--start", "1", "--json"],
        json!({"input": multi_page_pdf, "prefix": "BATES", "start": 1, "output": bates_mcp_out.to_str().unwrap()}),
        Some(bates_cli_out.clone()),
        &mut results
    );

    // 7. pdf_header_footer
    let hf_cli_out = out_dir.path().join("hf_cli.pdf");
    let hf_mcp_out = out_dir.path().join("hf_mcp.pdf");
    run_tool_test(
        "pdf_header_footer",
        &["header-footer", "--input", &sample_pdf, "--output", hf_cli_out.to_str().unwrap(), "--text", "HEADER", "--json"],
        json!({"input": sample_pdf, "text": "HEADER", "output": hf_mcp_out.to_str().unwrap()}),
        Some(hf_cli_out.clone()),
        &mut results
    );


    // 8. pdf_classify_type
    run_tool_test(
        "pdf_classify_type",
        &["classify", "--input", &sample_pdf, "--json"],
        json!({"input": sample_pdf}),
        None,
        &mut results
    );


    // 9. pdf_compare
    let compare_mcp_out = out_dir.path().join("compare_mcp.json");
    run_tool_test(
        "pdf_compare",
        &["compare", "--input", &sample_pdf, "--input-b", &sample_pdf, "--json"],
        json!({"input_a": sample_pdf, "input_b": sample_pdf, "output": compare_mcp_out.to_str().unwrap()}),
        None,
        &mut results
    );


    // 10. pdf_hash
    run_tool_test(
        "pdf_hash",
        &["hash", "--input", &sample_pdf, "--json"],
        json!({"input": sample_pdf}),
        None,
        &mut results
    );

    // 11. pdf_validate
    run_tool_test(
        "pdf_validate",
        &["validate", "--input", &sample_pdf, "--json"],
        json!({"input": sample_pdf}),
        None,
        &mut results
    );


    // 12. pdf_render
    let render_cli_out = out_dir.path().join("render_cli.png");
    let render_mcp_out = out_dir.path().join("render_mcp.png");
    run_tool_test(
        "pdf_render",
        &["render", "--input", &sample_pdf, "--output", render_cli_out.to_str().unwrap(), "--json"],
        json!({"input": sample_pdf, "output": render_mcp_out.to_str().unwrap()}),
        Some(render_cli_out.clone()),
        &mut results
    );


    // 13. pdf_rag_chunks
    run_tool_test(
        "pdf_rag_chunks",
        &["rag-chunks", "--input", &sample_pdf, "--json"],
        json!({"input": sample_pdf}),
        None,
        &mut results
    );


    // 14. workspace_search
    let ws = workspace_root.join("tests/e2e_fixtures/real").to_string_lossy().to_string();
    run_tool_test(
        "workspace_search",
        &["search", "--query", "test", "--dir", &ws, "--json"],
        json!({"query": "test", "dir": ws.clone()}),
        None,
        &mut results
    );


    // 15. semantic_search
    run_tool_test(
        "semantic_search",
        &["search", "--query", "test", "--dir", "./docs", "--json"],
        json!({"query": "test", "limit": 5}),
        None,
        &mut results
    );

    // Generate Markdown Report
    println!("Generating report...");
    let mut md = String::new();
    md.push_str("# Wave 8 Analysis Intelligence Penta-Interface E2E Scorecard\n\n");
    let passed = results.iter().filter(|r| r.passed).count();
    let total = results.len();
    md.push_str(&format!("- **Total Tests Evaluated:** {}\n", total));
    md.push_str(&format!("- **Total Passed:** {}\n\n", passed));

    md.push_str("## Operations Status\n\n");
    md.push_str("| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status | Error Notes |\n");
    md.push_str("|-----------|-----------|--------------|---------------------|--------|-------------|\n");

    let tools = vec!["pdf_extract_text", "pdf_ocr", "pdf_extract_tables", "document_pii_scan", "pdf_bookmarks", "pdf_bates", "pdf_header_footer", "pdf_classify_type", "pdf_compare", "pdf_hash", "pdf_validate", "pdf_render", "pdf_rag_chunks", "workspace_search", "semantic_search"];

    for tool in tools {
        for res in results.iter().filter(|r| r.tool_id == tool) {
            let status = if res.passed { "✅ PASS" } else { "❌ FAIL" };
            md.push_str(&format!("| `{}` | {} | {:.1} | {} | {} | {} |\n",
                res.tool_id, res.interface, res.latency_ms, res.output_size, status, res.error_notes));
        }
    }

    fs::create_dir_all(workspace_root.join("reports")).unwrap();
    let report_path = workspace_root.join("reports/WAVE_8_ANALYSIS_INTELLIGENCE_PENTA_SCORECARD.md");
    fs::write(&report_path, md).unwrap();
    println!("Report written to {:?}", report_path);

    Ok(())
}
