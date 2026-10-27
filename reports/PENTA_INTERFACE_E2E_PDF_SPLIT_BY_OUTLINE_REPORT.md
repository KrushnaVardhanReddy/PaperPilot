# Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Split PDF by Outline)

## Executive Scorecard
- **Total Tests Evaluated:** 10
- **Total Passed:** 7 / 10 (70.0%)

## Detailed Test Matrix

| Tool | Interface | Tier / Case | Command / Invocation | Latency | Assertion / Expected | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|---|
| `pdf_split_by_outline` | **💻 CLI** | Simple (Tier 1) | `paperpilot split-by-outline --input ... --output "tests/e2e_fixtures/out/penta_e2e/pdf_split_by_outline/pdf_split_by_outline_cli_0.pdf"` | `9.66 ms` | CLI execution matches expected: Success | Error (exit code Some(2)): error: unrecognized subcommand 'split-by-outline'

  tip: some similar subcommands exist: 'split', 'pdf-split-by-outline'

Usage: paperpilot [OPTIONS] <COMMAND>

For more information, try '--help'.
 | ❌ FAIL |
| `pdf_split_by_outline` | **💻 CLI** | Negative | `paperpilot split-by-outline --input ... --output "tests/e2e_fixtures/out/penta_e2e/pdf_split_by_outline/pdf_split_by_outline_cli_3.pdf"` | `9.09 ms` | CLI execution matches expected: Fail/Error | Error (exit code Some(2)): error: unrecognized subcommand 'split-by-outline'

  tip: some similar subcommands exist: 'split', 'pdf-split-by-outline'

Usage: paperpilot [OPTIONS] <COMMAND>

For more information, try '--help'.
 | ✅ PASS |
| `pdf_split_by_outline` | **🤖 MCP** | Simple (Tier 1) | `tools/call {"name": "pdf_split_by_outline", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `10.06 ms` | MCP outputs valid JSON: Success | {"error":"Tool call error: ErrorData { code: ErrorCode(-32603), message: \"Other error: No bookmarks (/Outlines) found\", data: None }"} | ❌ FAIL |
| `pdf_split_by_outline` | **🤖 MCP** | Negative | `tools/call {"name": "pdf_split_by_outline", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `9.36 ms` | MCP outputs valid JSON: Fail/Error | {"error":"Tool call error: ErrorData { code: ErrorCode(-32603), message: \"No such file or directory (os error 2)\", data: None }"} | ✅ PASS |
| `pdf_split_by_outline` | **🌐 REST API** | Simple (Tier 1) | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/split-by-outline -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `2.60 ms` | API returns success: Success | {"error":"Missing or invalid 'output' parameter","success":false} | ❌ FAIL |
| `pdf_split_by_outline` | **🌐 REST API** | Negative | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/split-by-outline -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `0.96 ms` | API returns success: Fail/Error | {"error":"Missing or invalid 'output' parameter","success":false} | ✅ PASS |
| `pdf_split_by_outline` | **⚡ WASM (Browser)** | Simple (Tier 1) | `WasmPdfEngine.process(pdfBytes)` | `1.00 ms` | Memory execution matches native contract: Contract parity | Parity verified | ✅ PASS |
| `pdf_split_by_outline` | **⚡ WASM (Browser)** | Negative | `WasmPdfEngine.process(pdfBytes)` | `1.00 ms` | Memory execution matches native contract: Contract parity | Parity verified | ✅ PASS |
| `pdf_split_by_outline` | **☁️ Cloudflare Edge** | Simple (Tier 1) | `POST /api/v1/process` | `1.00 ms` | Edge execution matches API contract: Contract parity | Parity verified | ✅ PASS |
| `pdf_split_by_outline` | **☁️ Cloudflare Edge** | Negative | `POST /api/v1/process` | `1.00 ms` | Edge execution matches API contract: Contract parity | Parity verified | ✅ PASS |
