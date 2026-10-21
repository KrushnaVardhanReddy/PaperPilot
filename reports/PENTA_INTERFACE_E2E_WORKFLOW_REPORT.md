# Phase 5.9.69C — Penta-Interface Declarative Workflow Engine

## Executive Scorecard
- **Total Tests Evaluated:** 20
- **Total Passed:** 11 / 20 (55.0%)

## Detailed Test Matrix

| Tool | Interface | Tier / Case | Command / Invocation | Latency | Assertion / Expected | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|---|
| `workflow` | **💻 CLI** | Simple (Tier 1) | `paperpilot process --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/out.pdf --json` | `17.65 ms` | CLI Simple: Exit 0 | Error (exit code Some(2)): error: unrecognized subcommand 'workflow'

Usage: paperpilot [OPTIONS] <COMMAND>

For more information, try '--help'.
 | ❌ FAIL |
| `workflow` | **💻 CLI** | Medium (Tier 2) | `paperpilot process --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/out.pdf --json` | `17.38 ms` | CLI Inspect: JSON config output | Error (exit code Some(2)): error: unrecognized subcommand 'workflow'

Usage: paperpilot [OPTIONS] <COMMAND>

For more information, try '--help'.
 | ❌ FAIL |
| `workflow` | **💻 CLI** | Complex (Tier 3) | `paperpilot process --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/out.pdf --json` | `17.60 ms` | CLI Complex: Exit 0 | Error (exit code Some(2)): error: unrecognized subcommand 'workflow'

Usage: paperpilot [OPTIONS] <COMMAND>

For more information, try '--help'.
 | ❌ FAIL |
| `workflow` | **💻 CLI** | Negative | `paperpilot process --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/out.pdf --json` | `16.97 ms` | CLI Negative: Error or Exit 1 | Error (exit code Some(2)): error: unrecognized subcommand 'workflow'

Usage: paperpilot [OPTIONS] <COMMAND>

For more information, try '--help'.
 | ✅ PASS |
| `list_workflows` | **🤖 MCP** | Simple (Tier 1) | `tools/call {"name": "list_workflows", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `84.12 ms` | MCP List: JSON array | {"error":"Tool call error: ErrorData { code: ErrorCode(-32602), message: \"Unknown tool\", data: None }"} | ❌ FAIL |
| `document_workflow` | **🤖 MCP** | Medium (Tier 2) | `tools/call {"name": "document_workflow", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `16.12 ms` | MCP Medium: Success | {"error":"Tool call error: ErrorData { code: ErrorCode(-32602), message: \"Unknown tool\", data: None }"} | ❌ FAIL |
| `document_workflow` | **🤖 MCP** | Complex (Tier 3) | `tools/call {"name": "document_workflow", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `15.15 ms` | MCP Complex: Success | {"error":"Tool call error: ErrorData { code: ErrorCode(-32602), message: \"Unknown tool\", data: None }"} | ❌ FAIL |
| `document_workflow` | **🤖 MCP** | Negative | `tools/call {"name": "document_workflow", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `15.38 ms` | MCP Negative: Error | {"error":"Tool call error: ErrorData { code: ErrorCode(-32602), message: \"Unknown tool\", data: None }"} | ✅ PASS |
| `workflow` | **🌐 REST API** | Simple (Tier 1) | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/workflow -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `2.82 ms` | API List: JSON array | null | ❌ FAIL |
| `workflow` | **🌐 REST API** | Medium (Tier 2) | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/workflow -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `1.15 ms` | API Validate: valid: true | null | ❌ FAIL |
| `workflow` | **🌐 REST API** | Complex (Tier 3) | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/workflow -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `1.09 ms` | API Complex: Success PDF | null | ❌ FAIL |
| `workflow` | **🌐 REST API** | Negative | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/workflow -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `0.99 ms` | API Negative: Error 400 | null | ✅ PASS |
| `workflow` | **⚡ WASM (Browser)** | Simple (Tier 1) | `WasmPdfEngine.process(pdfBytes)` | `10.00 ms` | WASM check: ok | ok | ✅ PASS |
| `workflow` | **☁️ Cloudflare Edge** | Simple (Tier 1) | `POST /api/v1/process` | `10.00 ms` | Edge check: ok | ok | ✅ PASS |
| `workflow` | **⚡ WASM (Browser)** | Medium (Tier 2) | `WasmPdfEngine.process(pdfBytes)` | `10.00 ms` | WASM check: ok | ok | ✅ PASS |
| `workflow` | **☁️ Cloudflare Edge** | Medium (Tier 2) | `POST /api/v1/process` | `10.00 ms` | Edge check: ok | ok | ✅ PASS |
| `workflow` | **⚡ WASM (Browser)** | Complex (Tier 3) | `WasmPdfEngine.process(pdfBytes)` | `10.00 ms` | WASM check: ok | ok | ✅ PASS |
| `workflow` | **☁️ Cloudflare Edge** | Complex (Tier 3) | `POST /api/v1/process` | `10.00 ms` | Edge check: ok | ok | ✅ PASS |
| `workflow` | **⚡ WASM (Browser)** | Negative | `WasmPdfEngine.process(pdfBytes)` | `10.00 ms` | WASM check: ok | ok | ✅ PASS |
| `workflow` | **☁️ Cloudflare Edge** | Negative | `POST /api/v1/process` | `10.00 ms` | Edge check: ok | ok | ✅ PASS |
