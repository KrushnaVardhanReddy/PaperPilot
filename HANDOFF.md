# PaperPilot — Session Handoff Guide

> **Single Source of Truth** for resuming work on PaperPilot.
> Start any new agent or human session by reading this document.
> **Date of Last Update:** October 15, 2026 (Evening EDT)
> **Active Target:** v0.5.0 Public Preview Release & Spec 055/056 Parity Hardening

---

## 🧭 Executive Context & Active Sessions

1. **Active Jules Sessions in Flight / Pending**:
   - **Task 5.9.42 / Spec 065 (High-Fidelity Responsive Flow Layout & Inline Image Reconstruction for PDF-to-DOCX)**: [Session 3442134502545175938](https://jules.google.com/session/3442134502545175938) — Upgrading `PdfToDocxOperation` in `paperpilot-pdf` with font subset prefix sanitization (preventing `□□□□` tofu glyphs in LibreOffice/Word), borderless 2-column OpenXML tables (`docx-rs`) for side-by-side icon/text pairing, inline PNG image embedding, and compact page margin geometry keeping single-page documents to strictly 1 page.
   - **Task 5.9.39 / Spec 063 (Mobile Touch UI, Camera Scanner & Adaptive Layout)**: [Session 5037301362112166263](https://jules.google.com/session/5037301362112166263) — Unified Tailwind v4 layout, in-memory mobile intake, camera scan with auto-OCR, bottom sheet, and APK size optimization.

2. **Recently Completed & Merged Milestones**:
   - **Spec 066 (Tool Disambiguation & Explicit Input/Output Format Badges)**: Merged (PR #207, commit `42b6f29`, Jules Session 14117039404412235313).
   - **Spec 064 (All 50 Tools Clean-Room Isolated Sandbox Audit)**: Completed (Session 13530573138717224797) — `make test-cleanroom-all50`, 50/50 100% PASS.
   - **Spec 062 (Tauri 2 Native Mobile Backend Scaffolding)**: Merged (PR #203, commit `07c4a56`).
   - **Spec 060 (1-Hour Sustained Soak Benchmark)**: Merged (PR #205, commit `a2b7c9d`) — 8,180,016 ops, 2,272.2 ops/sec, zero leaks.
   - **Spec 059 (Engine Hardening & Polish)**: Merged (PR #200, commit `537550c`).

---

## 🏗️ Architecture & Crate Boundaries

```
PaperPilot/
├── paperpilot-core/        # Rust: Core traits (PdfOperation, PdfDocument), error & result types
├── paperpilot-pdf/         # Rust: All 45+ PDF operations (lopdf, pdf_oxide, fulgur, docx-rs, rusto-rs)
├── paperpilot-nlp/         # Rust: Offline Intent Classification & NLP Rule Engine
├── paperpilot-ai/          # Rust: Universal LLM client & router
├── paperpilot-mcp/         # Rust: Model Context Protocol JSON-RPC server (51 tools)
├── paperpilot-gateway/     # Rust: Axum HTTP REST server & MCP SSE gateway (port 7823)
├── paperpilot-cli/         # Rust: Native CLI binary (paperpilot merge, split, convert, ocr, etc.)
├── paperpilot-wasm/        # Rust: In-memory WebAssembly engine for client-side browser execution
├── apps/desktop/           # Tauri 2.0 + Svelte 5 + TypeScript desktop app
├── apps/edge/              # Cloudflare Workers serverless microservice
├── tools/penta-interface-e2e/ # Pure-Rust Penta-Interface E2E & benchmark suite (CLI, MCP, REST, WASM, Edge)
├── tools/jules-submit/     # Pure-Rust CLI for dispatching Jules tasks
├── reports/                # E2E test reports, benchmarks, and launch scorecards
└── specs/                  # Architecture & verification specifications
```

---

## ⚖️ Rules of Engagement

1. **The Penta-Interface Mandate:**
   - 💻 **CLI (`paperpilot-cli`)**: Kebab-case subcommand with `--json` support.
   - 🤖 **MCP (`paperpilot-mcp`)**: JSON-RPC 2.0 tool declaration with required input schema and execution match arm.
   - 🌐 **REST API (`paperpilot-gateway`)**: POST route `/api/v1/pdf/tools/{tool}` and specialized endpoints.
   - ⚡ **WASM (`paperpilot-wasm`)**: In-memory byte processing without filesystem IO.
   - ☁️ **Cloudflare Edge (`apps/edge`)**: Memory-only stream route `/api/v1/{tool}`.

2. **CRITICAL Concurrency Limit (`-j 4` or `-j 6`):**
   - **NEVER** run unbounded `cargo test` or `cargo build`.
   - Always pass `-j 4` or `-j 6` to avoid system resource starvation.

3. **Workspace Safety Rules:**
   - **NEVER** execute `git reset`, `git checkout -- .`, `git clean -fd`, or `git restore` under any circumstance.
   - Preserve all uncommitted work; do not wipe working changes on test failures.

4. **Zero-Mock Policy:**
   - No `let mut passed = true;` or dummy echo scripts in test suites.

---

## 📋 Resuming Checklist for Next Agent / Developer

### Step 1: Check In on Active Jules Sessions
- Inspect status of:
  - [Session 623008379935493326](https://jules.google.com/session/623008379935493326) (Task 5.9.32: Spec 056).
  - [Session 2996870268221322258](https://jules.google.com/session/2996870268221322258) (Task 5.9.31: Spec 055).
- If a PR or branch is pushed by Jules:
  ```bash
  git fetch origin
  git log -n 5 --oneline origin/main
  ```

### Step 2: Validate the Conversions Suite
- Once Task 5.9.32 changes land:
  ```bash
  cargo test -p paperpilot-gateway -j 4
  cargo run -p penta-interface-e2e -- --group conversions -j 4
  ```
- Confirm `reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4D.md` shows **301 / 301 (100%) PASS**.

### Step 3: Run Full Penta Suite
- Run all groups to confirm no regressions:
  ```bash
  cargo run -p penta-interface-e2e -- --group all -j 4
  ```

### Step 4: Publish v0.5.0 Public Preview Release
- When both Spec 055 and Spec 056 are merged and verified:
  - Trigger `.github/workflows/release.yml` on GitHub Actions with tag `v0.5.0` (`prerelease: true`).
  - Confirm artifacts compile for Linux, macOS, and Windows.

---

## 🛠️ Quick Command Reference

```bash
# 1. Dev Server (Desktop Tauri + Svelte 5)
make dev

# 2. Rust Unit Tests
cargo test -p paperpilot-pdf --lib -j 4
cargo test -p paperpilot-mcp --lib -j 4
cargo test -p paperpilot-gateway --lib -j 4

# 3. Penta-Interface E2E Suite
cargo run -p penta-interface-e2e -- --group conversions -j 4
cargo run -p penta-interface-e2e -- --group all -j 4

# 4. Playwright Desktop E2E
pnpm --dir apps/desktop exec playwright test tests/e2e_real_desktop_execution.spec.ts --workers=1
pnpm --dir apps/desktop exec playwright test tests/e2e_all_tools_real_execution.spec.ts --workers=1

# 5. Jules Submission CLI
cargo run -p jules-submit -- --list
```
