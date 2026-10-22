# Task 5.9.75: Mobile Lean Architecture, Full 26+ WASM Tool Suite & On-Demand OCR Engine

## Status
- Spec: [Spec 089](file:///home/krushna/Project/PaperPilot/specs/089-mobile-lean-architecture-and-on-demand-ocr/spec.md)
- Target: Wave 3 (`v0.7.0`) Mobile Touch-First WASM & Lean APK Optimization
- Status: ⏳ In-Flight (Jules Session [`7525956406631093564`](https://jules.google.com/session/7525956406631093564))

---

## Objectives & Work Items

### 1. Lean Mobile Tauri Build Configuration
- In `apps/desktop/src-tauri/Cargo.toml`, move `paperpilot-gateway` and `paperpilot-mcp` behind `desktop-features` flag (or `not(target_os = "android", target_os = "ios")`).
- Keep `src-tauri` on Android/iOS as a lightweight shell (~3 MB native overhead) handling WebView lifecycle, camera capture, file sandboxing, and share intents.
- Verify APK size drops from 82 MB to $\le$ 22 MB via `make build-android`.

### 2. Full 26+ WASM Tool Catalog & Mobile Information Architecture
- In `apps/desktop/src/lib/components/layout/OperationsPanel.svelte`:
  - When `appState.isMobile` is active, implement the **Quick Actions + Categorized Catalog** hierarchy:
    1. **⚡ Quick Actions Row**: 4 hero actions (`Scan`, `Sign`, `Compress`, `Sanitize / PII`).
    2. **📂 Categorized Catalog Accordions**:
       - **Organize (8)**: `merge`, `split`, `rotate`, `delete_pages`, `extract_pages`, `reorder_pages`, `crop`, `flatten`.
       - **Security & Privacy (5)**: `sanitize`, `pii_scan`, `encrypt`, `decrypt`, `watermark`.
       - **Conversions & Text (7)**: `ocr`, `images_to_pdf`, `pdf_to_docx`, `json_to_pdf`, `extract_images`, `extract_text`, `render_page`.
       - **Enhance & Info (4)**: `page_numbers`, `header_footer`, `set_metadata`, `pdf_info`.
       - **Workflows (2)**: `workflow`, `workflow_presets`.
    3. **Live Search**: Instant client-side fuzzy filter across all 26 tools.
  - Maintain $\ge$48px touch targets, bottom-sheet drawer with drag handle, and `safe-area-inset-bottom` padding.

### 3. Cross-Platform On-Demand OCR Downloader
- Ensure `find_model()` in `paperpilot-pdf/src/operations/ocr.rs` and Tauri commands resolve to the application sandbox directory (`app_local_data_dir() + "/models/ocr/ppocrv6-tiny/"`).
- In `OperationsPanel.svelte` (OCR tool configuration):
  - Check if `ppocrv6-tiny` weights exist locally.
  - If missing, display `○ Model Not Downloaded (~6 MB)`.
  - When "Run OCR" is pressed, show an inline download progress modal streaming the weights, then auto-execute the OCR job.
  - Verify weights persist for offline subsequent executions.

### 4. Verification & Testing
- Android Build: Run `make build-android` and assert `.apk` $\le$ 22 MB.
- Desktop Regression: Run `pnpm --dir apps/desktop exec playwright test` (88+ tests).
- Rust Libs: Run `cargo test -p paperpilot-pdf --lib -j 2` and `cargo test -p paperpilot-wasm --lib -j 2`.
