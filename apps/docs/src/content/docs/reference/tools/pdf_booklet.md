---
title: "pdf_booklet"
description: "Reference, CLI, MCP, and REST API commands for pdf_booklet."
sidebar:
  order: 58
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Transforms a document into print-ready booklet signatures.**  
Calculates saddle-stitch page pairings and positions pages side-by-side on sheets so that when folded down the spine, the pages read in natural sequence.

### CLI
```bash
paperpilot booklet --input tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/booklet.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_booklet",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/booklet.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/booklet -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/booklet.pdf"}'
```
