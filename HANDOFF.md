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
├── apps/desktop/           # Svelte 5 + Tauri 2.0 desktop app
└── specs/                  # Architecture & verification specifications
```

---

## 📊 Current State: 100% Tri-Interface Backend Parity Complete! 🏆

### 1. Completed Tri-Interface Test Sweeps & Bug Fixes
All 44 operations across all three developer interfaces (CLI, MCP, and REST API) have achieved **100% test parity** with 0 failures, verified and documented at `reports/TRI_INTERFACE_E2E_100_VERIFIED.md`:
- **CLI (`paperpilot-cli`)**: 44 / 44 (100.0% PASS)
- **MCP (`paperpilot-mcp` stdio)**: 44 / 44 (100.0% PASS)
- **REST API (`paperpilot-gateway` :7823)**: 44 / 44 (100.0% PASS)

### 2. Recent Major Milestones & Merged PRs
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

## 🚧 Active Jules Sessions (In-Flight)

All tasks are partitioned with **strict file ownership** so that they run simultaneously with **zero merge conflicts**:

| Task | Scope (Owned Files) | Session URL | Target Branch | Focus |
|---|---|---|---|---|
| **4.E2E.R4** | `scripts/test_tri_interface_e2e.py`, `reports/TRI_INTERFACE_E2E_100_VERIFIED.md`, `reports/TRI_INTERFACE_E2E_AND_DOCS.md`, `wiki/21-Tri-Interface-Deep-Assertions.md` | [Session 13099315791828528723](https://jules.google.com/session/13099315791828528723) | `main` | ⏳ **In Flight** — Upgrade tri-interface E2E test harness to validate Expected vs Actual outcomes (pages, bytes) across CLI, MCP, REST, WASM, and Cloudflare Edge in unified comparison matrix tables |

---

## 🎯 Next Focus: Phase 4.1 & 4.3 AI Chat & Documentation RAG

While Jules runs the frontend suites in the background:
1. **On-Device AI Engine Selection**:
   - Free Tier: Embedded `TinyBERT-4L-312D` INT8 ONNX (~14MB / ~7MB `zstd`) via `include_bytes!` and `ort`.
   - Pro Tier: Embedded `SmolLM-135M-Instruct` (~75MB Q4 GGUF) + Bring-Your-Own-Key (BYOK) cloud LLMs (OpenAI, Gemini, Claude).
2. **Actionable RAG Workspace**:
   - `PdfChatPanel.svelte`: Side-drawer chat for document Q&A and action proposals.
   - `sqlite-vec` embedded documentation RAG for instant, factual command suggestions directly from `TRI_INTERFACE_E2E_100_VERIFIED.md`.
3. **Bi-Directional Canvas Jumping**:
   - Clicking citations in chat jumps directly to the page/bounding box and highlights the passage.

---

## 🔄 Post-Migration Re-Export: WASM & Edge Re-Sync (Task 5.5.1)
Once pure-Rust migrations (OCR `ocrs`, Page Rasterizer `hayro`, Images, Hash) land in `paperpilot-pdf`:
1. Re-export the new operations from `paperpilot-wasm` using `wasm-bindgen`.
2. Recompile: `make build-wasm`.
3. Synchronize `apps/web/node_modules/paperpilot-wasm/` and `apps/edge/node_modules/paperpilot-wasm/`.
4. Update `apps/web` (OperationsView + Playwright E2E) and `apps/edge` (worker routes + Vitest) to unlock 20+ in-browser & edge tools.

---

## 🛠️ Useful Commands

```bash
# Run workspace build & unit tests
cargo test --workspace

# Start the REST/MCP Gateway Server
./target/release/paperpilot-cli serve --port 7823

# Run the complete Tri-Interface E2E verification
python3 scripts/test_tri_interface_e2e.py

# Run Frontend Playwright E2E tests
cd apps/desktop && pnpm exec playwright test
```

---

*Last updated: 2026-10-05 11:11 EDT (PR #131 Thumbnail Organizer Merged, PR #133 Master AI Chat E2E Merged, Phase 4.8.1 WASM Dispatched to Jules)*

