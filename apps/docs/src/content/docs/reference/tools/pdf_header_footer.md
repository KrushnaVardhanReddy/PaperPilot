---
title: "pdf_header_footer"
description: "Reference, CLI, MCP, and REST API commands for pdf_header_footer."
sidebar:
  order: 28
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Adds custom header and footer text with configurable margins and alignments across pages.**  
Stamps company names, document titles, dates, or confidentiality notices at top and bottom margins.

### CLI
```bash
paperpilot header-footer --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/header_footer_cli_Simple.pdf --text HEADER --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_header_footer",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "text": "Confidential",
    "output": "tests/e2e_fixtures/out/penta_e2e/header.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/header-footer -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "text": "Confidential", "output": "tests/e2e_fixtures/out/penta_e2e/header.pdf"}'
```
