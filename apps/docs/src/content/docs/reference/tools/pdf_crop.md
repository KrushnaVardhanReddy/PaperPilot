---
title: "pdf_crop"
description: "Reference, CLI, MCP, and REST API commands for pdf_crop."
sidebar:
  order: 18
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Trims the margins or crops pages to a custom rectangular bounding box.**  
Adjusts the PDF `/CropBox` and `/MediaBox` to remove printer crop marks, excessive white borders, or isolate a specific page region.

### CLI
```bash
paperpilot crop --input tests/e2e_fixtures/real/single_page.pdf --rect 10,10,200,200 --output tests/e2e_fixtures/out/penta_e2e/cropped.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_crop",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "box": "10,10,200,200",
    "output": "tests/e2e_fixtures/out/penta_e2e/cropped.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/crop -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "box": "10,10,200,200", "output": "tests/e2e_fixtures/out/penta_e2e/cropped.pdf"}'
```
