---
title: "office_to_markdown"
description: "Reference, CLI, MCP, and REST API commands for office_to_markdown."
sidebar:
  order: 6
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Converts Word, Excel, and PowerPoint documents into clean Markdown.**  
Preserves heading hierarchies, bullet lists, bold/italics, and data tables in human-readable Markdown format for AI RAG pipelines.

### CLI
```bash
paperpilot office to-markdown --input tests/e2e_fixtures/real/test.xlsx --output tests/e2e_fixtures/out/penta_e2e/doc.md
```

### MCP JSON-RPC
```json
{
  "name": "office_to_markdown",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.xlsx",
    "output": "tests/e2e_fixtures/out/penta_e2e/doc.md"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/office-to-markdown -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/test.xlsx", "output": "tests/e2e_fixtures/out/penta_e2e/doc.md"}'
```
