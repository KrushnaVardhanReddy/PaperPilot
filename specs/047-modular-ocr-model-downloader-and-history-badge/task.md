# Task 047: Modular OCR Model Downloader & Operations Runtime History Badge

## Objective
Implement modular multi-tier OCR model downloading and management (matching the LLM downloader pattern) and add a tool execution latency / last-task status history badge to the desktop UI.

## Context & Architecture
- Reference: `specs/047-modular-ocr-model-downloader-and-history-badge/spec.md`
- Core OCR engine: `rusto-rs` with PP-OCRv6 (Tiny, Small, Medium) and PP-OCRv4 (Mobile).
- Models root: `~/.local/share/paperpilot/models/ocr/` with workspace fallback `models/ocr/`.

## Tasks & Checklist

### Part 1: Tauri Backend OCR Downloader (`ocr_commands.rs`)
- [ ] Create `apps/desktop/src-tauri/src/ocr_commands.rs`:
  - `get_available_ocr_models()`: returns available tiers (`ppocrv6-tiny`, `ppocrv6-small`, `ppocrv6-medium`, `ppocrv4-mobile`) and checks if downloaded on disk.
  - `download_ocr_model()`: downloads `det.onnx`, `rec.onnx`, `dict.txt` for the selected tier using `reqwest` streaming, emitting `ocr-download-progress` events.
  - `cancel_ocr_model_download()`: cancels active download.
  - `set_active_ocr_model()` and `get_active_ocr_model()`: manages active tier in memory and local storage.
- [ ] Register new commands in `apps/desktop/src-tauri/src/lib.rs`.

### Part 2: Desktop UI OCR Downloader (`OcrModelDownloader.svelte`)
- [ ] Create `apps/desktop/src/lib/components/settings/OcrModelDownloader.svelte` mirroring `ModelDownloader.svelte`:
  - Radio button to select active tier.
  * Download button with live progress bar (% and MB).
  * Direct file path indicator for installed models.
- [ ] Mount `OcrModelDownloader.svelte` in `apps/desktop/src/lib/components/layout/SettingsPanel.svelte`.

### Part 3: Tool Execution History & Runtime Latency Badge
- [ ] In `apps/desktop/src/lib/components/layout/OperationsPanel.svelte`:
  - Measure execution start and end time using `performance.now()`.
  - Format duration dynamically: `< 1000ms` as `342ms`, `>= 1000ms` as `1.4s`.
  - Display a clean status badge directly below the Run button:
    - Success: `✓ Completed in {duration}` (subtle green pill badge).
    - Failure: `✗ Failed after {duration}` (subtle red pill badge).
  - Track session history list in `jobsState` or local state so users can see the last completed tasks.

### Part 4: CLI Tier Support & Validation
- [ ] Add `--tier` flag to `paperpilot ocr` CLI command.
- [ ] Run `cargo check -p desktop` and `pnpm check` in `apps/desktop`.
- [ ] Verify unit tests pass: `cargo test -p paperpilot-pdf`.
