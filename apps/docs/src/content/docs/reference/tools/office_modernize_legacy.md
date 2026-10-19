---
title: "office_modernize_legacy"
description: "Reference, CLI, MCP, and REST API commands for office_modernize_legacy."
sidebar:
  order: 5
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot office modernize --input tests/e2e_fixtures/real/test.xls --output tests/e2e_fixtures/out/penta_e2e/out.xlsx
```

### MCP JSON-RPC
```json
{
  "name": "office_modernize_legacy",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.xls",
    "output": "tests/e2e_fixtures/out/penta_e2e/out.xlsx"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/office-modernize-legacy -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.xls", "output": "tests/e2e_fixtures/out/penta_e2e/out.xlsx"}'
```
