---
title: "pdf_split"
description: "Reference, CLI, MCP, and REST API commands for pdf_split."
sidebar:
  order: 45
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot split --input tests/e2e_fixtures/real/multi_page.pdf --pages 1,3 --output tests/e2e_fixtures/out/penta_e2e/pdf_split_simple --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_split",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output_dir": "tests/e2e_fixtures/out/penta_e2e/pdf_split_simple",
    "pages": "1,3"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/split -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output_dir": "tests/e2e_fixtures/out/penta_e2e/pdf_split_simple", "pages": "1,3"}'
```
