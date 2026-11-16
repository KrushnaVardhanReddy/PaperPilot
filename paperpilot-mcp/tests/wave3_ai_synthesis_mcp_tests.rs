use anyhow::Result;
use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::model::{CallToolRequestParams, ContentBlock};
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

pub struct TestExecutionResult {
    pub tool_id: String,
    pub interface: String,
    pub latency_ms: f64,
    pub output_size: usize,
    pub passed: bool,
    pub error: String,
}

fn call_mcp_tool(name: &str, args: serde_json::Value) -> (bool, usize, String, f64) {
    let mut params = CallToolRequestParams::default();
    params.name = name.to_string().into();
    if let serde_json::Value::Object(map) = args {
        params.arguments = Some(map);
    }

    let start = Instant::now();
    let res = PaperPilotMcpServer::new(None).execute_call_tool(params);
    let latency = start.elapsed().as_secs_f64() * 1000.0;

    match res {
        Ok(rmcp::model::CallToolResponse::Complete(result)) => {
            if let Some(ContentBlock::Text(text_content)) = result.content.first() {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text_content.text) {
                    let success = parsed["success"].as_bool().unwrap_or(false);
                    let err = parsed["error"].as_str().unwrap_or("").to_string();
                    let len = text_content.text.len();
                    return (success, len, err, latency);
                }
            }
            (false, 0, "Empty response".to_string(), latency)
        }
        Ok(_) => (false, 0, "Incomplete response".to_string(), latency),
        Err(e) => (false, 0, format!("{:?}", e), latency),
    }
}

pub fn get_workspace_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if manifest.ends_with("paperpilot-mcp") {
        manifest.parent().unwrap().to_path_buf()
    } else {
        manifest.parent().unwrap().parent().unwrap().to_path_buf()
    }
}

pub fn setup_test_env() -> PathBuf {
    let out_dir = get_workspace_root().join("tests/e2e_fixtures/out/wave3_penta_e2e");
    if out_dir.exists() {
        let _ = fs::remove_dir_all(&out_dir);
    }
    fs::create_dir_all(&out_dir).expect("Failed to create out_dir");
    out_dir
}

#[tokio::test]
async fn test_wave3_penta_interface() -> Result<()> {
    let out_dir = setup_test_env();
    let workspace_root = get_workspace_root();
    let mut results: Vec<TestExecutionResult> = Vec::new();

    let sample_pdf = workspace_root
        .join("fixtures/Get_Started_With_Smallpdf.pdf")
        .to_string_lossy()
        .to_string();

    let sample_json = workspace_root
        .join("tests/e2e_fixtures/real/json_doc.json")
        .to_string_lossy()
        .to_string();

    // 1. pdf_to_markdown
    {
        let out_file = out_dir.join("extracted.md").to_string_lossy().to_string();
        let (passed, _, err, latency) = call_mcp_tool(
            "pdf_to_markdown",
            json!({ "input": sample_pdf, "output": out_file }),
        );
        let out_size = fs::metadata(&out_file).map(|m| m.len() as usize).unwrap_or(0);
        results.push(TestExecutionResult {
            tool_id: "pdf_to_markdown".to_string(),
            interface: "MCP".to_string(),
            latency_ms: latency,
            output_size: out_size,
            passed: passed && out_size > 0,
            error: err,
        });
    }

    // 2. pdf_to_html
    {
        let out_file = out_dir.join("extracted.html").to_string_lossy().to_string();
        let (passed, _, err, latency) = call_mcp_tool(
            "pdf_to_html",
            json!({ "input": sample_pdf, "output": out_file }),
        );
        let out_size = fs::metadata(&out_file).map(|m| m.len() as usize).unwrap_or(0);
        results.push(TestExecutionResult {
            tool_id: "pdf_to_html".to_string(),
            interface: "MCP".to_string(),
            latency_ms: latency,
            output_size: out_size,
            passed: passed && out_size > 0,
            error: err,
        });
    }

    // 3. pdf_to_json
    {
        let out_file = out_dir.join("extracted.json").to_string_lossy().to_string();
        let (passed, _, err, latency) = call_mcp_tool(
            "pdf_to_json",
            json!({ "input": sample_pdf, "output": out_file }),
        );
        let out_size = fs::metadata(&out_file).map(|m| m.len() as usize).unwrap_or(0);
        results.push(TestExecutionResult {
            tool_id: "pdf_to_json".to_string(),
            interface: "MCP".to_string(),
            latency_ms: latency,
            output_size: out_size,
            passed: passed && out_size > 0,
            error: err,
        });
    }

    // 4. json_to_pdf
    {
        let out_file = out_dir.join("from_json.pdf").to_string_lossy().to_string();
        let (passed, _, err, latency) = call_mcp_tool(
            "json_to_pdf",
            json!({ "input": sample_json, "output": out_file }),
        );
        let out_size = fs::metadata(&out_file).map(|m| m.len() as usize).unwrap_or(0);
        results.push(TestExecutionResult {
            tool_id: "json_to_pdf".to_string(),
            interface: "MCP".to_string(),
            latency_ms: latency,
            output_size: out_size,
            passed: passed && out_size > 0,
            error: err,
        });
    }

    // 5. pdf_rag_chunks
    {
        let out_file = out_dir.join("rag_chunks.json").to_string_lossy().to_string();
        let (passed, _, err, latency) = call_mcp_tool(
            "pdf_rag_chunks",
            json!({ "input": sample_pdf, "output": out_file }),
        );
        let out_size = fs::metadata(&out_file).map(|m| m.len() as usize).unwrap_or(0);
        results.push(TestExecutionResult {
            tool_id: "pdf_rag_chunks".to_string(),
            interface: "MCP".to_string(),
            latency_ms: latency,
            output_size: out_size,
            passed: passed && out_size > 0,
            error: err,
        });
    }

    // 6. Documentation RAG Engine
    {
        let start = Instant::now();
        let engine = paperpilot_nlp::rag::DocumentationRagEngine::new();
        let ans = engine.query("How to encrypt with password?");
        let latency = start.elapsed().as_secs_f64() * 1000.0;
        let passed = ans.is_some() && ans.as_ref().unwrap().tool_name == "pdf_encrypt";
        results.push(TestExecutionResult {
            tool_id: "doc_rag_query".to_string(),
            interface: "Engine".to_string(),
            latency_ms: latency,
            output_size: if ans.is_some() { ans.unwrap().cli_example.len() } else { 0 },
            passed,
            error: if passed { String::new() } else { "No RAG match".to_string() },
        });
    }

    // Generate Markdown Scorecard
    let total_tests = results.len();
    let total_passed = results.iter().filter(|r| r.passed).count();

    let mut report = String::new();
    report.push_str("# Wave 3 AI & Document Synthesis Penta-Interface E2E Scorecard\n\n");
    report.push_str(&format!("- **Total Tests Evaluated:** {}\n", total_tests));
    report.push_str(&format!("- **Total Passed:** {}\n\n", total_passed));
    report.push_str("## Operations Status\n\n");
    report.push_str("| Tool Name | Interface | Latency (ms) | Output Size (bytes) | Status | Error |\n");
    report.push_str("|-----------|-----------|--------------|---------------------|--------|-------|\n");

    for r in &results {
        let status_str = if r.passed { "✅ PASS" } else { "❌ FAIL" };
        report.push_str(&format!(
            "| `{}` | {} | {:.1} | {} | {} | {} |\n",
            r.tool_id, r.interface, r.latency_ms, r.output_size, status_str, r.error
        ));
    }

    let report_path = workspace_root.join("reports/WAVE_3_AI_SYNTHESIS_PENTA_SCORECARD.md");
    fs::write(&report_path, report)?;
    println!("Scorecard written to: {:?}", report_path);

    Ok(())
}
