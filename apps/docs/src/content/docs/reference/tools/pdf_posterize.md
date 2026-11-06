---
title: "pdf_posterize"
description: "Reference, CLI, MCP, and REST API commands for pdf_posterize."
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Tiles oversized PDF pages across a grid of printable sheets with overlap margins.**

### CLI
```bash
paperpilot pdf-posterize --input large-map.pdf --output poster-tiles.pdf
```

### MCP JSON-RPC
```json
{
  "name": "pdf_posterize",
  "arguments": {
    "input": "large-map.pdf",
    "output": "poster-tiles.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/posterize -H 'Content-Type: application/json' -d '{"input": "large-map.pdf", "output": "poster-tiles.pdf"}'
```
