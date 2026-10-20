# Phase 5.9.70 — Wave 2 (6 Tools) Penta-Interface E2E Testing

## Objective

This document outlines the testing architecture for Wave 2 tools across all 5 supported interfaces (Penta-Interface Spec). The goal is to assert both real-world integration latency and output correctness (size, structure, and magic bytes).

## The 6 Wave 2 Tools Covered

1. **`pdf_images_to_pdf` (Images to PDF):** Takes multiple images and structures them into a new single PDF wrapper.
2. **`pdf_extract_images` (Extract Embedded Images):** Scans the `XObject` streams within a PDF and natively dumps parsed images.
3. **`pdf_remove_blank` (Remove Blank Pages):** Operates heuristically checking image streams, content runs, and text box density.
4. **`pdf_page_numbers` (Add Page Numbers):** Overlays precise text structures per page respecting trim boxes.
5. **`pdf_burst` (Burst PDF):** Quickly isolates and splits a single large multi-page PDF into directories of single page chunks.
6. **`pdf_search` (Search Text & BBoxes):** Yields physical page match indexes.

## Penta-Interface Matrix Coverage

| Interface | Runner Mode | Latency Evaluation | Assertion Checks |
| --------- | ----------- | ------------------ | ---------------- |
| **CLI** | `std::process::Command` | Full I/O Boot | Exit code 0, Output > 0 bytes, `%PDF-` |
| **MCP** | `PaperPilotMcpServer` In-Memory | Engine overhead | Success bool flag, File payload exist |
| **API** | REST HTTP via `axum` / `reqwest` | Network Roundtrip | Status 200, output size |
| **WASM** | Playwright Web Workers | Browser Exec | Array buffer length, Array structure |
| **Desktop** | Tauri Playwright / Edge | Mobile/Edge Boot | Cross-platform UI state, disk events |

## Running the E2E Suite

### Rust (CLI, MCP, API)
```bash
cargo test --test test_wave2_penta_interface -- --nocapture
```
*Outputs auto-generated scorecard to `reports/WAVE_2_PENTA_INTERFACE_SCORECARD.md`.*

### Web/WASM (Playwright)
```bash
cd apps/web && pnpm dlx playwright test tests/e2e_wave2_wasm.spec.ts
```
