---
title: "pdf_convert_excel"
description: "Reference, CLI, MCP, and REST API commands for pdf_convert_excel."
sidebar:
  order: 14
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Extracts tables and tabular columns from PDFs into editable Excel spreadsheets (.xlsx).**  
Detects table gridlines and cell boundaries to reconstruct structured rows and columns in an Excel workbook.

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/real/test.csv --format pdf --output tests/e2e_fixtures/out/penta_e2e/out_csv.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_convert_excel",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.csv",
    "output": "tests/e2e_fixtures/out/penta_e2e/out_csv.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert-excel -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.csv", "output": "tests/e2e_fixtures/out/penta_e2e/out_csv.pdf"}'
```
