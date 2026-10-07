# Spec 027: Deep Behavioral Assertions & Unified Comparison Matrix for Tri-Interface E2E Suite

## Status: APPROVED / READY FOR IMPLEMENTATION

## 1. Problem Statement
The current Tri-Interface E2E verification report (`reports/TRI_INTERFACE_E2E_100_VERIFIED.md` and `reports/TRI_INTERFACE_E2E_AND_DOCS.md`) validates that tools exit with code 0 and output files exist. However, the report outputs verbose, disconnected code blocks and does not display:
1. **Concrete Use Cases**: What scenario or input condition is being exercised (e.g. merging two specific PDFs of 1 and 2 pages).
2. **Behavioral Assertions**: Expected state vs. Actual / Received state (e.g., Expected: 2 pages, valid `%PDF-` header, non-zero bytes; Received: 2 pages, 1,711 bytes).
3. **Unified Interface Comparison**: Commands, latencies, expectations, and received outcomes across CLI, MCP, and REST API are separated into fragmented paragraphs instead of a clear, consolidated comparison table.

---

## 2. Objective & Scope
Enhance `scripts/test_tri_interface_e2e.py` to:
1. Define explicit **Use Case** and **Assertion Rules** (expected page count, header byte checks, output file size, or JSON payload keys) for each of the 44 tools.
2. Inspect actual output artifacts post-execution:
   - For PDF outputs: Verify `%PDF-` magic bytes, exact page count, and non-empty byte size.
   - For text/JSON/image/archive outputs: Verify file existence, byte size, format header, and payload content.
3. Replace the legacy fragmented report format with an **Executive Unified Verification Table** per tool, containing:
   - Interface (💻 CLI / 🤖 MCP / 🌐 REST API)
   - Invocation Command / Payload
   - Measured Latency (ms)
   - Expected Result
   - Actual / Received Result
   - Verdict (`✅ PASS` / `❌ FAIL`)

---

## 3. Unified Table Format Specification

For each tool, the report must output:

```markdown
### Tool: `<tool_id>`
> **Use Case**: <Clear description of input and purpose>

| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |
|---|---|---|---|---|---|---|
| `<tool_id>` | **💻 CLI** | `paperpilot <cmd> ...` | `14.2 ms` | <Expected outcome> | <Received outcome with exact metrics> | ✅ PASS |
| `<tool_id>` | **🤖 MCP** | `tools/call {"name": "...", ...}` | `15.8 ms` | <Expected outcome> | <Received outcome with exact metrics> | ✅ PASS |
| `<tool_id>` | **🌐 REST API** | `POST /api/v1/pdf/... {...}` | `24.1 ms` | <Expected outcome> | <Received outcome with exact metrics> | ✅ PASS |
| `<tool_id>` | **⚡ WASM (Browser)** | `WasmPdfEngine.<op>(...)` | `3.5 ms` | <Expected outcome> | <Received outcome with exact metrics> | ✅ PASS |
| `<tool_id>` | **☁️ Cloudflare Edge** | `POST /api/v1/<op> ...` | `9.2 ms` | <Expected outcome> | <Received outcome with exact metrics> | ✅ PASS |
```

> For tools not supported on browser WASM or Edge serverless (e.g. heavy OCR, desktop Office conversions), mark the WASM / Edge rows as `N/A (Desktop/Server only)`.

---

## 4. Verification Acceptance Criteria
1. Running `python3 scripts/test_tri_interface_e2e.py` executes all 44 tools across CLI, MCP, and REST API (132 assertions).
2. The generated report at `reports/TRI_INTERFACE_E2E_100_VERIFIED.md` (and `reports/TRI_INTERFACE_E2E_AND_DOCS.md`) renders the unified table for every tool.
3. Every table entry displays real, measured "Actual / Received" metrics (page counts, byte sizes, status codes) proving the tool achieved its intended transformation.
