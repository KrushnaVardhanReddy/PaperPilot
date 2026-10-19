---
title: "pdf_images_to_pdf"
description: "Reference, CLI, MCP, and REST API commands for pdf_images_to_pdf."
sidebar:
  order: 29
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot images-to-pdf --images tests/e2e_fixtures/real/test.png --output tests/e2e_fixtures/out/penta_e2e/img2pdf.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_images_to_pdf",
  "arguments": {
    "images": [
      "tests/e2e_fixtures/real/test.png"
    ],
    "output": "tests/e2e_fixtures/out/penta_e2e/img2pdf.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/images-to-pdf -H 'Content-Type: application/json' -d '{"images": ["tests/e2e_fixtures/real/test.png"], "output": "tests/e2e_fixtures/out/penta_e2e/img2pdf.pdf"}'
```
