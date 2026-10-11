# PaperPilot — Session Handoff Guide

> This file is your single source of truth for resuming work on PaperPilot.
> Point your AI assistant at this file to restore full context instantly.

---

## 🧠 The Team (Roles & Responsibilities)

### You (The Founder / Product Owner)
- Sets product direction, reviews PRs, approves architectural decisions.
- Defines new tasks and prioritizes the backlog in `PHASES.md`.

### Antigravity (AI Orchestrator / Senior Engineer)
- **Role:** Reads specs, designs architecture, generates conflict-free prompts, dispatches Jules, validates PRs, resolves merge conflicts, updates trackers, and maintains documentation.
- **How to activate:** Open this repo in Antigravity IDE and say *"continue from HANDOFF.md"*.

### Jules (Google Jules — Async AI Developer)
- **Role:** Implements code changes in cloud-based async sessions and opens GitHub PRs.
- **How tasks are submitted:** `python3 scripts/jules_submit.py --file prompts/tasks/<phase>/<task>.txt`
- **Dashboard:** https://jules.google.com/

---

## 🏗️ Architecture & Crate Boundaries

```
PaperPilot/
├── paperpilot-core/        # Rust: Core traits (PdfOperation, PdfDocument), error & result types
├── paperpilot-pdf/         # Rust: All 45+ PDF operations (merge, split, encrypt, OCR, render, etc.)
├── paperpilot-nlp/         # Rust: Offline Intent Classification & NLP Rule Engine
├── paperpilot-mcp/         # Rust: MCP JSON-RPC server exposing all operations as tools
├── paperpilot-gateway/     # Rust: Axum HTTP REST server & MCP SSE gateway (port 7823)
├── paperpilot-cli/         # Rust: Terminal CLI (paperpilot merge, split, serve, etc.)
├── paperpilot-wasm/        # Rust: In-memory WebAssembly engine for browser execution
├── apps/desktop/           # Svelte 5 + Tauri 2.0 desktop app
├── apps/edge/              # Cloudflare Workers serverless microservice
├── tools/penta-interface-e2e/ # Rust-Native Penta-Interface E2E & benchmark suite
└── specs/                  # Architecture & verification specifications
```

---

## ⚖️ The Penta-Interface Mandate & New Tool Law

> **MANDATORY FOR ALL FUTURE TOOLS**:
> Whenever a new operation or tool is created in PaperPilot, it **MUST NOT** be implemented as a CLI-only or single-interface feature. Every tool is a first-class citizen across all **5 deployment surfaces**:
> 1. **💻 CLI (`paperpilot-cli`)**: Kebab-case subcommand with `--json` support.
> 2. **🤖 MCP (`paperpilot-mcp`)**: JSON-RPC 2.0 tool declaration and handler.
> 3. **🌐 REST API (`paperpilot-gateway`)**: POST route `/api/v1/pdf/tools/{tool}` with OpenAPI spec.
> 4. **⚡ WASM (`paperpilot-wasm`)**: In-memory byte processing without filesystem IO.
> 5. **☁️ Cloudflare Edge (`apps/edge`)**: Memory-only stream route `/api/v1/{tool}`.
>
> ### 🧪 Rust-Native E2E & Performance Requirements:
> For any new tool:
> - **20 E2E Assertions (4 Tiers × 5 Surfaces)** must be added to `tools/penta-interface-e2e/src/cases/` (Simple, Medium, Complex, Negative).
> - **Edge Cases & Boundary Assertions** added to `edge_cases.rs`.
> - **Performance & Concurrency Profiling** added to `bench.rs` (measuring throughput, p50/p95/p99 latency, and <35MB RSS memory stability).
> - See detailed guide: [`wiki/25-Penta-Interface-Mandate-And-E2E-Protocol.md`](wiki/25-Penta-Interface-Mandate-And-E2E-Protocol.md).

---

## 📊 Current State: 100% Penta-Interface Backend & Edge Parity Complete! 🏆

### 1. Completed Tri-Interface & Penta-Interface Test Sweeps
All 44 operations across all three developer interfaces (CLI, MCP, and REST API) have achieved **100% test parity** with 0 failures, verified and documented at `reports/TRI_INTERFACE_E2E_100_VERIFIED.md`:
- **CLI (`paperpilot-cli`)**: 44 / 44 (100.0% PASS)
- **MCP (`paperpilot-mcp` stdio)**: 44 / 44 (100.0% PASS)
- **REST API (`paperpilot-gateway` :7823)**: 44 / 44 (100.0% PASS)

### 2. Active Tasks & In-Progress Work
- **Phase 4.1.1–4.1.3 (`paperpilot-ai` Crate & Universal Client)**:
  - **In Progress (Jules Session `7564763815279840049`)**: Standalone `paperpilot-ai` crate with `UniversalLlmClient`, `LlmNlpResolver` implementing `NlpResolver`, and `DynamicRouter` with fallback to `OfflineNlpResolver`.
- **Phase 4.1.5–4.1.6 (Local AI Settings UI & Model Downloader)**:
  - **In Progress (Jules Session `17474141107561700970`)**: Svelte 5 settings drawer with 4-way mode toggle (Offline NLP, Local Llamafile, Universal Endpoint, BYOK Cloud) and in-app model downloader with real-time progress.
- **Phase 5.9.13 (Spec 038 / High-Fidelity PDF-to-Word Layout Reconstruction)**:
  - **In Progress (Jules Session `2163210904964205811`)**: Upgrading `PdfToDocxOperation` in `paperpilot-pdf` with heading hierarchy inference, bold/italic font styling, RGB color extraction, alignment calculation, image extraction, and native OpenXML page breaks via `docx-rs`.
- **Phase 5.9.14 (Spec 039 / Advanced Multi-Column & Table Layout Analysis via `pdf-oxide`)**:
  - **In Progress (Jules Session `7944320664313422740`)**: Spec [`specs/039-pdf-oxide-layout-analysis/spec.md`](specs/039-pdf-oxide-layout-analysis/spec.md) with pure-Rust XY-Cut reading-order de-weaving and native OpenXML table reconstruction.
- **Phase 5.9.15 (Spec 040 / Universal `pdf_oxide` Core Migration & High-Performance Extraction Suite)**:
  - **Planned & Ready**: Spec `specs/040-pdf-oxide-universal-migration/spec.md` to migrate markdown/HTML conversion, text extraction, search, and RAG chunking to `pdf_oxide` (v0.3.78).
- **Phase 5.9.16 (Spec 041 / Pure-Rust URL-to-PDF Archival Engine via `browser_oxide`)**:
  - **Planned & Ready**: Spec `specs/041-pure-rust-url-to-pdf-browser-oxide/spec.md` for pure-Rust dynamic web archiving without Chrome/Chromium.
- **Phase 5.9.17 (Spec 042 / Agentic Code Quality & Dead-Scaffolding Pruning via `fossil-mcp`)**:
  - **Planned & Ready**: Spec `specs/042-agentic-code-quality-fossil-mcp/spec.md` to audit and prune dead functions, duplicate clones, and orphan scaffolding across workspace crates.
- **Phase 5.9.20 (Spec 045 / Desktop UX: Office + PDF Unified Intake & Run Button Feedback)**:
  - **In Progress (Jules Session `10047595857438929348`)**: Spec [`specs/045-desktop-ux-office-and-pdf-integration/spec.md`](specs/045-desktop-ux-office-and-pdf-integration/spec.md). Adds spinner/elapsed-timer loading state to all Run buttons, expands DropZone to accept PDF + Office files with dynamic icons and multi-format Tauri dialog, adds context-aware tool switcher (PDF Tools / Office Tools / AI Tools), and updates Documents heading to dynamic "Workspace" subtitle.

### 3. Summary of Recently Completed & Merged Milestones
- **Phase 5.9.19 (Spec 044 / `rusto-rs` OCR Engine)**: ✅ Merged (PR #178) — Sub-second PaddleOCR v4 + RTen inference with exact bounding boxes for selectable text overlays (`reports/RUSTO_RS_OCR_MIGRATION_REPORT.md`).
- **Phase 5.9.18 (Spec 043 / `office_oxide` Document Engine)**: ✅ Merged (PR #179) — Pure-Rust OOXML & legacy Office text extraction, markdown conversion, and modernization (`reports/OFFICE_OXIDE_INTEGRATION_REPORT.md`, `wiki/Office-Engine.md`).
- **Phase 5.9.12 (Spec 037 / Diagonal Watermarks)**: ✅ Merged (PR #176) — 45° diagonal angle, alpha transparency, MediaBox centering, and configurable parameters across all 5 surfaces.
- **Phase 5.9.11 (Spec 036 / JSON-to-PDF Synthesis)**: ✅ Merged (PR #169) — Pure-Rust `json_to_pdf` closing the bidirectional agent loop (`reports/JSON_TO_PDF_SYNTHESIS_REPORT.md`).
- **Phase 5.9.10 (Spec 035 / Local AI Legal Demo)**: ✅ Merged (PR #168) — 100% offline agent discovery workflow (`reports/LOCAL_AI_AGENT_LEGAL_DISCOVERY_DEMO_REPORT.md`).
- **Phase 5.9.9 (Spec 034 / Conformance Harness)**: ✅ Merged (PR #167) — `qpdf` and `veraPDF` ISO PDF/A-1b automated audits (`reports/PDF_CONFORMANCE_VERIFICATION_REPORT.md`).
- **Phase 5.9.8 (Spec 033 / Error Normalization)**: ✅ Merged (PR #163) — Human-friendly actionable error formatter and toasts (`reports/HUMAN_FRIENDLY_UI_ERRORS_REPORT.md`).
- **Phase 5.9.6 (Spec 031 / Concurrency Benchmark)**: ✅ Merged (PR #165) — 50-worker stress testing, p50/p95/p99 profiling, flat 34.5MB RSS (`reports/PENTA_INTERFACE_PERFORMANCE_BENCHMARK_REPORT.md`).
- **Phase 5.9.4 & 5.9.5 (Spec 031 / Penta-Interface E2E Suite)**: ✅ Merged (PRs #158-#161) — 100% green pass rate across all 44 tools and 5 interfaces (880/880 assertions passed, `reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT.md`).
- **Phase 5.9.2 (Spec 029 / Jules Submitter CLI)**: ✅ Merged (PR #164) — Pure-Rust standalone CLI in `tools/jules-submit/` (`reports/JULES_SUBMIT_RUST_MIGRATION_REPORT.md`).
- **Phase 5.9.1 (Spec 028 / Desktop Binary Optimization)**: ✅ Merged — Release binary reduced from 107MB to 35MB via pure-Rust `rten` SIMD inference (`reports/DESKTOP_BINARY_OPTIMIZATION_REPORT.md`).
- *(Earlier milestones and archived historical PRs are tracked in detail in [`PHASES.md`](PHASES.md))*

---

## 🎯 Next Focus: Phase 4.1 & 4.3 AI Agent Loop & Universal Engine

Now that backend penta-interface test parity (880 tests) and conformance checks (`qpdf`) are complete:
1. **Multi-Tier AI Engine Architecture**:
   - **Community (100% Offline)**: Embedded `TinyBERT` via `rten` for instant single-intent resolution.
   - **Pro Tier (Universal Local & Remote Runners)**:
     - **Mozilla `llamafile` Supervisor**: 1-click in-app download and process management of standalone `.llamafile` cross-platform binaries (`Qwen2.5-Coder-1.5B`, `Llama-3.2-3B`) running locally on port 8080 with zero setup.
     - **Universal OpenAI-Compatible Endpoint**: Connects to ANY URL (`localhost:11434` Ollama, LM Studio, vLLM/cluster, Groq, OpenRouter).
     - **BYOK (Bring Your Own Key)**: Direct API integration (OpenAI, Claude, Gemini) with keys stored securely in native OS Keychain (`tauri-plugin-stronghold`).
2. **Actionable Agent Canvas & Chat**:
   - `PdfChatPanel.svelte`: Side-drawer chat for document Q&A and autonomous multi-step legal/document operations.
   - Real-time tool-calling execution stream showing steps and sub-100ms timings.
3. **Bi-Directional Canvas Jumping**:
   - Clicking citations or tool targets in chat jumps directly to the page/bounding box and highlights the passage.

---

## 🔄 Post-Migration Re-Export: WASM & Edge Re-Sync (Task 5.5.1)
Once pure-Rust migrations (OCR `ocrs`, Page Rasterizer `hayro`, Images, Hash) land in `paperpilot-pdf`:
1. Re-export the new operations from `paperpilot-wasm` using `wasm-bindgen`.
2. Recompile: `make build-wasm`.
3. Synchronize `apps/web/node_modules/paperpilot-wasm/` and `apps/edge/node_modules/paperpilot-wasm/`.
4. Update `apps/web` (OperationsView + Playwright E2E) and `apps/edge` (worker routes + Vitest) to unlock 20+ in-browser & edge tools.

---

## 🛠️ Useful Commands

### 🤖 Multi-Agent Orchestration & Invocation

PaperPilot leverages a multi-tiered agent setup:
1. **Jules (Google Async Cloud Agent)**: Asynchronous cloud PR generator for isolated tasks, end-to-end integration tests, benchmarks, docs, and lint cleanups without consuming local tokens.
2. **OpenCode (Local Terminal LLM)**: Fast on-device terminal assistant (`/home/krushna/.opencode/bin/opencode`) for rapid local edits, interactive diff inspections, and non-blocking shell workflows.
3. **Claude Code (`claude`) / OpenAI Codex (`codex`)**: Local AI coding CLI assistants for interactive, context-heavy pair programming.
4. **Antigravity (IDE Agent)**: Complex architecture, cross-crate debugging, visual UI validation, and orchestrating the other agents.

#### 1. Invoking Jules (Batch Submitter & API)
```bash
# List pending prompts in prompts/tasks/
python3 scripts/jules_submit.py --list

# Submit a single prompt file (automatically creates a Jules session & moves prompt to prompts/tasks/done/)
python3 scripts/jules_submit.py --file prompts/tasks/verification/P4_E2E_R4_deep_assertions_report.txt

# Preview the submission payload and target repository without triggering API (Dry Run)
python3 scripts/jules_submit.py --file prompts/tasks/verification/P4_E2E_R4_deep_assertions_report.txt --dry-run

# Submit all tasks under a specific phase folder
python3 scripts/jules_submit.py --phase 5

# Submit targeting a specific Git base branch
python3 scripts/jules_submit.py --file prompts/tasks/phase5/P5_7_1_one_hour_soak_test.txt --branch main

# Direct curl invocation against Google Jules API (uses JULES_API_KEY from .env / .env.local):
curl -X POST "https://jules.googleapis.com/v1alpha/sessions" \
  -H "X-Goog-Api-Key: $(grep JULES_API_KEY .env.local | cut -d= -f2)" \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "MANDATORY RULES: 1. Keep all existing code. 2. Must pass cargo check. Task: Update scripts/test_tri_interface_e2e.py to record Expected vs Actual results in unified report.",
    "sourceContext": {
      "source": "sources/github/KrushnaVardhanReddy/PaperPilot",
      "githubRepoContext": { "startingBranch": "main" }
    }
  }'
```

#### 2. Invoking Local Agents (OpenCode, Claude, Codex)
```bash
# OpenCode (Terminal interactive UI or non-interactive pipe)
opencode                                     # Launch interactive TUI
opencode -p "Review diff between origin/main and HEAD"   # Run one-shot prompt

# Claude Code CLI
claude                                       # Launch interactive REPL
claude -p "Check Rust crate clippy warnings across workspace"

# Codex CLI
codex "Generate mock fixture for PDF form flattening test"
```

---

### 🦀 Rust Workspace Building & Testing
```bash
# Fast workspace type check across all crates & targets
cargo check --workspace --tests

# Run unit & integration tests with bounded concurrency (use -j 6 to prevent machine overload/restarts while allowing parallel tasks)
cargo test --workspace -- -j 6

# Test specific crates individually
cargo test -p paperpilot-pdf --lib -j 6
cargo test -p paperpilot-mcp --lib -j 6
cargo test -p paperpilot-gateway --lib -j 6
cargo test -p paperpilot-wasm --test wasm_tests -j 6

# Build optimized release binaries for CLI, MCP, and Gateway
cargo build --release -p paperpilot-cli -p paperpilot-mcp -p paperpilot-gateway
```

---

### 🌐 Server & Multi-Interface Testing
```bash
# Start the local REST & MCP Gateway Server (port 7823)
./target/release/paperpilot-cli serve --port 7823

# Direct MCP JSON-RPC invocation via stdio (no Python needed)
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "pdf_info", "arguments": {"input": "tests/e2e_fixtures/page_1.pdf"}}}' | ./target/release/paperpilot-mcp

# Run the Master Tri-Interface E2E suite (132 assertions across CLI, MCP, and REST API)
python3 scripts/test_tri_interface_e2e.py

# Run the 1-Hour continuous soak & endurance stress test harness (or custom duration in seconds)
python3 scripts/stress_test.py 3600
```

---

### 🖥️ Frontend & WASM Suites
```bash
# Compile client-side WASM engine with wasm-pack
make build-wasm

# Run Desktop app Playwright E2E suite
cd apps/desktop && pnpm exec playwright test

# Run Web App WASM Playwright E2E suite (in-browser 22 tools)
cd apps/web && pnpm exec playwright test

# Run Cloudflare Edge microservice Vitest suite
cd apps/edge && pnpm test
```

---

*Last updated: 2026-10-07 06:58 EDT (Spec 030 Web & Embed Batch Suite live on main; Dispatched Jules Sessions: 5.9.1 [1994491928465285687] and 5.9.3 [9023730240826721102])*

