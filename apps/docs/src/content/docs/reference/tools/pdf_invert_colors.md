---
title: "pdf_invert_colors"
description: "Reference, CLI, MCP, and REST API commands for pdf_invert_colors."
sidebar:
  order: 55
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Inverts colors or applies dark mode to a vector PDF while preserving text.**  
Applies vector-level color manipulation without rasterizing pages. Supports `dark_mode`, `true_invert`, `high_contrast`, and `sepia` modes while keeping text selectable.

### CLI
```bash
paperpilot invert-colors --input doc.pdf --output out.pdf --mode dark_mode
```

### MCP JSON-RPC
```json
{
  "name": "pdf_invert_colors",
  "arguments": {
    "input": "doc.pdf",
    "output": "out.pdf",
    "mode": "dark_mode"
  }
}
```

### REST API
```bash
curl -X POST http://localhost:7823/api/v1/pdf/invert-colors -H "Content-Type: application/json" -d '{"input":"doc.pdf","output":"out.pdf","mode":"true_invert"}'
```
