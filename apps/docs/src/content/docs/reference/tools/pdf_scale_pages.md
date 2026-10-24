---
title: "pdf_scale_pages"
description: "Reference, CLI, MCP, and REST API commands for pdf_scale_pages."
sidebar:
  order: 60
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Scales PDF pages between standard formats (A4, Letter, Legal, A3) or custom percentages.**  
Transforms page media and crop geometry, calculates exact scaling ratios with aspect ratio preservation, and centers content on target dimensions.

### CLI
```bash
paperpilot pdf-scale-pages --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/scaled.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_scale_pages",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/scaled.pdf",
    "options": {
      "target_size": "A4",
      "scale_factor": 1.0,
      "maintain_aspect_ratio": true,
      "center_content": true
    }
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/scale-pages -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/scaled.pdf"}'
```
