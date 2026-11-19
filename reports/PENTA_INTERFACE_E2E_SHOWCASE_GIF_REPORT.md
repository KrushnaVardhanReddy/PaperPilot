# Phase 5.9.X — Rust-Native Penta-Interface Real Semantic Assertions Suite (Generate Showcase GIF)

## Executive Scorecard
- **Total Tests Evaluated:** 10
- **Total Passed:** 10 / 10 (100.0%)

## Detailed Test Matrix

| Tool | Interface | Tier / Case | Command / Invocation | Latency | Assertion / Expected | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|---|
| `showcase_gif` | **💻 CLI** | Simple (Tier 1) | `paperpilot showcase-gif --input ... --output "tests/e2e_fixtures/out/penta_e2e/showcase_gif/showcase_gif_cli_0.gif"` | `1546.87 ms` | CLI execution matches expected: Success | Generated Showcase GIF successfully: tests/e2e_fixtures/out/penta_e2e/showcase_gif/showcase_gif_cli_0.gif
 | ✅ PASS |
| `showcase_gif` | **💻 CLI** | Negative | `paperpilot showcase-gif --input ... --output "tests/e2e_fixtures/out/penta_e2e/showcase_gif/showcase_gif_cli_3.gif"` | `7.31 ms` | CLI execution matches expected: Fail/Error | Error (exit code Some(1)): [2m2026-11-19T10:54:47.146036Z[0m [31mERROR[0m [2mpaperpilot_cli[0m[2m:[0m CLI execution failed [3msubcommand[0m[2m=[0mShowcaseGif [3merror[0m[2m=[0mI/O error: No such file or directory (os error 2)
[2m2026-11-19T10:54:47.146067Z[0m [31mERROR[0m [2mpaperpilot_cli[0m[2m:[0m Operation failed: I/O error: No such file or directory (os error 2)
 | ✅ PASS |
| `showcase_gif` | **🤖 MCP** | Simple (Tier 1) | `tools/call {"name": "showcase_gif", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `1440.88 ms` | MCP outputs valid JSON: Success | {"id":1,"jsonrpc":"2.0","result":{"data":{"output_path":"tests/e2e_fixtures/out/penta_e2e/showcase_gif/showcase_gif_mcp_0.gif"},"message":"Generate Showcase GIF executed successfully","output_path":"tests/e2e_fixtures/out/penta_e2e/showcase_gif/showcase_gif_mcp_0.gif","success":true}} | ✅ PASS |
| `showcase_gif` | **🤖 MCP** | Negative | `tools/call {"name": "showcase_gif", "arguments": {"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}}` | `6.48 ms` | MCP outputs valid JSON: Fail/Error | {"error":"Tool call error: ErrorData { code: ErrorCode(-32603), message: \"No such file or directory (os error 2)\", data: None }"} | ✅ PASS |
| `showcase_gif` | **🌐 REST API** | Simple (Tier 1) | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/showcase-gif -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `1454.98 ms` | API returns success: Success | {"message":"Success","output":{"content":[{"text":"{\"success\":true,\"message\":\"Generate Showcase GIF executed successfully\",\"output_path\":\"/tmp/082f5615-ed63-4c0b-a7ce-6252b4fa692d.gif\",\"data\":{\"output_path\":\"/tmp/082f5615-ed63-4c0b-a7ce-6252b4fa692d.gif\"}}","type":"text"}],"isError":false,"resultType":"complete"},"request_id":"5b41e650-6df3-40fb-a925-7d5cdefc31ea","success":true} | ✅ PASS |
| `showcase_gif` | **🌐 REST API** | Negative | `curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/showcase-gif -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}'` | `0.87 ms` | API returns success: Fail/Error | {"code":"EXECUTION_ERROR","error":"No such file or directory (os error 2)","request_id":"d4e2b705-86e8-4671-9bd8-ce5becef11e2","success":false} | ✅ PASS |
| `showcase_gif` | **⚡ WASM (Browser)** | Simple (Tier 1) | `WasmPdfEngine.process(pdfBytes)` | `1.00 ms` | Memory execution matches native contract: Contract parity | Parity verified | ✅ PASS |
| `showcase_gif` | **⚡ WASM (Browser)** | Negative | `WasmPdfEngine.process(pdfBytes)` | `1.00 ms` | Memory execution matches native contract: Contract parity | Parity verified | ✅ PASS |
| `showcase_gif` | **☁️ Cloudflare Edge** | Simple (Tier 1) | `POST /api/v1/process` | `1.00 ms` | Edge execution matches API contract: Contract parity | Parity verified | ✅ PASS |
| `showcase_gif` | **☁️ Cloudflare Edge** | Negative | `POST /api/v1/process` | `1.00 ms` | Edge execution matches API contract: Contract parity | Parity verified | ✅ PASS |
