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
| **5.9.59** | [Session 14243955832243643866](https://jules.google.com/session/14243955832243643866) | **Privacy & Deep Security Sanitizer (`pdf_sanitize`)** (`[Spec 085]`) | 🔄 In Flight — Pure-Rust deep security scrubber stripping `/Info`, XML `/Metadata`, `/EmbeddedFiles`, `/JS` executable actions, `/PieceInfo`, and thumbnail caches via the Single Core Runner + 5 Thin Wrappers pattern. |

---

## 🏆 Recently Completed & Merged Milestones (Oct 19–21, 2026)

1. **Task 5.9.69C (Penta-Interface Declarative Workflow Engine & Recipe Execution)**:
   - Completed & verified (Sessions `3461133562467863809` / `15691603918731845147`).
   - Implemented Single Core Runner `run_workflow_bytes` in `paperpilot-pdf/src/operations/workflow.rs`.
   - Built 5 thin wrappers across Native CLI (`paperpilot workflow`), MCP (`list_workflows`, `document_workflow`), REST Gateway (`/api/v1/workflow/*`), WASM engine (`workflow`), and Cloudflare Edge worker.
   - 20/20 (100.0%) unmocked Penta-Interface E2E tests passing in `reports/PENTA_INTERFACE_E2E_WORKFLOW_REPORT.md`.
2. **Task 5.9.72 (Embed Pipeline 1-to-1 Desktop Parity: Dropzone, Interactive Steps, Save/Export/Import)**:
   - Merged (PR #225, commit `3354863`, Jules Session 6908271606884666943).
   - Replaced hidden file input with prominent `📁 Browse Files` button and file status card inside the pipeline dropzone.
   - Implemented interactive stepper with step removal (`✕`) and dynamic tool picker modal (`+ Add Step`).
   - Implemented flow management toolbar: presets & saved flows dropdown, `💾 Save Flow` (`localStorage`), `⬇️ Export JSON`, and `⬆️ Import JSON`.
   - Updated `apps/web/public/embed-test.html` Demo 3 and verified 12/12 Playwright tests passing.
   - Documented in `wiki/EmbedPipeline.md`.
2. **Task 5.9.71 (Universal "Export as Code / API / MCP Prompt" Workbench for Action Cards)**:
   - Merged (PR #224, commit `95fcd5a`, Jules Session 17427853372924290951).
   - Created centralized headless snippet generation engine in `apps/desktop/src/lib/utils/snippetGenerator.ts`.
   - Updated `ExportCodeModal.svelte` to dynamically render snippets for both individual tools and multi-step pipeline recipes.
   - Added `</> Code / API` button to `EditableActionCard.svelte` with live parameter synchronization.
   - All 6 unit tests in `snippetGenerator.test.ts` passing, 0 svelte-check errors, and build verified.
2. **Task 5.9.55 (Decoupled Markdown-to-DOCX Compiler with DrawingML Images & Layout Synthesis)**:
   - Merged (commit `8ddc7ff`, Jules Session 378425988505835118).
   - Implemented Stage 2 decoupled conversion pipeline in `paperpilot-pdf/src/operations/conversion.rs`: consumes Stage 1 rich Markdown with inline base64 data URIs, parses via `pulldown_cmark`, decodes image bytes, scales dimensions to OpenXML DrawingML EMUs, and embeds genuine `docx_rs::Pic` image parts into `.docx` archives with 2-column card layout synthesis.
   - All unit tests passing (`test_pdf_to_docx_with_images`).
2. **Task 5.9.69D (Visual Pipeline "Export as Code / API / MCP Prompt" Modal)**:
   - Merged (PR #223, commit `5b4436b`, Jules Session 4310793047227771998).
   - Created `ExportCodeModal.svelte` for rendering live dynamic cURL, CLI, MCP prompt, and Embed JS code snippets from current pipeline steps.
   - Integrated `<button id="flow-export-code-btn">` into `FlowSelector.svelte` toolbar.
   - Added automated Playwright test in `e2e_pipeline_flow_management.spec.ts` asserting all 4 tabs and modal open/close lifecycle (100% passing).
2. **Task 5.9.69B (Visual Pipeline Flow Selector Toolbar & Save/Import Dialogs)**:
   - Merged (PR #222, commit `adf293c`, Jules Session 16322387546350290979).
   - Implemented `FlowSelector.svelte` toolbar with standard presets and saved user flows.
   - Implemented `SaveFlowModal.svelte` with step sequence badges and local persistence.
   - Updated `Widget.svelte` in `apps/embed` to map preset IDs properly.
   - Added automated Playwright test `apps/desktop/tests/e2e_pipeline_flow_management.spec.ts` passing 100%.
2. **Task 5.9.70 (Wave 2 Penta-Interface E2E Testing & Playwright Verification)**:
   - Merged (PR #221, commit `b7ff7c3`, Jules Session 4388001643990007173).
   - Validated unmocked real assertions across all 5 interfaces (CLI, MCP, REST Gateway, WASM Playwright, Desktop) for 6 Wave 2 tools (`img_to_pdf`, `extract_images`, `remove_blank`, `page_numbers`, `burst`, `search`).
   - Verified 100% test pass rate in `paperpilot-mcp/tests/wave2_mcp_tests.rs`, CLI audit suite, and Gateway tests.
   - Published testing architecture in `wiki/09-Wave-2-Penta-Testing.md`.
2. **Task 5.9.69A / Spec 081 (Pipeline Flow State, Local Persistence & Industry Presets)**:
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

3. **The Single Core Runner + Interface Wrappers Architecture Pattern (MANDATORY FOR ALL NEW TOOLS & WORKFLOWS):**
   - **Problem It Solves:** Prevents logic duplication and fragile implementations across the 5 surfaces (CLI, MCP, REST Gateway, WASM, Desktop/Embed).
   - **The Rule:** Every tool or workflow operation MUST have exactly **ONE** core in-memory engine implementation:
     ```rust
     // Pure in-memory: bytes in -> bytes out (in paperpilot-pdf)
     pub fn run_operation_bytes(input_bytes: &[u8], config: &Config) -> Result<Vec<u8>, OperationError>
     ```
   - **The 5 Thin Interface Wrappers (1–5 lines each):**
     1. 💻 **CLI (`paperpilot-cli`)**: `let bytes = fs::read(&input)?; let out = run_operation_bytes(&bytes, &cfg)?; fs::write(&output, out)?;`
     2. 🤖 **MCP (`paperpilot-mcp`)**: Reads file path $\to$ calls `run_operation_bytes` $\to$ writes output path $\to$ returns JSON summary.
     3. 🌐 **REST Gateway (`paperpilot-gateway`)**: Receives multipart upload $\to$ calls `run_operation_bytes` $\to$ streams PDF binary response.
     4. ⚡ **WASM Engine (`paperpilot-wasm`)**: Receives `Uint8Array` $\to$ calls `run_operation_bytes` $\to$ returns `Uint8Array`.
     5. 📱 **Web Embed & Desktop (`apps/embed`, `apps/desktop`)**:
        - In Embed/Web WASM: Calls the WASM wrapper directly in-memory via `window.PaperPilot` / WebWorker.
        - In Desktop: Calls either the native Rust sidecar via Tauri IPC or the Axum Gateway.
   - **Zero Intermediate Disk I/O:** Pipelines chain output bytes from step $N$ directly into step $N+1$ in RAM without temporary disk roundtrips.

4. **Workspace Safety Rules:**
   - **NEVER** execute `git reset --hard`, `git checkout -- .`, `git clean -fd`, or `git restore` under any circumstance.
   - Always commit or stash working progress.

5. **Zero-Mock Policy:**
   - All tests must use real disk fixtures and real in-memory byte buffers (`fixtures/`, `tests/e2e_fixtures/real/`). No dummy `{ success: true }` echo stubs.

---

## 📋 Resuming Checklist for Next Session

### Step 1: Monitor Active Jules Sessions
- **Task 5.9.69A** ([Session 8874502492856873220](https://jules.google.com/session/8874502492856873220)):
  - ✅ Verified `npm test` passing (79/79) and merged into `main` (PR #220, commit `460ec1d`).
- **Task 5.9.70** ([Session 4388001643990007173](https://jules.google.com/session/4388001643990007173)):
  - ✅ Verified Wave 2 Penta-Interface tests across MCP, CLI, and Gateway, and merged into `main` (PR #221, commit `b7ff7c3`).
- Check status of **Task 5.9.55** ([Session 378425988505835118](https://jules.google.com/session/378425988505835118)).

### Step 2: Monitor Task 5.9.69B (Pipeline UI & Save Modal)
- Dispatched to Jules ([Session 11757279139714605661](https://jules.google.com/session/11757279139714605661)):
  - Implements `FlowSelector.svelte`, `SaveFlowModal.svelte`, Embed preset resolution, and Playwright E2E verification.
  - When PR is submitted by Jules, verify `npm --prefix apps/desktop test` passes and merge into `main`.

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
