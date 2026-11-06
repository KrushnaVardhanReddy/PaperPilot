---
title: "pdf_stamp"
description: "Reference, CLI, MCP, and REST API commands for pdf_stamp."
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Places an image stamp or logo onto specified PDF pages with optional opacity.**

### CLI
```bash
paperpilot pdf-stamp --input input.pdf --output output.pdf --image-path stamp.png
```

### MCP JSON-RPC
```json
{
  "name": "pdf_stamp",
  "arguments": {
    "input": "input.pdf",
    "output": "output.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/stamp -H 'Content-Type: application/json' -d '{"input": "input.pdf", "output": "output.pdf"}'
```
