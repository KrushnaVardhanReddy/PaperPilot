# Spec 089: Mobile Lean Architecture, Full 26+ WASM Tool Suite & On-Demand OCR Engine

## 1. Background & Executive Summary
PaperPilot’s mobile target (Android APK & iOS IPA via Tauri 2.0) requires rapid startup, responsive touch interactions, a lean download footprint, and complete offline capability.

Currently:
1. **Binary Bloat**: Building native desktop gateway and MCP sidecars into `libdesktop_lib.so` inflates the Android APK to **82 MB**.
2. **Full WASM Engine Capabilities**: The in-memory `paperpilot-wasm` engine already compiles and exposes **26+ production operations** (Organize, Security, Conversions, Page Numbering, Metadata, Workflows, etc.). Mobile should **not** artificially restrict users to a small subset; rather, it should offer the **complete 26+ WASM tool catalog** while presenting a clean, thumb-friendly **Quick Actions + Categorized Catalog** hierarchy.
3. **Cross-Platform On-Demand OCR Engine**:
   - Bundling neural OCR weights (`det.rten`/`rec.rten` or `det.onnx`/`rec.onnx`) directly into the APK/IPA assets adds 12–18 MB directly to the mobile download.
   - Relying on Google Play Services breaks cross-platform parity on iOS.
   - Adopting **Spec 047 (On-Demand Model Download)** keeps the mobile install at **~15–20 MB** while delivering 100% byte-for-byte identical pure-Rust OCR across Android and iOS.

---

## 2. Technical Architecture & Requirements

### 2.1 Lean Mobile Tauri Build Configuration
In `apps/desktop/src-tauri/Cargo.toml`:
- Place desktop-specific daemon and sidecar dependencies behind the `desktop-features` flag:
  - `paperpilot-gateway`
  - `paperpilot-mcp`
- For mobile targets (`target_os = "android"`, `target_os = "ios"`):
  - `src-tauri` acts as a lean native bridge (~3 MB native overhead) handling:
    - WebView initialization.
    - Native camera hardware access (`<input type="file" capture="camera">` / native camera intent).
    - Sandboxed persistent storage paths (`app_local_data_dir`).
    - OS Share Sheet intents (`navigator.share`).
- Document operations on mobile run purely in-memory via `paperpilot-wasm` (`WasmPdfEngine`), avoiding native sidecar overhead.

### 2.2 Complete 26+ WASM Mobile Tool Catalog
All 26+ operations compiled in `paperpilot-wasm` are enabled and accessible on mobile:

| Category | Count | Supported Operations in WASM |
| :--- | :--- | :--- |
| **Organize & Structure** | 8 | `merge`, `split`, `rotate`, `delete_pages`, `extract_pages`, `reorder_pages`, `crop`, `flatten` |
| **Security & Privacy** | 5 | `encrypt`, `decrypt`, `watermark`, `sanitize`, `pii_scan` |
| **Conversions & Extraction**| 7 | `images_to_pdf`, `extract_images`, `extract_text`, `pdf_to_docx`, `json_to_pdf`, `ocr`, `render_page` |
| **Enhance & Metadata** | 4 | `page_numbers`, `header_footer`, `set_metadata`, `pdf_info` / `pdf_hash` |
| **Automation** | 2 | `workflow` (declarative recipes), `workflow_presets` |

### 2.3 Mobile Information Architecture (UI/UX Hierarchy)
To prevent cognitive overload on small screens while giving access to all 26+ operations:
1. **⚡ Quick Actions (Pinned Row / Grid at Top)**:
   - 4–6 prominent, thumb-reachable hero cards for rapid emergency on-the-go tasks:
     - 📷 **Scan (Camera to PDF)**
     - ✍️ **Sign Document**
     - ⚡ **Compress PDF**
     - 🧹 **Deep Sanitize / PII Scan**
2. **📂 Categorized Tool Catalog (Accordions / Tabs)**:
   - Below Quick Actions, the full 26-tool catalog is grouped into expandable sections:
     - **Organize (8)**: Merge, Split, Rotate, Delete, Extract, Reorder, Crop, Flatten
     - **Security & Privacy (5)**: Sanitize, PII Scan, Encrypt, Decrypt, Watermark
     - **Conversions & Text (7)**: OCR, PDF to DOCX, JSON to PDF, Extract Images/Text
     - **Formatting & Metadata (4)**: Page Numbers, Header & Footer, Metadata, PDF Info
3. **Search & Filter Bar**:
   - Quick search input (`Search 26+ tools...`) with instantaneous fuzzy filtering.

### 2.4 Cross-Platform On-Demand OCR Downloader
1. **Lightweight Base Install**:
   - Initial APK / IPA download does **not** package model weights.
   - APK size stays lean at **~15–20 MB**.
2. **First-Run Download & Storage**:
   - When the user selects `ocr` from the mobile catalog:
   - The app inspects the local sandboxed directory:
     - **Android**: `app_local_data_dir() + "/models/ocr/ppocrv6-tiny/"`
     - **iOS**: `app_local_data_dir() + "/models/ocr/ppocrv6-tiny/"`
   - If missing:
     - Prompts user with a touch dialog: **"Enable Offline OCR (6 MB)"** highlighting that recognition is 100% on-device and private.
     - On user tap, streams `det.onnx`, `rec.onnx`, and `ppocr_keys_v1.txt` with chunked progress reporting (`ocr-download-progress`).
     - Once downloaded, it is permanently cached for offline use.

---

## 3. UI Implementation Details

### 3.1 `OperationsPanel.svelte` (Mobile Mode)
- When `appState.isMobile` is active:
  - Header: Search bar + quick category chips.
  - Section 1: **Quick Actions** (horizontal swipeable carousel or 2x2 grid).
  - Section 2: **Categorized Catalog** with count badges (e.g. `Organize (8)`, `Security (5)`).
  - All cards have $\ge$48px touch targets, rounded borders, clear icons, and haptic-friendly tap states.
- Active Tool Drawer:
  - Opens as a slide-up bottom sheet with `safe-area-inset-bottom` padding and a clear drag handle / ✕ close button.

### 3.2 In-Panel OCR Model Downloader UI
- Under the OCR configuration view:
  - Status indicator: `● OCR Ready (Offline)` or `○ Model Not Downloaded (~6 MB)`.
  - Tapping **"Run OCR"** when not downloaded triggers an in-place download progress bar with cancel capability.

---

## 4. Acceptance Criteria & Verification
1. **APK Binary Size**: Final Android APK built via `make build-android` is $\le$ 22 MB (down from 82 MB).
2. **Full Tool Parity**: All 26+ WASM tools execute without crashing inside the mobile WebView.
3. **Mobile UX Validation**: Quick Actions are immediately visible on mobile without scrolling; category accordions expand to reveal all tools.
4. **On-Demand OCR**:
   - Clean install starts without models.
   - First OCR run downloads `ppocrv6-tiny` (~6 MB) smoothly.
   - Subsequent runs (and offline mode) execute OCR instantly using cached weights.
5. **Zero Desktop Regression**: Desktop application continues to offer all 50+ tools, developer sidecars, and MCP workflows.
