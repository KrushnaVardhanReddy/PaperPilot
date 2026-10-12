# Spec 049: Portable App Architecture & Modular Feature Pack Onboarding

## Overview
PaperPilot is engineered as a privacy-first, zero-cloud desktop application. A core product requirement is **true portability**:
1. **Zero System Pollution**: No mandatory Windows Registry writes, no required admin privileges, no global filesystem locks.
2. **USB / Self-Contained Execution**: The ability to run directly from a portable folder or USB drive.
3. **Modular Feature Packs**: Allowing users to select which tool capabilities they need (Core PDF, Office & Spreadsheets, Vision/OCR, Web Archival, Local AI) so the initial download remains ultra-lightweight (~10–35 MB) while expanding into a full suite on demand.

This specification details:
1. The **Portable Data Directory Discovery Engine** (`./data/` detection relative to the executable).
2. The **Modular Feature Pack Manifest & Registry** (`core`, `office`, `ocr`, `web`, `ai`).
3. The **First-Run Onboarding Setup Wizard** in the Svelte desktop UI for custom package selection.

---

## 1. Portable Storage Architecture

### 1.1 Discovery Hierarchy
Every stateful subsystem (OCR models, AI weights, user preferences, tool plugins, cache) discovers its base storage root using the following strict priority:

```rust
pub fn get_app_data_root() -> std::path::PathBuf {
    // 1. Portable Mode: Check for relative ./data/ next to executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let portable_dir = exe_dir.join("data");
            if portable_dir.exists() && portable_dir.is_dir() {
                return portable_dir;
            }
        }
    }

    // 2. Environment Variable Override
    if let Ok(env_override) = std::env::var("PAPERPILOT_DATA_DIR") {
        return std::path::PathBuf::from(env_override);
    }

    // 3. Standard Local System App Data Fallback
    dirs::data_local_dir()
        .map(|d| d.join("paperpilot"))
        .unwrap_or_else(|| std::path::PathBuf::from(".data"))
}
```

### 1.2 Portable Directory Structure
When running in portable mode, the directory layout is completely self-contained:
```text
PaperPilot-Portable/
├── PaperPilot.exe (or PaperPilot.AppImage)
└── data/
    ├── config.json          # User preferences & enabled feature packs
    ├── tools/               # Dynamic feature bundles
    │   ├── office/          # Office & Spreadsheet expansion binaries/assets
    │   └── web/             # BrowserOxide web archive assets
    └── models/
        ├── ocr/             # PaddleOCR / RapidOCR model tiers
        └── ai/              # Local SLM weights (Llamafile / GGUF)
```

---

## 2. Modular Feature Pack Registry

The PaperPilot suite is divided into 5 clear capability packages:

| Pack ID | Pack Name | Size | Included Tools / Scope | Default Status |
| :--- | :--- | :--- | :--- | :--- |
| `core` | **Core PDF Manipulation** | Built-in | Merge, Split, Rotate, Crop, Encrypt, Decrypt, Watermark, Compress, Info, Annotate | ✅ Pre-installed (Always Active) |
| `office` | **Office & Spreadsheets** | ~12 MB | `office_to_markdown`, `office_extract_text`, `excel_to_json`, `excel_to_csv`, DOCX/PPTX modernization | ⚡ Optional (On-demand) |
| `ocr` | **Vision & Searchable OCR** | ~35 MB | PaddleOCR neural detector & recognizer (`rusto-rs` models), PDF text layer injection | ⚡ Optional (On-demand) |
| `web` | **Web & URL Capture** | ~22 MB | Chromium-free pure-Rust web page archival & URL-to-PDF engine (`browser_oxide`) | ⚡ Optional (On-demand) |
| `ai` | **Local AI Discovery Agent** | ~1.5 GB | Offline intent classification, RAG legal discovery assistant, local SLM | ⚡ Optional (Power Users) |

---

## 3. Desktop UI: First-Run Portable Setup Wizard

### 3.1 Initial Launch Detection
On startup, `apps/desktop` checks if `onboarding_completed` is set in local storage / `config.json`.
If `false`:
- Renders the **Portable Feature Setup Modal** before opening the main document workspace.

### 3.2 Setup Wizard UI Features
1. **Portable Mode Status Indicator**:
   - Displays a green badge: `💾 Portable Mode: Active (data/ detected)` or `💻 System Installed Mode`.
2. **Package Selection Checkboxes**:
   - Each package displays:
     - Clear icon and descriptive title.
     - Approximate download size.
     - Checklist of tools it unlocks.
3. **One-Click Quick Presets**:
   - **⚡ Minimalist**: Core PDF only (0 MB download, instant launch).
   - **💼 Productivity (Recommended)**: Core PDF + Office & Spreadsheets + Vision OCR (~47 MB).
   - **🚀 Full Suite**: All packs enabled.
4. **Action Buttons**:
   - **"Download & Launch"**: Streams downloads with real-time progress bars directly into `./data/`.
   - **"Skip for Now"**: Launches immediately with Core tools; other tools display download badges on click in the Operations Panel.

---

## 4. Backend Tauri Commands (`pack_commands.rs`)

1. `is_portable_mode() -> Result<bool, String>`: Returns whether `./data/` is active.
2. `get_feature_packs() -> Result<Vec<FeaturePackInfo>, String>`: Returns all 5 packs with installed status and local path.
3. `download_feature_pack(pack_id: String)`: Streams package files with chunked progress events (`pack-download-progress`).
4. `set_feature_pack_enabled(pack_id: String, enabled: bool)`: Toggles active state in `config.json`.

---

## 5. Quality & Acceptance Criteria
1. `is_portable_mode` correctly returns `true` when a `data/` directory exists next to the binary.
2. All downloaded models and packs land inside `./data/` when portable mode is active.
3. Desktop UI onboarding wizard cleanly dismisses and saves preference state.
4. Existing operations continue to pass all tests with zero regression.
