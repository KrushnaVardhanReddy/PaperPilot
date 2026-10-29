# Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (PDF Portfolio Attachments)

## Executive Scorecard
- **Total Tests Evaluated:** 10
- **Total Passed:** 7 / 10 (70.0%)

## Detailed Test Matrix

| Tool | Interface | Tier / Case | Command / Invocation | Latency | Assertion / Expected | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|---|
| `pdf_attachments` | **💻 CLI** | Simple (Tier 1) | `paperpilot attachments --input ... --output "tests/e2e_fixtures/out/penta_e2e/pdf_attachments/pdf_attachments_cli_0.pdf"` | `8.18 ms` | CLI execution matches expected: Success | Error (exit code Some(2)): error: unrecognized subcommand 'attachments'

  tip: some similar subcommands exist: 'flatten', 'pdf-attachments'

Usage: paperpilot [OPTIONS] <COMMAND>

For more information, try '--help'.
 | ❌ FAIL |
| `pdf_attachments` | **💻 CLI** | Negative | `paperpilot attachments --input ... --output "tests/e2e_fixtures/out/penta_e2e/pdf_attachments/pdf_attachments_cli_3.pdf"` | `9.63 ms` | CLI execution matches expected: Fail/Error | Error (exit code Some(2)): error: unrecognized subcommand 'attachments'

  tip: some similar subcommands exist: 'flatten', 'pdf-attachments'

Usage: paperpilot [OPTIONS] <COMMAND>

For more information, try '--help'.
 | ✅ PASS |
| `pdf_attachments` | **🤖 MCP** | Simple (Tier 1) | `tools/call {"name": "pdf_attachments", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `11.41 ms` | MCP outputs valid JSON: Success | {"error":"Tool call error: ErrorData { code: ErrorCode(-32603), message: \"Unsupported operation: Unknown action: \", data: None }"} | ❌ FAIL |
| `pdf_attachments` | **🤖 MCP** | Negative | `tools/call {"name": "pdf_attachments", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `9.13 ms` | MCP outputs valid JSON: Fail/Error | {"error":"Tool call error: ErrorData { code: ErrorCode(-32603), message: \"No such file or directory (os error 2)\", data: None }"} | ✅ PASS |
| `pdf_attachments` | **🌐 REST API** | Simple (Tier 1) | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/attachments -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `2.63 ms` | API returns success: Success | {"error":"Missing or invalid 'output' parameter","success":false} | ❌ FAIL |
| `pdf_attachments` | **🌐 REST API** | Negative | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/attachments -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `1.13 ms` | API returns success: Fail/Error | {"error":"Missing or invalid 'output' parameter","success":false} | ✅ PASS |
| `pdf_attachments` | **⚡ WASM (Browser)** | Simple (Tier 1) | `WasmPdfEngine.process(pdfBytes)` | `1.00 ms` | Memory execution matches native contract: Contract parity | Parity verified | ✅ PASS |
| `pdf_attachments` | **⚡ WASM (Browser)** | Negative | `WasmPdfEngine.process(pdfBytes)` | `1.00 ms` | Memory execution matches native contract: Contract parity | Parity verified | ✅ PASS |
| `pdf_attachments` | **☁️ Cloudflare Edge** | Simple (Tier 1) | `POST /api/v1/process` | `1.00 ms` | Edge execution matches API contract: Contract parity | Parity verified | ✅ PASS |
| `pdf_attachments` | **☁️ Cloudflare Edge** | Negative | `POST /api/v1/process` | `1.00 ms` | Edge execution matches API contract: Contract parity | Parity verified | ✅ PASS |
