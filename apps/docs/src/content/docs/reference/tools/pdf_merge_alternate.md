---
title: "pdf_merge_alternate"
description: "Reference, CLI, MCP, REST API, and WASM commands for pdf_merge_alternate."
sidebar:
  order: 57
---

**Status**: ✅ Verified  |  **Format**: 2 PDFs (`.pdf` + `.pdf`) $\to$ PDF (`.pdf`)

### What it does
**Interleaves and collates pages from two PDF documents with independent step intervals and reversing.**  
Designed specifically for duplex scanning workflows where front pages (odd pages) and back pages (even pages) are scanned in separate batches or inverted stacks. Objects from the secondary document are renumbered and integrated with corrected reference IDs.

---

### CLI
Interleave pages from two PDF files using `paperpilot pdf-merge-alternate`:

```bash
paperpilot pdf-merge-alternate \
  --input-a tests/e2e_fixtures/page_1.pdf \
  --input-b tests/e2e_fixtures/page_2.pdf \
  --output tests/e2e_fixtures/out/merged_alternate.pdf \
  --step 1 \
  --step-b 1 \
  --reverse-b \
  --json
```

#### CLI Parameters
- `--input-a <INPUT_A>` *(string, required)*: Path to the primary input document (Document A, e.g. front pages).
- `--input-b <INPUT_B>` *(string, required)*: Path to the secondary input document (Document B, e.g. back pages).
- `--output <OUTPUT>` *(string, required)*: Destination path for the merged interleaved document.
- `--step <STEP>` *(integer, optional, default: `1`)*: Number of consecutive pages to consume from Document A per cycle.
- `--step-b <STEP_B>` *(integer, optional, default: `1`)*: Number of consecutive pages to consume from Document B per cycle.
- `--reverse-a` *(flag, optional, default: `false`)*: Reverse the page order of Document A before interleaving.
- `--reverse-b` *(flag, optional, default: `false`)*: Reverse the page order of Document B before interleaving (common when feeder scanners reverse document order).
- `--json` *(flag, optional)*: Return execution results in structured JSON format.

---

### MCP JSON-RPC
Invoke the alternating merge tool through Model Context Protocol (MCP):

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_merge_alternate",
    "arguments": {
      "input_a": "tests/e2e_fixtures/page_1.pdf",
      "input_b": "tests/e2e_fixtures/page_2.pdf",
      "output": "tests/e2e_fixtures/out/merged_alternate_mcp.pdf",
      "step": 1,
      "step_b": 1,
      "reverse_a": false,
      "reverse_b": true
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input_a` *(string, required)*: Path to Document A.
- `input_b` *(string, required)*: Path to Document B.
- `output` *(string, required)*: Destination path for the merged file.
- `step` *(integer, optional, default: `1`)*: Step stride count for Document A.
- `step_b` *(integer, optional, default: `1`)*: Step stride count for Document B.
- `reverse_a` *(boolean, optional, default: `false`)*: Whether to reverse Document A sequence.
- `reverse_b` *(boolean, optional, default: `false`)*: Whether to reverse Document B sequence.

---

### REST API
Execute alternating merge via PaperPilot Gateway:

#### 1. Dedicated Endpoint (`/api/v1/pdf/merge-alternate`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/merge-alternate \
  -H "Content-Type: application/json" \
  -d '{
    "input_a": "tests/e2e_fixtures/page_1.pdf",
    "input_b": "tests/e2e_fixtures/page_2.pdf",
    "output": "tests/e2e_fixtures/out/merged_alternate_api.pdf",
    "step": 1,
    "step_b": 1,
    "reverse_a": false,
    "reverse_b": true
  }'
```

#### 2. Canonical Tool Route (`/api/v1/pdf/tools/pdf_merge_alternate`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_merge_alternate \
  -H "Content-Type: application/json" \
  -d '{
    "input_a": "tests/e2e_fixtures/page_1.pdf",
    "input_b": "tests/e2e_fixtures/page_2.pdf",
    "output": "tests/e2e_fixtures/out/merged_alternate_canonical.pdf",
    "reverse_b": false
  }'
```

---

### WebAssembly (WASM)
Merge two in-memory document buffers using `@paperpilot/wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

const options = JSON.stringify({
  step: 1,
  step_b: 1,
  reverse_a: false,
  reverse_b: true
});

// Pass an Array containing both Uint8Array document buffers
const mergedBytes = WasmPdfEngine.pdf_merge_alternate(
  [docABytes, docBBytes],
  options
);
```

---

### Known Limitations & Troubleshooting
- **Unequal Document Page Counts**: When one document runs out of pages before the other, the remaining unconsumed pages of the longer document are appended sequentially to the end of the output.
- **Buffer Requirements in WASM**: Exactly two document buffers must be supplied in the `buffers` array. Passing fewer or more than two buffers results in an `InvalidInput` error.
