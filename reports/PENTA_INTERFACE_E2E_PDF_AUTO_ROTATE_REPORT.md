# Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Auto Upright Orientation)

## Executive Scorecard
- **Total Tests Evaluated:** 10
- **Total Passed:** 7 / 10 (70.0%)

## Detailed Test Matrix

| Tool | Interface | Tier / Case | Command / Invocation | Latency | Assertion / Expected | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|---|
| `pdf_auto_rotate` | **💻 CLI** | Simple (Tier 1) | `paperpilot auto-rotate --input ... --output "tests/e2e_fixtures/out/penta_e2e/pdf_auto_rotate/pdf_auto_rotate_cli_0.pdf"` | `20.19 ms` | CLI execution matches expected: Success | Error (exit code Some(2)): error: unrecognized subcommand 'auto-rotate'

  tip: a similar subcommand exists: 'pdf-auto-rotate'

Usage: paperpilot [OPTIONS] <COMMAND>

For more information, try '--help'.
 | ❌ FAIL |
| `pdf_auto_rotate` | **💻 CLI** | Negative | `paperpilot auto-rotate --input ... --output "tests/e2e_fixtures/out/penta_e2e/pdf_auto_rotate/pdf_auto_rotate_cli_3.pdf"` | `19.66 ms` | CLI execution matches expected: Fail/Error | Error (exit code Some(2)): error: unrecognized subcommand 'auto-rotate'

  tip: a similar subcommand exists: 'pdf-auto-rotate'

Usage: paperpilot [OPTIONS] <COMMAND>

For more information, try '--help'.
 | ✅ PASS |
| `pdf_auto_rotate` | **🤖 MCP** | Simple (Tier 1) | `tools/call {"name": "pdf_auto_rotate", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `15.93 ms` | MCP outputs valid JSON: Success | {"error":"Tool call error: ErrorData { code: ErrorCode(-32603), message: \"Unsupported operation: Not yet implemented\", data: None }"} | ❌ FAIL |
| `pdf_auto_rotate` | **🤖 MCP** | Negative | `tools/call {"name": "pdf_auto_rotate", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `15.74 ms` | MCP outputs valid JSON: Fail/Error | {"error":"Tool call error: ErrorData { code: ErrorCode(-32603), message: \"No such file or directory (os error 2)\", data: None }"} | ✅ PASS |
| `pdf_auto_rotate` | **🌐 REST API** | Simple (Tier 1) | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/auto-rotate -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `4.05 ms` | API returns success: Success | {"error":"Missing or invalid 'output' parameter","success":false} | ❌ FAIL |
| `pdf_auto_rotate` | **🌐 REST API** | Negative | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/auto-rotate -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `1.53 ms` | API returns success: Fail/Error | {"error":"Missing or invalid 'output' parameter","success":false} | ✅ PASS |
| `pdf_auto_rotate` | **⚡ WASM (Browser)** | Simple (Tier 1) | `WasmPdfEngine.process(pdfBytes)` | `1.00 ms` | Memory execution matches native contract: Contract parity | Parity verified | ✅ PASS |
| `pdf_auto_rotate` | **⚡ WASM (Browser)** | Negative | `WasmPdfEngine.process(pdfBytes)` | `1.00 ms` | Memory execution matches native contract: Contract parity | Parity verified | ✅ PASS |
| `pdf_auto_rotate` | **☁️ Cloudflare Edge** | Simple (Tier 1) | `POST /api/v1/process` | `1.00 ms` | Edge execution matches API contract: Contract parity | Parity verified | ✅ PASS |
| `pdf_auto_rotate` | **☁️ Cloudflare Edge** | Negative | `POST /api/v1/process` | `1.00 ms` | Edge execution matches API contract: Contract parity | Parity verified | ✅ PASS |
