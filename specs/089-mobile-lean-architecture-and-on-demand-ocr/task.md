# Task 5.9.75: Mobile Lean Architecture, Full 26+ WASM Tool Suite & On-Demand OCR Engine

Status: ✅ Completed

We successfully minimized the Tauri mobile binary payload by properly feature-gating `paperpilot-gateway` and `paperpilot-mcp` behind `desktop-features` in `apps/desktop/src-tauri/Cargo.toml` and `apps/desktop/src-tauri/src/lib.rs`.

In the frontend (`apps/desktop/src/lib/components/layout/OperationsPanel.svelte`), we completely rebuilt the UI to support the `isMobile` mode:
1. Created a **Quick Actions Grid** for the top 4 Mobile actions (Scan/img_to_pdf, Sign, Compress, Sanitize).
2. Clustered all 26+ remaining tools into collapsed accordions logically grouped by category.
3. Implemented a specific view for `ocr` that actively invokes `get_available_ocr_models` to check if `ppocrv6-tiny` is present in the `app_local_data_dir() + "/models/ocr/ppocrv6-tiny/"` folder.
4. Added an active download component that listens to `ocr-download-progress` via Tauri events and renders a real-time progress bar.