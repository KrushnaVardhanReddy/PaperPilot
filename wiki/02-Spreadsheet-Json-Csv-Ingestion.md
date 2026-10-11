# Spreadsheet JSON/CSV Ingestion (`office_oxide` & `calamine`)

PaperPilot supports native, zero-dependency ingestion of spreadsheet documents (`.xlsx`, `.xls`) without relying on Python (pandas/openpyxl) or LibreOffice.

## Architecture & Logic

We leverage the `calamine` crate within `paperpilot-pdf` to implement structured extraction of workbook data. This logic preserves data types such as floats, ints, strings, and dates natively.

### Core Engine (`paperpilot-pdf/src/operations/office.rs`)
1. **`ExcelToJsonOperation`**: Parses a `.xlsx` or `.xls` file and extracts sheets as JSON structured objects. When `has_headers` is enabled, row vectors are serialized as HashMaps against the header column keys.
2. **`ExcelToCsvOperation`**: Reads spreadsheet rows and dumps them directly into RFC 4180 standard CSV output using the `csv::Writer`.

### CLI Interface (`paperpilot-cli/src/commands/office.rs`)
Users can access these tools natively from the `office` subcommand namespace:
- `paperpilot office excel-to-json --input data.xlsx --output data.json`
- `paperpilot office excel-to-csv --input data.xlsx --output data.csv`

### MCP Integration (`paperpilot-mcp/src/server.rs`)
Agents utilizing PaperPilot's Model Context Protocol (MCP) server can call these functions natively:
- **`excel_to_json`**: Extract highly formatted data matrices to direct LLM JSON structures.
- **`excel_to_csv`**: Export sheet data efficiently for RAG text extraction tools.
