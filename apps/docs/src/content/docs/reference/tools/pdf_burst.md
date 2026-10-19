---
title: "pdf_burst"
description: "Reference, CLI, MCP, and REST API commands for pdf_burst."
sidebar:
  order: 10
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot burst --input tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/burst --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_burst",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output_dir": "tests/e2e_fixtures/out/penta_e2e/burst"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/burst -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output_dir": "tests/e2e_fixtures/out/penta_e2e/burst"}'
```
