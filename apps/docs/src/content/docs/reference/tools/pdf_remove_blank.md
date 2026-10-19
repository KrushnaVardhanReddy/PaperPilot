---
title: "pdf_remove_blank"
description: "Reference, CLI, MCP, and REST API commands for pdf_remove_blank."
sidebar:
  order: 38
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot remove-blank --input tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/noblank.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_remove_blank",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/noblank.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/remove-blank -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/noblank.pdf"}'
```
