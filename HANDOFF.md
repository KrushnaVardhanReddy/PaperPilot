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
| Phase 1.1.4 (Test Fixtures) | ✅ Complete | Test corpus built and tests validated |
| Phase 3.5.6 (Responsive UI) | ✅ Complete | Mobile bottom-nav and operations panel |
| Phase 4.0.1 (NLP Crate) | ✅ Complete | Created `paperpilot-nlp` |
| Phase 4.0.2 (NLP Trait) | ✅ Complete | Defined `NlpResolver` trait |
| Phase 4.0.3 (Intent Vocab) | ✅ Complete | ~45 intents & aliases defined in `intent.rs` |
| Phase 4.0.4 (Rule Engine) | ✅ Complete | Layer 1 Aho-Corasick keyword matcher |
| Phase 4.0.5a (ONNX Training) | ✅ Complete | Python pipeline to train and export MobileBERT |
| Phase 4.0.6 (Entity Extractor) | ✅ Complete | Regex & NER for files, pages, angles |
| Phase 4.0.7+4.0.8 (Resolver) | ✅ Complete | Wired `OfflineNlpResolver` and ambiguity checks |
| Phase 4.0.9 (Offline NLP Tests) | ✅ Complete | Accuracy test suite in `resolver_accuracy_test.rs` |
| Phase 4.0.10 (Offline E2E) | ✅ Complete | NLP output to actual MCP file modifications |
| Phase E.1 (CLI E2E) | ✅ Complete | Automated E2E testing sweep of `paperpilot-cli` |
| Phase E.2 (MCP E2E) | ✅ Complete | Automated JSON-RPC stdio testing of `paperpilot-mcp` |
| Phase E.3 (UI E2E) | ✅ Complete | Playwright UI tests for the Svelte/Tauri app |
| Phase E.4 (Benchmark) | ✅ Complete | System performance & latency testing |

---

## 🚧 Active Jules Sessions (In-Progress)

| Task | Session ID | What they are doing |
|---|---|---|
| Phase 4.F.2 Annotations | `10367502172143029328` | Building the PDF annotation overlays (highlight, pen, sticky notes) |
| Phase 4.F.3 Form Filling | `6603654323246619177` | Detecting and rendering HTML form inputs over the PDF canvas |

---

## 🔲 What's Next (Backlog Priority Order)

1. **Wait for F.2 & F.3 Completion:** Wait for both Jules sessions to complete and verify them.
2. **Phase 5 (NLP):** Once the PDF editor is entirely functional, start integrating the local Llama backend.

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

*Last updated: 2026-09-29*
