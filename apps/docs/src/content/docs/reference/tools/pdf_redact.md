---
title: "pdf_redact"
description: "Reference, CLI, MCP, and REST API commands for pdf_redact."
sidebar:
  order: 37
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Permanently blackouts and scrubs sensitive text or coordinates from a PDF.**  
Unlike simple black highlight rectangles, this completely deletes the underlying text operators and vector data so redacted information cannot be recovered.

### CLI
```bash
paperpilot redact --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/redact_cli_Simple.pdf --pages 1 --rect 0,0,100,100 --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_redact",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "page": 1,
    "rect": [
      50,
      50,
      200,
      50
    ],
    "output": "tests/e2e_fixtures/out/penta_e2e/redacted.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/redact -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "page": 1, "rect": [50, 50, 200, 50], "output": "tests/e2e_fixtures/out/penta_e2e/redacted.pdf"}'
```
