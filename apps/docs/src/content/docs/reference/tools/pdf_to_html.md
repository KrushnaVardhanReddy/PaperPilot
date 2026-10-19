---
title: "pdf_to_html"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_html."
sidebar:
  order: 47
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/real/single_page.pdf --format html --output tests/e2e_fixtures/out/penta_e2e/doc.html
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_html",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/doc.html"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-html -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/doc.html"}'
```
