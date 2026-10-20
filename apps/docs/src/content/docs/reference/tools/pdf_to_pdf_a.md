---
title: "pdf_to_pdf_a"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_pdf_a."
sidebar:
  order: 48
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Converts a PDF into the ISO 19005 standard archival format (PDF/A).**  
Embeds all fonts, strips external device dependencies and dynamic actions to guarantee the document can be read decades into the future.

### CLI
```bash
paperpilot pdf-a --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/pdf_a.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_pdf_a",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/pdf_a.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-pdf-a -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/pdf_a.pdf"}'
```
