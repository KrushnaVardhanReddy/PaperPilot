---
title: "pdf_flatten"
description: "Reference, CLI, MCP, and REST API commands for pdf_flatten."
sidebar:
  order: 26
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Paints AcroForm widget appearances into page content and removes the interactive form fields.**
Preserves existing page resources and unrelated annotations. Missing appearances for supported text fields and checkboxes are generated from their values. Hidden widgets are removed without drawing their values. Existing normal appearances, including transformed appearance boxes, retain their placement. This operation does not remove or flatten unrelated annotation types and does not establish signature validity.

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
