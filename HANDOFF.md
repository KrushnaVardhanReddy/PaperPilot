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

### 2. Merged Bug Fix PRs
- **PR #114 (FIX 1)**: CLI flag normalization, `--angle` alias on rotate, multi-argument support on merge & images-to-pdf.
- **PR #115 (FIX 2)**: Gateway parameter mapping for `rotate`, `crop`, and `split`.
- **PR #116 (FIX 3B)**: MCP & Gateway E2E parameter stability and fallback render routing.
- **PR #117 (FIX 3A)**: Core engine operation fixes (decrypt empty pages, split output dir creation, burst).
- **PR #118 (FIX 4B)**: Gateway `/api/v1/pdf/convert` JSON route mapping (`docx`, `xlsx`, `pptx`, `html`, `markdown`).
- **PR #119 (FIX 4A)**: CLI annotate inline JSON parsing, burst directory auto-creation, and `sign` alias.
- **PR #120 (FIX 4C)**: Data-driven master E2E test harness covering all 44 tools with 100% green parity.

---

## 🚧 Active Jules Sessions (In-Flight Frontend Parity)

All 3 tasks have been partitioned with **strict file ownership** so that they run simultaneously with **zero merge conflicts**:

| Task | Scope (Owned Files) | Session URL | Target Branch | Focus |
|---|---|---|---|---|
| **R3.FE.1** | `OperationsPanel.svelte` only | [Session 15233664947548284011](https://jules.google.com/session/15233664947548284011) | `main` | Register all 44 tools in `allTools` with cards & invocation mappings |
| **R3.FE.2** | `tests/e2e_44_operations_parity.spec.ts` only | [Session 10690891751709790998](https://jules.google.com/session/10690891751709790998) | `main` | Data-driven Playwright test verifying search, form inputs & IPC invoke for all 44 tools |
| **R3.FE.3** | `tests/e2e_canvas_viewer_features.spec.ts` only | [Session 13554866274792768778](https://jules.google.com/session/13554866274792768778) | `main` | Playwright test for Sticky Notes, Drawing/Markup, Diff Slider & Zoom Toolbar |

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

*Last updated: 2026-10-04 22:00 EDT (100% Backend Parity Complete; Frontend Parity In-Flight)*

