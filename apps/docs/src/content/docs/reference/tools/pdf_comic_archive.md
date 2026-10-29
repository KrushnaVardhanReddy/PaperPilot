---
title: "pdf_comic_archive"
description: "Reference, CLI, MCP, and REST API commands for pdf_comic_archive."
sidebar:
  order: 49
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Converts between PDF documents and comic book archive formats (CBZ/CBR).**  
Packs and unpacks sequential comic illustration pages losslessly between PDF and standard digital comic reader containers.

### CLI
```bash
paperpilot pdf-comic-archive --input comic.cbz --output comic.pdf
```

### MCP JSON-RPC
```json
{
  "name": "pdf_comic_archive",
  "arguments": {
    "input": "comic.cbz",
    "output": "comic.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/comic-archive -H 'Content-Type: application/json' -d '{"input": "comic.cbz", "output": "comic.pdf"}'
```
