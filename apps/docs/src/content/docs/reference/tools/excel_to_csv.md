---
title: "excel_to_csv"
description: "Reference, CLI, MCP, and REST API commands for excel_to_csv."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot office excel-to-csv --input tests/e2e_fixtures/real/test.xlsx --output tests/e2e_fixtures/out/penta_e2e/table.csv
```

### MCP JSON-RPC
```json
{
  "name": "excel_to_csv",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.xlsx",
    "output": "tests/e2e_fixtures/out/penta_e2e/table.csv"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/excel-to-csv -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.xlsx", "output": "tests/e2e_fixtures/out/penta_e2e/table.csv"}'
```
