---
title: "json_to_pdf"
description: "Reference, CLI, MCP, and REST API commands for json_to_pdf."
sidebar:
  order: 3
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Generates a publication-quality PDF from structured JSON data.**  
Takes layout, styled text blocks, and embedded base64 images from JSON and compiles them directly into a rendered PDF.

### CLI
```bash
paperpilot convert --input document.json --format pdf --output document.pdf
```

### MCP JSON-RPC
```json
{
  "name": "json_to_pdf",
  "arguments": {
    "input": "document.json",
    "output": "document.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/from-json -H "Content-Type: application/json" -d '{"input": "document.json", "output": "document.pdf"}'
```
