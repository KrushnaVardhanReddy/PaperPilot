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
| [`PHASES.md`](./PHASES.md) | The master roadmap tracker. Every task, its status (✅/🚧/🔲), and which Jules session merged it. **Always check this first.** |
| [`COMPETITORS.md`](./COMPETITORS.md) | Feature comparison matrix vs. Adobe, iLovePDF, PDF24, Stirling-PDF, PDFgear. Identifies our moat and gaps. |
| [`pricing_strategy.md`](./pricing_strategy.md) | Tiered pricing model: Free → Pro ($7/mo) → Teams ($15/user/mo) → Enterprise ($25/user/mo). Includes the Contributor Revenue Sharing algorithm. |
| [`Makefile`](./Makefile) | All dev commands. `make dev` starts the app, `make test-all` runs every test tier. |
| [`COMPETITORS.md`](./COMPETITORS.md) | Feature comparison matrix vs. Adobe, iLovePDF, PDF24, Stirling-PDF, PDFgear. |
| `scripts/jules_submit.py` | The submission script for Jules. Archives the prompt after submission. |
| `prompts/tasks/` | Pending task prompts for Jules (not yet submitted). |
| `prompts/tasks/done/` | Archived prompts for tasks already submitted to Jules. |

---

## 🏗️ Architecture Overview

```
PaperPilot/
├── paperpilot-core/        # Rust: Core traits (PdfOperation), error types, shared types
├── paperpilot-pdf/         # Rust: All 45+ PDF operations (merge, split, encrypt, OCR, etc.)
├── paperpilot-mcp/         # Rust: MCP JSON-RPC server exposing all operations as tools
├── paperpilot-cli/         # Rust: Terminal CLI (paperpilot merge, split, create, etc.)
├── apps/desktop/           # Svelte 5 + Tauri 2.0 desktop app
│   ├── src/                # Svelte frontend (runes, components, state stores)
│   ├── src-tauri/          # Rust Tauri backend (invoke_mcp_tool IPC command)
│   └── tests/              # Playwright E2E tests
└── Cargo.toml              # Workspace root (all Rust crates)
```

**Key Patterns:**
- All PDF operations implement the `PdfOperation` trait in `paperpilot-core/src/traits.rs`.
- MCP Server in `paperpilot-mcp/src/server.rs` uses **manual `ServerHandler` trait** — do NOT use `rmcp` macros.
- Frontend uses **Svelte 5 runes** (`$state`, `$derived`) — not Svelte 4 stores.
- Frontend calls Tauri backend via `invoke('invoke_mcp_tool', { tool, args })` in `apps/desktop/src-tauri/src/lib.rs`.

---

## ✅ What's Done (Phase Status)

| Phase | Status | Notes |
|---|---|---|
| Phase 1.3 Groups A, B, C | ✅ Complete | All 29+ core PDF operations merged |
| Phase 1.3 Group F (PDF Creation) | ✅ Complete | Markdown→PDF, HTML→PDF, Template Engine, CLI |
| Phase 1.4 (Testing) | ✅ Complete | Unit, fixture, and E2E tests all passing |
| Phase 1.5 (CLI) — 1.5.1–1.5.17 | ✅ Complete | Full CLI with JSON mode, webhooks |
| Phase 2 (MCP Server) | ✅ Complete | 15+ MCP tools, validation, E2E tests |
| Phase 3.1 (Project Setup) | ✅ Complete | Tauri + Svelte 5 + Android target initialized |
| Phase 3.2–3.5 (UI + IPC) | ✅ Complete | Full UI shell, job system, Tauri IPC |
| Phase 3.5.7 (Playwright E2E) | ✅ Complete | 3 Playwright tests passing |
| Phase 3.6 (Vitest) | ✅ Complete | Toast, DocumentList, OperationsPanel unit tests |

---

## 🚧 Active Jules Sessions (In-Progress)

| Session ID | Task | Files Touched |
|---|---|---|
| `3236745347341611735` | CLI Security: HMAC webhook, hash/verify, E2E CLI tests (1.5.18–1.5.24) | `paperpilot-cli/` |
| `10906031722199501492` | Group D Conversion: PDF→HTML/MD/JSON, LLM Export, Integrity Hash (1.3.33–1.3.46) | `paperpilot-pdf/`, `paperpilot-mcp/` |

> ⚠️ **Merge Conflict Warning:** Both sessions touch `paperpilot-cli/src/cli.rs` (CLI enum variants). When merging, manually resolve by keeping all new `Subcommands` variants from both branches.

---

## 🔲 What's Next (Backlog Priority Order)

1. **[WAITING]** Validate and merge both active Jules PRs above.
2. **Phase 3.4.10 — Visual Pipeline Builder:** Node-based drag-and-drop canvas inside Svelte UI for chaining operations. (No Jules competitor is building this for PDFs.)
3. **Phase 1.3 Group E — Forms:** AcroForm read/fill/create (1.3.35–1.3.37).
4. **Phase 1.3 Group D Remaining:** PDF→DOCX, XLSX, PPTX (these need more complex crates — defer until after pipeline builder).
5. **Phase 3.1.7 — iOS Target:** Needs a Mac with Xcode to run `cargo tauri ios init`.
6. **Phase 4 — AI Layer:** Natural language → MCP tool execution. (Blocked by Phase 3.4.10.)

---

## 🛠️ Useful Commands

```bash
# Start the desktop app (UI + Rust backend)
make dev

# Run ALL tests (Rust + Svelte typecheck + Playwright)
make test-all

# Run only Rust tests
make test-backend

# Run only Playwright E2E
make test-e2e

# Check for compile errors without building
make check

# Submit a new task to Jules
python3 scripts/jules_submit.py --file prompts/tasks/<phase>/<task>.txt

# Check active Jules sessions
# Visit: https://jules.google.com/

# Fetch Jules's latest PRs
git fetch origin && git branch -r | grep jules
```

---

## 📐 How to Write a Jules Prompt

Prompts live in `prompts/tasks/<phase>/`. The format is:
```
---
Task: <Phase X.Y — Task Name>
Context: <Brief description of the codebase state Jules needs to know>
---

Hey Jules! <Friendly greeting, then bullet-pointed requirements>

## Requirements:
1. ...

## Rules:
- Follow the existing PdfOperation trait pattern in paperpilot-core/src/traits.rs
- STRICTLY DO NOT USE TailwindCSS
- Add #[cfg(test)] unit tests for each operation
- No Mocking in integration tests — use real PDF files

Please open a PR to `main` when finished!
```

---

## 💰 Pricing Summary (Updated)

| Tier | Price | Key Hook |
|---|---|---|
| Personal | Free | All core PDF ops |
| Pro | $7/mo | AI mode, Visual Pipeline Builder |
| Teams | $15/user/mo | Shared recipe library, audit log |
| Enterprise | $25/user/mo | SSO, MDM, SLAs, legal indemnification |

15% of paid revenue → Contributor pool, distributed quarterly based on feature criticality multipliers.

---

*Last updated: 2026-09-29*
