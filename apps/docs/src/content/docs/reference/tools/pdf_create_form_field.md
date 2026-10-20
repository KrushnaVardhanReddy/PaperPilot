---
title: "pdf_create_form_field"
description: "Reference, CLI, MCP, and REST API commands for pdf_create_form_field."
sidebar:
  order: 17
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Programmatically adds new interactive AcroForm form fields into a PDF.**  
Inserts text inputs, checkboxes, or signature fields at specified coordinates so users can fill them out in any PDF reader.

### CLI
```bash
paperpilot form add-field tests/e2e_fixtures/real/single_page.pdf --name signature --type text --rect 50,50,150,30 --output tests/e2e_fixtures/out/penta_e2e_security/create_form_field_cli_Simple.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_create_form_field",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "name": "f1",
    "rect": [
      0,
      0,
      10,
      10
    ],
    "output": "tests/e2e_fixtures/out/penta_e2e/added_field.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/create-form-field -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "name": "f1", "rect": [0,0,10,10], "output": "tests/e2e_fixtures/out/penta_e2e/added_field.pdf"}'
```
