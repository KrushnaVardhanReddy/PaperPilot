---
title: "pdf_metadata"
description: "Reference, CLI, MCP, and REST API commands for pdf_metadata."
sidebar:
  order: 32
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot metadata --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/metadata.json --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_metadata",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/metadata -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf"}'
```
