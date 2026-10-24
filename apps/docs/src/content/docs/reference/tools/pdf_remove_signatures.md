---
title: "pdf_remove_signatures"
description: "Reference, CLI, MCP, and REST API commands for pdf_remove_signatures."
sidebar:
  order: 54
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Strips digital signatures, signature field appearances, and AcroForm signature dictionaries from a PDF.**  
Flattens or strips signature annotations while preserving document tree integrity and remaining form fields.

### CLI
```bash
paperpilot pdf-remove-signatures --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/stripped.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_remove_signatures",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/stripped.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/remove-signatures -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/stripped.pdf"}'
```
