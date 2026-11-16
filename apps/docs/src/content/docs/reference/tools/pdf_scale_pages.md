---
title: "pdf_scale_pages"
description: "Reference, CLI, MCP, REST API, and WASM commands for pdf_scale_pages."
sidebar:
  order: 60
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ PDF (`.pdf`)

### What it does
**Scales PDF page dimensions between standard sheet formats (A4, Letter, Legal, A3, A5, Tabloid) or custom scale percentages.**  
Transforms page media and crop geometry, calculates exact scaling ratios with aspect ratio preservation, and centers content on target dimensions.

---

### CLI
Scale document pages using `paperpilot pdf-scale-pages`:

```bash
paperpilot pdf-scale-pages \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/scaled_a4.pdf \
  --target-size A4 \
  --scale-factor 1.0 \
  --json
```

#### CLI Parameters
- `--input <INPUT>` *(string, required)*: Path to the source PDF document.
- `--output <OUTPUT>` *(string, required)*: Destination file path for the scaled PDF document.
- `--target-size <TARGET_SIZE>` *(string, optional, default: `"A4"`)*: Standard format target name (`"A4"`, `"LETTER"`, `"LEGAL"`, `"A3"`, `"A5"`, `"TABLOID"`).
- `--scale-factor <SCALE_FACTOR>` *(float, optional, default: `1.0`)*: Multiplier applied to dimensions (e.g. `0.8` for 80% scale, `1.5` for 150% scale).
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke `pdf_scale_pages`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_scale_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/scaled_a4.pdf",
      "target_size": "A4",
      "scale_factor": 1.0
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `output` *(string, required)*: Destination path for the output PDF file.
- `target_size`, `format`, or `size` *(string, optional, default: `"A4"`)*: Name of target sheet standard. Supported values: `"A4"`, `"LETTER"`, `"LEGAL"`, `"A3"`, `"A5"`, `"TABLOID"`.
- `scale_factor` *(number, optional, default: `1.0`)*: Scaling ratio factor.

---

### REST API
Execute page scaling via PaperPilot Gateway:

#### 1. Direct Endpoint (`/api/v1/pdf/scale-pages`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/scale-pages \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/scaled_direct.pdf",
    "target_size": "LETTER",
    "scale_factor": 1.0
  }'
```

#### 2. Canonical Tool Route (`/api/v1/pdf/tools/pdf_scale_pages`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_scale_pages \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/scaled_canonical.pdf",
    "target_size": "A3",
    "scale_factor": 1.0
  }'
```

---

### WebAssembly (WASM)
In browser or worker contexts using `@paperpilot/wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

const scaleOptions = JSON.stringify({
  target_size: "A4",
  scale_factor: 0.9,
  maintain_aspect_ratio: true,
  center_content: true
});

// Returns Uint8Array of scaled PDF bytes
const scaledPdfBytes = WasmPdfEngine.pdf_scale_pages(inputPdfBytes, scaleOptions);
```

---

### Known Limitations & Troubleshooting
- **Supported Standard Sizes**: `"A4"`, `"LETTER"`, `"LEGAL"`, `"A3"`, `"A5"`, and `"TABLOID"`. If an unrecognized standard name is provided, the engine scales based on the original page dimensions multiplied by `scale_factor`.
- **Vector Stream Transformation**: Scaling applies affine transformation matrices `[sx 0 0 sy tx ty cm]` to existing page content streams, preserving sharpness of vector artwork and fonts without rasterization blur.
- **Aspect Ratio**: The core engine preserves aspect ratio by default (`maintain_aspect_ratio: true`) and centers content on the target canvas (`center_content: true`).
