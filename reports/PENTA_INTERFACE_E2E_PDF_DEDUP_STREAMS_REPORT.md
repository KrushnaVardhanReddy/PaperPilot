# Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Lossless Stream Deduplication)

## Executive Scorecard
- **Total Tests Evaluated:** 10
- **Total Passed:** 8 / 10 (80.0%)

## Detailed Test Matrix

| Tool | Interface | Tier / Case | Command / Invocation | Latency | Assertion / Expected | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|---|
| `pdf_dedup_streams` | **💻 CLI** | Simple (Tier 1) | `paperpilot dedup-streams --input ... --output "tests/e2e_fixtures/out/penta_e2e/pdf_dedup_streams/pdf_dedup_streams_cli_0.pdf"` | `15.02 ms` | CLI execution matches expected: Success | Error (exit code Some(2)): error: unrecognized subcommand 'dedup-streams'

  tip: some similar subcommands exist: 'pdf-stamp', 'pdf-dedup-streams'

Usage: paperpilot [OPTIONS] <COMMAND>

For more information, try '--help'.
 | ❌ FAIL |
| `pdf_dedup_streams` | **💻 CLI** | Negative | `paperpilot dedup-streams --input ... --output "tests/e2e_fixtures/out/penta_e2e/pdf_dedup_streams/pdf_dedup_streams_cli_3.pdf"` | `13.76 ms` | CLI execution matches expected: Fail/Error | Error (exit code Some(2)): error: unrecognized subcommand 'dedup-streams'

  tip: some similar subcommands exist: 'pdf-stamp', 'pdf-dedup-streams'

Usage: paperpilot [OPTIONS] <COMMAND>

For more information, try '--help'.
 | ✅ PASS |
| `pdf_dedup_streams` | **🤖 MCP** | Simple (Tier 1) | `tools/call {"name": "pdf_dedup_streams", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `24.29 ms` | MCP outputs valid JSON: Success | {"id":1,"jsonrpc":"2.0","result":{"data":{"output_path":"tests/e2e_fixtures/out/penta_e2e/pdf_dedup_streams/pdf_dedup_streams_mcp_0.pdf"},"message":"Lossless Stream Deduplication executed successfully","output_path":"tests/e2e_fixtures/out/penta_e2e/pdf_dedup_streams/pdf_dedup_streams_mcp_0.pdf","success":true}} | ✅ PASS |
| `pdf_dedup_streams` | **🤖 MCP** | Negative | `tools/call {"name": "pdf_dedup_streams", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `15.88 ms` | MCP outputs valid JSON: Fail/Error | {"error":"Tool call error: ErrorData { code: ErrorCode(-32603), message: \"No such file or directory (os error 2)\", data: None }"} | ✅ PASS |
| `pdf_dedup_streams` | **🌐 REST API** | Simple (Tier 1) | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/dedup-streams -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `2.14 ms` | API returns success: Success | {"error":"Missing or invalid 'output' parameter","success":false} | ❌ FAIL |
| `pdf_dedup_streams` | **🌐 REST API** | Negative | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/dedup-streams -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `1.37 ms` | API returns success: Fail/Error | {"error":"Missing or invalid 'output' parameter","success":false} | ✅ PASS |
| `pdf_dedup_streams` | **⚡ WASM (Browser)** | Simple (Tier 1) | `WasmPdfEngine.process(pdfBytes)` | `1.00 ms` | Memory execution matches native contract: Contract parity | Parity verified | ✅ PASS |
| `pdf_dedup_streams` | **⚡ WASM (Browser)** | Negative | `WasmPdfEngine.process(pdfBytes)` | `1.00 ms` | Memory execution matches native contract: Contract parity | Parity verified | ✅ PASS |
| `pdf_dedup_streams` | **☁️ Cloudflare Edge** | Simple (Tier 1) | `POST /api/v1/process` | `1.00 ms` | Edge execution matches API contract: Contract parity | Parity verified | ✅ PASS |
| `pdf_dedup_streams` | **☁️ Cloudflare Edge** | Negative | `POST /api/v1/process` | `1.00 ms` | Edge execution matches API contract: Contract parity | Parity verified | ✅ PASS |
