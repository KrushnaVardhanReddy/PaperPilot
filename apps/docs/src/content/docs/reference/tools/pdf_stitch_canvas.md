---
title: "pdf_stitch_canvas"
description: "Reference, CLI, MCP, REST API, and WASM commands for pdf_stitch_canvas."
sidebar:
  order: 58
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ Single Canvas PDF (`.pdf`)

### What it does
**Combines multiple pages vertically or horizontally into a seamless, continuous single-page canvas.**  
Transforms each source page into a Form XObject and computes combined canvas bounding box dimensions, applying optional outer margins and inter-page spacing gaps. This is ideal for continuous mobile scrolling viewports, endless-scroll web presentation assets, roll banners, receipts, and panoramic print layouts.

---

### CLI
Stitch a multi-page document into a single continuous page using `paperpilot pdf-stitch-canvas`:

```bash
paperpilot pdf-stitch-canvas \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/stitched_canvas.pdf \
  --direction vertical \
  --margin 10.0 \
  --spacing 5.0 \
  --json
```

#### CLI Parameters
- `--input <INPUT>` *(string, required)*: Path to the multi-page source PDF.
- `--output <OUTPUT>` *(string, required)*: Destination file path for the stitched single-page PDF.
- `--direction <DIRECTION>` *(string, optional, default: `"vertical"`)*: Layout orientation:
  - `"vertical"`: Stacks pages from top to bottom into a tall vertical scroll.
  - `"horizontal"`: Lines pages side-by-side from left to right into a wide panoramic strip.
- `--margin <MARGIN>` *(float, optional, default: `0.0`)*: Outer margin border padding in points applied around the entire perimeter of the canvas.
- `--spacing <SPACING>` *(float, optional, default: `0.0`)*: Distance in points separating consecutive page segments.
- `--json` *(flag, optional)*: Return execution receipt in JSON format.

---

### MCP JSON-RPC
Invoke `pdf_stitch_canvas` using Model Context Protocol (MCP):

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_stitch_canvas",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/stitched_mcp.pdf",
      "direction": "vertical",
      "margin_pt": 15.0,
      "spacing_pt": 10.0
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: File path to the input multi-page PDF.
- `output` *(string, required)*: File path to save the generated stitched PDF.
- `direction` *(string, optional, enum: `["vertical", "horizontal"]`, default: `"vertical"`)*: Stacking orientation.
- `margin_pt` *(number, optional, default: `0.0`)*: Outer padding around the unified canvas boundary in points.
- `spacing_pt` *(number, optional, default: `0.0`)*: Gap between adjacent page tiles in points.

---

### REST API
Execute canvas stitching through PaperPilot Gateway:

#### 1. Dedicated Endpoint (`/api/v1/pdf/stitch-canvas`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/stitch-canvas \
  -H "Content-Type: application/json" \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/stitched_api.pdf",
    "direction": "vertical",
    "margin_pt": 20.0,
    "spacing_pt": 5.0
  }'
```

#### 2. Canonical Tool Route (`/api/v1/pdf/tools/pdf_stitch_canvas`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_stitch_canvas \
  -H "Content-Type: application/json" \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/stitched_canonical.pdf",
    "direction": "horizontal",
    "margin_pt": 0.0,
    "spacing_pt": 0.0
  }'
```

---

### WebAssembly (WASM)
Execute client-side canvas stitching using `@paperpilot/wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

const options = JSON.stringify({
  direction: "vertical",
  margin_pt: 12.0,
  spacing_pt: 8.0
});

// Returns Uint8Array representing the stitched single-page PDF
const stitchedPdfBytes = WasmPdfEngine.pdf_stitch_canvas(inputPdfBytes, options);
```

---

### Known Limitations & Troubleshooting
- **Maximum PDF Page Dimensions**: The PDF 1.7 standard specifies an architectural user unit limit of $14,400 \times 14,400$ points (200 inches) unless `/UserUnit` scaling is enabled. Very large documents with hundreds of pages may exceed standard viewport dimensions in legacy PDF readers.
- **Form Fields & Annotations**: Interactive AcroForms and annotations on source pages are flattened into static appearance streams when converted into Form XObjects during stitching.
