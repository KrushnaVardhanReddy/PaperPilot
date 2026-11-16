---
title: "pdf_nup"
description: "Reference, CLI, MCP, REST API, and WASM commands for pdf_nup."
sidebar:
  order: 59
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ PDF (`.pdf`)

### What it does
**Imposes multiple pages onto a single sheet in an N-Up grid layout (2, 4, 9, or 16 pages per sheet).**  
Arranges pages into printable grid matrices with automatic scaling, aspect ratio preservation, customizable margins, inter-tile spacing, and optional cut/separator borders.

---

### CLI
Impose pages into an N-up layout using `paperpilot pdf-nup` (or aliases `nup`, `pdf_nup`):

```bash
paperpilot pdf-nup \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/nup_2up.pdf \
  --nup 2 \
  --ordering horizontal \
  --margin 18 \
  --spacing 10 \
  --draw-borders \
  --json
```

#### CLI Parameters
- `--input <INPUT>` *(string, required)*: Path to the source PDF document.
- `--output <OUTPUT>` *(string, required)*: Destination file path for the imposed PDF.
- `--nup <NUP>` *(integer, optional, default: `2`)*: Grid count of pages per sheet (`2`, `4`, `9`, or `16`).
- `--ordering <ORDERING>` *(string, optional, default: `"horizontal"`)*: Layout ordering: `"horizontal"` (left-to-right, then top-to-bottom) or `"vertical"` (top-to-bottom, then left-to-right).
- `--margin <MARGIN>` *(float, optional, default: `18.0`)*: Outer sheet margin in points ($18\text{ pt} = 0.25\text{ in}$).
- `--spacing <SPACING>` *(float, optional, default: `10.0`)*: Spacing gutter between adjacent page tiles in points.
- `--draw-borders` *(flag, optional)*: Draw thin black boundary borders around each imposed page tile.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke `pdf_nup`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_nup",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/nup_4up.pdf",
      "nup": 4,
      "ordering": "horizontal",
      "margin_pt": 20.0,
      "spacing_pt": 10.0,
      "draw_borders": true
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `output` *(string, required)*: Destination path for the output PDF file.
- `nup` *(integer, optional, default: `2`)*: Target pages per sheet (`2`, `4`, `9`, or `16`).
- `ordering` *(string, optional, default: `"horizontal"`)*: Placement flow (`"horizontal"` or `"vertical"`).
- `margin_pt` or `margin` *(number, optional, default: `18.0`)*: Outer margin in points.
- `spacing_pt` or `spacing` *(number, optional, default: `10.0`)*: Tile gutter spacing in points.
- `draw_borders` *(boolean, optional, default: `false`)*: Whether to draw borders around individual pages.

---

### REST API
Execute N-Up imposition via PaperPilot Gateway:

#### 1. Direct Endpoint (`/api/v1/pdf/nup`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/nup \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/nup_direct.pdf",
    "nup": 4,
    "ordering": "horizontal",
    "draw_borders": true
  }'
```

#### 2. Canonical Tool Route (`/api/v1/pdf/tools/pdf_nup`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_nup \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/nup_canonical.pdf",
    "nup": 2
  }'
```

---

### WebAssembly (WASM)
In browser or worker contexts using `@paperpilot/wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

const nupOptions = JSON.stringify({
  nup: 2,
  ordering: "horizontal",
  margin_pt: 18.0,
  spacing_pt: 10.0,
  draw_borders: true
});

// Returns Uint8Array of imposed N-up PDF bytes
const nupPdfBytes = WasmPdfEngine.pdf_nup(inputPdfBytes, nupOptions);
```

---

### Known Limitations & Troubleshooting
- **Supported Grid Sizes**: N-Up officially supports `2` ($1 \times 2$), `4` ($2 \times 2$), `9` ($3 \times 3$), and `16` ($4 \times 4$). Any unsupported number defaults gracefully to `2`.
- **Automatic Landscape Orientation for 2-Up**: For 2-up imposition, portrait pages are automatically arranged side-by-side onto a landscape canvas ($2 \text{ cols} \times 1 \text{ row}$) to maximize readable page area.
- **Form XObject Architecture**: Input pages are converted to PDF Form XObjects (`/Subtype /Form`) and drawn with affine coordinate matrices, preserving fonts and vector resolutions without rasterization.
