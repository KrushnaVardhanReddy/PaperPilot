# Spec 046 — Structured Spreadsheet Ingestion & Tabular JSON/CSV Agent Tooling via `office_oxide`

## 1. Overview & Problem Statement
Modern AI agents (e.g. Claude Desktop, Cursor, OpenCode, AutoGPT) frequently encounter spreadsheet documents (`.xlsx`, `.xls`). 
Currently, PaperPilot's conversion pipeline can render spreadsheets to styled HTML/PDF (`ExcelToStyledHtmlOperation`), but AI agents cannot easily query or extract structured tabular data directly:
1. Converting an Excel spreadsheet to JSON currently requires either external Python tools (`pandas`, `openpyxl`) or manual parsing.
2. Ingesting multi-sheet workbooks into structured tabular arrays with cell types (strings, numbers, dates, booleans) is missing from the MCP and CLI interface.
3. Fast, zero-dependency CSV export per sheet (`excel_to_csv`) is required for streaming data into vector stores or database tables.

`office_oxide` (v0.1.13) provides pure-Rust, zero-dependency, ultra-fast reading and editing of OOXML (`.xlsx`) and legacy binary (`.xls`) spreadsheets with zero Python and zero native C libraries. In addition, PaperPilot's workspace already bundles `calamine` and `csv` in `paperpilot-pdf`.

**Objective:**
Implement high-performance, structured spreadsheet extraction tools:
1. **`excel_to_json`**: Ingest `.xlsx` or `.xls` workbooks into a clean structured JSON schema:
   ```json
   {
     "sheets": [
       {
         "name": "Q3 Revenue",
         "rows_count": 120,
         "columns": ["Region", "Revenue", "Margin", "Growth"],
         "data": [
           {"Region": "North America", "Revenue": 1420500.0, "Margin": 0.34, "Growth": 0.12},
           {"Region": "EMEA", "Revenue": 980200.0, "Margin": 0.28, "Growth": 0.08}
         ]
       }
     ]
   }
   ```
2. **`excel_to_csv`**: Export specified or active sheet from `.xlsx` / `.xls` directly to clean, standard RFC 4180 CSV with configurable delimiter.

---

## 2. Target Features & Operations

### 2.1 Core Operations (`paperpilot-pdf/src/operations/office.rs`)
- **`ExcelToJsonOperation`**:
  - `execute(input_path: &Path, sheet_name: Option<&str>, has_headers: bool) -> OperationResult<String>`
  - If `sheet_name` is `None`, extracts all sheets into the top-level `sheets` array.
  - If `has_headers` is `true` (default), the first non-empty row is treated as column keys, yielding an array of key-value row objects.
  - If `has_headers` is `false`, returns a 2D array of values `[["Col0", "Col1"], ["Val0", "Val1"]]`.
  - Type inference preserves numbers (integers / floats) and booleans rather than turning everything into raw strings.

- **`ExcelToCsvOperation`**:
  - `execute(input_path: &Path, output_path: &Path, sheet_name: Option<&str>) -> OperationResult<()>`
  - Streams selected sheet (defaulting to the first sheet) into standard RFC 4180 CSV using the workspace `csv` crate.

### 2.2 CLI Commands (`paperpilot-cli/src/commands/office.rs`)
- `paperpilot office excel-to-json --input <PATH.xlsx> [--sheet <NAME>] [--no-headers] [--output <PATH.json>]`
  - If `--output` is omitted, prints formatted JSON to stdout.
- `paperpilot office excel-to-csv --input <PATH.xlsx> --output <PATH.csv> [--sheet <NAME>]`

### 2.3 MCP Tool Registration (`paperpilot-mcp/src/server.rs`)
- Register tools:
  - `excel_to_json`: parameters `input` (string, required), `sheet` (string, optional), `has_headers` (boolean, optional).
  - `excel_to_csv`: parameters `input` (string, required), `output` (string, required), `sheet` (string, optional).
- Wire into `execute_list_tools` and `execute_call_tool`.
- Return standard JSON responses with structured data.

### 2.4 REST Gateway (`paperpilot-gateway/src/handlers.rs`)
- Route `POST /api/v1/office/excel-to-json`
- Route `POST /api/v1/office/excel-to-csv`

---

## 3. Strict Concurrency & Safety Guardrails
1. **Bounded Concurrency**: All `cargo test`, `cargo build`, and `cargo clippy` calls must use `-j 6`.
2. **Pure-Rust & Memory Budget**: Pure Rust only, zero Python, zero headless browser, RSS memory < 35MB.
3. **MCP Tool Count Assertion**: In `paperpilot-mcp/src/server.rs::tests::test_execute_list_tools`, update the expected tool count assertion accordingly (+2 tools: 48 -> 50, or match actual total).

---

## 4. Acceptance Criteria
1. `paperpilot-pdf`: Unit tests in `office.rs` verify parsing multi-row and multi-column spreadsheets into valid JSON objects and CSV files.
2. `paperpilot-cli`: CLI commands `office excel-to-json` and `office excel-to-csv` succeed.
3. `paperpilot-mcp`: Both tools pass `test_execute_list_tools` and direct JSON-RPC execution.
4. Clean workspace: `cargo clippy --workspace --all-targets --all-features -- -j 6 -- -D warnings` passes with zero warnings.
5. Create report `reports/SPREADSHEET_JSON_CSV_INGESTION_REPORT.md` and update `PHASES.md`.
