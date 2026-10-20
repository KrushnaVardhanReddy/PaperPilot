---
title: "pdf_extract_pages"
description: "Reference, CLI, MCP, and REST API commands for pdf_extract_pages."
sidebar:
  order: 23
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Extracts specific pages or page ranges (e.g., pages 1-3, 5) into a new PDF.**  
Pulls the selected pages into a standalone document without affecting the original file.

### CLI
```bash
paperpilot extract --input tests/e2e_fixtures/real/multi_page.pdf --pages 1,3 --output tests/e2e_fixtures/out/penta_e2e/extracted.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_extract_pages",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/extracted.pdf",
    "pages": "1,3"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/extract-pages -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/extracted.pdf", "pages": "1,3"}'
```
