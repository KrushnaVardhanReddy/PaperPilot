---
title: "pdf_invert_colors"
description: "Reference, CLI, MCP, and REST API commands for pdf_invert_colors."
sidebar:
  order: 55
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Inverts colors or applies dark mode styling across PDF vector content while preserving selectable text.**  
Transforms text color, stroke paths, and fill operations at the PDF object level without rasterizing pages. Supports dark mode, true inversion, high contrast, and sepia modes.

---

### CLI
Invert colors or apply dark mode via the `pdf-invert-colors` CLI command:

```bash
paperpilot pdf-invert-colors \
  --input tests/e2e_fixtures/single_page.pdf \
  --output tests/e2e_fixtures/out/batch14_test/cli_invert.pdf \
  --mode dark_mode \
  --pages 1 \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input PDF file.
- `--output` *(string, required)*: Destination file path for the color-inverted PDF.
- `--mode` *(string, optional)*: Inversion style mode (`dark_mode`, `true_invert`, `high_contrast`, `sepia`). Defaults to `dark_mode`.
- `--pages` *(string, optional)*: Page selection range (e.g. `1`, `1-3`, `all`). Defaults to all pages.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_invert_colors`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_invert_colors",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "tests/e2e_fixtures/out/batch14_test/mcp_invert.pdf",
      "mode": "dark_mode",
      "pages": "1"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Input PDF path.
- `output` *(string, required)*: Output PDF path.
- `mode` *(string, optional)*: Color inversion mode: `dark_mode`, `true_invert`, `high_contrast`, or `sepia`.
- `pages` *(string, optional)*: Page range or selector string (e.g. `1`, `1-5`, or `all`).

---

### REST API
Execute color inversion through PaperPilot Gateway:

#### 1. Dedicated Route (`/api/v1/pdf/invert-colors` or `/api/v1/pdf/pdf_invert_colors`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/invert-colors \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/single_page.pdf",
    "output": "tests/e2e_fixtures/out/batch14_test/rest_invert.pdf",
    "mode": "dark_mode",
    "pages": "1"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "9cf8ed1e-f587-4cc4-9367-4352d8ea4bf5",
  "message": "Success",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Color inverted successfully to tests/e2e_fixtures/out/batch14_test/rest_invert.pdf\",\"output_path\":\"tests/e2e_fixtures/out/batch14_test/rest_invert.pdf\",\"data\":null}"
      }
    ]
  }
}
```

---

### WebAssembly (WASM)
- **Status:** Currently available natively in CLI, MCP, and Gateway REST API. Vector manipulation for custom color space inversion in WASM is planned for a future release. For client-side rendering with dark themes, render pages to canvas and apply CSS inverted color filters.

---

### Known Limitations & Troubleshooting
- **Embedded Raster Images:** This tool primarily transforms PDF vector drawing instructions, fills, strokes, and text color operators. Pre-rendered raster images embedded inside the PDF (such as photos or scanned bitmaps) retain their original color palettes unless combined with image-processing pipelines.
- **CLI Subcommand Name:** Use `paperpilot pdf-invert-colors` (not bare `invert-colors`).
