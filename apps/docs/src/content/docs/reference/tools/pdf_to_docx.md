---
title: "pdf_to_docx"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_docx."
sidebar:
  order: 46
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Converts PDF documents into editable Microsoft Word (.docx) files.**  
Reconstructs text layout, headings, side-by-side tables, and embeds images into a fully editable Word document.

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/real/single_page.pdf --format docx --output tests/e2e_fixtures/out/penta_e2e/out.docx --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_docx",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/out.docx"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-docx -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.docx"}'
```
