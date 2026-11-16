---
title: "pdf_remove_blank"
description: "Reference, CLI, MCP, and REST API commands for pdf_remove_blank."
sidebar:
  order: 38
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`)

### What it does
**Scans a PDF and strips out all blank or empty pages.**  
Analyzes text operators, vector paths, and pixel ink density across every page. Any accidental empty pages (common from double-sided scanning or printer slips) are safely removed while preserving all content, structural integrity, and annotations on the remaining pages.

---

### CLI
Remove blank pages via the `remove-blank` subcommand:

```bash
paperpilot remove-blank \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/batch14_test/cli_remove_blank.pdf \
  --sensitivity 95 \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input PDF document.
- `--output` *(string, required)*: Destination file path for the cleaned PDF.
- `--sensitivity` *(number, optional)*: Threshold percentage for blank page detection (1-100). Defaults to `95`.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_remove_blank`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_remove_blank",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/batch14_test/mcp_remove_blank.pdf",
      "sensitivity": 95
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF document.
- `output` *(string, required)*: Path for the cleaned output document.
- `sensitivity` *(number, optional)*: Sensitivity detection threshold (default: `95`).

---

### REST API
Execute blank page removal through PaperPilot Gateway:

#### 1. Generic Tool Route (`/api/v1/pdf/tools/{tool_name}`)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_remove_blank \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/batch14_test/rest_remove_blank.pdf",
    "sensitivity": 95
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "31f078ab-50cc-42d9-9d8c-82eda62de72f",
  "message": "Success",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Blank pages removed successfully.\",\"output_path\":\"tests/e2e_fixtures/out/batch14_test/rest_remove_blank.pdf\",\"data\":null}"
      }
    ]
  }
}
```

---

### WebAssembly (WASM)
- **Status:** Supported natively in CLI, MCP, and Gateway REST API. Client-side page removal in WASM can currently be performed by detecting pages with `pdf_extract_text` or `render_page` and calling `WasmPdfEngine.delete_pages(pdfBytes, blankPages)`.

---

### Known Limitations & Troubleshooting
- **Faint Watermarks & Scanner Noise:** A page with faint background scanner smudges or microscopic speckles may not be considered blank at very low sensitivity thresholds. If pages with light scan artifacts are not being stripped, increase the `--sensitivity` parameter towards `98`-`99`.
- **All Blank Edge Case:** If every page in the document is blank, the operation preserves at least one empty page to prevent creating an invalid, zero-page PDF document.
