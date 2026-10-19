---
title: "pdf_search"
description: "Reference, CLI, MCP, and REST API commands for pdf_search."
sidebar:
  order: 43
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot search --input tests/e2e_fixtures/real/single_page.pdf --query test --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_search",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "query": "test"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/search -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "query": "test"}'
```
