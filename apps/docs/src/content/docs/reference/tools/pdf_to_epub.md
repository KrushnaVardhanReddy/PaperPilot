---
title: "pdf_to_epub"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_epub."
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Converts PDF documents into reflowable EPUB 3.0 ebooks.**

### CLI
```bash
paperpilot pdf-to-epub --input input.pdf --output book.epub
```

### MCP JSON-RPC
```json
{
  "name": "pdf_to_epub",
  "arguments": {
    "input": "input.pdf",
    "output": "book.epub"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-epub -H 'Content-Type: application/json' -d '{"input": "input.pdf", "output": "book.epub"}'
```
