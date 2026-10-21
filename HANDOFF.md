# PaperPilot — Session Handoff Guide

> **Single Source of Truth** for resuming work on PaperPilot.  
> Start any new agent or human session by reading this document.  
> **Date of Last Update:** October 20, 2026 (Afternoon EDT)  
> **Active Target:** Wave 2 (`v0.6.0`) Media & Page Utilities + Pipeline Flow Management

---

## 🧭 Executive Context & Active In-Flight Sessions

### 1. Active Jules Sessions in Flight
*No active sessions in flight. All Wave 2, Pipeline Flow, and Deep Security Sanitizer milestones complete.*

---

## 🏆 Recently Completed Milestones

1. **Task 5.9.59 (Privacy & Deep Security Sanitizer `pdf_sanitize` `[Spec 085]`)**:
   - Merged (PR #228, commit `00c2f67`).
   - Implemented Single Core Runner `sanitize_pdf` in `paperpilot-pdf/src/operations/sanitize.rs` using pure in-memory `lopdf` manipulation to strip `/Info`, XML `/Metadata`, `/EmbeddedFiles`, `/AF`, active scripts (`/JavaScript`, `/OpenAction`, `/AA`), `/Thumb`, and private editing data (`/PieceInfo`).
   - Built 5 thin wrappers across Native CLI (`paperpilot sanitize`), MCP (`pdf_sanitize`), REST Gateway (`/api/v1/pdf/sanitize`), WASM engine (`WasmPdfEngine.sanitize`), and Cloudflare Edge worker (`/api/v1/sanitize`).
   - 10/10 (100.0%) unmocked real Penta-Interface semantic tests passing in `reports/PENTA_INTERFACE_E2E_SANITIZE_REPORT.md`.
2. **Task 5.9.69C (Penta-Interface Declarative Workflow Engine & Recipe Execution `[Spec 081]`)**:
   - Implemented Single Core Runner `run_workflow_bytes` in `paperpilot-pdf/src/operations/workflow.rs`.
   - Built 5 thin wrappers across Native CLI (`paperpilot workflow`), MCP (`list_workflows`, `document_workflow`), REST Gateway (`/api/v1/workflow/*`), WASM engine (`workflow`), and Cloudflare Edge worker.
   - 20/20 (100.0%) unmocked Penta-Interface E2E tests passing in `reports/PENTA_INTERFACE_E2E_WORKFLOW_REPORT.md`.
2. **Task 5.9.71 (Universal "Export as Code / API / MCP Prompt" Workbench `[Spec 082]`)**:
   - Centralized snippet generation engine in `apps/desktop/src/lib/utils/snippetGenerator.ts`.
   - Integrated live `</> Code / API` button and dynamic modal in **both** the Visual Pipeline Builder (`FlowSelector.svelte`) and Workspace Operations Panel (`OperationsPanel.svelte`, `EditableActionCard.svelte`).
3. **Task 5.9.72 (Embed Pipeline 1-to-1 Desktop Parity `[Spec 083]`)**:
   - Replaced hidden file input with prominent browse button & file chip summary.
   - Added interactive step stepper with dynamic tool picker (`+ Add Step`), deletion, presets dropdown, and flow persistence.
4. **Task 5.9.55 (Decoupled Markdown-to-DOCX Compiler with DrawingML Images `[Spec 077]`)**:
   - High-fidelity conversion decoding inline base64 images into true DrawingML `.docx` image parts with 2-column card layout synthesis.
5. **Spec 084 Authored (On-Device Document SLM Fine-Tuning)**:
   - Authored specification for `PaperPilot-Qwen-0.6B` (~350 MB GGUF, sub-50ms offline inference) and training roadmap.

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
