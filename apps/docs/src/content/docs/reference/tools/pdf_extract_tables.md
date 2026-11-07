---
title: "pdf_extract_tables"
description: "Reference, CLI, MCP, and REST API commands for pdf_extract_tables."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: PDF / JSON/CSV

### What it does
**Extract tables from PDFs.**
This tool detects and extracts tabular data structures from PDF documents into structured formats like CSV or JSON.

### CLI
```bash
paperpilot extract-tables --format json --pages all --input tests/e2e_fixtures/real/test.pdf --output tests/e2e_fixtures/out/penta_e2e/pdf_extract_tables_result.json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_extract_tables",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/pdf_extract_tables_result.json",
    "format": "json",
    "pages": "all"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/extract-tables \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/test.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/pdf_extract_tables_result.json",
    "format": "json",
    "pages": "all"}'
```
