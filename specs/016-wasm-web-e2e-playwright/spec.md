# Spec 016: WASM 12 Tools Playwright E2E Verification & Scorecard

## Status: APPROVED & ACTIVE (Phase 4.8.5)

## 1. Overview & Objective
Validate that all 12 pure-Rust in-browser PDF manipulation tools in `paperpilot-wasm` and `apps/web/` operate end-to-end within a real browser environment without errors, memory crashes, or regressions.

The verification harness must run automated, unmocked E2E tests against real PDF fixtures, recording per-operation latency, binary outputs, and verifying UI workflow parity. It generates `reports/WASM_12_TOOLS_E2E_SCORECARD.md` with 100% pass metrics.

---

## 2. Tested Operations Surface

The suite validates the complete set of 12 tools:

| # | Operation ID | Display Title | Test Fixtures | Parameters | Output Validation |
|---|---|---|---|---|---|
| 1 | `merge` | Merge PDFs | `single_page.pdf` + `multi_page.pdf` | 2 buffers | Magic `%PDF-`, size > input, combined 3 pages |
| 2 | `split` | Split PDF | `multi_page.pdf` | ranges: `"1"` | Magic `%PDF-`, 1 PDF returned |
| 3 | `rotate` | Rotate Pages | `single_page.pdf` | angle: `90`, pages: `"all"` | Magic `%PDF-`, `/Rotate 90` catalog entry |
| 4 | `compress` | Compress PDF | `multi_page.pdf` | default | Magic `%PDF-`, Flate streams compressed |
| 5 | `encrypt` | Encrypt PDF | `single_page.pdf` | password: `"testpass123"` | Magic `%PDF-`, encrypted payload |
| 6 | `watermark` | Watermark | `single_page.pdf` | text: `"CONFIDENTIAL"` | Magic `%PDF-`, text content injected |
| 7 | `delete_pages` | Delete Pages | `multi_page.pdf` | pages: `"2"` | Magic `%PDF-`, page count reduced to 1 |
| 8 | `extract_pages` | Extract Pages | `multi_page.pdf` | pages: `"1"` | Magic `%PDF-`, page count equals 1 |
| 9 | `reorder_pages` | Reorder Pages | `multi_page.pdf` | new_order: `[2, 1]` | Magic `%PDF-`, valid rearranged PDF |
| 10 | `crop` | Crop PDF | `single_page.pdf` | left: 10, bottom: 10, right: 500, top: 700 | Magic `%PDF-`, `/CropBox` defined |
| 11 | `flatten` | Flatten Forms | `form.pdf` or `single_page.pdf` | default | Magic `%PDF-`, `/AcroForm` removed/flattened |
| 12 | `set_metadata` | Edit Metadata | `single_page.pdf` | title: `"PaperPilot"`, author: `"Test"` | Magic `%PDF-`, `/Info` dictionary updated |

---

## 3. Test Layers

1. **Layer 1: Web Worker Bridge Real Execution**:
   - Executes each tool using `wasmPdfClient` inside the browser context (`page.evaluate`).
   - Ensures Web Worker initialization, WASM memory allocation, and message passing work without main-thread blocking.
   - Verifies the output `Uint8Array` contains `%PDF-` header (first 5 bytes `0x25, 0x50, 0x44, 0x46, 0x2D`).

2. **Layer 2: UI Parity & User Interaction**:
   - Selects each tool in `apps/web/src/views/OperationsView.svelte`.
   - Uploads real fixture files via `<input type="file">`.
   - Inputs parameter values (e.g., angle, password, text, page strings).
   - Clicks action button and intercepts browser download event.

3. **Layer 3: Scorecard & Living Documentation**:
   - Measures exact latency in milliseconds (`performance.now()`).
   - Generates `reports/WASM_12_TOOLS_E2E_SCORECARD.md` with Executive Summary and per-operation breakdown.
   - Documents architecture in `wiki/08-Wasm-E2E-Testing.md`.
