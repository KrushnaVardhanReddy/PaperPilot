# Spec 052 — Penta-Interface E2E Suite Expansion for New Tools & Unmocked Playwright Desktop Verification

## 1. Overview & Problem Statement
PaperPilot previously established a 44-tool Penta-Interface test matrix across 5 surfaces (880 tests) and edge cases (385 tests). However, two critical verification gaps have emerged:

1. **Missing Penta-Interface Coverage for Newly Added Tools**:
   Recent milestones introduced tools that are not yet covered in `tools/penta-interface-e2e`:
   - `excel_to_json` (Spec 046 / Phase 5.9.21): Tabular spreadsheet ingestion.
   - `excel_to_csv` (Spec 046 / Phase 5.9.21): RFC 4180 CSV export.
   - `pdf_rag_chunks` (Spec 040 / Phase 5.9.15): Structured semantic chunking for vector DBs.
   - `pdf_to_html` (Spec 040 / Phase 5.9.15): Responsive semantic HTML5 conversion.
   - `workspace_search` (Spec 050 / Phase 5.9.22): Cross-document multi-format search.

2. **The "Mock Trap" in Desktop Playwright E2E**:
   `apps/desktop/tests/e2e_44_operations_parity.spec.ts` mocks `invoke_mcp_tool` with `{ success: true }`. Consequently, if `OperationsPanel.svelte`, `operationArgsBuilder.ts`, or Tauri IPC serializes the wrong parameter keys (e.g. passing `inputs` instead of `input_files`), the Playwright test still passes 100% while the desktop UI breaks in real user testing!

---

## 2. Technical Requirements

### 2.1 Expanding `tools/penta-interface-e2e` for the 5 New Tools (100 New Assertions)
For each new tool (`excel_to_json`, `excel_to_csv`, `pdf_rag_chunks`, `pdf_to_html`, `workspace_search`), implement **20 assertions across 4 tiers and 5 surfaces**:
1. **Surfaces**:
   - 💻 **CLI**: Subcommand invocations via `CliRunner`.
   - 🤖 **MCP**: In-memory `execute_call_tool` via `McpRunner`.
   - 🌐 **REST API**: HTTP POST via `GatewayServer` & `ApiRunner`.
   - ⚡ **WASM**: In-memory byte processing without filesystem IO.
   - ☁️ **Cloudflare Edge**: In-memory buffer simulation.
2. **Complexity Tiers**:
   - **Tier 1 (Simple)**: Minimal valid input (e.g., standard 1-sheet `.xlsx` or 1-page PDF).
   - **Tier 2 (Medium)**: Multi-sheet Excel, multi-page layout with tables/columns.
   - **Tier 3 (Complex)**: Real-world large documents (formatted financial sheets, academic multi-column papers).
   - **Tier 4 (Negative)**: Corrupted bytes, missing files, or out-of-bounds queries returning typed errors without panicking.

### 2.2 Unmocked Real-Execution Playwright E2E Suite (`tests/desktop_real_execution.spec.ts`)
Implement an unmocked Playwright test suite for the **Top 12 Core Launch Tools**:
1. `pdf_merge`
2. `pdf_split`
3. `pdf_rotate`
4. `pdf_delete_pages`
5. `pdf_reorder_pages`
6. `pdf_compress`
7. `pdf_watermark`
8. `pdf_ocr`
9. `pdf_extract_text`
10. `pdf_to_docx`
11. `excel_to_json`
12. `excel_to_csv`

**Execution Flow**:
- Do **NOT** mock `invoke_mcp_tool`.
- Open real fixture files from `tests/e2e_fixtures/real/`.
- Interact with `OperationsPanel` / tool cards, fill parameter inputs, and click **Run**.
- Await real file creation on disk in `target/e2e_output/desktop/`.
- Inspect generated files using `lopdf` or filesystem checks to assert file non-emptiness, valid PDF magic header (`%PDF-`), and correct page counts.

---

## 3. Architecture & Constraints
- Concurrency limit: Bounded to 6 cores (`-j 6`).
- Zero flake: Playwright tests must use explicit status element polling (`data-testid="operation-status"`) and timeout guards.
- Output artifacts: All test runs generate markdown scorecards in `reports/` with latencies and exact assertions.
