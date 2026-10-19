---
title: "pdf_validate"
description: "Reference, CLI, MCP, and REST API commands for pdf_validate."
sidebar:
  order: 51
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot validate --input tests/e2e_fixtures/real/single_page.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_validate",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/validate -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf"}'
```
