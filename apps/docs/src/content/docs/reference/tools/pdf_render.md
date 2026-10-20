---
title: "pdf_render"
description: "Reference, CLI, MCP, and REST API commands for pdf_render."
sidebar:
  order: 39
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Renders PDF pages into high-resolution raster images (PNG, JPEG).**  
Rasterizes vector paths, text glyphs, and embedded graphics at target DPI for web previews, thumbnail generation, or image processing.

### CLI
```bash
paperpilot render --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/rendered.png --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_render",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/rendered.png"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/render -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/rendered.png"}'
```
