---
title: "pdf_sanitize"
description: "Reference, CLI, MCP, and REST API commands for pdf_sanitize."
sidebar:
  order: 57
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Performs deep structural sanitization and threat neutralization on PDF files.**  
Strips metadata, removes embedded file attachments, neutralizes JavaScript actions and launch triggers, deletes embedded thumbnails, and cleans private application data.

### CLI
```bash
paperpilot sanitize --input tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/sanitized.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_sanitize",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/sanitized.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/sanitize -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/sanitized.pdf"}'
```
