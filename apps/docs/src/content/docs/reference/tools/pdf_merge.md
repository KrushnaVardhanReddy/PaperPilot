---
title: "pdf_merge"
description: "Reference, CLI, MCP, and REST API commands for pdf_merge."
sidebar:
  order: 31
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### CLI
```bash
paperpilot merge --input tests/e2e_fixtures/real/merge_a.pdf tests/e2e_fixtures/real/merge_b.pdf --output tests/e2e_fixtures/out/penta_e2e/merge_cli_simple.pdf --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_merge",
  "arguments": {
    "inputs": [
      "tests/e2e_fixtures/real/merge_a.pdf",
      "tests/e2e_fixtures/real/merge_b.pdf"
    ],
    "output": "tests/e2e_fixtures/out/penta_e2e/merge_mcp_simple.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/merge -H 'Content-Type: application/json' -d '{"inputs": ["tests/e2e_fixtures/real/merge_a.pdf", "tests/e2e_fixtures/real/merge_b.pdf"], "output": "tests/e2e_fixtures/out/penta_e2e/merge_mcp_simple.pdf"}'
```
