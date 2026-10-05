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

## 🚧 Active Jules Sessions (Phase 4 In-Flight)

All tasks are partitioned with **strict file ownership** so that they run simultaneously with **zero merge conflicts**:

| Task | Scope (Owned Files) | Session URL | Target Branch | Focus |
|---|---|---|---|---|
| **4.3.12** | `tests/e2e_ai_chat_real_pipeline.spec.ts`, `reports/AI_CHAT_REAL_PIPELINE_E2E_REPORT.md`, `reports/UI_CHAT_44_OPERATIONS_E2E_SCORECARD.md` | [Session 2374703582441728568](https://jules.google.com/session/2374703582441728568) | `main` | Real-Pipeline Unmocked E2E AI Chat Test Suite across all 44 operations |
| **4.F.19** | `apps/desktop/src/lib/components/PdfThumbnails.svelte`, `apps/desktop/src/routes/+page.svelte`, `reports/UI_THUMBNAIL_ORGANIZER_REPORT.md` | [Session 5753728026560742170](https://jules.google.com/session/5753728026560742170) | `main` | Interactive Thumbnail Page Organizer, Drag Drop Insertion, and Right-Click Context Menu |

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

*Last updated: 2026-10-05 05:03 EDT (100% Backend Parity Complete; PR #121 44 Tools OperationsPanel Merged)*

