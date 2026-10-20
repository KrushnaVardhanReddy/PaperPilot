---
title: "pdf_compress"
description: "Reference, CLI, MCP, and REST API commands for pdf_compress."
sidebar:
  order: 13
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Reduces PDF file size through intelligent image resampling and stream compression.**  
Re-encodes high-resolution images, applies Flate compression to content streams, and strips redundant orphan objects to make PDFs email and web friendly.

### CLI
```bash
paperpilot compress --input tests/e2e_fixtures/real/single_page.pdf --quality medium --output tests/e2e_fixtures/out/penta_e2e/compressed.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_compress",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "quality": "medium",
    "output": "tests/e2e_fixtures/out/penta_e2e/compressed.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/compress -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "quality": "medium", "output": "tests/e2e_fixtures/out/penta_e2e/compressed.pdf"}'
```
