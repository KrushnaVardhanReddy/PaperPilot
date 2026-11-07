---
title: "pdf_convert_html"
description: "Reference, CLI, MCP, and REST API commands for pdf_convert_html."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: PDF / HTML

### What it does
**Convert PDFs to HTML.**
This tool extracts text and layout from a PDF document and converts it into HTML format.

### CLI
```bash
paperpilot convert --format html --input tests/e2e_fixtures/real/test.pdf --output tests/e2e_fixtures/out/penta_e2e/pdf_to_html_result.html
```

### MCP JSON-RPC
```json
{
  "name": "pdf_convert_html",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/pdf_to_html_result.html"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-html \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/test.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/pdf_to_html_result.html"}'
```
