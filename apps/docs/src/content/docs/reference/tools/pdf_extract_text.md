---
title: "pdf_extract_text"
description: "Reference, CLI, MCP, and REST API commands for pdf_extract_text."
sidebar:
  order: 24
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot extract-text --input tests/e2e_fixtures/real/single_page.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_extract_text",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/extract-text -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf"}'
```
