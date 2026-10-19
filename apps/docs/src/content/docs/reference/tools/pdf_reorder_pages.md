---
title: "pdf_reorder_pages"
description: "Reference, CLI, MCP, and REST API commands for pdf_reorder_pages."
sidebar:
  order: 40
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot reorder --input tests/e2e_fixtures/real/multi_page.pdf --order 2,1,3,4,5 --output tests/e2e_fixtures/out/penta_e2e/reordered.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_reorder_pages",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "order": "2,1,3,4,5",
    "output": "tests/e2e_fixtures/out/penta_e2e/reordered.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/reorder-pages -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "order": "2,1,3,4,5", "output": "tests/e2e_fixtures/out/penta_e2e/reordered.pdf"}'
```
