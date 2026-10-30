---
title: pdf_comic_archive
description: "Converts a PDF document into a CBZ comic archive by extracting each page as a PNG image."
---

# `pdf_comic_archive`

Converts a PDF document into a Comic Book Archive (.cbz) format. This tool extracts every page in the PDF and renders it into a PNG image, archiving all the output images sequentially into a standard Zip format compatible with comic book readers.

## Usage

### CLI

```bash
paperpilot-cli pdf pdf_comic_archive --input document.pdf --output comic.cbz --dpi 150
```

### MCP JSON-RPC

```json
{
  "jsonrpc": "2.0",
  "method": "call_tool",
  "params": {
    "name": "pdf_comic_archive",
    "arguments": {
      "input": "base64_encoded_pdf_string",
      "dpi": 150.0
    }
  },
  "id": 1
}
```

### REST API

```bash
curl -X POST http://localhost:8080/api/v1/tools/pdf_comic_archive \
  -H "Content-Type: application/json" \
  -d '{
    "input": "base64_encoded_pdf_string",
    "dpi": 150.0
  }'
```

## Options

- `dpi` (float): The target dots per inch for image rasterization. Default is `150.0`. Higher values yield better image quality at the expense of significantly larger file size.
