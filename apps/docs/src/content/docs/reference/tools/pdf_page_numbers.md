---
title: "pdf_page_numbers"
description: "Reference, CLI, MCP, and REST API commands for pdf_page_numbers."
sidebar:
  order: 34
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Adds dynamic page numbering stamps (e.g. "Page 1 of 5" or "1") to your document.**  
Stamps clean, professional page numbers at your chosen position (bottom-center, bottom-right, top-right, etc.) across the document without shifting or altering any existing text.

### CLI
```bash
paperpilot page-numbers --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/page_numbers_cli_Simple.pdf --position bottom-right --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_page_numbers",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/numbers.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/page-numbers -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/numbers.pdf"}'
```
