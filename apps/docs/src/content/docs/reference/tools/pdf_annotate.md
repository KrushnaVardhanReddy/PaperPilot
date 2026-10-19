---
title: "pdf_annotate"
description: "Reference, CLI, MCP, and REST API commands for pdf_annotate."
sidebar:
  order: 7
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot annotate --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e_security/annotate_cli_Simple.pdf --data [{\"id\":\"1\",\"type\":\"highlight\",\"page\":1,\"x\":50.0,\"y\":50.0,\"w\":50.0,\"h\":50.0,\"color\":\"#ffff00\",\"content\":\"Test\"}] --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_annotate",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "annotations": [],
    "output": "tests/e2e_fixtures/out/penta_e2e/annotated.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/annotate -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "annotations": [], "output": "tests/e2e_fixtures/out/penta_e2e/annotated.pdf"}'
```
