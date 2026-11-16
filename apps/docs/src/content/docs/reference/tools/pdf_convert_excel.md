---
title: "pdf_convert_excel"
description: "Reference, CLI, MCP, and REST API commands for pdf_convert_excel."
sidebar:
  order: 14
---

**Status**: ✅ Verified  |  **Format**: Excel (`.xlsx` / `.xls`) / CSV (`.csv`) $\to$ Publication PDF (`.pdf`)

### What it does
**Renders spreadsheet workbooks (Excel `.xlsx`/`.xls` or CSV) directly into publication-ready styled PDF documents.**  
Powered by a semantic table-to-PDF formatting engine, `pdf_convert_excel` transforms raw worksheets, columns, and tabular datasets into beautifully formatted PDF documents with customizable typography presets, orientation controls, and pagination.

- **Theme Presets**: Built-in CSS styling presets including `github`, `elegant`, `minimal`, `branded`, and `compact`.
- **Custom CSS Support**: Inject custom stylesheets or inline CSS overrides for table headers, row striping, borders, and typography.
- **Page Geometry Control**: Configurable page orientations (`portrait` or `landscape`) and standard paper sizes (`A4`, `Letter`).
- **Multi-Sheet Handling**: Formats sheets cleanly with appropriate page flow.

> [!NOTE]
> To extract data in the reverse direction (exporting PDF tables to Excel), see `pdf_to_xlsx` or `pdf_extract_tables`. `pdf_convert_excel` converts spreadsheets *into* PDF.

---

### CLI

Execute Excel-to-PDF conversions using the dedicated `convert-excel` command or the unified `convert` command:

```bash
# Using convert-excel
paperpilot convert-excel \
  --input tests/e2e_fixtures/real/test.xlsx \
  --output tests/e2e_fixtures/out/batch16_test/excel_out.pdf \
  --preset elegant \
  --orientation landscape \
  --json
```

```bash
# Using unified convert with CSV or XLSX
paperpilot convert \
  --input tests/e2e_fixtures/real/test.csv \
  --format pdf \
  --output tests/e2e_fixtures/out/batch16_test/csv_out.pdf \
  --json
```

*(Command aliases supported: `convert-excel`, `convert_excel`, `excel-to-pdf`, `pdf_convert_excel`)*

#### CLI Parameters
- `--input` *(string, required)*: Path to source `.xlsx`, `.xls`, or `.csv` spreadsheet file.
- `--output` *(string, required)*: Destination path for rendered PDF document.
- `--preset` *(string, optional)*: Visual theme preset (`github`, `elegant`, `minimal`, `branded`, `compact`).
- `--css` / `--custom-css` *(string, optional)*: Path to an external CSS stylesheet or custom styling string.
- `--orientation` *(string, optional)*: Page orientation (`portrait` or `landscape`).
- `--json` *(flag, optional)*: Output structured JSON result.

#### Example CLI Output
```json
{"success":true,"operation":"ConvertExcel","error":null}
```

---

### MCP JSON-RPC

Invoke `pdf_convert_excel` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_excel",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.xlsx",
      "output": "tests/e2e_fixtures/out/batch16_test/mcp_excel.pdf",
      "preset": "elegant",
      "orientation": "landscape",
      "page_size": "A4"
    }
  },
  "id": 1
}
```

*(MCP tool name aliases supported: `pdf_convert_excel`, `convert_excel`, `excel_to_pdf`)*

#### MCP Schema Parameters
- `input` *(string, required)*: Path to input spreadsheet file (`.xlsx`, `.xls`, or `.csv`).
- `output` *(string, required)*: Path for output PDF document.
- `preset` *(string, optional)*: Visual theme preset (`github`, `elegant`, `minimal`, `branded`, `compact`).
- `custom_css` *(string, optional)*: Custom CSS rules to apply.
- `page_size` *(string, optional)*: Page dimensions (e.g. `A4`, `Letter`).
- `orientation` *(string, optional)*: Page orientation (`portrait` or `landscape`).

#### Example MCP Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Excel converted to PDF successfully.\",\"output_path\":\"tests/e2e_fixtures/out/batch16_test/mcp_excel.pdf\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API

Execute spreadsheet conversions via PaperPilot Gateway:

```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_convert_excel \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.xlsx",
    "output": "tests/e2e_fixtures/out/batch16_test/rest_excel.pdf",
    "preset": "elegant",
    "orientation": "landscape"
  }'
```

#### Example Gateway Response
```json
{
  "message": "Success",
  "output": {
    "content": [
      {
        "text": "{\"success\":true,\"message\":\"Excel converted to PDF successfully.\",\"output_path\":\"tests/e2e_fixtures/out/batch16_test/rest_excel.pdf\",\"data\":null}",
        "type": "text"
      }
    ],
    "isError": false,
    "resultType": "complete"
  },
  "request_id": "477a25e5-cfdd-46d8-9522-62f600233df3",
  "success": true
}
```

---

### WASM Support
Spreadsheet layout rendering depends on OS-level font loading and filesystem access, so execution runs natively via the CLI, REST API Gateway, or MCP daemon.

---

### Known Limitations & Troubleshooting
- **Wide Tables**: For spreadsheets with many columns, set `"orientation": "landscape"` or choose the `compact` preset to prevent horizontal column clipping across margins.
- **Formulas vs. Cached Values**: Complex Excel dynamic formulas evaluate using cached cell values computed during the last workbook save. Ensure workbooks are saved with calculated values before conversion.
- **Directionality Note**: To export PDF tables into Excel sheets, use `pdf_to_xlsx` (`paperpilot convert --input doc.pdf --format xlsx`). `pdf_convert_excel` converts spreadsheets *to* PDF.
