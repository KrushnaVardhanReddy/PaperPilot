---
title: "pdf_extract_tables"
description: "Reference, CLI, MCP, and REST API commands for pdf_extract_tables."
sidebar:
  order: 48
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Extracts structured tabular data from PDF pages into CSV or JSON.**  
Detects grid lines, cellular coordinates, and cell text alignment to output clean tabular matrices without manual copying.

### CLI
```bash
paperpilot extract-tables --input report.pdf --output tables.csv --format csv --pages all
```

### MCP JSON-RPC
```json
{
  "name": "pdf_extract_tables",
  "arguments": {
    "input": "report.pdf",
    "output": "tables.json",
    "format": "json",
    "pages": "all"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/extract-tables -H 'Content-Type: application/json' -d '{"input": "report.pdf", "output": "tables.csv", "format": "csv", "pages": "all"}'
```
