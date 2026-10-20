# PaperPilot — Session Handoff Guide

> **Single Source of Truth** for resuming work on PaperPilot.  
> Start any new agent or human session by reading this document.  
> **Date of Last Update:** October 20, 2026 (Afternoon EDT)  
> **Active Target:** Wave 2 (`v0.6.0`) Media & Page Utilities + Pipeline Flow Management

---

## 🧭 Executive Context & Active In-Flight Sessions

### 1. Active Jules Sessions in Flight
| Task ID | Session Link | Target & Purpose | Status / Notes |
|---|---|---|---|
| **5.9.70** | [Session 4388001643990007173](https://jules.google.com/session/4388001643990007173) | **Wave 2 Penta-Interface E2E & Unmocked Playwright Testing Suite (6 Tools)** | 🔄 In Progress — Unmocked verification across CLI, MCP, REST, WASM Playwright, and Desktop for Wave 2 tools (`img_to_pdf`, `extract_images`, `remove_blank`, `page_numbers`, `burst`, `search`). Note: Previous sessions 8451709229146498568 & 12126426629876311080 were superseded. |
| **5.9.55** | [Session 378425988505835118](https://jules.google.com/session/378425988505835118) | **Decoupled Markdown-to-DOCX Compiler with DrawingML Media & Layout Synthesis** (`[Spec 077]`) | 🔄 In Flight — Stage 2: Consumes rich Markdown with base64 data URIs from Spec 076, translates to `docx_rs` elements, and embeds genuine `docx_rs::Pic` images. |

---

## 🏆 Recently Completed & Merged Milestones (Oct 19–20, 2026)

1. **Task 5.9.69A / Spec 081 (Pipeline Flow State, Local Persistence & Industry Presets)**:
   - Merged (PR #220, commit `460ec1d`, Jules Session 8874502492856873220).
   - Implemented `SavedPipeline` data model, `PipelineState` methods (`initStorage`, `loadFlow`, `saveCurrentFlow`, `deleteFlow`, `exportFlowJson`, `importFlowFromJson`).
   - Integrated 3 default industry presets (`preset-legal-discovery`, `preset-clean-scan`, `preset-redact-protect`).
   - Added complete Vitest unit test suite (7/7 tests passing in `pipeline.test.ts`).
2. **PR #219 (Windows & Cross-Platform Makefile Compatibility)**:
   - Resolved merge conflicts with `main` (`.cargo/config.toml`, `Makefile`, `package-lock.json`).
   - Added cross-platform port 1420 cleanup in `Makefile` using PowerShell on Windows and `fuser` on Linux.
   - Preserved MSVC `/STACK:8388608` and `/PDBPAGESIZE:8192` flags.
2. **53 Tools Documentation Plain-Language Sync (`apps/docs`)**:
   - Added clear, simple "### What it does" explanations across all 53 individual tool markdown files in `apps/docs/src/content/docs/reference/tools/`.
3. **Spec 081 Authored & Split for Wave 2**:
   - Structured into Community Free Tier ($0, unlimited local storage in `localStorage`, file-based JSON export/import, standard industry presets) vs Enterprise Tier (team workspaces, cloud sync, RBAC).
   - Authoring completed in [`specs/081-pipeline-flow-management-save-export-import/spec.md`](file:///home/krushna/Project/PaperPilot/specs/081-pipeline-flow-management-save-export-import/spec.md).
   - Split into Task 5.9.69A (State & Storage) and Task 5.9.69B (UI & FlowSelector matching `AiSettingsModal.svelte`).
4. **Task 5.9.68 / Spec 079 (Universal Pipeline Parity & Comprehensive Parameter Schemas)**:
   - Merged in commit `b31bc2c`: added full parameter configuration schemas for all tools in Desktop and Embed, and resolved intermediate split fan-out.
5. **CI Release Workflow & Multi-Platform Fixes**:
   - Upgraded Android runner to `packages: 'platform-tools'`.
   - Updated macOS build target to Apple Silicon `aarch64-apple-darwin`.
   - Added `swatinem/rust-cache@v2` across all platform jobs.
6. **Task 5.9.54 / Spec 076 (High-Fidelity PDF-to-Markdown & HTML with Media)**:
   - Merged (PR #216, commit `66534e7`): Stage 1 extracts raster photos and vector Form XObjects (`hayro` rasterization) into base64 data URIs.

---

## 🏗️ Architecture & Key Crate Boundaries

```
PaperPilot/
├── paperpilot-core/        # Core traits (PdfOperation, PdfDocument), error & result types
├── paperpilot-pdf/         # All 50+ PDF operations (lopdf, pdf_oxide, fulgur, docx-rs, rusto-rs)
├── paperpilot-nlp/         # Offline Intent Classification & NLP Rule Engine
├── paperpilot-ai/          # Universal LLM client & router (Ollama, Llamafile, BYOK)
├── paperpilot-mcp/         # Model Context Protocol JSON-RPC server (52 tools)
├── paperpilot-gateway/     # Axum HTTP REST server & MCP SSE gateway (port 7823)
├── paperpilot-cli/         # Native CLI binary (paperpilot merge, split, burst, search, etc.)
├── paperpilot-wasm/        # In-memory WebAssembly engine for client-side browser execution
├── apps/desktop/           # Tauri 2.0 + Svelte 5 + Tailwind desktop app (with visual pipeline runner)
├── apps/embed/             # Embeddable zero-config JS widget & automation pipeline
├── apps/web/               # In-browser WASM client portal
├── apps/docs/              # Astro Starlight documentation portal (docs.usepaperpilot.com)
├── tools/penta-interface-e2e/ # Pure-Rust Penta-Interface E2E suite across all 5 surfaces
├── tools/jules-submit/     # Pure-Rust CLI for dispatching Jules tasks
├── reports/                # E2E test reports, benchmarks, and scorecards
└── specs/                  # Architecture, product specs, and verification checklists
```

---

## ⚖️ Rules of Engagement & System Constraints

1. **System Resource Constraint (`-j 2` is MANDATORY on local machine):**
   - **ALWAYS** pass `-j 2` for all local `cargo test`, `cargo build`, or `cargo clippy` commands (e.g. `cargo test -p paperpilot-mcp -j 2`).
   - Unbounded cargo compilations freeze the host machine.

2. **The Penta-Interface Parity Mandate:**
   - 💻 **CLI (`paperpilot-cli`)**: Kebab-case subcommand with `--json` support.
   - 🤖 **MCP (`paperpilot-mcp`)**: JSON-RPC 2.0 tool declaration with required input schema and execution match arm.
   - 🌐 **REST API (`paperpilot-gateway`)**: POST route `/api/v1/pdf/...` with JSON payload.
   - ⚡ **WASM (`paperpilot-wasm`)**: In-memory byte processing without filesystem IO.
   - 📱 **Desktop / Embed API (`apps/desktop`, `apps/embed`)**: Visual cards, parameter schemas, and pipeline execution.

3. **Workspace Safety Rules:**
   - **NEVER** execute `git reset --hard`, `git checkout -- .`, `git clean -fd`, or `git restore` under any circumstance.
   - Always commit or stash working progress.

4. **Zero-Mock Policy:**
   - All tests must use real disk fixtures and real in-memory byte buffers (`fixtures/`, `tests/e2e_fixtures/real/`). No dummy `{ success: true }` echo stubs.

---

## 📋 Resuming Checklist for Next Session

### Step 1: Monitor Active Jules Sessions
- **Task 5.9.69A** ([Session 8874502492856873220](https://jules.google.com/session/8874502492856873220)):
  - ✅ Verified `npm test` passing (79/79) and merged into `main` (PR #220, commit `460ec1d`).
- Check status of **Task 5.9.70** ([Session 4388001643990007173](https://jules.google.com/session/4388001643990007173)):
  - Review `reports/WAVE_2_PENTA_INTERFACE_SCORECARD.md` and test results.
- Check status of **Task 5.9.55** ([Session 378425988505835118](https://jules.google.com/session/378425988505835118)).

### Step 2: Trigger Task 5.9.69B (Pipeline UI & Save Modal)
- Ready to dispatch now that Task 5.9.69A has landed on `main`:
  ```bash
  ./scripts/jules_submit.sh --file prompts/tasks/phase5/P5_9_69B_pipeline_flow_selector_ui.txt
  ```
- This will implement `FlowSelector.svelte` and `SaveFlowModal.svelte` based on the tested state API, plus embed preset resolution.

### Step 3: Trigger Task 5.9.69C (Penta-Interface Declarative Workflow Engine)
- Follow-up task bringing full Penta-Interface parity for pipeline workflows:
  ```bash
  ./scripts/jules_submit.sh --file prompts/tasks/phase5/P5_9_69C_workflow_engine_penta_interface.txt
  ```
- Implements in-memory sequential execution in `paperpilot-pdf`, CLI `paperpilot workflow`, MCP tool `document_workflow`, REST Gateway endpoints, and 20 E2E assertions in `tools/penta-interface-e2e`.

### Step 4: Run Wave 2 Verification Tests
- Run Wave 2 MCP tests:
  ```bash
  cargo test -p paperpilot-mcp --test wave2_mcp_tests -j 2
  ```
- Run desktop check:
  ```bash
  npm --prefix apps/desktop run check
  ```

---

## 🛠️ Quick Command Reference

```bash
# 1. Dev Server (Desktop Tauri + Svelte 5)
make dev

# 2. Rust Unit Tests (Always use -j 2)
cargo test -p paperpilot-pdf --lib -j 2
cargo test -p paperpilot-mcp --lib -j 2
cargo test -p paperpilot-gateway --lib -j 2

# 3. Clean-Room Audits
make test-cleanroom-top12
make test-cleanroom-all50

# 4. Playwright Desktop E2E
pnpm --dir apps/desktop exec playwright test

# 5. Dispatch Jules Tasks
./scripts/jules_submit.sh --file prompts/tasks/phase5/<prompt-file>.txt
```
