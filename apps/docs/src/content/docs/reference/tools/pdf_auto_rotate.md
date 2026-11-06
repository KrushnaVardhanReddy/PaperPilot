---
title: "pdf_auto_rotate"
description: "Reference, CLI, MCP, and REST API commands for pdf_auto_rotate."
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Automatically detects and corrects page orientations based on dimensions.**

### CLI
```bash
paperpilot pdf-auto-rotate --input input.pdf --output output.pdf
```

### MCP JSON-RPC
```json
{
  "name": "pdf_auto_rotate",
  "arguments": {
    "input": "input.pdf",
    "output": "output.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/auto-rotate -H 'Content-Type: application/json' -d '{"input": "input.pdf", "output": "output.pdf"}'
```
