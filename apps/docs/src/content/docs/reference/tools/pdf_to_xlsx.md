---
title: "pdf_to_xlsx"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_xlsx."
sidebar:
  order: 50
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Extracts tabular data and financial statements from PDF into an Excel workbook (.xlsx).**  
Detects table structures and cell values to generate clean, editable spreadsheets.

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/real/single_page.pdf --format xlsx --output tests/e2e_fixtures/out/penta_e2e/out.xlsx --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_xlsx",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/out.xlsx"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-xlsx -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.xlsx"}'
```
