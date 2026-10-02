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
| [`docs/architecture/testing_and_troubleshooting_guide.md`](./docs/architecture/testing_and_troubleshooting_guide.md) | Authoritative 3-tier (MCP, API/Tauri, Svelte UI) diagnostic and testing guide for Jules & contributors. |
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
| Phase 4.F.7-A (Editable Page Jump & Zoom Presets) | ✅ Complete | Interactive page number input + zoom preset dropdown & Fit button (PR #77) |
| Phase 4.F.7-B (Multi-Document Tabs & Operations Dock) | ✅ Complete | Multi-Document tabs `[Tab1][Tab2][+]` + right-docked Operations panel (PR #78) |
| Phase 4.F.8 (Desktop Instant Open & Power UX) | ✅ Complete | Instant open, Command Palette (`Ctrl+K`), Floating Toolbar, Resizable Right Panel (PR #79) |
| Phase 4.F.9-A (Canvas Text Search) | ✅ Complete | In-canvas text search bar (`Ctrl+F` / `⌘F`), match navigation (`1 of 12`, `‹ / ›`), Titlebar window controls anchor fix (PR #81) |
| Phase 4.F.9-B (Thumbnail Page Management) | ✅ Complete | Direct thumbnail drag-and-drop page reordering + hover quick actions (Rotate 🔄, Delete 🗑️) (PR #80) |
| Phase 4.F.10 (Stirling-Style Tools Dock) | ✅ Complete | Categorized tools directory + search filter + slide-in parameter inspectors (PR #82) |
| Phase 4.F.11 (Visual Pixel Diff Slider) | ✅ Complete | Visual split-screen pixel diff comparison slider, pixel discrepancy map, side-by-side mode (PR #83) |
| Phase 4.F.12 (Custom CSS Document Conversions) | ✅ Complete | 5 CSS presets (`github`, `elegant`, `minimal`, etc.), custom CSS upload/inline, Excel semantic table styling (PR #84) |
| Phase 4.F.13 (Stirling Parity: Category Pills & Extended Tools) | ✅ Complete | Category filter pills, blank page removal, page numbers, to-PDF conversions in Dock (PR #87) |
| Phase 5 (V1 API Gateway & Headless Automation Server) | ✅ Complete | Axum REST API, Dynamic 44-tool routing, MCP-over-SSE, CLI `paperpilot serve --port 7823` (PR #88) |
| Phase 4.E2E R2.E5 (Triple-Channel Autonomous QA Suites) | 🚧 In-Progress | 8 active Jules sessions verifying all 44 tools across Desktop UI, REST API, and MCP channels |

---

## 🚧 Active Jules Tasks (In-Progress)

| Task | Session ID | What it does | Session Link |
|---|---|---|---|
| **QA-1**: Document Lifecycle & Tabs | `544885962421601530` | Multi-doc tabs, `pdf_info`, `pdf_validate`, `pdf_hash`, `pdf_metadata` | [Session 544885962421601530](https://jules.google.com/session/544885962421601530) |
| **QA-2**: Viewer Navigation & Zoom | `17034994067282609951` | Canvas page jump, zoom fit, `pdf_render`, `pdf_classify_type`, `pdf_bookmarks` | [Session 17034994067282609951](https://jules.google.com/session/17034994067282609951) |
| **QA-3**: Search & Extraction | `10793971291527024299` | Canvas search bar, `pdf_search`, `pdf_extract_text`, `pdf_extract_images`, `pdf_ocr` | ✅ Merged (PR #91) |
| **QA-4**: Thumbnail Sidebar & Reorder | `13669625293175016599` | Drag reorder, `pdf_rotate`, `pdf_crop`, `pdf_delete_pages`, `pdf_extract_pages`, `pdf_reorder_pages`, `pdf_burst` | ✅ Merged (PR #90) |
| **QA-5**: Annotations & Markup | `3824896424156358922` | Toolbar, `pdf_watermark`, `pdf_annotate`, `pdf_header_footer`, `pdf_bates`, `pdf_create_form_field`, `pdf_read_form`, `pdf_fill_form`, `pdf_sign` | ✅ Merged (PR #89) |
| **QA-6**: Operations Dock & Page Tools | `3241010897792427201` | Category pills, `pdf_merge`, `pdf_split`, `pdf_page_numbers`, `pdf_remove_blank`, `pdf_compress`, `pdf_linearize`, `pdf_repair`, `pdf_flatten` | [Session 3241010897792427201](https://jules.google.com/session/3241010897792427201) |
| **QA-7**: Security & Conversions | `7264050569938354833` | Password encryption, `pdf_encrypt`, `pdf_decrypt`, `pdf_redact`, `pdf_convert_html`, `pdf_convert_markdown`, `pdf_convert_excel`, `pdf_images_to_pdf`, `pdf_to_pdf_a`, `pdf_to_docx`, `pdf_to_xlsx`, `pdf_to_pptx` | [Session 7264050569938354833](https://jules.google.com/session/7264050569938354833) |
| **QA-8**: Visual Pixel Diff Slider | `4164997819618967941` | Split slider (`Ctrl+D`), discrepancy map, `pdf_compare` | [Session 4164997819618967941](https://jules.google.com/session/4164997819618967941) |

---

## 🔲 What's Next (Backlog Priority Order)

1. **Phase 4.E2E R2.E4:** Execute `scripts/benchmark.py` for system performance re-benchmarking post-Phase 4.F.
3. **Phase 5 (Advanced Intelligence & Local AI Engine):**
   - High-speed OCR (`paperpilot-ocr`)
   - Local embedded vector engine via `ort` ONNX Runtime (<30MB model) for 100% offline RAG & semantic search
   - MinerU / Marker-style layout-aware clean Markdown and LaTeX table parsing
   - Automatic local PII detection & structural redaction (SSNs, credit cards, names)
   - Multi-core memory-mapped streaming (`memmap2`) for 1,000+ page documents
   - Headless CLI Pipeline Runner (`paperpilot pipeline run --spec recipe.json`) & Streamable HTTP MCP server (`paperpilot-mcp --port 8080 --http`)

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
