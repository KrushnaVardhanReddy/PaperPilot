---
title: "pdf_stitch_canvas"
description: "Reference, CLI, MCP, and REST API commands for pdf_stitch_canvas."
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Stitches multiple PDF pages into a single continuous canvas.**

### CLI
```bash
paperpilot pdf-stitch-canvas --input input.pdf --output stitched.pdf --direction horizontal
```

### MCP JSON-RPC
```json
{
  "name": "pdf_stitch_canvas",
  "arguments": {
    "input": "input.pdf",
    "output": "stitched.pdf",
    "direction": "horizontal"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/stitch-canvas -H 'Content-Type: application/json' -d '{"input": "input.pdf", "output": "stitched.pdf", "direction": "horizontal"}'
```
