---
title: "pdf_delete_pages"
description: "Reference, CLI, MCP, and REST API commands for pdf_delete_pages."
sidebar:
  order: 20
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Removes specific pages from a document (e.g., delete pages 2, 5, and 8).**  
Prunes unwanted pages from the PDF page tree and rewrites the document catalog cleanly.

### CLI
```bash
paperpilot delete --input tests/e2e_fixtures/real/multi_page.pdf --pages 2,4 --output tests/e2e_fixtures/out/penta_e2e/deleted.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_delete_pages",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/deleted.pdf",
    "pages": "2,4"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/delete-pages -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/deleted.pdf", "pages": "2,4"}'
```
