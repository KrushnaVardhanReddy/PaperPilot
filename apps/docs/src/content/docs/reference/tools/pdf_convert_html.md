---
title: "pdf_convert_html"
description: "Reference, CLI, MCP, and REST API commands for pdf_convert_html."
sidebar:
  order: 15
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Converts PDF documents into responsive, semantic HTML5 web pages.**  
Translates layout, headings, paragraphs, and embedded graphics into clean HTML and CSS suitable for browser viewing.

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/real/test.html --format pdf --output tests/e2e_fixtures/out/penta_e2e/out_html.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_convert_html",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.html",
    "output": "tests/e2e_fixtures/out/penta_e2e/out_html.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert-html -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.html", "output": "tests/e2e_fixtures/out/penta_e2e/out_html.pdf"}'
```
