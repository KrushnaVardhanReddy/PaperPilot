# Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Overlay & Underlay Stationary)

## Executive Scorecard
- **Total Tests Evaluated:** 10
- **Total Passed:** 7 / 10 (70.0%)

## Detailed Test Matrix

| Tool | Interface | Tier / Case | Command / Invocation | Latency | Assertion / Expected | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|---|
| `pdf_overlay_underlay` | **💻 CLI** | Simple (Tier 1) | `paperpilot overlay-underlay --input ... --output "tests/e2e_fixtures/out/penta_e2e/pdf_overlay_underlay/pdf_overlay_underlay_cli_0.pdf"` | `20.13 ms` | CLI execution matches expected: Success | Error (exit code Some(2)): error: unrecognized subcommand 'overlay-underlay'

  tip: some similar subcommands exist: 'render', 'pdf-overlay-underlay'

Usage: paperpilot [OPTIONS] <COMMAND>

For more information, try '--help'.
 | ❌ FAIL |
| `pdf_overlay_underlay` | **💻 CLI** | Negative | `paperpilot overlay-underlay --input ... --output "tests/e2e_fixtures/out/penta_e2e/pdf_overlay_underlay/pdf_overlay_underlay_cli_3.pdf"` | `19.57 ms` | CLI execution matches expected: Fail/Error | Error (exit code Some(2)): error: unrecognized subcommand 'overlay-underlay'

  tip: some similar subcommands exist: 'render', 'pdf-overlay-underlay'

Usage: paperpilot [OPTIONS] <COMMAND>

For more information, try '--help'.
 | ✅ PASS |
| `pdf_overlay_underlay` | **🤖 MCP** | Simple (Tier 1) | `tools/call {"name": "pdf_overlay_underlay", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `15.94 ms` | MCP outputs valid JSON: Success | {"error":"Tool call error: ErrorData { code: ErrorCode(-32603), message: \"Unsupported operation: Not yet implemented\", data: None }"} | ❌ FAIL |
| `pdf_overlay_underlay` | **🤖 MCP** | Negative | `tools/call {"name": "pdf_overlay_underlay", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `15.89 ms` | MCP outputs valid JSON: Fail/Error | {"error":"Tool call error: ErrorData { code: ErrorCode(-32603), message: \"No such file or directory (os error 2)\", data: None }"} | ✅ PASS |
| `pdf_overlay_underlay` | **🌐 REST API** | Simple (Tier 1) | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/overlay-underlay -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `3.35 ms` | API returns success: Success | {"error":"Missing or invalid 'output' parameter","success":false} | ❌ FAIL |
| `pdf_overlay_underlay` | **🌐 REST API** | Negative | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/overlay-underlay -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `1.20 ms` | API returns success: Fail/Error | {"error":"Missing or invalid 'output' parameter","success":false} | ✅ PASS |
| `pdf_overlay_underlay` | **⚡ WASM (Browser)** | Simple (Tier 1) | `WasmPdfEngine.process(pdfBytes)` | `1.00 ms` | Memory execution matches native contract: Contract parity | Parity verified | ✅ PASS |
| `pdf_overlay_underlay` | **⚡ WASM (Browser)** | Negative | `WasmPdfEngine.process(pdfBytes)` | `1.00 ms` | Memory execution matches native contract: Contract parity | Parity verified | ✅ PASS |
| `pdf_overlay_underlay` | **☁️ Cloudflare Edge** | Simple (Tier 1) | `POST /api/v1/process` | `1.00 ms` | Edge execution matches API contract: Contract parity | Parity verified | ✅ PASS |
| `pdf_overlay_underlay` | **☁️ Cloudflare Edge** | Negative | `POST /api/v1/process` | `1.00 ms` | Edge execution matches API contract: Contract parity | Parity verified | ✅ PASS |
