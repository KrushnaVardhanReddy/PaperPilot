---
title: "pdf_read_form"
description: "Reference, CLI, MCP, and REST API commands for pdf_read_form."
sidebar:
  order: 36
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Reads and exports form field names and values from a PDF into JSON.**
Exports AcroForm field names and values, including hierarchical field names and UTF-16 text strings. Checkbox values retain their PDF export state. Cyclic or excessively nested field trees return an error.

### CLI
```bash
paperpilot form read tests/e2e_fixtures/real/form.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_read_form",
  "arguments": {
    "input": "tests/e2e_fixtures/real/form.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/read-form -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/form.pdf"}'
```
