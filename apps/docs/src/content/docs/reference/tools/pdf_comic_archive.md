---
title: "pdf_comic_archive"
description: "Reference, CLI, MCP, and REST API commands for pdf_comic_archive."
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Converts a PDF document into a Comic Book Archive (.cbz).**

### CLI
```bash
paperpilot pdf-comic-archive --input document.pdf --output comic.cbz
```

### MCP JSON-RPC
```json
{
  "name": "pdf_comic_archive",
  "arguments": {
    "input": "document.pdf",
    "output": "comic.cbz"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/comic-archive -H 'Content-Type: application/json' -d '{"input": "document.pdf", "output": "comic.cbz"}'
```
