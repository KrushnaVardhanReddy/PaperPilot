---
title: "PDF to EPUB"
description: "Convert a PDF document into a reflowable EPUB 3.0 e-book."
---

# PDF to EPUB

Convert a standard PDF document into an accessible and reflowable EPUB 3.0 file format.

## Overview
This tool analyzes the text layout, removes artificial line breaks and soft hyphens, and attempts to rebuild the document logically so that it can be read smoothly on an e-reader or mobile device.

## Parameters

| Field | Type | Description |
|-------|------|-------------|
| `title` | `String` (Optional) | The title for the generated EPUB file metadata. |
| `author` | `String` (Optional) | The author for the generated EPUB file metadata. |
| `language` | `String` (Optional) | The language code (e.g. `en`) for the generated EPUB file metadata. |

## Examples

### CLI Invocation

```bash
paperpilot pdf-to-epub --input input.pdf --output book.epub --title "My Book" --author "Jane Doe" --language "en"
```

### MCP JSON-RPC

```json
{
  "jsonrpc": "2.0",
  "method": "call_tool",
  "params": {
    "name": "pdf_to_epub",
    "arguments": {
      "input_file": "/tmp/book.pdf",
      "options": {
        "title": "My Book",
        "author": "Jane Doe",
        "language": "en"
      }
    }
  },
  "id": 1
}
```

### REST API

```bash
curl -X POST https://api.paperpilot.test/v1/pdf/epub \
  -F "file=@book.pdf" \
  -F 'options={"title":"My Book","author":"Jane Doe","language":"en"}'
```
