# PaperPilot — Session Handoff Guide

> **Single Source of Truth** for resuming work on PaperPilot.  
> Start any new agent or human session by reading this document.  
> **Date of Last Update:** October 27, 2026 (Asia/Hong_Kong)
>
> **Active Target:** Post-Launch Security Hardening + Wave 3 & 4 Core Operations + Mobile/Web Parity (78+ Tools Total)

---

## 🧭 Executive Summary & Current Repository State

- **Integration Branch**: `feature/dev`; Task 126A baseline is `origin/feature/dev` at `9c8e605`.
- **Local Continuation Branch**: `feat/open-webui-tika-adapter` (Task 126A changes are local and unmerged).
- **Target Branch Protocol**: Always merge and push to `feature/dev`. Never push unreviewed code directly to `main`.
- **System Stability & Audits**:
  - `cargo check --workspace -j 2`: **100% Clean**.
  - Local security audit (Semgrep, Trivy, and Cargo-Audit): **0 blocking vulnerabilities**.
  - MCP Tool Count: **78 tools registered and verified** (`paperpilot-mcp`).
  - Desktop Vitest Suite: **112/112 passing tests** (`pnpm --dir apps/desktop test`).

---

## 🚀 Active Dispatch & In-Flight Status

| Task ID | Spec | Priority | Component | Target Branch | Status & Details |
|---|:---:|:---:|---|---|---|
| **Task 122A** | `[Spec 122]` | P1 | Core / Optimizer | `feature/dev` | 🚀 **In-Flight in Jules**: [Session 5396882730014124345](https://jules.google.com/session/5396882730014124345)<br>Hash-based indirect stream deduplication (`pdf_dedup_streams`) ported from `SimonBure/presse` with CMYK safety. |
| **Task 127A** | `[Spec 127]` | P2 | Core / Format Conversion | `feature/dev` | 📋 **Backlog / Planned** (`specs/127-...`)<br>PDF-to-EPUB semantic engine via structured intermediate AST. |
| **Task 129A** | `[Spec 129]` | P1 | Quality / Fuzzing | `feature/dev` | 🚀 **In-Flight in Jules**: [Session 7317970073403118704](https://jules.google.com/session/7317970073403118704)<br>Automated adversarial regression test suite (`malformed_regression.rs`) against corrupted headers, circular page trees, stream bombs, and xref corruption. |
| **Task 84D** | `[Spec 084]` | P1 | SLM / AI Pipeline | `feature/dev` | 📋 **Ready for Dispatch to Jules**: Prompt in `jules_prompts/P5_9_73_slm_compound_workflows.txt`<br>Compound multi-step dataset expansion, dynamic arbitrary tool chaining, and parameter piping for visual pipeline generation. |

---

## 🏆 Recently Merged & Completed

1. **Task 84 (On-Device Document SLM Fine-Tuning & Local Verification `[Spec 084]`, Task 5.9.73)**:
   - Generated 5,000 ChatML training pairs across 45+ tools with 5 linguistic noise tiers (`tools/slm-training/generate_dataset.py`).
   - Fine-tuned `Qwen/Qwen2.5-0.5B-Instruct` using 4-bit QLoRA and `paged_adamw_8bit` on local NVIDIA RTX 4070 (846 steps, train loss dropped from ~2.1 to 0.47, eval loss 0.496, 90.4% token accuracy).
   - Saved 17 MB adapter weights to `tools/slm-training/output/paperpilot-qwen-0.5b-lora/`.
   - Verified 100% exact JSON tool call generation across test bench queries (`tools/slm-training/infer_local.py`).
2. **Task 125A (Single-File Offline Self-Contained HTML Sandbox `[Spec 125]`)**:
   - Packaged standalone `paperpilot-portable.html` distribution embedding the full UI and in-memory base64-encoded WebAssembly engine.
   - Enforced strict air-gapped Content Security Policy (`default-src 'none'`).
   - Automated generator in `scripts/build_portable_html.js` and targets `make build-portable` / `make test-portable`.
   - Verified with Playwright: 0 external network requests and working client-side operations.
2. **Task 126A (Open-WebUI & Apache Tika Ingestion Adapter `[Spec 126]`, PR #273)**:
   - Implemented in-memory Open-WebUI Tika 3 (`PUT /tika/text`), Tika 4 (`PUT /tika/json/md`), and root metadata routes in `paperpilot-gateway`.
   - Shared in-memory `run_extract_text_bytes` engine in `paperpilot-pdf` with Tokio `spawn_blocking` offloading.
   - Includes Open-WebUI integration guide (`docs/integrations/open_webui.md`) and custom folder tool script.
   - 5 core tests + 21 gateway tests passing cleanly.
2. **Task 128A (CI Supply Chain Hardening & Action SHA Pinning `[Spec 128]`, PR #274)**:
   - Pinned all 3rd-party GitHub Actions in `release.yml`, `rust.yml`, and `security.yml` to immutable 40-character commit SHAs.
   - Replaced unverified `curl | sh` script for `wasm-pack` with `which wasm-pack || cargo install wasm-pack --locked`.
   - Added `.github/dependabot.yml` tracking `github-actions` updates weekly.
   - All security gates and CI workflows verified 100% green.
2. **Task 124A (Mobile Web / Browser Sandbox Responsive Overhaul `[Spec 124]`)**:
   - Upgraded `HeroPlayground.svelte` and `OperationsView.svelte` for small/mobile screens (`< 768px` and `< 480px`).
   - Added horizontal scrollable tool tab bar, minimum 44px touch targets on buttons/dropzones, fluid full-height layouts.
3. **Task 123A (Client-Side XSS, Path Traversal, SSRF & Dockerfile Hardening `[Spec 123]`, PR #271)**:
   - Sanitized search snippets in `workspaceSearch.ts` with HTML entity escaping and added test coverage.
   - Configured strict CSP in `tauri.conf.json`.
   - Hardened `pack_commands.rs` against path traversal using alphanumeric regex.
   - Blocked SSRF loopback, link-local, unspecified, and CGNAT IP ranges and disabled automatic redirects in `url_fetch.rs`.
   - Set non-root `paperpilot` user in `Dockerfile`.
2. **Task 119A (Multi-Sheet Poster Tile Slicer `[Spec 119]`, PR #270, Task 5.9.89)**:
   - Slices oversized or scaled documents across standard printable grid sheets (A4, Letter) with glue gutters (`overlap_pt`) and cut trim marks.
   - Cleanly merged across Core, CLI, MCP (78 tools total), Gateway, and Desktop.
3. **Task 123B (Gateway Security Lockdown & Automated Security Gates `[Spec 123]`)**:
   - Enforced localhost/app-origin CORS restrictions and 100MB body limit (`DefaultBodyLimit::max(100MB)`).
4. **Task 121A (Mobile Touch Gestures & Viewport Panning `[Spec 121]`, PR #269, Task 5.9.90)**:
   - Viewport double-tap zoom, 2D panning, and swipe page navigation in `PdfPreview.svelte`.
   - Viewport double-tap zoom, 2D panning, and swipe page navigation in `PdfPreview.svelte`.

---

## 📋 Resuming Checklist for Next Session

Follow this exact sequence to resume work:

0. **Review the local Task 126A continuation**:
   - Read `reports/TIKA_INGESTION_ADAPTER_REPORT.md` for test results, scope, and the reusable task-local Windows Rust/MinGW environment.
   - Integration setup and the optional Open-WebUI Tool are in `docs/integrations/open_webui.md`.
   - Keep the original `feat/coding-agent-mcp-setup` branch and existing `reports/agent-validation/` artifacts intact. Target any reviewed integration at `feature/dev`.

1. **Check Jules Task 123A (Security Hardening)**:
   - Check [Jules Session 18198933508777026291](https://jules.google.com/session/18198933508777026291).
   - Run `git fetch origin` to see if a PR branch has been pushed.
   - When ready, review the diff on `pack_commands.rs`, `url_fetch.rs`, and `tauri.conf.json`.
   - Run verification:
     ```bash
     cargo test -p paperpilot-core -j 2
     cargo test -p desktop -j 2
     ```
   - Merge into `feature/dev` and push: `git push origin feature/dev`.

2. **Dispatch Next Priority Feature (Task 124A or Task 122A)**:
   - **Task 124A** (Mobile Web Playground UI):
     - Prompt Jules or implement in `apps/web/src/components/HeroPlayground.svelte` following `specs/124-mobile-web-responsive-playground/spec.md`.
   - **Task 122A** (Lossless Stream Deduplication):
     - Prompt Jules or implement in `paperpilot-pdf` following `specs/122-lossless-stream-deduplication-and-cmyk-safety/spec.md`.
   - **Task 128A** (CI Action SHA Pinning):
     - Prompt Jules or edit `.github/workflows/*.yml` following `specs/128-ci-supply-chain-sha-pinning/spec.md`.

3. **Check GitHub Actions CI Status on PR #267**:
   - Verify that all CI checks remain 100% green before merging `feature/dev` into `main`.

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
   - `pricing_strategy.md` and security scanner reports must remain untracked / ignored (`.gitignore`).
   - Never execute destructive git resets (`git reset --hard`, `git clean -fd`).

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

# 5. Jules Cloud Session Trigger (Always target feature/dev)
cargo run -j 2 -p jules-submit -- --file jules_prompts/P5_9_97_ci_action_sha_pinning.txt --branch feature/dev
# or via wrapper script:
./scripts/jules_submit.sh --file jules_prompts/P5_3_27_pdf_dedup_streams.txt --branch feature/dev

# 6. Knowledge Graph Refresh
graphify update .
```
