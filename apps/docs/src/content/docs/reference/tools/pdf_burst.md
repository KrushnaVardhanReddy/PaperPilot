---
title: "pdf_burst"
description: "Reference, CLI, MCP, and REST API commands for pdf_burst."
sidebar:
  order: 10
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Explodes a multi-page PDF into separate 1-page PDF files.**  
If you have a 10-page document, this creates 10 individual single-page files (`page_1.pdf`, `page_2.pdf`, etc.) in an output directory.
- **Opposite of Merge**: Where `merge` combines multiple files into 1, `burst` splits 1 file into every individual page.
- **Different from Split**: `split` requires cut points or page ranges; `burst` automatically breaks up all pages individually.

### CLI
```bash
paperpilot burst --input tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/burst --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_burst",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output_dir": "tests/e2e_fixtures/out/penta_e2e/burst"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/burst -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output_dir": "tests/e2e_fixtures/out/penta_e2e/burst"}'
```
