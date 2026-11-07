---
title: "pdf_to_epub"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_epub."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: PDF / EPUB

### What it does
**Convert PDFs to EPUB.**
This tool extracts text and semantic formatting from a PDF document and converts it into a reflowable EPUB eBook format.

### CLI
```bash
paperpilot convert --format epub --input tests/e2e_fixtures/real/test.pdf --output tests/e2e_fixtures/out/penta_e2e/pdf_to_epub_result.epub
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_epub",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/pdf_to_epub_result.epub"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-epub \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/test.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/pdf_to_epub_result.epub"}'
```
