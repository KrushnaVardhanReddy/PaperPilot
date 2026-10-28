# PaperPilot — Session Handoff Guide

> **Single Source of Truth** for resuming work on PaperPilot.  
> Start any new agent or human session by reading this document.  
> **Date of Last Update:** October 28, 2026 (Asia/Hong_Kong)
>
> **Active Target:** Phase 3 & Phase 4 Community Completion Sequence (Tasks 4.5 ➔ 4.7.4 ➔ 4.7.6 ➔ 3.4.11)

---

## 🧭 Executive Summary & Current Repository State

- **Integration Branch**: `feature/dev` (Clean & synchronized with remote `origin/feature/dev`).
- **Target Branch Protocol**: Always merge and push to `feature/dev`. Never push unreviewed code directly to `main`.
- **System Stability & Audits**:
  - `cargo check --workspace -j 2`: **100% Clean**.
  - Local security audit (`make audit`): **0 blocking vulnerabilities**.
  - MCP Tool Count: **78 tools registered and verified** (`paperpilot-mcp`).
  - Desktop Vitest Suite: **112/112 passing tests** (`pnpm --dir apps/desktop test`).
  - Architecture Roadmaps: Modularized into `phases/` directory with master dashboard in `PHASES.md`.
- **Scope Boundary Policy**: Focus exclusively on **Community (Open-Source)** tasks in this repository. All Enterprise governance (SAML/SSO, SCIM, SIEM, Ed25519 air-gapped licensing) is reserved for the private `PaperPilot-Enterprise` repo.

---

## 🚀 Active Dispatch & Phase 3 & 4 Closure Execution Order

We are systematically closing out all remaining Community gaps in **Phase 4 (AI Layer)** and **Phase 3 (Desktop & Mobile)** in this exact order:

| **1** | **Task 4.5** | Phase 4.5.1 / Spec 134 | **CLI Natural Language Interface (`paperpilot ask "..."`)**<br>Connect natural language command parsing (`paperpilot-nlp` resolver) to CLI with `--yes` and `--offline` flags. | `feature/dev` | ✅ **Completed & Merged (PR #291)** |
| **2** | **Task 4.7.4** | Phase 4.7.4 | **Batch AcroForm Mail Merge (`paperpilot fill-batch`)**<br>Iterate CSV/JSON records, batch fill AcroForm fields, flatten, and write template tokens `{row.name}_{date}.pdf`. | `feat/cli-fill-batch-task-4-7-4` | 📋 **Next Up** |
| **3** | **Task 4.7.6 / 8.2.3** | Phase 4.7.6 / 8.2.3 | **Conditional Recipe Execution Engine (`.ppflow`)**<br>Evaluate runtime conditions (`if_scanned`, `if_page_count_gt`, `on_no_match`) before executing pipeline steps. | `feat/conditional-pipeline-engine-task-4-7-6` | 📋 Scheduled |
| **4** | **Task 3.4.11** | Phase 3.4.11 | **Desktop Visual Flowchart & Conditional Branching UI**<br>Add IF/ELSE condition decision nodes and branching visual connections to `PipelineCanvas.svelte`. | `feat/desktop-conditional-branching-ui` | 📋 Scheduled |

---

## 🏆 Recently Merged & Completed

### Local stabilization follow-up (Task 84D; not merged)

- Branch: `fix/slm-compound-workflow-validation`, based on `origin/feature/dev` at `51c59fd`.
- Repaired all 15 compound recipes and restricted generation to 42 registered, usable MCP tools with a live-schema snapshot and typed artifact validation. JSON reports preserve their source document; directories are terminal; Office exports retain their actual formats.
- Real multi-step MCP tests uncovered and fixed two core defects: PPTX export lacked OPC relationships, and decrypting a loaded password-protected PDF could silently lose its pages. Also corrected the `pdf_remove_blank` and `pdf_compare` required-argument schemas.
- Validation: 223 Python checks including 11 unmocked MCP executions, 10,000 generated/validated labels, and targeted Rust regressions. See [the stabilization report](reports/SLM_COMPOUND_WORKFLOW_VALIDATION_REPORT.md) for exact scope and remaining model evaluation.
- No model was retrained or republished. The local inference benchmark now checks 6 single requests, 15 workflows and a refusal, and fails on missing weights. Published model accuracy must be re-evaluated after retraining on the corrected data.
- The Phase 3/4 execution order above remains the upstream plan; this is a separate stabilization branch awaiting review.

### Previously merged work

1. **Task 134A / Phase 4.5.1 (CLI Natural Language Interface: `paperpilot ask`, PR #291)**:
   - Added `Ask` subcommand in `paperpilot-cli` (`src/cli.rs`, `src/commands/ask.rs`) with `--dry-run`, `--yes`, `--offline`, and `-o/--output` flags.
   - Wires `paperpilot-nlp`'s `OfflineNlpResolver` and exhaustive dispatch across all 40+ canonical intents to existing CLI operation handlers.
   - Verified with 4/4 passing integration tests in `paperpilot-cli/tests/test_ask.rs`.
2. **Task 84D (Compound SLM Workflow Recipes & Multi-Step Chaining `[Spec 084]`, PR #277)**:
   - Added 15 compound multi-step workflow recipes in `tools/slm-training/templates.py` (e.g. Legal Discovery, Contract Redaction, Print Prep Booklet, Form Processing, Archival OCR).
   - Implemented dynamic parameter piping (`/tmp/inter_*.pdf`) ensuring sequential step outputs chain directly into subsequent tool inputs.
   - Rebalanced training dataset distribution to 50% single-tool, 40% compound multi-step pipelines, and 10% safety refusals.
   - Added comprehensive test suite in `tools/slm-training/test_dataset_generator.py` (8/8 tests passing).
2. **Task 129A (Automated Malformed PDF Fuzzing & Security Regression Harness `[Spec 129]`, PR #276)**:
   - Added `paperpilot-pdf/tests/malformed_regression.rs` verifying 5 distinct corruption attack vectors exit cleanly via `Result::Err` with zero panics or memory loops:
     - Corrupted XRef table offsets & negative indices.
     - Corrupted `%PDF-` header and random arbitrary byte sequences.
     - Stream length mismatch and stream length bombs.
     - Deeply nested dictionary hierarchies (stack overflow protection).
     - Circular page tree loops (`/Parent` referencing self or cycle).
   - Added `LopdfDocument::from_bytes(bytes)` in `paperpilot-pdf/src/document.rs` for fast zero-disk-I/O in-memory regression tests.
   - Verified 100% clean compilation, 5/5 passing regression tests, and zero Semgrep findings (`make audit`).
2. **Task 122A (Lossless Stream Deduplication & CMYK Safety `[Spec 122]`, PR #275)**:
   - Ported hash-based indirect stream deduplication from `SimonBure/presse` (`pdf_dedup_streams`) collapsing identical embedded images, fonts, and streams across pages into single indirect objects.
   - Enforced CMYK color-space safety guards in `paperpilot-pdf/src/operations/compress.rs`.
   - Verified 100% clean compilation, zero Semgrep findings, and 10/10 unmocked tests passing in `penta-interface-e2e`.
2. **Modular Phases Architecture Refactor**:
   - Split the monolithic ~2,000-line `PHASES.md` into 13 modular documents under `phases/` with a high-level executive dashboard at root `PHASES.md`.
2. **Task 84 (On-Device Document SLM Fine-Tuning & Local Verification `[Spec 084]`, Task 5.9.73)**:
   - Generated 5,000 ChatML training pairs across 45+ tools with 5 linguistic noise tiers.
   - Fine-tuned `Qwen/Qwen2.5-0.5B-Instruct` using 4-bit QLoRA and `paged_adamw_8bit` on local RTX 4070 (846 steps, loss drop ~2.1 -> 0.47, eval loss 0.496, 90.4% token accuracy).
   - Saved 17 MB adapter weights to `tools/slm-training/output/paperpilot-qwen-0.5b-lora/` and fused into `output/paperpilot-qwen-0.5b-merged/` via `export_and_merge.py`.
   - Verified 100% exact JSON tool call generation across single-tool test queries.
3. **Task 125A (Single-File Offline Self-Contained HTML Sandbox `[Spec 125]`)**:
   - Packaged standalone `paperpilot-portable.html` embedding the full UI and in-memory base64-encoded WASM engine.
   - Strict air-gapped CSP (`default-src 'none'`), Playwright verified with 0 network requests.
4. **Task 126A (Open-WebUI & Apache Tika Ingestion Adapter `[Spec 126]`, PR #273)**:
   - In-memory Open-WebUI Tika 3 (`PUT /tika/text`), Tika 4 (`PUT /tika/json/md`), and root metadata routes in `paperpilot-gateway`.
5. **Task 128A (CI Supply Chain Hardening & Action SHA Pinning `[Spec 128]`, PR #274)**:
   - Pinned all GitHub Actions across workflows to immutable 40-char commit SHAs with automated weekly Dependabot tracking.

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
