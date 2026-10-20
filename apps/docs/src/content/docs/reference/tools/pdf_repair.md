---
title: "pdf_repair"
description: "Reference, CLI, MCP, and REST API commands for pdf_repair."
sidebar:
  order: 41
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Rebuilds corrupted, truncated, or unreadable PDF files.**  
Scans raw file streams, recovers damaged cross-reference tables (xref), fixes broken object trailers, and salvages readable pages.

### CLI
```bash
paperpilot repair --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/repaired.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_repair",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/repaired.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/repair -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/repaired.pdf"}'
```
