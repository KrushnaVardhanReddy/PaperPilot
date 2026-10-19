---
title: "office_extract_text"
description: "Reference, CLI, MCP, and REST API commands for office_extract_text."
sidebar:
  order: 4
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot office extract-text --input tests/e2e_fixtures/real/test.xlsx --output tests/e2e_fixtures/out/penta_e2e/out.txt
```

### MCP JSON-RPC
```json
{
  "name": "office_extract_text",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.xlsx",
    "output": "tests/e2e_fixtures/out/penta_e2e/out.txt"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/office-extract-text -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.xlsx", "output": "tests/e2e_fixtures/out/penta_e2e/out.txt"}'
```
