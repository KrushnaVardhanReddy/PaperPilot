---
title: "pdf_extract_images"
description: "Reference, CLI, MCP, and REST API commands for pdf_extract_images."
sidebar:
  order: 22
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Pulls out all photos, graphics, and figures embedded inside a PDF file and saves them to a folder.**  
Extracts embedded image streams without lossy re-compression directly to `.png` or `.jpg` image files, making it easy to recover high-res graphics and photos from documents.

### CLI
```bash
paperpilot extract-images --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/images --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_extract_images",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output_dir": "tests/e2e_fixtures/out/penta_e2e/images"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/extract-images -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output_dir": "tests/e2e_fixtures/out/penta_e2e/images"}'
```
