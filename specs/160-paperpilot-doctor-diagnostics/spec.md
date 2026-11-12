# Spec 160: PaperPilot System Doctor & Health Diagnostics (`paperpilot doctor`)

## 1. Executive Summary & Problem Statement
PaperPilot orchestrates multiple complex native components:
- Tauri desktop WebView and native backend
- REST API gateway running on localhost port 7823
- MCP server communication pipes
- Pure-Rust and WASM execution modes
- Local AI model cache (`~/.cache/paperpilot/models` or OS local app data)
- Temporary working directories and system permissions

When users or developers face issues (e.g. port 7823 already bound by another daemon, missing OCR model weights, missing write access in temp directories, or pnpm version mismatches), troubleshooting requires manual terminal inspection.

Spec 160 introduces **`paperpilot doctor`** (CLI) and an automated `/health` / `/diagnostics` endpoint in the REST gateway and Tauri Settings dialog.

---

## 2. Diagnostics Checklist & Probes

The diagnostic engine evaluates 7 core dimensions:
1. **Binary & Version Consistency**: Checks running version against latest GitHub release tag.
2. **Gateway Port Availability**: Probes `127.0.0.1:7823`. Warns if occupied by a conflicting process.
3. **Model Weights & Cache**: Inspects existence and checksums of default SLM/OCR weights (`rten`, `paddleocr`, or local quantized models).
4. **Temporary Filesystem Permissions**: Verifies write, read, and delete permissions in the OS temp directory (`/tmp` or `%LOCALAPPDATA%\Temp`).
5. **Node & Package Tooling (Dev environment)**: Checks `node -v` (>= 18) and `pnpm -v` (>= 8).
6. **Zero-Trace Privacy Assurance**: Validates that zero-trace logging suppression flags are functioning properly without leaking temp files.
7. **Tauri IPC Bridge Readiness**: In desktop context, verifies all registered Tauri commands respond without errors.

---

## 3. CLI Interface Output Specification

```text
$ paperpilot doctor

  PaperPilot Doctor (v0.6.0)
  ==========================
  [✓] Operating System: Linux 6.8.0-x86_64
  [✓] Working Directory Permissions: Read/Write OK
  [✓] Temp Scratch Storage (/tmp/paperpilot): OK (0 orphaned files)
  [✓] REST Gateway Port (7823): Available
  [✓] Core PDF Engine: Pure-Rust lopdf / pdf_oxide active
  [!] OCR Engine: PaddleOCR model missing from ~/.cache/paperpilot/
      ↳ Fix: Run `paperpilot download-models ocr` to enable offline OCR
  [✓] MCP Transport: STDIO stream ready
  [✓] Desktop Integration: Tauri v2 bridge active

  Result: 1 recommendation found. PaperPilot is ready to process documents.
```

Exit code: `0` if all pass or only minor recommendations; `1` if critical blocking failure (e.g. read-only filesystem or port lock).

---

## 4. REST & Desktop Integration
- **REST Gateway**: `GET /health` returns basic `{ "status": "ok" }`; `GET /api/v1/doctor` returns full JSON report.
- **Desktop Settings Panel**: Adds a **"Run System Diagnostics"** button under Settings > Advanced that invokes `run_system_doctor` and renders an intuitive checklist with green/yellow status badges.
