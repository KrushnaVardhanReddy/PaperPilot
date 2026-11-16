---
title: "pdf_posterize"
description: "Reference, CLI, MCP, REST API, and WASM commands for pdf_posterize."
sidebar:
  order: 62
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ PDF (`.pdf`)

### What it does
**Tiles oversized PDF pages across a multi-sheet grid of standard printable paper with overlap margins and cut guides.**  
Takes single-page or multi-page documents with large dimensions (architectural blueprints, engineering schematics, large maps, or exhibition posters) and slices them into a grid of standard sheets (e.g. A4, Letter, A3) suitable for home or office printing and manual assembly.

---

### CLI
Tile an oversized PDF using `paperpilot pdf-posterize`:

```bash
paperpilot pdf-posterize \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/poster_tiles.pdf \
  --target-sheet a4 \
  --grid 2x2 \
  --overlap 14.17 \
  --scale 1.0 \
  --guides \
  --json
```

#### CLI Parameters
- `--input <INPUT>` *(string, required)*: Path to the source PDF document.
- `--output <OUTPUT>` *(string, required)*: Destination file path for the tiled poster PDF.
- `--target-sheet <TARGET_SHEET>` *(string, optional, default: `"a4"`)*: Output printable sheet format (`"a4"`, `"letter"`, `"legal"`, `"a3"`, `"a5"`).
- `--grid <GRID>` *(string, optional)*: Explicit grid layout in `COLSxROWS` format (e.g. `"2x2"`, `"3x2"`). If omitted, dimensions are auto-calculated from page dimensions and sheet size.
- `--overlap <OVERLAP>` *(float, optional, default: `14.17`)*: Overlap margin (glue gutter) in points ($14.17\text{ pt} \approx 5\text{ mm}$).
- `--scale <SCALE>` *(float, optional, default: `1.0`)*: Enlargement or reduction factor applied before tiling.
- `--guides` *(flag, optional)*: Draw boundary alignment and cutting guide markers on tile edges.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke `pdf_posterize`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_posterize",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/poster_tiles.pdf",
      "target_sheet": "a4",
      "grid": "2x2",
      "overlap_pt": 14.17,
      "scale": 1.0,
      "include_guides": true
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `output` *(string, required)*: Destination path for the tiled output PDF.
- `target_sheet` *(string, optional, default: `"a4"`)*: Printable sheet format (`"a4"`, `"letter"`, `"legal"`, `"a3"`, `"a5"`).
- `grid` *(string, optional)*: `COLSxROWS` format string (e.g. `"2x2"`).
- `grid_cols` *(integer, optional)*: Explicit column count.
- `grid_rows` *(integer, optional)*: Explicit row count.
- `overlap_pt` or `overlap` *(number, optional, default: `14.17`)*: Overlap margin in points.
- `scale` *(number, optional, default: `1.0`)*: Page scaling factor before slicing.
- `include_guides` or `guides` *(boolean, optional, default: `true`)*: Whether to draw cut guides.

---

### REST API
Execute poster tiling via PaperPilot Gateway:

#### 1. Direct Endpoint (`/api/v1/pdf/posterize`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/posterize \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/poster_tiles_direct.pdf",
    "target_sheet": "a4",
    "grid": "2x2",
    "overlap_pt": 14.17,
    "scale": 1.0,
    "include_guides": true
  }'
```

#### 2. Canonical Tool Route (`/api/v1/pdf/tools/pdf_posterize`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_posterize \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/poster_tiles_canonical.pdf",
    "target_sheet": "letter",
    "overlap_pt": 18.0
  }'
```

---

### WebAssembly (WASM)
In browser or worker contexts using `@paperpilot/wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

const posterizeOptions = JSON.stringify({
  target_sheet: "a4",
  grid_cols: 2,
  grid_rows: 2,
  overlap_pt: 14.17,
  scale: 1.0,
  include_guides: true
});

// Returns Uint8Array of sliced poster tile sheets
const posterPdfBytes = WasmPdfEngine.pdf_posterize(inputPdfBytes, posterizeOptions);
```

---

### Known Limitations & Troubleshooting
- **Auto Grid Calculation**: When neither `grid` nor `grid_cols`/`grid_rows` are specified, the engine calculates the minimum number of rows and columns needed by dividing scaled page dimensions by effective printable sheet dimensions $(W - \text{overlap}, H - \text{overlap})$.
- **Vector Transformation**: Each tile sheet uses coordinate translation matrices `[scale 0 0 scale tx ty cm]` applied to original stream objects, preserving 100% vector fidelity and text sharpness at large print sizes.
- **Overlap Gutter Assembly**: The default overlap of $14.17\text{ pt}$ ($5\text{ mm}$) provides a glue gutter for pasting neighboring sheets together. If borderless printing is unavailable on your printer, set overlap to at least $18\text{--}25\text{ pt}$ to prevent margin clipping.
