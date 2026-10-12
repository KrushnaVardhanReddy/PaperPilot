# Portable Mode & Modular Feature Packs

PaperPilot is designed for absolute privacy, allowing zero-pollution execution natively on host systems and from portable drives.

## Discovery Engine
The storage mechanism checks directories in the following priority order:
1. `./data/` alongside the executable.
2. `PAPERPILOT_DATA_DIR` environment variable.
3. System default (e.g. `~/.local/share/paperpilot` on Linux).

If `./data/` exists, PaperPilot operates in **Portable Mode**, storing all models, AI weights, and configurations inside this folder rather than the host system.

## Modular Packs
To keep the binary size minimal while supporting heavy features (like AI or OCR), PaperPilot uses a Modular Feature Pack approach.

### Available Packs:
- **Core (Built-in)**: Base PDF operations.
- **Office**: Spreadsheets and doc conversions.
- **OCR**: PaddleOCR vision detection.
- **Web**: Chromium-free URL archival.
- **AI**: Local language models and RAG components.

Packs can be selected during the **First-Run Onboarding Wizard** or enabled manually in the Settings panel under the "Feature Packs" tab.
