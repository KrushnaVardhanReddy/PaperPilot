---
title: "pdf_split_scanned_book"
description: "Reference, CLI, MCP, REST API, and WASM commands for pdf_split_scanned_book."
sidebar:
  order: 59
---

**Status**: ✅ Verified  |  **Format**: Two-Up Scanned PDF (`.pdf`) $\to$ Single Page Sequence PDF (`.pdf`)

### What it does
**Detects center binding gutter spines and splits two-up scanned book spreads into individual pages.**  
Splits landscape book spreads into separate portrait pages (or vice-versa) by calculating gutter centers and adjusting page viewport bounding boxes (`/MediaBox` and `/CropBox`). Because it operates non-destructively on underlying vector objects and content streams, pages retain original vector fidelity without rasterization blur.

---

### CLI
Split two-up scanned spreads into single sequential pages using `paperpilot pdf-split-scanned-book`:

```bash
paperpilot pdf-split-scanned-book \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/split_book_pages.pdf \
  --reading-direction ltr \
  --gutter-overlap 5.0 \
  --json
```

#### CLI Parameters
- `--input <INPUT>` *(string, required)*: Path to the scanned two-up PDF document.
- `--output <OUTPUT>` *(string, required)*: Destination file path for the resulting single-page PDF document.
- `--reading-direction <READING_DIRECTION>` *(string, optional, default: `"ltr"`)*: Reading order sequence:
  - `"ltr"`: Left-to-right (standard for Western languages; extracts left page first, then right page).
  - `"rtl"`: Right-to-left (standard for Arabic, Hebrew, and Japanese manga; extracts right page first, then left page).
- `--gutter-overlap <GUTTER_OVERLAP>` *(float, optional, default: `0.0`)*: Spine gutter overlap in points. Adds safety margin across the split boundary to prevent text loss in tight book bindings.
- `--split-horizontal` *(flag, optional)*: Split pages horizontally (top half and bottom half) instead of vertical center (left/right). Useful for calendar or top-bound document scans.
- `--json` *(flag, optional)*: Output execution result in JSON format.

---

### MCP JSON-RPC
Invoke the tool via Model Context Protocol (MCP) using `pdf_split_scanned_book`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_split_scanned_book",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/split_book_mcp.pdf",
      "reading_direction": "ltr",
      "gutter_overlap": 5.0,
      "split_horizontal": false
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input two-up PDF file.
- `output` *(string, required)*: Destination path for the single-page document.
- `reading_direction` *(string, optional, default: `"ltr"`)*: `"ltr"` or `"rtl"`.
- `gutter_overlap` *(number, optional, default: `0.0`)*: Overlap margin width in points.
- `split_horizontal` *(boolean, optional, default: `false`)*: Whether to split top/bottom rather than left/right.

---

### REST API
Split scanned book spreads via PaperPilot Gateway:

#### 1. Dedicated Endpoint (`/api/v1/pdf/split-scanned-book`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/split-scanned-book \
  -H "Content-Type: application/json" \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/split_book_api.pdf",
    "reading_direction": "ltr",
    "gutter_overlap": 5.0,
    "split_horizontal": false
  }'
```

#### 2. Canonical Tool Route (`/api/v1/pdf/tools/pdf_split_scanned_book`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_split_scanned_book \
  -H "Content-Type: application/json" \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/split_book_canonical.pdf",
    "reading_direction": "rtl"
  }'
```

---

### WebAssembly (WASM)
Split scanned book spreads directly in the browser or worker using `@paperpilot/wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

const options = JSON.stringify({
  reading_direction: "ltr",
  gutter_overlap: 4.0,
  split_horizontal: false
});

// Returns Uint8Array of the de-spread PDF document
const splitPdfBytes = WasmPdfEngine.pdf_split_scanned_book(inputPdfBytes, options);
```

---

### Known Limitations & Troubleshooting
- **Curved or Skewed Spines**: Splitting divides pages along a geometric linear midpoint. Severely distorted or curved page scans may require prior deskewing or OCR flattening.
- **Output Page Count**: Each processed input page produces two output pages (e.g., a 5-page two-up scan generates a 10-page single-page document).
