---
title: "pdf_convert_markdown"
description: "Reference, CLI, MCP, and REST API commands for pdf_convert_markdown."
sidebar:
  order: 16
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/real/test.md --format pdf --output tests/e2e_fixtures/out/penta_e2e/out_md.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_convert_markdown",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.md",
    "output": "tests/e2e_fixtures/out/penta_e2e/out_md.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert-markdown -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.md", "output": "tests/e2e_fixtures/out/penta_e2e/out_md.pdf"}'
```
