# Spec 047: Modular OCR Model Downloader and Execution History Badge

## Overview
This specification details two high-impact user experience upgrades:
1. **Modular OCR Model Downloader & Registry**: A tiered OCR model download manager (matching the existing LLM ModelDownloader architecture) allowing users to select and download different PP-OCR model tiers (`PP-OCRv6 Tiny`, `PP-OCRv6 Small`, `PP-OCRv6 Medium`, and `PP-OCRv4 Mobile`).
2. **Tool Execution History & Runtime Badge**: An execution feedback bar below the action button in `OperationsPanel.svelte` displaying the precise completion latency (e.g. `Completed in 342ms` or `Completed in 1.4s`) and the status of the last executed operation.

---

## 1. Modular OCR Model Registry Architecture

### Supported Tiers & Model Details
| Tier ID | Model Name | Total Size | Characteristics | Target Use Case |
| :--- | :--- | :--- | :--- | :--- |
| `ppocrv6-tiny` | **PP-OCRv6 Tiny (Default)** | ~6.0 MB | PPLCNetV4, 50+ languages unified dict | Fast web/desktop execution, lightweight footprint |
| `ppocrv6-small` | **PP-OCRv6 Small** | ~30 MB | Higher precision detector & recognizer | Recommended for legal contracts, fine print, complex layouts |
| `ppocrv6-medium`| **PP-OCRv6 Medium** | ~134 MB | Server-grade accuracy | Extreme precision, low-contrast scans |
| `ppocrv4-mobile`| **PP-OCRv4 Mobile** | ~15.5 MB | Legacy RapidOCR baseline | Compatibility with older single-language pipelines |

### Storage & Discovery Hierarchy
Models are downloaded to and discovered from:
1. User home model cache: `~/.local/share/paperpilot/models/ocr/<tier_id>/`
2. Workspace directory fallback: `models/ocr/<tier_id>/`
3. Custom environment variable override: `PAPERPILOT_OCR_MODELS_DIR`

Each model directory contains:
* `det.onnx` (Detection model)
* `rec.onnx` (Recognition model)
* `dict.txt` (Character mapping dictionary)

### Backend Tauri Commands (`ocr_commands.rs`)
Matching the pattern of `ai_commands.rs`:
* `get_available_ocr_models() -> Result<Vec<OcrModelInfo>, String>`: Inspects local storage and returns registry with download status.
* `download_ocr_model(app: AppHandle, model_id: String) -> Result<(), String>`: Streams model files with chunked progress events (`ocr-download-progress`).
* `cancel_ocr_model_download(model_id: String) -> Result<(), String>`: Cancels in-flight download.
* `set_active_ocr_model(model_id: String) -> Result<(), String>`: Persists the active tier in app configuration.
* `get_active_ocr_model() -> Result<String, String>`: Returns the active tier (defaults to `ppocrv6-tiny` or available installed tier).

---

## 2. Desktop UI Integration

### A. Settings Panel: OCR Model Downloader (`OcrModelDownloader.svelte`)
* Located under Settings > **Models & AI** (or alongside LLM ModelDownloader).
* Shows cards for each tier with:
  * Radio button to activate installed model.
  * Download progress bar (% and downloaded MB / total MB).
  * Direct file path indicator.
  * Instant feedback on active model.

### B. Action Area: Runtime Latency & Last Task Status Badge (`OperationsPanel.svelte`)
Directly below the primary `Run {Tool}` button in `OperationsPanel.svelte`:
* **During execution**: Live ticking timer (`⏱ 1.4s elapsed`).
* **After completion**: A sleek status badge:
  * Success: `✓ Completed in 380ms` (or `✓ Completed in 2.1s`) with a subtle green accent pill.
  * Error: `✗ Failed after 120ms` with a red accent pill and quick link to view error details.
  * History popover / drawer showing the last operations run with latency breakdown.

---

## 3. CLI Command Expansion
* `paperpilot ocr download --tier <tiny|small|medium|v4>`: CLI downloader command.
* `paperpilot ocr --input <file> --tier <tier>`: Ad-hoc tier selection flag.
