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

### 2. Recent Major Milestones & Merged Changes
- **5.9.4 & 5.9.5 (Spec 031 / Penta-Interface E2E Suite)**:
  - **Phase 5.9.4 Master Parity Complete**: 100% green pass rate achieved across all 44 tools, all 5 interfaces (CLI, MCP, REST, WASM, Edge), and all 4 complexity tiers (**880 / 880 tests passed**), fully documented in `reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT.md` and sub-reports `5_9_4A.md` through `5_9_4D.md`.
- **5.9.2 (Spec 029 / Pure-Rust Jules Submitter CLI)**:
  - **Merged (PR #164, commit `1e673bc`, Session 8537116514657265561)**: Replaced `scripts/jules_submit.py` with 100% pure-Rust standalone CLI in `tools/jules-submit/` (`reqwest`, `clap`, `dotenvy`, `serde_json`) and shell forwarder `scripts/jules_submit.sh` for zero-Python developer workflow (`reports/JULES_SUBMIT_RUST_MIGRATION_REPORT.md`, `wiki/23-Pure-Rust-Jules-Submitter.md`).

- **5.9.6 (Spec 031 / Penta-Interface High-Throughput & Concurrency Benchmark)**:
  - **Merged (PR #165, commit `a8de2ae`, Session 10557618272357635464)**: Concurrency stress testing (10, 25, 50 workers) with 100% request delivery across REST and MCP, p50/p95/p99 latency profiling across top 10 tools, memory leak checks (flat 34.5MB RSS, <35MB ceiling), Makefile integration (`make bench-penta-e2e`), documented in `reports/PENTA_INTERFACE_PERFORMANCE_BENCHMARK_REPORT.md`.

- **Phase 4.1.1–4.1.3 (paperpilot-ai Crate & Universal Client)**:
  - **In Progress (Jules Session `7564763815279840049`)**: Dispatched via `./scripts/jules_submit.sh --file prompts/tasks/phase4/P4_1_1_paperpilot_ai_crate.txt`. Scaffolding standalone `paperpilot-ai` crate with `UniversalLlmClient`, `LlmNlpResolver` implementing `NlpResolver`, and `DynamicRouter` with fallback to `OfflineNlpResolver`.
- **Phase 4.1.5–4.1.6 (Local AI Settings UI & Model Downloader)**:
  - **In Progress (Jules Session `17474141107561700970`)**: Dispatched via `./scripts/jules_submit.sh --file prompts/tasks/phase4/P4_1_5_local_ai_settings_ui.txt`. Building Svelte 5 settings drawer with 4-way mode toggle (Offline NLP, Local Llamafile, Universal Endpoint, BYOK Cloud) and in-app model downloader with real-time progress.
- **5.9.8 (Spec 033 / Human-Friendly UI Error Normalization)**:
  - **Merged (PR #163, commit `3886288`, Session 11507040602700534548)**: Centralized `errorFormatter.ts`, category taxonomy (passwords, corruption, page ranges, locks, missing files), enhanced toasts with headlines, actionable hints, and expandable technical details with clipboard copy for power users. 100% pass across 11 test suites (37/37 tests) in `reports/HUMAN_FRIENDLY_UI_ERRORS_REPORT.md`.
- **5.9.9 (Spec 034 / Automated PDF Structural & Conformance Verification)**:
  - **Merged (PR #167, commit `0d61c0f`, Session 12877595132953884388)**: Automated structural and conformance verification harness with `qpdf --check` and `veraPDF` ISO PDF/A-1b audits across all core outputs (`repair`, `linearize`, `compress`, `merge`, `split`, `pdf-a`), integrated into `scripts/verify_pdf_conformance.sh`, `Makefile` (`make test-conformance`), and `.github/workflows/rust.yml` CI runner (`reports/PDF_CONFORMANCE_VERIFICATION_REPORT.md`).
- **5.9.10 (Spec 035 / Local AI Agent Legal Due Diligence & Discovery Demo Harness)**:
  - **Merged (PR #168, commit `8150fff`, Session 3245014174250604924)**: Complete 100% offline local AI agent demonstration chaining `pdf_rotate`, `pdf_redact`, `pdf_bates`, `pdf_merge`, `pdf_watermark`, `pdf_linearize`, and `pdf_hash` via `tools/demo-legal-fixtures`, `scripts/run_ai_legal_demo.sh`, and `make demo-legal-discovery` (`reports/LOCAL_AI_AGENT_LEGAL_DISCOVERY_DEMO_REPORT.md`, `demo/legal_discovery/README.md`).
- **5.9.11 (Spec 036 / Bidirectional JSON-to-PDF Synthesis with Multimodal Images)**:
  - **Merged (PR #169, commit `3688ba3`, Session 5118308487879675704)**: Implemented pure-Rust `json_to_pdf` synthesis via `lopdf` + `image` (Approach 1: Native Engine Layout), closing the bidirectional PDF ⇄ JSON synthesis loop for autonomous AI agents across all 5 surfaces (CLI, MCP, REST, WASM, Edge) with 20 E2E assertions, round-trip fidelity tests, and benchmark profiling (`reports/JSON_TO_PDF_SYNTHESIS_REPORT.md`, `wiki/036-Json-To-Pdf-Synthesis.md`).
- **PR #166 (5.9.7 / Spec 032)**: Multimodal PDF-to-JSON with Image Extraction & Base64 Inlining across 5 interfaces (`2604a8e`). Expanded `pdf_to_json` with `--image-mode <none|files|base64>`, full JSON schema with page metadata, text blocks, and inline base64 images, 20 Penta-Interface E2E assertions, latency benchmarks, and documentation (`reports/PDF_TO_JSON_MULTIMODAL_REPORT.md`, `wiki/08-Multimodal-PDF-to-JSON.md`).
- **5.9.1 (Spec 028)**: Desktop Release Binary Size Optimization & Pure-Rust `rten` SIMD Inference Migration (`45a1566`). Standalone Desktop release executable dropped from **107 MB down to 35 MB** (~67% reduction) via root workspace release profile (`strip = true`, `lto = "fat"`, `panic = "abort"`). Completely eliminated C++ ONNX Runtime (`ort`) in favor of pure-Rust `rten` with embedded `tinybert.rten.zst` (~9.7 MB). 100% verified across `paperpilot-nlp` test suite, Tri-Interface suite (132 assertions), and Desktop Playwright AI chat pipeline (`reports/DESKTOP_BINARY_OPTIMIZATION_REPORT.md`, `wiki/22-Desktop-Binary-Optimization.md`).
- **Spec 030 / Web & Embed Batch Suite (`dbe3ba1`, `ef77956`, `086231b`, `b60f41e`)**:
  - **Embedded Widget (`apps/embed/`)**: Real `pdf-lib` multi-page merging, multi-file batch execution across `extract`, `delete`, `rotate`, `watermark`, `split`, `reorder`, and `compress`. Added interactive tool configuration panel for custom page numbers, rotation angles, and watermark text stamps.
  - **Live Embed Sandbox (`/embed-test`)**: Showcasing standard and whitelisted/pro embeds live in browser with direct links from Web Portal footer, Embed Generator, and documentation.
  - **PaperPilot WASM Engine (`apps/web/`)**: Migrated `OperationsView.svelte` to support multi-file batch queues across all WASM operations with staggered downloads and individual file removal controls.
- **5.6.2 (Spec 026)**: Universal High-Ratio PDF Compression Engine (`image` JPEG recompression and downsampling, `--quality` control across CLI, MCP, REST, UI, and WASM, `reports/COMPRESSION_RATIO_VERIFICATION_REPORT.md`, `wiki/20-High-Ratio-PDF-Compression.md`).
- **PR #151 (5.7.1)**: 1-Hour Sustained Soak & Endurance Benchmark (353,892 ops, 98.3 ops/s, 100% success rate, flat 25MB RSS, `wiki/19-One-Hour-Endurance-Benchmark.md`, `reports/ONE_HOUR_SUSTAINED_BENCHMARK_REPORT.md`).
- **PR #150 (5.6.1)**: Pure-Rust High-Speed Office Conversions (`fulgur` engine replacing `headless_chrome`, sub-60ms conversion latency, `wiki/18-Pure-Rust-Office-Conversions.md`, `reports/PURE_RUST_OFFICE_CONVERSIONS_REPORT.md`).
- **PR #148 (4.9.13)**: Certificate & Template Studio (HTML/SVG templates, bulk CSV generator, `apps/web/`).
- **PR #147 (4.9.12)**: `docs.usepaperpilot.com` Official Starlight Docs Hub (4-pillar documentation, `apps/docs/`).
- **PR #146 (4.9.11)**: `usepaperpilot.com` Official Marketing & Embed Playground Portal.
- **PR #145 (4.9 Core)**: `embed.js` CDN loader and Svelte 5 embed widget with Shadow DOM style isolation (`apps/embed/`).
- **PR #143 (5.1.1)**: Pure-Rust Image & Hash WASM Expansion to 15 Tools (`paperpilot-wasm`, `apps/web/`).
- **PR #142 (5.3.1)**: Pure-Rust PDF Rasterization & Rendering (`hayro` + `tiny-skia` replacing PDFium in `paperpilot-pdf`, `reports/RENDERING_PURE_RUST_REPORT.md`).
- **PR #141 (5.2.1)**: Pure-Rust OCR Migration (`ocrs` via `rten` replacing Tesseract in `paperpilot-pdf`, `reports/OCR_PURE_RUST_REPORT.md`).
- **PR #140 (4.8.3)**: Cloudflare Workers Edge Microservice (`apps/edge/` standalone worker, 12 core operations, `reports/EDGE_MICROSERVICE_REPORT.md`).
- **PR #139 (4.8.5)**: WASM 12 Tools Playwright E2E Suite & In-Memory Encryption (`reports/WASM_12_TOOLS_E2E_SCORECARD.md`).
- **PR #138 (4.8.4)**: Expand WASM Engine and Web Suite to 12 Pure-Rust Tools (`reports/WASM_12_TOOLS_EXPANSION_REPORT.md`).
- *(Earlier Phase 1–4 foundational PRs #114–#137 archived in `PHASES.md`)*

---

All 4 subtasks of the **Rust-Native Penta-Interface Real Semantic Assertions Suite (Spec 031 - 880 Tests)** have landed and merged into `main`:
- **5.9.4A (Page Operations - 180 Tests)**: ✅ Merged (PR #161, `472ae69`)
- **5.9.4B (Security, Stamping & Forms - 280 Tests)**: ✅ Merged (PR #159, `95eae07`)
- **5.9.4C (Analysis, Extraction & Optimization - 260 Tests)**: ✅ Merged (PR #160, `ba79af0`)
- **5.9.4D (Document Conversions - 160 Tests)**: ✅ Merged (PR #158, `f0607a3`)

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

# Run unit & integration tests with low concurrency (prevents memory spikes / IDE OOM kills)
cargo test --workspace -- -j 2

# Test specific crates individually
cargo test -p paperpilot-pdf --lib -j 2
cargo test -p paperpilot-mcp --lib -j 2
cargo test -p paperpilot-gateway --lib -j 2
cargo test -p paperpilot-wasm --test wasm_tests

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

