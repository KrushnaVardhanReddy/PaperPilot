---
title: "pdf_classify_type"
description: "Reference, CLI, MCP, and REST API commands for pdf_classify_type."
sidebar:
  order: 11
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Classifies a PDF document into standard taxonomy types using structural analysis.**  
Automatically identifies whether a document is an Invoice, Contract, Scientific Paper, Presentation, or Form based on layout and typography.

### CLI
```bash
paperpilot classify --input tests/e2e_fixtures/real/single_page.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_classify_type",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/classify-type -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf"}'
```
