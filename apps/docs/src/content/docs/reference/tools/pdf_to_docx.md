---
title: "pdf_to_docx"
description: "Reference, CLI, MCP, and REST API commands for pdf_to_docx."
sidebar:
  order: 46
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ Word OpenXML (`.docx`)

### What it does
**Decompiles PDF documents into editable Microsoft Word (.docx) files.**  
Reconstructs text layout, headings, side-by-side tables, and embeds images into a fully editable Word document.

- **Layout & Structure Preservation**: Reconstructs paragraphs, text blocks, and headings according to font sizes and spatial bounds.
- **Table Detection**: Identifies structured cell boundaries, column layouts, and multi-row grids and converts them into native Word OpenXML tables (`w:tbl`).
- **Offline & Privacy-Preserving**: Performs all decompression, font-to-unicode extraction, and OpenXML packaging locally without external cloud dependencies.

---

### CLI
Convert PDF documents to `.docx` format using the unified `convert` command:

```bash
paperpilot convert \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --format docx \
  --output tests/e2e_fixtures/out/penta_e2e/document.docx \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to input PDF file.
- `--format` *(string, required)*: Target output format (`docx`).
- `--output` *(string, optional, default: `"output.docx"`)*: Path where the converted `.docx` file will be written.
- `--json` *(flag, optional)*: Emit structured JSON completion status.

---

### MCP JSON-RPC
Invoke `pdf_to_docx` through Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_docx",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.docx"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `output` *(string, required)*: The output DOCX destination file path.

#### Example MCP Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Converted to DOCX successfully.\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/document.docx\",\"data\":null}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API
Execute DOCX conversions via PaperPilot Gateway:

```bash
# Universal MCP Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_docx \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.docx"
  }'
```

```bash
# Format Conversion Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "format": "docx",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.docx"
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
`pdf_to_docx` is fully supported in `paperpilot-wasm` in preview status:
```typescript
import { pdf_to_docx } from "@paperpilot/wasm";

const docxUint8Array = pdf_to_docx(inputPdfUint8Array);
```

---

### Known Limitations & Troubleshooting
- **CLI Subcommand Invocation**: Always use `paperpilot convert --input <INPUT> --format docx --output <OUTPUT>`. Direct subcommand `paperpilot pdf-to-docx` is not a standalone CLI verb; it is exposed via the `convert` subcommand.
- **Scanned Image PDFs**: Scanned image-only PDFs without an embedded text layer will produce an empty or image-only Word document. Run `paperpilot ocr` first before converting scanned documents to DOCX.
- **Complex Floating Overlays**: Documents with overlapping absolute coordinate vector drawings and watermarks may be reflowed into sequential paragraphs in OpenXML.
