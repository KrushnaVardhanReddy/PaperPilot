# PaperPilot — Session Handoff Guide

> **Single Source of Truth** for resuming work on PaperPilot.  
> Start any new agent or human session by reading this document.  
> **Date of Last Update:** October 27, 2026 (Asia/Hong_Kong)
>
> **Active Target:** Task 84D (SLM Compound Pipeline Chaining) + Task 129A (Fuzzing) + Task 122A (Stream Dedup)

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

---

## 🚀 Active Dispatch & In-Flight Status

| Task ID | Spec | Priority | Component | Target Branch | Status & Details |
|---|:---:|:---:|---|---|---|
| **Task 84D** | `[Spec 084]` | P1 | SLM / AI Pipeline | `feature/dev` | 🚀 **In-Flight in Jules**: [Session 17536454483435898495](https://jules.google.com/session/17536454483435898495)<br>Compound multi-step dataset expansion, dynamic arbitrary tool chaining (2–5 tools), and parameter piping for visual pipeline generation. |
| **Task 129A** | `[Spec 129]` | P1 | Quality / Fuzzing | `feature/dev` | 🚀 **In-Flight in Jules**: [Session 7317970073403118704](https://jules.google.com/session/7317970073403118704)<br>Adversarial regression test suite (`malformed_regression.rs`) against corrupted headers, circular page trees, stream bombs, and xref corruption. |
| **Task 127A** | `[Spec 127]` | P2 | Core / Format Conversion | `feature/dev` | 📋 **Backlog / Planned** (`specs/127-...`)<br>PDF-to-EPUB semantic engine via structured intermediate AST. |

---

## 🏆 Recently Merged & Completed

1. **Task 129A (Automated Malformed PDF Fuzzing & Security Regression Harness `[Spec 129]`, PR #276)**:
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

Follow this exact sequence to resume work:

1. **Check Jules In-Flight Sessions & PRs**:
   - Check **Task 129A** (Fuzzing Harness): [Session 7317970073403118704](https://jules.google.com/session/7317970073403118704).
   - Check **Task 122A** (Stream Deduplication): [Session 5396882730014124345](https://jules.google.com/session/5396882730014124345).
   - Check **Task 84D** (Compound SLM Workflow Generation): Prompt ready in `jules_prompts/P5_9_73_slm_compound_workflows.txt`.
   - Run `git fetch origin` to inspect any new PR branches.
   - Validate and merge with `-j 2`:
     ```bash
     cargo test -p paperpilot-pdf --test malformed_regression -j 2
     make audit
     git push origin feature/dev
     ```

2. **Trigger Retraining on RTX 4070 (Once Task 84D PR Merges)**:
   - Generate expanded 10,000-sample balanced dataset:
     ```bash
     tools/slm-training/.venv/bin/python tools/slm-training/generate_dataset.py --count 10000 --compound-ratio 0.4 --output-dir tools/slm-training/dataset/
     ```
   - Kick off local fine-tuning run:
     ```bash
     tools/slm-training/.venv/bin/python tools/slm-training/train_local.py
     ```
   - Verify multi-step array generation with `infer_local.py`.

---

## ⚖️ Rules of Engagement & Production Constraints

> ⚠️ **CRITICAL**: PaperPilot is a live, trending open-source product on Hacker News. All development must preserve production stability.

1. **System Resource Constraint (`-j 2` is MANDATORY on local machine):**
   - **ALWAYS** pass `-j 2` for all local `cargo test`, `cargo build`, or `cargo check` commands (e.g. `cargo test -p paperpilot-pdf -j 2`).
   - Unbounded cargo compilations will freeze the host machine.

2. **Target Branch Protocol:**
   - All feature development, Jules cloud sessions, and local merges MUST target `feature/dev`. Never push unreviewed code directly to `main`.

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
