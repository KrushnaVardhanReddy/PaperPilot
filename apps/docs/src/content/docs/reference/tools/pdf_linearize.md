---
title: "pdf_linearize"
description: "Reference, CLI, MCP, and REST API commands for pdf_linearize."
sidebar:
  order: 30
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot linearize --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/linearized.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_linearize",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/linearized.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/linearize -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/linearized.pdf"}'
```
