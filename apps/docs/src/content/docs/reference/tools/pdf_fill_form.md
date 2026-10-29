---
title: "pdf_fill_form"
description: "Reference, CLI, MCP, and REST API commands for pdf_fill_form."
sidebar:
  order: 25
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Fills interactive PDF AcroForm fields automatically from a JSON key-value map.**  
Populates text fields and checkboxes, including fully qualified hierarchical names and UTF-16 PDF values. Regenerates widget appearances so filled text is visible before and after form flattening. Radio buttons, pushbuttons and other unsupported field types return an error.

For CSV/JSON mail merge across many records, use [`paperpilot fill-batch`](/reference/tools/cli_fill_batch/).

### CLI
```bash
paperpilot form fill tests/e2e_fixtures/form.pdf --data tests/e2e_fixtures/form_data.json --output tests/e2e_fixtures/out/penta_e2e_security/fill_form_cli_Simple.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_fill_form",
  "arguments": {
    "input": "tests/e2e_fixtures/real/form.pdf",
    "data": {},
    "output": "tests/e2e_fixtures/out/penta_e2e/filled.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/fill-form -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/form.pdf", "data": {}, "output": "tests/e2e_fixtures/out/penta_e2e/filled.pdf"}'
```
