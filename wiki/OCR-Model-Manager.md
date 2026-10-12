# Modular OCR Model Manager

## Overview

The OCR Model Manager brings the same modular download and selection architecture used for LLMs to the OCR engine. It allows users to easily download, activate, and manage different tiers of PaddlePaddle OCR (PP-OCR) models through a user-friendly UI in the Desktop app, providing an optimal balance between download footprint, execution latency, and recognition accuracy depending on the use case.

## Supported Model Tiers

The manager supports four tiers:

1.  **PP-OCRv6 Tiny (Default)**
    *   **Size**: ~6.0 MB
    *   **Characteristics**: PPLCNetV4, 50+ languages unified dict.
    *   **Target Use Case**: Fast web/desktop execution, lightweight footprint.
2.  **PP-OCRv6 Small**
    *   **Size**: ~30 MB
    *   **Characteristics**: Higher precision detector & recognizer.
    *   **Target Use Case**: Recommended for legal contracts, fine print, complex layouts.
3.  **PP-OCRv6 Medium**
    *   **Size**: ~134 MB
    *   **Characteristics**: Server-grade accuracy.
    *   **Target Use Case**: Extreme precision, low-contrast scans.
4.  **PP-OCRv4 Mobile**
    *   **Size**: ~15.5 MB
    *   **Characteristics**: Legacy RapidOCR baseline.
    *   **Target Use Case**: Compatibility with older single-language pipelines.

## Architecture & Integration

### Backend (`apps/desktop/src-tauri/src/ocr_commands.rs`)

The Rust backend handles local filesystem management and network downloading.
*   **Storage Hierarchy**: Models are saved to `~/.local/share/paperpilot/models/ocr/<tier_id>/`. Developers can override this using the `PAPERPILOT_OCR_MODELS_DIR` environment variable, or fallback to the workspace directory (`models/ocr/<tier_id>/`).
*   **Tauri Commands**:
    *   `get_available_ocr_models`: Returns a list of `OcrModelInfo`, scanning the filesystem to determine which tiers are installed.
    *   `download_ocr_model`: Streams the `det.onnx`, `rec.onnx`, and `dict.txt` files for the specified tier, emitting Svelte-compatible progress events (`ocr-download-progress`).
    *   `cancel_ocr_model_download`: Sets a cancellation flag to abort in-flight downloads.
    *   `set_active_ocr_model` & `get_active_ocr_model`: Manages a global mutex for the actively selected OCR model tier.

### Frontend (`apps/desktop/src/lib/components/settings/OcrModelDownloader.svelte`)

The `OcrModelDownloader` component provides a rich UI mirroring the LLM model downloader:
*   Displays available tiers with size, description, and status.
*   Provides one-click downloading with real-time percentage and byte tracking.
*   Allows selecting an active installed model via radio buttons.
*   Shows explicit error states and allows download cancellation.
*   Embedded seamlessly into the main Settings Panel (`SettingsPanel.svelte`).

### CLI Integration

The Command Line Interface (`paperpilot-cli`) exposes the `ocr` command with a new optional `--tier <tier>` argument. While the current Rust inference engine might use the default, the pipeline is plumbed up through the `handle_ocr` action to receive and acknowledge the user's tier preference, paving the way for dynamic model loading in CLI batch tasks.
