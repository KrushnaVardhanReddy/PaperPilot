# Task 5.9.58: Pure-Rust Tabular Data Extraction API (`pdf_extract_tables`)

## Status
- Spec: [Spec 090](file:///home/krushna/Project/PaperPilot/specs/090-pure-rust-pdf-extract-tables/spec.md)
- Target: Wave 3 (`v0.7.0`) Advanced Intelligence & Structured Data
- Status: ⏳ In-Flight (Jules Session [`334056780263731633`](https://jules.google.com/session/334056780263731633))

---

## Objectives & Work Items

### 1. Single Core Runner (`paperpilot-pdf/src/operations/table.rs`)
- Implement `extract_tables_bytes(input_bytes: &[u8], options: &TableExtractOptions) -> Result<TableExtractResult, PdfError>`.
- Parse page text streams and positions using `lopdf`.
- Form rows via vertical proximity clustering ($Y$-axis threshold).
- Form columns via horizontal alignment clustering ($X$-axis projections).
- Construct `ExtractedTable` matrices with bounding boxes, row/col counts, header rows, and data cells.
- Serialize to structured JSON and RFC 4180 CSV.
- Add comprehensive unit tests in `table.rs`.

### 2. 5 Thin Interface Wrappers
- **CLI (`paperpilot-cli`)**: Add subcommand `extract-tables` (`Commands::ExtractTables`) in `cli.rs` and handler in `commands/table.rs`.
- **MCP (`paperpilot-mcp`)**: Add `pdf_extract_tables` declaration and execute arm in `server.rs`.
- **REST Gateway (`paperpilot-gateway`)**: Add `POST /api/v1/pdf/extract-tables` in `server.rs` and handler in `handlers.rs`.
- **Browser WASM (`paperpilot-wasm`)**: Expose `WasmPdfEngine.extract_tables` in `lib.rs` and update `list_tools`.
- **Edge Microservice (`apps/edge`)**: Expose `POST /api/v1/extract-tables` in `src/index.ts`.

### 3. Desktop & Embed UI Integration
- Add `pdf_extract_tables` tool definition in `apps/desktop/src/lib/types/toolList.ts`.
- Add parameter card in `OperationsPanel.svelte` with format selector (`JSON` / `CSV`).
- Update `apps/desktop/src/lib/utils/snippetGenerator.ts` to output copyable CLI, cURL, MCP, and Embed JS snippets.

### 4. Penta-Interface E2E Verification
- Create `tools/penta-interface-e2e/src/cases/table.rs`.
- Verify 10/10 unmocked real assertions across all 5 surfaces.
- Generate `reports/PENTA_INTERFACE_E2E_TABLE_REPORT.md`.
