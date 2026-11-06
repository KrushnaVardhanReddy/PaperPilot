---
title: "pdf_split_by_outline"
description: "Reference, CLI, MCP, and REST API commands for pdf_split_by_outline."
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Split PDF by outline bookmarks into separate chapters.**

### CLI
```bash
paperpilot pdf-split-by-outline --input book.pdf --output output.zip --max-depth 1
```

### MCP JSON-RPC
```json
{
  "name": "pdf_split_by_outline",
  "arguments": {
    "input": "book.pdf",
    "output": "output.zip",
    "max_depth": 1
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/split-by-outline -H 'Content-Type: application/json' -d '{"input": "book.pdf", "output": "output.zip", "max_depth": 1}'
```
