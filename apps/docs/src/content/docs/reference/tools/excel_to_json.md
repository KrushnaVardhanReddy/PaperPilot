---
title: "excel_to_json"
description: "Reference, CLI, MCP, and REST API commands for excel_to_json."
sidebar:
  order: 2
---

**Status**: ✅ Verified  |  **Format**: Excel (`.xlsx` / `.xls`) $\to$ Structured JSON (`.json`)

### What it does
**Ingests Excel workbooks (.xlsx, .xls) and converts sheets, rows, and columns into structured JSON arrays.**  
Parses rows and tabular columns into key-value JSON objects ready for AI agents, RAG pipelines, REST APIs, or database imports.

- **Cell Type Preservation**: Accurately preserves numbers as floats/integers, booleans, dates, and strings rather than flattening everything into text.
- **Configurable Header Parsing**: Automatically treats the first row as object keys (`has_headers: true` by default), or maps rows into index-based structures when disabled (`--no-headers`).
- **Disk or Direct Memory Output**: Optionally write output directly to an on-disk JSON file, or stream structured JSON directly in the response payload.

---

### CLI

`excel_to_json` is invoked via `paperpilot office excel-to-json`:

```bash
# Extract to disk with JSON output format
paperpilot office excel-to-json \
  --input tests/e2e_fixtures/real/test.xlsx \
  --output tests/e2e_fixtures/out/batch16_test/excel_out.json \
  --json
```

```bash
# Extract specific sheet without headers
paperpilot office excel-to-json \
  --input tests/e2e_fixtures/real/test.xlsx \
  --sheet "Financial_Summary" \
  --no-headers \
  --output tests/e2e_fixtures/out/batch16_test/financial.json \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to input Excel workbook (`.xlsx` or `.xls`).
- `--output` *(string, optional)*: Destination path to save JSON file.
- `--sheet` *(string, optional)*: Specific sheet name to extract (defaults to extracting all sheets).
- `--no-headers` *(flag, optional)*: Treat row 1 as data instead of column headers.
- `--json` *(flag, optional)*: Output structured JSON result.

#### Example CLI Output
```json
{"success":true,"operation":"Office","error":null}
```

#### Example Generated JSON Structure
```json
{
  "sheets": [
    {
      "name": "Sheet1",
      "rows_count": 6,
      "columns": ["ColA", "ColB", "ColC", "id", "product_name", "category", "unit_price", "quantity", "total_revenue"],
      "data": [
        {
          "ColA": "Value1",
          "ColB": 42.0,
          "ColC": 43.0,
          "id": 1.0,
          "product_name": "Item Book Pro",
          "category": "Office Supplies",
          "unit_price": 10.0,
          "quantity": 5.0,
          "total_revenue": 50.0
        }
      ]
    },
    {
      "name": "Financial_Summary",
      "rows_count": 5,
      "columns": ["Quarter", "Revenue", "OperatingExpenses", "EBITDA"],
      "data": [
        {
          "Quarter": "Q1 2026",
          "Revenue": 15200000.0,
          "OperatingExpenses": 4200000.0,
          "EBITDA": 11000000.0
        }
      ]
    }
  ]
}
```

---

### MCP JSON-RPC

Invoke `excel_to_json` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "excel_to_json",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.xlsx",
      "has_headers": true
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input Excel file.
- `output` *(string, optional)*: Optional destination path to save extracted JSON to disk.
- `sheet` *(string, optional)*: Optional specific sheet name to extract.
- `has_headers` *(boolean, optional, default: `true`)*: Whether the first row contains headers.

#### Example MCP Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Excel data extracted to JSON\",\"output_path\":null,\"data\":{\"sheets\":[{\"columns\":[\"ColA\",\"ColB\",\"ColC\",\"id\",\"product_name\",\"category\",\"unit_price\",\"quantity\",\"total_revenue\"],\"data\":[{\"ColA\":\"Value1\",\"ColB\":42.0,\"ColC\":43.0,\"category\":\"Office Supplies\",\"id\":1.0,\"product_name\":\"Item Book Pro\",\"quantity\":5.0,\"total_revenue\":50.0,\"unit_price\":10.0}],\"name\":\"Sheet1\",\"rows_count\":6}]}}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API

Extract spreadsheet data to JSON via PaperPilot Gateway endpoints:

```bash
# Office route
curl -s -X POST http://127.0.0.1:7823/api/v1/office/excel-to-json \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.xlsx"
  }'
```

```bash
# Canonical PDF route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/excel_to_json \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.xlsx"
  }'
```

#### Example Gateway Response
```json
{
  "message": "Success",
  "output": {
    "content": [
      {
        "text": "{\"success\":true,\"message\":\"Excel data extracted to JSON\",\"output_path\":null,\"data\":{\"sheets\":[...]}}",
        "type": "text"
      }
    ],
    "isError": false,
    "resultType": "complete"
  },
  "request_id": "f3792b7d-8b81-4264-99b8-1c7e349bd266",
  "success": true
}
```

---

### WASM Support
Excel spreadsheet binary ingestion and file extraction require native filesystem access and run locally via the CLI, REST API Gateway, or MCP server.

---

### Known Limitations & Troubleshooting
- **Merged Cell Headers**: If column headers span merged cells, unmerged columns will use generated column names or default string representations. Ensure header rows are flat for optimal key generation.
- **Large Workbooks**: Extracting workbooks with hundreds of thousands of rows directly into memory generates large JSON documents. For large spreadsheets, specify `--sheet` to extract one worksheet at a time or use `excel_to_csv` for streaming throughput.
