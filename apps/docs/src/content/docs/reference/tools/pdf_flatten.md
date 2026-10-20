---
title: "pdf_flatten"
description: "Reference, CLI, MCP, and REST API commands for pdf_flatten."
sidebar:
  order: 26
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Bakes interactive form fields, signatures, and annotations directly into the page content.**  
Converts editable form inputs into static vector graphics so they can no longer be edited, moved, or altered.

### CLI
```bash
paperpilot flatten --input tests/e2e_fixtures/form.pdf --output tests/e2e_fixtures/out/penta_e2e_security/flatten_cli_Simple.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_flatten",
  "arguments": {
    "input": "tests/e2e_fixtures/real/form.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/flattened.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/flatten -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/form.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/flattened.pdf"}'
```
