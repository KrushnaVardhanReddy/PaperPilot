---
title: "pdf_rag_chunks"
description: "Reference, CLI, MCP, and REST API commands for pdf_rag_chunks."
sidebar:
  order: 35
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot rag-chunks --input tests/e2e_fixtures/real/single_page.pdf
```

### MCP JSON-RPC
```json
{
  "name": "pdf_rag_chunks",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/rag-chunks -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf"}'
```
