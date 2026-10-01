# PaperPilot — Session Handoff Guide

> This file is your single source of truth for resuming work on PaperPilot.
> Point your AI assistant at this file to restore full context instantly.

---

## 🧠 The Team (Roles & Responsibilities)

### You (The Founder / Product Owner)
- Sets product direction, reviews PRs, approves architectural decisions.
- Defines new tasks and prioritizes the backlog in `PHASES.md`.

### Antigravity (AI Orchestrator / Senior Engineer)
- **Role:** Reads the spec, breaks work into prompts, dispatches Jules, validates PRs, resolves merge conflicts, updates the tracker, and maintains all non-code project docs.
- **How to activate:** Open this repo in Antigravity IDE and say *"continue from HANDOFF.md"*.
- **Key Capabilities:**
  - Runs `make test-all` locally to validate before merging
  - Resolves Git merge conflicts directly in the CLI
  - Updates `PHASES.md` as tasks complete
  - Dispatches new tasks via `python3 scripts/jules_submit.py --file <prompt>`

### Jules (Google Jules — Async AI Developer)
- **Role:** Does the actual coding work in cloud-based async sessions.
- **How tasks are submitted:** `python3 scripts/jules_submit.py --file prompts/tasks/<phase>/<task>.txt`
- **Dashboard:** https://jules.google.com/
- **Rules Jules must follow:** See `RULES_FOR_JULES.md` (if present) — always include rule to not use TailwindCSS, follow the `PdfOperation` trait pattern, and write tests.
- **Branches:** Jules opens PRs from branches named `jules-<task>-<sessionId>` or `feat-<task>-<sessionId>`.

---

## 📁 Key Files & What They Do

| File | Purpose |
|---|---|
| [`v1_launch_plan.md`](./v1_launch_plan.md) | High-level summary of tasks needed for the immediate v1 launch (Free Tier, NLP). |
| [`PHASES.md`](./PHASES.md) | The master roadmap tracker. Every task, its status (✅/🚧/🔲), and which Jules session merged it. **Always check this first.** |
| [`COMPETITORS.md`](./COMPETITORS.md) | Feature comparison matrix vs. Adobe, iLovePDF, PDF24, Stirling-PDF, PDFgear. Identifies our moat and gaps. |
| [`pricing_strategy.md`](./pricing_strategy.md) | Tiered pricing model: Free → Pro ($7/mo) → Teams ($15/user/mo) → Enterprise ($25/user/mo). Includes the Contributor Revenue Sharing algorithm. |
| [`Makefile`](./Makefile) | All dev commands. `make dev` starts the app, `make test-all` runs every test tier. |
| `scripts/jules_submit.py` | The submission script for Jules. Archives the prompt after submission. |
| `prompts/tasks/` | Pending task prompts for Jules (not yet submitted). |
| `prompts/tasks/done/` | Archived prompts for tasks already submitted to Jules. |
| `prompts/tasks/testing/` | Prompts dedicated to End-to-End Validation testing sweeps. |

---

## 🏗️ Architecture Overview

```
PaperPilot/
├── paperpilot-core/        # Rust: Core traits (PdfOperation), error types, shared types
├── paperpilot-pdf/         # Rust: All 45+ PDF operations (merge, split, encrypt, OCR, etc.)
├── paperpilot-nlp/         # Rust: Offline Intent Classification & NLP Rule Engine
├── paperpilot-mcp/         # Rust: MCP JSON-RPC server exposing all operations as tools
├── paperpilot-cli/         # Rust: Terminal CLI (paperpilot merge, split, create, etc.)
├── tools/train-nlp/        # Python: Dataset generation and ONNX model training pipeline
├── apps/desktop/           # Svelte 5 + Tauri 2.0 desktop app
│   ├── src/                # Svelte frontend (runes, components, state stores)
│   ├── src-tauri/          # Rust Tauri backend (invoke_mcp_tool IPC command)
│   └── tests/              # Playwright E2E tests
└── Cargo.toml              # Workspace root (all Rust crates)
```

**Key Patterns:**
- All PDF operations implement the `PdfOperation` trait in `paperpilot-core/src/traits.rs`.
- MCP Server uses **manual `ServerHandler` trait** — do NOT use `rmcp` macros.
- Frontend uses **Svelte 5 runes** (`$state`, `$derived`) — not Svelte 4 stores.
- Frontend calls Tauri backend via `invoke('invoke_mcp_tool', { tool, args })`.

---

## ✅ What's Done (Recent Achievements)

| Phase | Status | Notes |
|---|---|---|
| Phase E.2 (MCP E2E) | ✅ Complete | Automated JSON-RPC stdio testing of `paperpilot-mcp` |
| Phase E.3 (UI E2E) | ✅ Complete | Playwright UI tests for the Svelte/Tauri app |
| Phase E.4 (Benchmark) | ✅ Complete | System performance & latency testing |
| Phase 4.F.4 (Custom HTML Titlebar) | ✅ Complete | Replaced native OS decorations with Svelte titlebar |
| Phase 4.F.6 (UX Improvements & Permissions) | ✅ Complete | View button, collapsible sidebars & right panel, status bar, Tauri permissions & native file open (PRs #73, #74, #75, #76) |

---

## 🚧 Active Jules Tasks (In-Progress)

| Task | Prompt File | What it does |
|---|---|---|
| Phase 4.F.7-A | `prompts/tasks/phase_4/P4_F_7A_editable_page_zoom_toolbar.txt` | Editable page jump input + zoom presets dropdown in `PdfToolbar.svelte` |
| Phase 4.F.7-B | `prompts/tasks/phase_4/P4_F_7B_multidoc_tabs_operations_dock.txt` | Multi-Document tabs `[Tab1][Tab2][+]` + right-docked Operations panel |

---

## 🔲 What's Next (Backlog Priority Order)

1. **Phase 4.F.8 (Desktop Instant Open & Power UX):**
   - Instant document open flow: dropping or selecting a PDF immediately opens it in the viewer
   - Raycast-style Command Palette (`Ctrl+K` / `⌘K`) overlay with fuzzy search across operations, files, and tabs
   - Floating pill annotation toolbar over PDF canvas with backdrop blur
   - Resizable right panel with drag handle (240px–600px) and persistence
2. **Phase 4.F.9 (Canvas Search & Direct Thumbnail Page Management):**
   - In-canvas text search bar (`Ctrl+F` / `⌘F`) with match navigation and counter (`P4_F_9A_canvas_search_ctrl_f.txt`)
   - Direct drag-and-drop page reordering in thumbnail sidebar (`P4_F_9B_thumbnail_page_management.txt`)
   - Hover quick action buttons on thumbnails (Rotate 🔄, Delete 🗑️)
3. **Phase 4.E2E Round 2 (Real E2E Testing without Mocking):** Implement native driver tests and system re-validation post-Viewer UX improvements.
4. **Phase 4.E2E R2.E4:** Execute `scripts/benchmark.py` for system performance re-benchmarking.
5. **Phase 5 (NLP & OCR):** Begin integration of the offline NLP classifier and OCR components.

---

## 🛠️ Useful Commands

```bash
# Start the desktop app (UI + Rust backend)
make dev

# Run ALL tests (Rust + Svelte typecheck + Playwright)
make test-all

# Submit a new task to Jules
python3 scripts/jules_submit.py --file prompts/tasks/<phase>/<task>.txt
```

---

*Last updated: 2026-09-30*
