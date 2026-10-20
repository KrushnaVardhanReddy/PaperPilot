---
title: "pdf_read_form"
description: "Reference, CLI, MCP, and REST API commands for pdf_read_form."
sidebar:
  order: 36
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Reads and exports all form field names, types, and values from a PDF into JSON.**  
Dumps all filled AcroForm data (text inputs, checkboxes, dropdowns) for automated data ingestion.

### CLI
```bash
paperpilot form read tests/e2e_fixtures/real/missing.pdf --json
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
