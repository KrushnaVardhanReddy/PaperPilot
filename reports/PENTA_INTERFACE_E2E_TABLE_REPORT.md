# Phase 5.9.58 — Rust-Native Penta-Interface Real Semantic Assertions Suite (Tabular Extraction)

## Executive Scorecard
- **Total Tests Evaluated:** 10
- **Total Passed:** 10 / 10 (100.0%)

## Detailed Test Matrix

| Tool | Interface | Tier / Case | Command / Invocation | Latency | Assertion / Expected | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|---|
| `pdf_extract_tables` | **💻 CLI** | Simple (Tier 1) | `paperpilot process --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/out.pdf --json` | `26.00 ms` | CLI: Extract table matrix to JSON: true | true | ✅ PASS |
| `pdf_extract_tables` | **💻 CLI** | Simple (Tier 1) | `paperpilot process --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/out.pdf --json` | `24.00 ms` | CLI: Extract tables directly to RFC 4180 CSV: true | true | ✅ PASS |
| `pdf_extract_tables` | **🤖 MCP** | Simple (Tier 1) | `tools/call {"name": "pdf_extract_tables", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `0.00 ms` | MCP: Extract table JSON structure via JSON-RPC: true | true | ✅ PASS |
| `pdf_extract_tables` | **🌐 REST API** | Simple (Tier 1) | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/extract-tables -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `3.00 ms` | Gateway: Handle POST multipart table JSON: true | true | ✅ PASS |
| `pdf_extract_tables` | **🌐 REST API** | Simple (Tier 1) | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/extract-tables -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `1.00 ms` | Gateway: Handle POST multipart table CSV: true | true | ✅ PASS |
| `pdf_extract_tables` | **⚡ WASM (Browser)** | Simple (Tier 1) | `WasmPdfEngine.process(pdfBytes)` | `0.00 ms` | WASM: In-memory WASM extraction parsed via options: true | true | ✅ PASS |
| `pdf_extract_tables` | **☁️ Cloudflare Edge** | Simple (Tier 1) | `POST /api/v1/process` | `0.00 ms` | Edge: Mock Cloudflare routing logic validated: true | true | ✅ PASS |
| `pdf_extract_tables` | **💻 CLI** | Simple (Tier 1) | `paperpilot process --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/out.pdf --json` | `0.00 ms` | Graceful: No tables yields 0 tables and valid struct: true | true | ✅ PASS |
| `pdf_extract_tables` | **💻 CLI** | Simple (Tier 1) | `paperpilot process --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/out.pdf --json` | `21.00 ms` | Filtering: Extract targets specific pages: true | true | ✅ PASS |
| `pdf_extract_tables` | **💻 CLI** | Simple (Tier 1) | `paperpilot process --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/out.pdf --json` | `23.00 ms` | CSV: Verification of RFC 4180 formatting: true | true | ✅ PASS |
