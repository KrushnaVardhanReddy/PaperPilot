# Spec 056 — Penta-Interface Conversions Parity & Gateway Route Completion

## 1. Overview & Objective
PaperPilot requires 100% real semantic execution parity across all 5 deployment surfaces:
1. 💻 **CLI** (`paperpilot-cli`)
2. 🤖 **MCP** (`paperpilot-mcp`)
3. 🌐 **REST API** (`paperpilot-gateway`)
4. ⚡ **WASM** (`paperpilot-wasm`)
5. ☁️ **Cloudflare Edge** (`apps/edge`)

In Phase 5.9.4D (`tools/penta-interface-e2e`), 280 / 301 tests (93.0%) pass with deep semantic validations. Exactly 21 tests fail due to:
1. Missing Gateway REST route handlers for `office_to_markdown`, `office_extract_text`, and `pdf_rag_chunks`.
2. CLI subcommand name mismatch in the test runner (`extract-text` vs `extract`).
3. Missing binary fixture `tests/e2e_fixtures/real/test.xls` for `office_modernize_legacy`.

This specification defines the exact contracts, routes, and fixture requirements to bring the conversion suite to 100% pass rate.

---

## 2. Technical Requirements

### A. Gateway REST Route Handlers (`paperpilot-gateway`)
Add the following routes to `paperpilot-gateway/src/server.rs` and implement their handlers in `paperpilot-gateway/src/handlers.rs`:

1. **`POST /api/v1/pdf/office-to-markdown`**:
   - Request Body: `{"input": "<path>", "output": "<path>"}`
   - Action: Calls `paperpilot_pdf::operations::office::convert_office_to_markdown(input, output)`
   - Response: `200 OK` with JSON `{"success": true, "output": "<path>", "message": "Converted office to markdown"}`

2. **`POST /api/v1/pdf/office-extract-text`**:
   - Request Body: `{"input": "<path>", "output": "<path>"}`
   - Action: Calls `paperpilot_pdf::operations::office::extract_office_text(input, output)`
   - Response: `200 OK` with JSON `{"success": true, "output": "<path>", "message": "Extracted text from office document"}`

3. **`POST /api/v1/pdf/rag-chunks`**:
   - Request Body: `{"input": "<path>", "max_chunk_size": <optional usize>, "overlap": <optional usize>}`
   - Action: Calls `paperpilot_pdf::operations::rag_chunks::generate_rag_chunks(input, ...)`
   - Response: `200 OK` with JSON `{"success": true, "chunks": [...], "count": <usize>}`

### B. CLI Runner Argument Synchronization
In `tools/penta-interface-e2e/src/cases/conversions_office_tools.rs`:
- Line 210 invokes `paperpilot office extract-text`.
- The CLI command in `paperpilot-cli` is `paperpilot office extract`.
- Update the runner invocation or ensure CLI alias accepts `extract-text` as well as `extract`.

### C. Legacy Binary Fixture Generation (`test.xls`)
In `tools/penta-interface-e2e/src/fixtures.rs`:
- Ensure `FixtureManager::ensure_fixtures()` checks for `tests/e2e_fixtures/real/test.xls`.
- If missing, synthesize a valid BIFF8 / compound document binary `.xls` or generate it automatically so that `office_modernize_legacy` tests run against a real, existing fixture.

### D. Zero Mocking Mandate
- No hardcoded `let mut passed = true;` or dummy bypasses.
- All 301 tests in `cargo run -p penta-interface-e2e -- --group conversions` must pass authentically.
