---
title: "pdf_bookmarks"
description: "Reference, CLI, MCP, and REST API commands for pdf_bookmarks."
sidebar:
  order: 9
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot bookmarks --input tests/e2e_fixtures/real/multi_page.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_bookmarks",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/bookmarks -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf"}'
```
