# PaperPilot — Session Handoff Guide
 
> **Single Source of Truth** for resuming work on PaperPilot.  
> Start any new agent or human session by reading this document.  
> **Date of Last Update:** October 29, 2026 (Asia/Hong_Kong)
>
> **Active Target:** Spec 141 (Multi-Rule Condition Builder) & Spec 142 (Watched Folders & Smart Filters) successfully merged into `feature/dev` via PR #305 and PR #306. Active next step is Spec 143 (Universal Document Workspace Migration).

---

## 🧭 Executive Summary & Current Repository State

- **Integration Branch**: `feature/dev` (`d88033d`), incorporating both PR #305 (`dda6861`) and PR #306 (`365f8c0`).
- **Target Branch Protocol**: Feature branches and reviewed PRs into `feature/dev`; never push directly to `main`.
- **System Stability & Audits**:
  - Desktop Vitest Suite: **147 passing tests across 24 files** (100% green).
  - Rust Backend Integration Suites: `desktop_automation` (8/8 passing), `watcher_lifecycle` (2/2 passing).
  - Cargo Workspace: `cargo check --workspace` compiles cleanly with zero errors.
- **Active Feature Branch**: `feat/desktop-universal-doc-workspace-migration` (Spec 143) executing via Jules session `18342025999379061323`.

---

## Merged Spec 141 & Spec 142 Implementations (PR #305 & PR #306)

- **Spec 141 (PR #305)**: Full recursive multi-rule condition editing (`ALL` / `ANY` combinators, dynamic rule row additions/deletions, backwards-compatible serialization to flat leaves, and `StepCard.svelte` condition badges).
- **Spec 142 (PR #306)**: 
  - Settings → Watched Folders uses native filesystem workers and the shared PDF workflow runner (`paperpilot_pdf::operations::watch`).
  - Desktop, embedded MCP and REST use one local manager.
  - Smart filters inspect actual PDF page/scan/form/signature facts (`isScanned`, `isSigned`, `hasFormFields`, `pageCount > 1`) with bounded concurrency.
  - Bounded concurrency, output collision handling, source preservation, and SHA-256 receipts verified by real automated tests.

---

## 🚀 Active Roadmap & Execution Order

We are systematically closing out all remaining Community gaps in **Phase 4 (AI Layer)** and **Phase 3 (Desktop & Mobile)**:

| # | Task | Scope & Title | Status |
|:---:|---|---|:---:|
| **1** | **Task 3.4.14 / Spec 141** | **Desktop Multi-Rule Condition Builder**<br>Compound `ALL` / `ANY` predicates, dynamic rule builder, and backward-compatible serialization. | ✅ **Completed & Merged (PR #305)** |
| **2** | **Task 4.7.8 / Spec 142** | **Desktop Real Watched Folders Daemon & Document Smart Filters**<br>Pure-Rust Tauri IPC filesystem watcher, `WatchedFoldersPanel.svelte`, and reactive smart filter pills. | ✅ **Completed & Merged (PR #306)** |
| **3** | **Task 3.4.15 / Spec 143** | **Universal Document Workspace Migration**<br>Consumer-friendly studio with universal drop zone (PDF, Word, Excel, CSV, Markdown), 8 Everyday Actions, and smart format filters. | 🚀 **In Progress** (`feat/desktop-universal-doc-workspace-migration`) |
| **4** | **Phase 5.9.100 / Spec 133** | **OpenSSF Scorecard Supply Chain Hardening**<br>Pin GitHub Actions SHAs, establish `SECURITY.md`, and configure Scorecard SARIF workflow. | 📋 **Queued** (`feat/openssf-scorecard-hardening`) |

---

## 🏆 Recently Merged Milestones

1. **Spec 142 / Task 4.7.8 (Desktop Real Watched Folders & Smart Filters, PR #306)**:
   - Built pure-Rust Tauri IPC commands (`start_watch_folder`, `stop_watch_folder`, `list_watched_folders`) connected to `paperpilot_pdf::operations::watch`.
   - Replaced mocked categories with real, reactive document filter pills in `DocumentList.svelte` based on real document inspection (`isScanned`, `isSigned`, `hasFormFields`, `pageCount > 1`).
   - Added `WatchedFoldersPanel.svelte` inside Settings. Verified with 147 passing Vitest tests and all Rust integration tests (`desktop_automation`, `watcher_lifecycle`).
2. **Spec 141 / Task 3.4.14 (Desktop Multi-Rule Condition Builder, PR #305)**:
   - Upgraded `ConditionInspector.svelte` to support recursive AST `ALL (AND)` / `ANY (OR)` logic with dynamic rule additions and removals.
   - Preserved 100% backward compatibility for single-leaf conditions and saved flows.
3. **Spec 139 / Task 4.7.1 (Hot Folder Watcher Daemon, PR #304)**:
   - Implemented cross-interface watcher daemon across CLI, MCP, and REST Gateway with debounce settling and error quarantine.
4. **Spec 138 / Task 3.4.11 (Desktop Visual Flowchart & Branching UI, PR #301)**:
   - Added visual condition decision cards and branching badges in `PipelineCanvas.svelte`.

---

## 📋 Resuming Checklist for Next Session

When resuming a new session, follow these steps:

1. **Check for New User Issues or Community PRs**:
   - Inspect GitHub Issues (filed via `.github/ISSUE_TEMPLATE/` bug and feature request forms).
   - Inspect any incoming community PRs against `feature/dev`.

2. **Current Repository & Model State**:
   - **Production Release**: PR #267 merged into `main` (`c487810`).
   - **Active Development Branch**: `feature/dev` (in sync with `main`).
   - **Fine-Tuned SLM Weights**:
     - **Hugging Face Hub**: [kpakkiragari/PaperPilot-Qwen-0.5B-Instruct](https://huggingface.co/kpakkiragari/PaperPilot-Qwen-0.5B-Instruct) (Public Apache-2.0 release)
     - Local LoRA adapter: `tools/slm-training/output/paperpilot-qwen-0.5b-lora/`
     - Local Fused model: `tools/slm-training/output/paperpilot-qwen-0.5b-merged/`
     - Quick test: `tools/slm-training/.venv/bin/python tools/slm-training/infer_local.py`

3. **If Implementing a User Request**:
   - Follow Rule 7: Always update tool documentation in `apps/docs/src/content/docs/reference/tools/<tool>.md`.
   - Run `python3 apps/docs/src/content/docs/reference/generate_handbook.py`.
   - Run verification with memory-safe `-j 2`: `cargo test -p paperpilot-pdf --lib -j 2` and `make audit`.

---

## ⚖️ Rules of Engagement & Production Constraints

> ⚠️ **CRITICAL**: PaperPilot is a live, trending open-source product on Hacker News. All development must preserve production stability.

1. **System Resource Constraint (`-j 2` is MANDATORY on local machine):**
   - **ALWAYS** pass `-j 2` for all local `cargo test`, `cargo build`, or `cargo check` commands (e.g. `cargo test -p paperpilot-pdf -j 2`).
   - Unbounded cargo compilations will freeze the host machine.

2. **Feature Branch Protocol (No Direct Pushes to `feature/dev` or `main`):**
   - All development, bug fixes, and experiments must occur on dedicated feature branches (e.g. `feat/...`, `fix/...`).
   - Never push directly to `feature/dev` or `main`. Code is merged into `feature/dev` only after review and complete local test verification.

3. **The Single Core Runner + 5 Wrappers Architecture Pattern:**
   - Every tool operation MUST have exactly **ONE** core in-memory engine implementation in `paperpilot-pdf`:
     ```rust
     pub fn run_operation_bytes(input_bytes: &[u8], config: &Config) -> Result<Vec<u8>, OperationError>
     ```
   - 5 Thin Interface Wrappers: CLI (`paperpilot-cli`), MCP (`paperpilot-mcp`), REST Gateway (`paperpilot-gateway`), WASM Engine (`paperpilot-wasm`), and Desktop/Embed (`apps/desktop`, `apps/embed`).

4. **Zero-Mock Policy on Tests:**
   - Test fixtures must be real disk fixtures or synthesized byte buffers (`fixtures/`, `tests/e2e_fixtures/real/`). Never mock PDF bytes or operation responses.

5. **Zero-Cloud & Absolute Privacy Guarantee:**
   - 0 bytes uploaded to external servers. Never introduce unexpected analytics beacons, telemetry, or external network requests into Core, CLI, MCP, Desktop, or WASM.

6. **Local Git Hygiene:**
   - Model weights (`tools/slm-training/output/`), `pricing_strategy.md`, and security scanner reports must remain untracked / ignored (`.gitignore`).

---

## 🛠️ Quick Command Reference

```bash
# 1. Dev Server (Web Portal)
cd apps/web && npm run dev

# 2. Rust Unit Tests (Always use -j 2)
cargo test -p paperpilot-pdf --lib -j 2
cargo test -p paperpilot-mcp --lib -j 2
cargo test -p paperpilot-gateway -j 2
cargo test -p paperpilot-cli -j 2

# 3. Desktop Vitest Tests
pnpm --dir apps/desktop test

# 4. Local Security Audit Scan
make audit

# 5. Knowledge Graph Refresh
graphify update .
```
