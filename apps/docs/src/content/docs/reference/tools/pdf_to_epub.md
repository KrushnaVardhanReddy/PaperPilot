---
title: "pdf_to_epub"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_epub."
sidebar:
  order: 50
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ Reflowable EPUB 3.0 (`.epub`)

### What it does
**Converts fixed-layout PDF documents into reflowable EPUB 3.0 eBooks.**  
Analyzes document layout, extracts logical reading flows, removes artificial line wraps, and bundles chapters, navigation manifest, and CSS stylesheets into standard Open Container Format (OCF) `.epub` packages readable by standard e-readers (Apple Books, Calibre, Kindle, Kobo).

- **EPUB 3.0 Compliant**: Creates valid PKZIP archives with `mimetype` uncompressed first, complete `META-INF/container.xml`, OPF package manifest, and NCX / XHTML navigation.
- **TOC & Chapter Extraction**: Maps PDF outline bookmarks to chapter boundaries and table-of-contents landmarks.
- **Clean Semantic Flow**: Reflows paragraphs and cleans hyphenation artifacts for responsive mobile reading.

---

### CLI

Execute EPUB conversion directly using `paperpilot pdf-to-epub`:

```bash
paperpilot pdf-to-epub \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/document.epub \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input PDF file.
- `--output` *(string, required)*: Destination path for the `.epub` archive.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

#### Example CLI Output
```json
{"success":true,"operation":"PdfToEpub","error":null}
```

---

### MCP JSON-RPC

Invoke `pdf_to_epub` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_epub",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.epub"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `output` *(string, required)*: The output EPUB destination file path.

#### Example MCP Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"PDF successfully converted to EPUB 3.0 eBook.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/document.epub\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API

Convert documents through PaperPilot Gateway:

```bash
# Dedicated Gateway Endpoint
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-epub \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.epub"
  }'
```

```bash
# Standard Tool API Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_epub \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.epub"
  }'
```

#### Example Gateway Response
```json
{
  "success": true,
  "data": null,
  "error": null
}
```

---

### WASM Support
In-memory EPUB export is currently not compiled into `paperpilot-wasm`. For browser environments, execute conversions remotely via the PaperPilot Gateway REST API or local MCP server daemon.

---

### Known Limitations & Troubleshooting
- **Scanned PDF Text**: EPUB conversion requires embedded text streams or fonts. For image-only scans, run `paperpilot ocr` prior to running `pdf-to-epub`.
- **Complex Multi-column Grids**: Multi-column text layouts (e.g. newspaper format) reflow into linear paragraph reading orders based on stream traversal order.
