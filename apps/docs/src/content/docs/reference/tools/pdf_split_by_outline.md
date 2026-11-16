---
title: "pdf_split_by_outline"
description: "Reference, CLI, MCP, REST API, and WASM commands for pdf_split_by_outline."
sidebar:
  order: 60
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ ZIP Archive (`.zip`)

### What it does
**Splits multi-chapter PDF documents into discrete section files based on their Bookmark (`/Outlines`) hierarchy.**  
Parses document catalog outlines and bookmark destination pointers to identify chapter boundaries. Each identified section is extracted into an independent PDF named after its sanitized outline title and packed into a consolidated ZIP archive.

---

### CLI
Split a bookmarked document by outline using `paperpilot pdf-split-by-outline`:

```bash
paperpilot pdf-split-by-outline \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/split_outline.zip \
  --json
```

#### CLI Parameters
- `--input <INPUT>` *(string, required)*: Path to the input PDF document.
- `--output <OUTPUT>` *(string, required)*: Destination file path for the output `.zip` archive containing the section PDFs.
- `--json` *(flag, optional)*: Return execution receipt in JSON format.

---

### MCP JSON-RPC
Invoke `pdf_split_by_outline` through Model Context Protocol (MCP):

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_split_by_outline",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/split_outline_mcp.zip"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `output` *(string, required)*: Path where the output `.zip` archive will be written.

---

### REST API
Execute outline splitting via PaperPilot Gateway:

#### 1. Dedicated Endpoint (`/api/v1/pdf/split-by-outline`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/split-by-outline \
  -H "Content-Type: application/json" \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/split_outline_api.zip"
  }'
```

#### 2. Canonical Tool Route (`/api/v1/pdf/tools/pdf_split_by_outline`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_split_by_outline \
  -H "Content-Type: application/json" \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/split_outline_canonical.zip"
  }'
```

---

### WebAssembly (WASM)
Split documents by outline bookmarks in browser or worker runtimes using `@paperpilot/wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

const options = JSON.stringify({
  max_depth: 1,
  sanitize_names: true
});

// Returns Uint8Array containing the ZIP archive bytes
const zipBytes = WasmPdfEngine.pdf_split_by_outline(inputPdfBytes, options);
```

---

### Known Limitations & Troubleshooting
- **Documents Without Bookmarks (Outlines)**: When an input document contains no `/Outlines` dictionary or bookmark hierarchy, the tool safely falls back to single-page fallback mode (naming each section `01_Page_1.pdf`, `02_Page_2.pdf`, etc.) so execution never fails or loses data.
- **Output Format**: The output of this operation is a compressed **ZIP archive (`.zip`)** rather than a standalone PDF file. Always specify an output file ending in `.zip` when calling the CLI, MCP, or REST API.
- **Filename Sanitization**: Bookmark titles are automatically sanitized (special characters converted to underscores `_`) to ensure generated ZIP filenames remain valid across Windows, macOS, and Linux filesystems.
