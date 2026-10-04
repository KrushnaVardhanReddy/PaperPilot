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

## 📊 Current State: Phase 4.E2E-R3 Tri-Interface Living Documentation & Fixes

### 1. Completed Tri-Interface Test Sweeps
The 44 tools were tested across all three interfaces (CLI, MCP, and REST API) and documented in `reports/TRI_INTERFACE_E2E_AND_DOCS.md` without masking errors:
- **Batch 1 (Tools 1–10)**: Merged (PR #110).
- **Batch 2 (Tools 11–20)**: Merged (PR #112).
- **Batch 3 (Tools 21–30)**: Merged (PR #113).
- **Batch 4 (Tools 31–44)**: Merged (PR #111).

### 2. Discovered Issues (Root Cause Classification)
- **CLI Argument Mismatches**: Missing `--angle` alias on `rotate`, `num_args=1..` needed on `merge` and `images-to-pdf`, `burst` needing `--output-dir` alias, individual `x/y/w/h` flags on `crop`, missing `remove-blank` and `page-numbers` CLI subcommands, binary name alias `paperpilot`.
- **Gateway Parameter Mismatches**: `pdf_rotate` requiring `pages` instead of defaulting to `"all"`, `pdf_crop` expecting `box` rather than `x/y/w/h`, `pdf_split` expecting `split_points` instead of `output_dir`, `pdf_render` needing JSON route alias.
- **Core Engine & File Generation**: `pdf_render` lacking pure Rust fallback to write output file on disk, `pdf_split` output directory handling, `pdf_fill_form` JSON parsing, and `pdf_ocr` output persistence.

---

## 🚧 Active Jules Sessions (In-Flight Tonight)

All 3 tasks have been partitioned with **strict crate-level file ownership** so that they run simultaneously with **zero merge conflicts**:

| Task | Scope (Owned Crates) | Session URL | Target Branch | Focus |
|---|---|---|---|---|
| **P4_FIX_1** | `paperpilot-cli/` only | [Session 15143213812508351944](https://jules.google.com/session/15143213812508351944) | `main` | **Merged (PR #114)**: Clap flags, aliases & subcommands fixed |
| **P4_FIX_2** | `paperpilot-gateway/` only | [Session 3687370227158297202](https://jules.google.com/session/3687370227158297202) | `main` | **Merged (PR #115)**: MCP/REST API params normalized |
| **P4_FIX_3** | `paperpilot-pdf/`, `paperpilot-mcp/` | [Session 11798290308056337855](https://jules.google.com/session/11798290308056337855) | `main` | Fix render fallback output, split output dir, form fill, OCR + decrypt empty pages fix |

---

## 🌅 Morning Checklist (What to Do Tomorrow)

When you resume:

1. **Check Jules PRs**:
   - Check PRs opened by Jules for the 3 sessions above on GitHub.
   - Run `git fetch origin` and review the PR branches.
2. **Merge PRs (Zero-Conflict Expected)**:
   - Because file ownership is strictly divided between `paperpilot-cli`, `paperpilot-gateway`, and `paperpilot-pdf/mcp`, PRs can be merged directly into `main`.
3. **Re-run Full Tri-Interface E2E Suite**:
   ```bash
   python3 scripts/test_tri_interface_e2e.py
   ```
   Verify that previously failed tools turn green in `reports/TRI_INTERFACE_E2E_AND_DOCS.md`.
4. **Proceed to Phase 4.E2E R2.E4 & Phase 5**:
   - Run `scripts/benchmark.py` for latency and resource re-benchmarking.
   - Advance into Phase 5 (Advanced Intelligence & Local AI Engine).

---

## 🛠️ Useful Commands

```bash
# Run workspace build & unit tests
cargo test --workspace

# Start the REST/MCP Gateway Server
./target/debug/paperpilot-serve --port 7823

# Run the complete Tri-Interface E2E verification
python3 scripts/test_tri_interface_e2e.py

# Run CLI regression suite
bash scripts/test_cli_e2e.sh

# Run MCP regression suite
python3 scripts/test_mcp_e2e.py
```

---

*Last updated: 2026-10-04 00:16 EDT (Post Tri-Interface E2E Fix Dispatches)*
