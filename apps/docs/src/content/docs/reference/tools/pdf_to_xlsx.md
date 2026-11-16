---
title: "pdf_to_xlsx"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_xlsx."
sidebar:
  order: 50
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ Excel Spreadsheet (`.xlsx`)

### What it does
**Extracts tabular data and financial statements from PDF into an Excel workbook (.xlsx).**  
Detects table structures, spatial cell boundaries, and textual values to generate clean, editable multi-column spreadsheets.

- **Heuristic Table Boundary Detection**: Identifies aligned multi-column rows and financial tables from spatial coordinate layouts.
- **Direct Workbook Generation**: Compiles extracted cell records directly into modern OpenXML spreadsheet sheets (`.xlsx`).
- **Privacy & Security**: Operates completely in-memory and locally without third-party cloud data transmission.

---

### CLI
Convert PDF tables to `.xlsx` spreadsheet format using the `convert` command:

```bash
paperpilot convert \
  --input tests/e2e_fixtures/real/financial_tables.pdf \
  --format xlsx \
  --output tests/e2e_fixtures/out/penta_e2e/financial_statement.xlsx \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to input PDF file containing tables.
- `--format` *(string, required)*: Target output format (`xlsx`).
- `--output` *(string, optional, default: `"output.xlsx"`)*: Destination path for the `.xlsx` workbook.
- `--json` *(flag, optional)*: Emit structured JSON completion status.

---

### MCP JSON-RPC
Invoke `pdf_to_xlsx` through Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_xlsx",
    "arguments": {
      "input": "tests/e2e_fixtures/real/financial_tables.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/financial_statement.xlsx"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `output` *(string, required)*: The output XLSX destination file path.

#### Example MCP Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Converted to XLSX successfully.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/financial_statement.xlsx\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API
Extract tables to Excel workbooks via PaperPilot Gateway:

```bash
# Universal MCP Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_xlsx \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/financial_tables.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/financial_statement.xlsx"
  }'
```

```bash
# Format Conversion Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/financial_tables.pdf",
    "format": "xlsx",
    "output": "tests/e2e_fixtures/out/penta_e2e/financial_statement.xlsx"
  }'
```

#### Example Gateway Response
```json
{
  "success": true,
  "data": null,
  "error": null
}
```

---

### WASM Support
Direct client-side `pdf_to_xlsx` generation is not currently exposed in `paperpilot-wasm`. For browser environments requiring tabular extraction, use `extract_tables` or call the backend via REST API / MCP.

---

### Known Limitations & Troubleshooting
- **CLI Subcommand Invocation**: Use `paperpilot convert --input <INPUT> --format xlsx --output <OUTPUT>`. There is no separate `paperpilot pdf-to-xlsx` binary subcommand.
- **Borderless & Merged Cells**: Highly complex irregular merged cells or borderless multi-line headers may be extracted into adjacent single cells. For raw coordinate cell inspection, run `extract_tables` to obtain JSON grid coordinates.
- **Formula Reconstruction**: Extracted values are written as plain scalar strings or numbers; calculated formula relationships (`=SUM(...)`) are not reconstructed from static PDF files.
