---
title: "excel_to_json"
description: "Reference, CLI, MCP, and REST API commands for excel_to_json."
sidebar:
  order: 2
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot office excel-to-json --input tests/e2e_fixtures/real/test.xlsx
```

### MCP JSON-RPC
```json
{
  "name": "excel_to_json",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.xlsx"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/excel-to-json -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.xlsx"}'
```
