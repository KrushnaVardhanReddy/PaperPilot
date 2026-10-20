---
title: "pdf_bates"
description: "Reference, CLI, MCP, and REST API commands for pdf_bates."
sidebar:
  order: 8
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Applies consecutive legal Bates numbering stamps across documents.**  
Stamps formatted identification numbers (e.g. `DOC-000001`, `EXHIBIT-A-00042`) onto page margins for legal discovery and court filings.

### CLI
```bash
paperpilot bates --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/bates_cli_Simple.pdf --prefix BATES --start 1 --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_bates",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "prefix": "CONF-",
    "start": 1,
    "output": "tests/e2e_fixtures/out/penta_e2e/bates.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/bates -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "prefix": "CONF-", "start": 1, "output": "tests/e2e_fixtures/out/penta_e2e/bates.pdf"}'
```
