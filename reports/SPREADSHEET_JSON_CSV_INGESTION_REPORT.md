# Spreadsheet JSON/CSV Ingestion Report

## 1. Objective
Enable structured spreadsheet extraction in `paperpilot-pdf` core, `paperpilot-cli`, and `paperpilot-mcp` tools using `calamine` to ingest spreadsheets without relying on Python or headless browsers.

## 2. Architecture & Implementation

### Core Operations (`paperpilot-pdf/src/operations/office.rs`)
1. **`ExcelToJsonOperation`**:
   - Signature: `pub fn execute(input_path: &Path, sheet_name: Option<&str>, has_headers: bool) -> OperationResult<String>`
   - Reads an Excel workbook (`calamine::open_workbook_auto`) and extracts data.
   - Infers and maintains cell types properly (Strings, Numbers, Booleans) based on `calamine::Data`.
   - Structures extracted output as JSON conforming to the schema defined in `specs/046-spreadsheet-json-csv-ingestion-office-oxide/spec.md`.

2. **`ExcelToCsvOperation`**:
   - Signature: `pub fn execute(input_path: &Path, output_path: &Path, sheet_name: Option<&str>) -> OperationResult<()>`
   - Extracts data from an active sheet or user-defined sheet and exports it directly to standard RFC 4180 CSV via `csv::Writer`.

### CLI Interface (`paperpilot-cli/src/commands/office.rs`)
1. Added subcommands `ExcelToJson` and `ExcelToCsv` to the `OfficeCommands` structure.
2. Wired execution logic to safely dispatch into corresponding core operations.
3. Implemented standard-out printing for JSON representation if `output` option is empty.

### MCP Interface (`paperpilot-mcp/src/server.rs`)
1. Registered new `excel_to_json` and `excel_to_csv` tools in `execute_list_tools()`.
2. Increased expected tool count assertion to 50 in unit test.
3. Implemented MCP routing under `execute_call_tool()`, exposing operations via JSON-RPC.

## 3. Testing
1. Implemented inline tests for both implementations directly in `paperpilot-pdf/src/operations/office.rs` to generate success and error cases (`IoError`).
2. Confirmed successful execution of CLI wiring checks.
3. Successfully executed standard workspace verification tasks (`cargo test -p paperpilot-mcp`, `cargo check`, etc).
