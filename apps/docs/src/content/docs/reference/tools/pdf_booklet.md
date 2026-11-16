---
title: "pdf_booklet"
description: "Reference, CLI, MCP, REST API, and WASM commands for pdf_booklet."
sidebar:
  order: 58
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ PDF (`.pdf`)

### What it does
**Transforms a document into print-ready saddle-stitch booklet signatures.**  
Calculates saddle-stitch page pairings and reorders pages into booklet sequence so that when printed double-sided, folded down the spine, and stapled, the pages read in natural numerical sequence. Documents whose page counts are not multiples of 4 are automatically padded with blank trailing pages.

---

### CLI
Impose pages into booklet ordering using `paperpilot pdf-booklet` (or aliases `booklet`, `pdf_booklet`):

```bash
paperpilot pdf-booklet \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/booklet.pdf \
  --json
```

#### CLI Parameters
- `--input <INPUT>` *(string, required)*: Path to the source PDF document.
- `--output <OUTPUT>` *(string, required)*: Destination file path for the reordered booklet PDF.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke `pdf_booklet`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_booklet",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/booklet.pdf"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `output` *(string, required)*: Destination path for the booklet PDF file.

---

### REST API
Execute booklet imposition via PaperPilot Gateway:

#### 1. Direct Endpoint (`/api/v1/pdf/booklet` or `/api/v1/booklet`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/booklet \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/booklet_direct.pdf"
  }'
```

#### 2. Canonical Tool Route (`/api/v1/pdf/tools/pdf_booklet`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_booklet \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/booklet_canonical.pdf"
  }'
```

---

### WebAssembly (WASM)
In browser or worker contexts using `@paperpilot/wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

const bookletOptions = JSON.stringify({
  create_sheets: true,
  center_fold_line: false,
  gutter_pt: 0.0
});

// Returns Uint8Array of booklet-sequenced PDF bytes
const bookletPdfBytes = WasmPdfEngine.pdf_booklet(inputPdfBytes, bookletOptions);
```

---

### Known Limitations & Troubleshooting
- **4-Page Signature Padding**: Booklets require a total page count divisible by 4. If an input document has 5 pages, 3 blank pages are automatically appended at the end to form an 8-page booklet.
- **Double-Sided Printing Setup**: When sending the output to a duplex printer, select "Flip on short edge" (short-edge binding) to ensure consecutive booklet pages align right-side up.
- **Vector Integrity**: The operation reorganizes the document's `/Pages` node hierarchy and inserts blank page dictionaries directly without rasterization, preserving vector data and hyperlinks.
