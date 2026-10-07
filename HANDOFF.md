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

### 2. Merged Frontend & Bug Fix PRs
- **PR #138 (4.8.4)**: Expand WASM Engine and Web Suite to 12 Pure-Rust Tools (`paperpilot-wasm` memory operations for `delete_pages`, `extract_pages`, `reorder_pages`, `crop`, `flatten`, `set_metadata`, Web Worker bridges, 12 interactive tool cards in `apps/web/`, `reports/WASM_12_TOOLS_EXPANSION_REPORT.md`, `wiki/06-Wasm-12-Tools-Suite.md`).
- **PR #137 (4.8.2)**: Zero-Install Client-Side Web App Demo (`apps/web/` Vite + Svelte 5 + Web Worker bridge, 6 core ops in-browser, privacy badge, desktop download funnel CTA, `reports/ZERO_INSTALL_WEB_APP_REPORT.md`, `wiki/05-Zero-Install-Web-App.md`).
- **PR #136 (4.1.8)**: Embedded Documentation RAG (`sqlite-vec` / TF-IDF) & Chat Integration (`paperpilot-nlp/src/rag.rs`, `query_documentation_rag` Tauri IPC, `.docs-answer-card` UI in `PdfChatPanel.svelte`, `wiki/04-Documentation-RAG.md`, and `reports/DOCUMENTATION_RAG_REPORT.md`).
- **PR #135 (4.8.1)**: `paperpilot-wasm` Complete Client-Side Engine & Web Worker Bridge (`paperpilot-wasm` crate with `wasm-bindgen`, in-memory buffer adapters for 6 core ops, `pdfWorker.ts`, `wiki/03-Wasm-Client-Engine.md`, and `reports/WASM_CLIENT_ENGINE_REPORT.md`).
- **PR #134 (4.3.11)**: Universal Multi-Document Omnibar (`GlobalCommandBar.svelte`) docked at bottom of Documents view (`Ctrl+K` / `Cmd+K`) with `@filename` autocomplete popover, suggestion chips, offline NLP plan resolution, and inline blueprint preview (`reports/UI_GLOBAL_COMMAND_BAR_REPORT.md`).
- **PR #133 (4.3.12)**: Master E2E AI Chat Pipeline Test Suite (`e2e_ai_chat_real_pipeline.spec.ts`) across all 44 PDF operations with 100% pass rate (`reports/UI_CHAT_44_OPERATIONS_E2E_SCORECARD.md` 44/44 PASS, `reports/AI_CHAT_REAL_PIPELINE_E2E_REPORT.md`).
- **PR #131 (4.F.19)**: Interactive Thumbnail Page Organizer (`PdfThumbnails.svelte` & `+page.svelte`) with `⋮⋮` drag grip handles, visual top/bottom blue drop insertion lines, hover actions, and custom right-click context menu (Rotate 90°/180°/270°, Duplicate Page, Extract Page, Delete Page) with live canvas reload (`reports/UI_THUMBNAIL_ORGANIZER_REPORT.md`).
- **PR #130 (4.3.14)**: Editable Action Blueprint Cards (`EditableActionCard.svelte`) with target pages selector (`all`, `current`, custom `1, 4`), contextual hints, format badges, and complete 44-tool MCP execution mapping (`reports/UI_EDITABLE_CHAT_ACTION_CARDS_REPORT.md`).
- **PR #129 (4.3.13)**: Searchable Command Help & Cheat Sheet Drawer (`ChatCheatSheet.svelte`) with 44-tool categorized directory, hotkey `?`, one-click prompt insertion, plus dynamic width scaling (240px–800px) and auto-comfort chat width in `ViewerRightPanel.svelte` (`reports/UI_CHAT_CHEATSHEET_REPORT.md`).
- **PR #128 (4.3.10)**: Context-Aware NLP Resolution Bridge (`ResolverContext`) with implicit active document binding (`reports/CONTEXT_AWARE_NLP_REPORT.md`).
- **PR #127 (4.0.5)**: Layer 2 Embedded ONNX Intent Classifier (`TinyBERT-4L-312D` INT8 ONNX compressed with `zstd`, `include_bytes!` in-memory `ort` CPU inference <2ms, `reports/NLP_ONNX_VERIFIED.md`).
- **PR #124 (4.3.7)**: Multi-Turn Chat Panel component (`PdfChatPanel.svelte`), `ViewerRightPanel.svelte` integration, and `resolve_natural_language` Tauri IPC offline NLP bridge (`reports/UI_CHAT_PANEL_REPORT.md`).
- **PR #123 (R3.FE.2)**: Data-driven Playwright E2E Master Suite covering all 44 PDF operations with 100% green UI parity (`apps/desktop/tests/e2e_44_operations_parity.spec.ts` & `reports/UI_44_OPERATIONS_E2E_SCORECARD.md` 44/44 PASS).
- **PR #122 (R3.FE.3)**: Visual Canvas Annotations, Sticky Notes, Markup, and Diff Slider Playwright E2E Test Suite (`apps/desktop/tests/e2e_canvas_viewer_features.spec.ts` & `reports/UI_CANVAS_ANNOTATIONS_DIFF_REPORT.md`).
- **PR #121 (R3.FE.1)**: Registered all 44 PDF operations with parameter cards and invocation mappings in `OperationsPanel.svelte` (`reports/UI_44_TOOLS_PANEL_REPORT.md`).
- **PR #114 (FIX 1)**: CLI flag normalization, `--angle` alias on rotate, multi-argument support on merge & images-to-pdf.
- **PR #115 (FIX 2)**: Gateway parameter mapping for `rotate`, `crop`, and `split`.
- **PR #116 (FIX 3B)**: MCP & Gateway E2E parameter stability and fallback render routing.
- **PR #117 (FIX 3A)**: Core engine operation fixes (decrypt empty pages, split output dir creation, burst).
- **PR #118 (FIX 4B)**: Gateway `/api/v1/pdf/convert` JSON route mapping (`docx`, `xlsx`, `pptx`, `html`, `markdown`).
- **PR #119 (FIX 4A)**: CLI annotate inline JSON parsing, burst directory auto-creation, and `sign` alias.
- **PR #120 (FIX 4C)**: Data-driven master E2E test harness covering all 44 tools with 100% green parity.

---

- **5.6.2 (Spec 026)**: Universal High-Ratio PDF Compression Engine (JPEG recompression and downsampling via `image` crate, quality control across CLI, MCP, API, WASM, and UI slider, commit `16f8ede`).
- **PR #151 (5.7.1)**: 1-Hour Sustained Soak & Endurance Benchmark across 44 pure-Rust tools (353,892 operations, 98.3 ops/s sustained throughput, 100.00% success rate, 0 panics/crashes, 25 MB flat RSS memory curve, `wiki/19-One-Hour-Endurance-Benchmark.md`, `reports/ONE_HOUR_SUSTAINED_BENCHMARK_REPORT.md`).
- **PR #150 (5.6.1)**: Pure-Rust High-Speed Office & Document Conversions (`fulgur` engine replacing `headless_chrome` in `paperpilot-pdf/src/operations/conversion.rs`, sub-60ms conversion latency for HTML, Markdown, and Excel, `wiki/18-Pure-Rust-Office-Conversions.md`, `reports/PURE_RUST_OFFICE_CONVERSIONS_REPORT.md` 132/132 assertions PASS).
- **PR #148 (4.9.13)**: Certificate & Template Studio (HTML/SVG template engine, dynamic customization, bulk CSV certificate generator, `apps/web/`).
- **PR #147 (4.9.12)**: `docs.usepaperpilot.com` Official Starlight Docs Hub (4-pillar guides, tri-interface reference, `apps/docs/`).
- **PR #146 (4.9.11)**: `usepaperpilot.com` Official Marketing & Embed Playground Portal.
- **PR #145 (4.9 Core)**: `embed.js` CDN loader and Svelte 5 embed widget with Shadow DOM style isolation (`apps/embed/`).
- **PR #143 (5.1.1)**: Pure-Rust Image & Hash WASM Expansion to 15 Tools (`paperpilot-wasm`, `apps/web/`).
- **PR #142 (5.3.1)**: Pure-Rust PDF Rasterization & Rendering (`hayro` + `tiny-skia` replacing PDFium in `paperpilot-pdf/src/operations/render.rs`, `wiki/12-Pure-Rust-Rendering.md`, and `reports/RENDERING_PURE_RUST_REPORT.md` 100% PASS).
- **PR #141 (5.2.1)**: Pure-Rust OCR Migration (`ocrs` via `rten` replacing Tesseract C++ in `paperpilot-pdf/src/operations/ocr.rs`, image stream extraction, DBNet text detection & CTC recognition with graceful fallback, `wiki/09-Pure-Rust-OCR.md`, and `reports/OCR_PURE_RUST_REPORT.md` 44/44 PASS).
- **PR #140 (4.8.3)**: Cloudflare Workers Edge Microservice (`apps/edge/` standalone worker, REST handlers for all 12 core operations, Vitest tests 14/14 PASS, `reports/EDGE_MICROSERVICE_REPORT.md`, `wiki/10-Cloudflare-Edge-Service.md`).
- **PR #139 (4.8.5)**: WASM 12 Tools Playwright E2E Suite & In-Memory Encryption (`apps/web/tests/e2e_wasm_12_tools.spec.ts`, `apps/web/playwright.config.ts`, pure-Rust `encrypt` in `paperpilot-wasm/src/operations.rs`, `wiki/08-Wasm-E2E-Testing.md`, and `reports/WASM_12_TOOLS_E2E_SCORECARD.md` 12/12 PASS).
- **PR #138 (4.8.4)**: Expand WASM Engine and Web Suite to 12 Pure-Rust Tools (`paperpilot-wasm` memory operations for `delete_pages`, `extract_pages`, `reorder_pages`, `crop`, `flatten`, `set_metadata`, Web Worker bridges, 12 interactive tool cards in `apps/web/`, `reports/WASM_12_TOOLS_EXPANSION_REPORT.md`, `wiki/06-Wasm-12-Tools-Suite.md`).

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

