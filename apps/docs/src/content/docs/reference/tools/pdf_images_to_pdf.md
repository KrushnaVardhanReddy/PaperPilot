---
title: "pdf_images_to_pdf"
description: "Reference, CLI, MCP, and REST API commands for pdf_images_to_pdf."
sidebar:
  order: 29
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Combines one or more image files (PNG, JPG, WebP) into a single PDF document.**  
Each image is placed onto its own page with original image dimensions preserved at standard print resolution (72 DPI baseline). Ideal for photo albums, scanned receipts, or creating a PDF presentation from graphic slides.

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
