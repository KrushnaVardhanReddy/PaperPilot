---
title: "pdf_rotate"
description: "Reference, CLI, MCP, and REST API commands for pdf_rotate."
sidebar:
  order: 42
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Rotates PDF pages by 90, 180, or 270 degrees clockwise.**  
Adjusts the `/Rotate` dictionary attribute to fix upside-down or sideways orientation across all pages or selected pages.

### CLI
```bash
paperpilot rotate --input tests/e2e_fixtures/real/single_page.pdf --degrees 90 --pages 1 --output tests/e2e_fixtures/out/penta_e2e/rotated.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_rotate",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "angle": 90,
    "pages": "1",
    "output": "tests/e2e_fixtures/out/penta_e2e/rotated.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/rotate -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "angle": 90, "pages": "1", "output": "tests/e2e_fixtures/out/penta_e2e/rotated.pdf"}'
```
