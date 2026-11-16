---
title: "excel_to_csv"
description: "Reference, CLI, MCP, and REST API commands for excel_to_csv."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: Excel (`.xlsx` / `.xls`) $\to$ Standard CSV (`.csv`)

### What it does
**Exports Excel workbooks (.xlsx, .xls) into standard RFC 4180 CSV text files.**  
Extracts spreadsheet data quickly with high fidelity without requiring Microsoft Office or external runtime dependencies.

- **Multi-Sheet Automatic Decomposition**: When exporting a multi-sheet workbook without specifying a single sheet, the tool creates the target CSV for the primary sheet and automatically exports each individual worksheet as `<output>_<SheetName>.csv`.
- **Targeted Sheet Extraction**: Pass `--sheet <name>` (or `"sheet"` in MCP/REST) to export only a specific sheet.
- **Data Type Fidelity**: Accurately formats text, numeric, date, and boolean cell values into standard comma-separated representations with proper quoting.

---

### CLI

`excel_to_csv` is invoked via `paperpilot office excel-to-csv`:

```bash
# Export all sheets (primary + individual per-sheet CSVs)
paperpilot office excel-to-csv \
  --input tests/e2e_fixtures/real/test.xlsx \
  --output tests/e2e_fixtures/out/batch16_test/excel_out.csv \
  --json
```

```bash
# Export a specific worksheet
paperpilot office excel-to-csv \
  --input tests/e2e_fixtures/real/test.xlsx \
  --sheet "Financial_Summary" \
  --output tests/e2e_fixtures/out/batch16_test/financial.csv \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to input `.xlsx` or `.xls` spreadsheet.
- `--output` *(string, required)*: Destination CSV file path.
- `--sheet` *(string, optional)*: Specific worksheet name to export.
- `--json` *(flag, optional)*: Output structured JSON result.

#### Example CLI Output
```json
{"success":true,"operation":"Office","error":null}
```

Generated files when multi-sheet workbook is exported without `--sheet`:
- `tests/e2e_fixtures/out/batch16_test/excel_out.csv`
- `tests/e2e_fixtures/out/batch16_test/excel_out_Sheet1.csv`
- `tests/e2e_fixtures/out/batch16_test/excel_out_Financial_Summary.csv`

---

### MCP JSON-RPC

Invoke `excel_to_csv` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "excel_to_csv",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.xlsx",
      "output": "tests/e2e_fixtures/out/batch16_test/mcp_excel.csv",
      "sheet": "Sheet1"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to input Excel file.
- `output` *(string, required)*: Path to save the output CSV file.
- `sheet` *(string, optional)*: Specific sheet name to export.

#### Example MCP Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Excel sheet exported to CSV\",\"output_path\":\"tests/e2e_fixtures/out/batch16_test/mcp_excel.csv\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API

Export Excel sheets to CSV via the PaperPilot Gateway endpoints:

```bash
# Office route
curl -s -X POST http://127.0.0.1:7823/api/v1/office/excel-to-csv \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.xlsx",
    "output": "tests/e2e_fixtures/out/batch16_test/rest_excel.csv"
  }'
```

```bash
# Canonical PDF route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/excel_to_csv \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.xlsx",
    "output": "tests/e2e_fixtures/out/batch16_test/rest_excel.csv"
  }'
```

#### Example Gateway Response
```json
{
  "message": "Success",
  "output": {
    "content": [
      {
        "text": "{\"success\":true,\"message\":\"Excel sheet exported to CSV\",\"output_path\":\"tests/e2e_fixtures/out/batch16_test/rest_excel.csv\",\"data\":null}",
        "type": "text"
      }
    ],
    "isError": false,
    "resultType": "complete"
  },
  "request_id": "aabee97f-eb51-4059-931f-5a58193fdaf6",
  "success": true
}
```

---

### WASM Support
Excel spreadsheet binary ingestion and file extraction require native filesystem access and run locally via the CLI, REST API Gateway, or MCP server.

---

### Known Limitations & Troubleshooting
- **Sheet Name Case Sensitivity**: When filtering by `--sheet`, sheet names are case-sensitive and must match the exact tab name in the Excel workbook.
- **Embedded Charts & Macros**: Only tabular cell data and cell values are exported; embedded macro scripts (`.xlsm`) and charts are omitted from CSV output.
