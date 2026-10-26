# PaperPilot — Session Handoff Guide

> **Single Source of Truth** for resuming work on PaperPilot.  
> Start any new agent or human session by reading this document.  
> **Date of Last Update:** October 26, 2026 (Evening EDT)  
> **Active Target:** Post-Launch Security Hardening + Wave 3 & 4 Core Operations + Mobile/Web Parity (78+ Tools Total)

---

## 🧭 Executive Summary & Current Repository State

- **Current Git Branch**: `feature/dev` (strictly synchronized with `origin/feature/dev`).
- **Target Branch Protocol**: Always merge and push to `feature/dev`. Never push unreviewed code directly to `main`.
- **System Stability & Audits**:
  - `cargo check --workspace -j 2`: **100% Clean**.
  - Local security audit (`make audit` running Semgrep, Trivy, and Cargo-Audit): **0 blocking vulnerabilities**.
  - MCP Tool Count: **78 tools registered and verified** (`paperpilot-mcp`).
  - Desktop Vitest Suite: **111/111 passing tests** (`pnpm --dir apps/desktop test`).

---

## 🚀 Active Dispatch & In-Flight Status

| Task ID | Spec | Priority | Component | Target Branch | Status & Details |
|---|:---:|:---:|---|---|---|
| **Task 123A** | `[Spec 123]` | P0 | Security / Tauri / Core | `feature/dev` | 🔄 **In-Flight in Jules**: [Session 18198933508777026291](https://jules.google.com/session/18198933508777026291)<br>Hardening `pack_commands.rs`, `url_fetch.rs` (SSRF IP validation), and `tauri.conf.json` (strict CSP). |
| **Task 124A** | `[Spec 124]` | P1 | Web / UI Mobile | `feature/dev` | 🎯 **Ready for Immediate Dispatch** (`specs/124-...`)<br>Mobile web playground overhaul (`apps/web/src/components/HeroPlayground.svelte`): responsive bottom sheet / drawer, >=44px touch targets. |
| **Task 122A** | `[Spec 122]` | P1 | Core / Optimizer | `feature/dev` | 🎯 **Ready for Immediate Dispatch** (`specs/122-...`)<br>Hash-based indirect stream deduplication (`pdf_dedup_streams`) ported from `SimonBure/presse` with CMYK safety. |
| **Task 128A** | `[Spec 128]` | P1 | CI / Security | `feature/dev` | 🎯 **Ready for Immediate Dispatch** (`specs/128-...`)<br>Pin all third-party GitHub Actions across `.github/workflows/` to immutable 40-char commit SHAs. |
| **Task 125A** | `[Spec 125]` | P2 | Web / Standalone | `feature/dev` | 📋 **Backlog / Planned** (`specs/125-...`)<br>Air-gapped single-file `paperpilot-portable.html` distribution with inline base64 WASM bundle. |
| **Task 126A** | `[Spec 126]` | P2 | Gateway / Ingestion | `feature/dev` | 📋 **Backlog / Planned** (`specs/126-...`)<br>Sub-15ms Tika-compatible `/tika/rmeta/text` REST & MCP tool for Open-WebUI integration. |
| **Task 127A** | `[Spec 127]` | P2 | Core / Format Conversion | `feature/dev` | 📋 **Backlog / Planned** (`specs/127-...`)<br>PDF-to-EPUB semantic engine via structured intermediate AST. |
| **Task 129A** | `[Spec 129]` | P2 | Quality / Fuzzing | `feature/dev` | 📋 **Backlog / Planned** (`specs/129-...`)<br>Malformed PDF fuzzing harness (circular xrefs, corrupted streams, memory safety). |

---

## 🏆 Recently Merged & Completed

1. **Task 119A (Multi-Sheet Poster Tile Slicer `[Spec 119]`, PR #270, Task 5.9.89)**:
   - Slices oversized or scaled documents across standard printable grid sheets (A4, Letter) with glue gutters (`overlap_pt`) and cut trim marks.
   - Cleanly resolved merge conflicts across CLI, MCP (78 tools total), Gateway, and `operationArgsBuilder.ts` (Desktop). Merged to `feature/dev`.
2. **Task 123B (Gateway Security Lockdown & Automated Security Gates `[Spec 123]`)**:
   - Enforced localhost/app-origin CORS restrictions and 100MB body limit (`DefaultBodyLimit::max(100MB)`).
   - Local `make audit` and `.github/workflows/security.yml` integrated.
3. **Task 121A (Mobile Touch Gestures & Viewport Panning `[Spec 121]`, PR #269, Task 5.9.90)**:
   - Viewport double-tap zoom, 2D panning, and swipe page navigation in `PdfPreview.svelte`.

---

## 📋 Resuming Checklist for Next Session

Follow this exact sequence to resume work:

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

# 5. Knowledge Graph Refresh
graphify update .
```
