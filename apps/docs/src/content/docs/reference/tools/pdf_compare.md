---
title: "pdf_compare"
description: "Reference, CLI, MCP, and REST API commands for pdf_compare."
sidebar:
  order: 12
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot compare --input tests/e2e_fixtures/real/merge_a.pdf --input-b tests/e2e_fixtures/real/merge_b.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_compare",
  "arguments": {
    "file1": "tests/e2e_fixtures/real/merge_a.pdf",
    "file2": "tests/e2e_fixtures/real/merge_b.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/compare -H 'Content-Type: application/json' -d '{"file1": "tests/e2e_fixtures/real/merge_a.pdf", "file2": "tests/e2e_fixtures/real/merge_b.pdf"}'
```
