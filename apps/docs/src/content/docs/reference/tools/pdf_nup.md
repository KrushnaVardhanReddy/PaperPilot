---
title: "pdf_nup"
description: "Reference, CLI, MCP, and REST API commands for pdf_nup."
sidebar:
  order: 59
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Imposes multiple pages onto a single sheet in an N-Up grid layout.**  
Arranges pages in 2-up, 4-up, 8-up, or 16-up orientations with automatic aspect ratio scaling and customizable margins for paper-saving prints and slide handouts.

### CLI
```bash
paperpilot nup --input tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/nup.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_nup",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/nup.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/nup -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/nup.pdf"}'
```
