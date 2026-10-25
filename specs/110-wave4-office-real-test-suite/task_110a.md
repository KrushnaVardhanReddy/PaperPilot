# Task 110A: Implement Wave 4 Real Unmocked Testing Suite & Documentation

## Metadata
- **Spec**: [Spec 110: Wave 4 Testing Suite](file:///home/krushna/Project/PaperPilot/specs/110-wave4-office-real-test-suite/spec.md)
- **Phase**: Wave 4 (`v0.8.0`) Testing & Verification
- **Primary Language**: Rust 2021 & Markdown
- **Impacted Areas**: `paperpilot-mcp/tests/`, `wiki/`

---

## Detailed Requirements

### 1. Create `paperpilot-mcp/tests/wave4_mcp_tests.rs`
Mirror the established architecture in `paperpilot-mcp/tests/wave2_mcp_tests.rs`:
```rust
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
    if let rmcp::model::CallToolResponse::Complete(result) = res
        && let Some(ContentBlock::Text(text_content)) = result.content.first()
    {
        return serde_json::from_str(&text_content.text).expect("Valid JSON response");
    }
    panic!("Unexpected response format");
}
```
Test all 6 operations using workspace fixtures (`fixtures/Get_Started_With_Smallpdf.pdf` and `tests/e2e_fixtures/real/test.xlsx`):
1. `pdf_to_docx`
2. `pdf_to_xlsx`
3. `pdf_to_pptx`
4. `office_to_markdown`
5. `office_extract_text`
6. `office_modernize_legacy`

For each:
- Assert `res["success"] == true`.
- Assert output file exists and `metadata().len() > 0`.
- Assert magic bytes for `.docx`, `.xlsx`, `.pptx` start with `PK` (`[0x50, 0x4B]`).
- Assert `.md` output contains expected table formatting.

### 2. Create `wiki/33-Wave-4-Office-Penta-Testing.md`
Mirror `wiki/09-Wave-2-Penta-Testing.md`:
- Document the 6 Wave 4 tools.
- Provide copy-pasteable CLI commands, MCP JSON-RPC payloads, and REST API `curl` commands.
- Provide instructions on how to run `cargo test -p paperpilot-mcp --test wave4_mcp_tests`.

### 3. Verification
Run:
```bash
cargo test -p paperpilot-mcp --test wave4_mcp_tests -j 2
```
Ensure test passes cleanly.
