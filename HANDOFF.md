# PaperPilot — Session Handoff Guide

> **Single Source of Truth** for resuming work on PaperPilot.  
> Start any new agent or human session by reading this document.  
> **Date of Last Update:** October 25, 2026 (Night EDT)  
> **Active Target:** Wave 3 & 4 Core Operations + Web/SLM Parity + Continuous Tooling (75+ Tools Total)

---

## 🧭 Executive Context & Active In-Flight Sessions

### 1. Active In-Flight Jules Cloud Sessions (Zero-Conflict Fast Track)
The following 2 cloud sessions are actively executing in parallel on isolated branches from `main`:

| Task ID | Spec | Feature Name | Starting Branch | Jules Session Link | Status |
|---|:---:|---|---|---|:---:|
| **Task 117A** | `[Spec 117]` | Interleave / Alternate PDF Collation (`pdf_merge_alternate`) | `main` | [Session 17612337481863412417](https://jules.google.com/session/17612337481863412417) | ✅ Merged into `feature/dev` (PR #268) |
| **Task 118A** | `[Spec 118]` | Split PDF by Outline / Bookmarks (`pdf_split_by_outline`) | `main` | [Session 6774330169288969238](https://jules.google.com/session/6774330169288969238) | ✅ Merged (PR #264) |
| **Task 119A** | `[Spec 119]` | Multi-Sheet Poster Tile Slicer (`pdf_posterize`) | `feature/dev` | [Session 14717167755470526744](https://jules.google.com/session/14717167755470526744) | 🚀 In Progress (Targeting `feature/dev`) |

---

## 🏆 Current Session Highlights

1. **Task 117A (Interleave / Alternate PDF Collation `[Spec 117]`, PR #268, Task 5.9.87)**:
   - Validated and merged into `feature/dev`. Pure-Rust alternating two-document PDF page collator.
   - Supports configurable step cadences (`step`, `step_b`) and page reversal (`reverse_a`, `reverse_b`).
   - Svelte tool definition added to `apps/desktop`, documentation in `wiki/117-Pdf-Merge-Alternate.md`, and 106/106 Vitest tests passing.

2. **Task 119A (Multi-Sheet Poster Tile Slicing Engine `[Spec 119]`, Task 5.9.89)**:
   - Dispatched to Jules ([Session 14717167755470526744](https://jules.google.com/session/14717167755470526744)) targeting base branch `feature/dev`.
   - Penta-scaffolding and core unit tests (`test_pdf_posterize_2x2_grid`) verified on `feature/dev`.
   - Fixed generator warning templates in `tools/penta-scaffold/src/generator.rs`.

3. **Task 118A (Split PDF by Outline / Bookmarks `[Spec 118]`, PR #264, Task 5.9.88)**:
   - Implemented `pdf_split_by_outline` across Core, CLI, MCP Server, Gateway REST, WASM, and Edge.
   - Parses document `/Outlines` / `/First` / `/Next` hierarchy recursively with configurable `max_depth` and filename sanitization.
   - Emits a ZIP archive containing segmented PDFs corresponding to each bookmark/TOC chapter.
   - Added Svelte tool definition in `apps/desktop` and documentation in `apps/docs` and `wiki/118-Pdf-Split-By-Outline.md`.
   - All 4 unit tests + 21 MCP tests + 105 Desktop Vitest unit tests passing 100%.

2. **Task 116A (Actionable RAG Workspace & Bi-Directional Citation Canvas `[Spec 116]`, PR #263, Task 4.3.9)**:
   - Wired bidirectional linkage between `PdfChatPanel.svelte` citations and the interactive canvas in `PdfViewer.svelte`.
   - Assistant citations (`[Page N]`, `[p. N]`) render as interactive badges (`📄 p. N`); clicking dispatches `paperpilot:jump-to-citation`.
   - `PdfViewer.svelte` listens for event, navigates `pageNum`, and renders a glowing pulse bounding box overlay (`.citation-pulse`) for 2.5s.
   - Svelte check & all 104 Vitest unit tests in `apps/desktop` passing 100%.

2. **Task 111A (Modify PDF Permissions `[Spec 111]`, PR #262, Task 5.9.86)**:
   - Added standard ISO 32000-1 `/P` permissions bitflags via `lopdf::EncryptionState` and `EncryptionVersion::V2`.
   - Supports permission toggles for printing, copying, modifications, annotations, form filling, and accessibility across Core, CLI, MCP, and Gateway.
   - 3/3 unittests passed.

3. **Task 112A (Scanned Dual-Page Book Center Spine Split `[Spec 112]`, PR #261, Task 5.9.85)**:
   - Pure-Rust dual-page center spine splitting engine supporting horizontal/vertical splits, configurable gutter overlap margins, and LTR/RTL page ordering.
   - Tested across Core, CLI, Gateway, and MCP (`wave4_mcp_tests.rs`), 5/5 unit tests passed.

4. **Task 115A (Web Portal Use Cases Showcase & Scenario Cards `[Spec 115]`, Task 5.9.84)**:
   - Added responsive `<UseCasesShowcase />` (`apps/web/src/components/UseCasesShowcase.svelte`) directly under hero playground.
   - 6 workflow personas: Legal/Compliance, Healthcare/HIPAA, Finance/Operations, Academic/Research, Print/Prepress, Developers/AI Agents.
   - Interactive deep-links: Click CTA to auto-select tool via `window.dispatchEvent('select-tool')` and smooth scroll to playground.
   - Pure Svelte 5 runes (`$state`, `$derived`), zero NPM dependencies, dark glassmorphism styling.

5. **Task 114A (Web Portal Dynamic Releases Download Counter Badge `[Spec 114]`, Task 5.9.83)**:
   - Built `<DownloadBadge />` with 15-minute `sessionStorage` caching, querying releases assets dynamically.
   - Configured behind feature flag `ENABLE_DOWNLOAD_COUNT_BADGE = true` with auto-threshold `MIN_DOWNLOAD_COUNT_THRESHOLD = 1000` in `apps/web/src/constants.ts`.

6. **Task 113A (Web Portal Mobile Handoff QR Code & Share Modal `[Spec 113]`, Task 5.9.82)**:
   - Built `<ShareModal />` with 1-click URL copy, native `navigator.share`, and verified static SVG asset (`apps/web/public/qr_code.svg`).

7. **Task 84C (Wave 3 SLM Dataset Generator `[Spec 084]`, PR #260)**:
   - Expanded `tools/slm-training` to 45 tools and 10% negative guardrails for ChatML finetuning, 6/6 tests passing.

---

## 📋 Resuming Checklist for Next Session

When starting the next session, perform the following sequence:

1. **Check Status of Active In-Flight Jules Sessions**:
   - Run `git fetch origin` and check for PRs on:
     - **Task 117A (`pdf_merge_alternate`)**: [Session 17612337481863412417](https://jules.google.com/session/17612337481863412417)
     - **Task 118A (`pdf_split_by_outline`)**: [Session 6774330169288969238](https://jules.google.com/session/6774330169288969238)
   - When PRs open:
     - Check out branch, test with `cargo test -p paperpilot-pdf -j 2`, merge to `main`, and push.
     - Update `PHASES.md` and `HANDOFF.md`.
     - Run `graphify update .`.

2. **Next Backlog Features to Dispatch or Implement**:
   - **Task 5.3.26 (`pdf_posterize`)**: Multi-sheet poster tile slicing with overlap margins.
   - **Task 5.3.22 (`docx_to_pdf` / `office_to_pdf`)**: Direct pure-Rust/WASM document rendering to PDF.
   - **Task 5.3.23 (`epub_to_pdf`)**: EPUB e-book rendering and pagination into standard PDF.

---

## ⚖️ Rules of Engagement & Production Constraints (LIVE PRODUCT)

> ⚠️ **CRITICAL**: PaperPilot is now a live, trending product on Hacker News with active users downloading binaries and running the web sandbox. All development must maintain production stability.

1. **System Resource Constraint (`-j 2` is MANDATORY on local machine):**
   - **ALWAYS** pass `-j 2` for all local `cargo test`, `cargo build`, or `cargo clippy` commands (e.g. `cargo test -p paperpilot-pdf -j 2`).
   - Unbounded cargo compilations will freeze the host machine.

2. **The Single Core Runner + 5 Wrappers Architecture Pattern:**
   - Every tool operation MUST have exactly **ONE** core in-memory engine implementation in `paperpilot-pdf`:
     ```rust
     pub fn run_operation_bytes(input_bytes: &[u8], config: &Config) -> Result<Vec<u8>, OperationError>
     ```
   - 5 Thin Interface Wrappers: CLI (`paperpilot-cli`), MCP (`paperpilot-mcp`), REST Gateway (`paperpilot-gateway`), WASM Engine (`paperpilot-wasm`), and Desktop/Embed (`apps/desktop`, `apps/embed`).

3. **Zero-Mock Policy on Tests:**
   - Real users use malformed PDFs and varied environments. All test fixtures must be real disk fixtures or synthesized byte buffers (`fixtures/`, `tests/e2e_fixtures/real/`). Never mock PDF bytes or operation responses.

4. **Zero-Cloud & Absolute Privacy Guarantee:**
   - 0 bytes uploaded to external servers. Never introduce unexpected analytics beacons, telemetry, or external network requests into Core, CLI, MCP, Desktop, or WASM.

5. **Mandatory Pre-Push Verification Matrix:**
   - Run before committing to `main`:
     1. `cargo check --workspace -j 2`
     2. `cargo test -p paperpilot-pdf -j 2`
     3. `cargo test -p paperpilot-mcp -j 2`
     4. `pnpm --dir apps/desktop test` (105/105 passing)

6. **Workspace Safety Rules:**
   - **NEVER** execute `git reset --hard`, `git checkout -- .`, `git clean -fd`, or `git restore`.

---

## 🛠️ Quick Command Reference

```bash
# 1. Dev Server (Web Portal)
cd apps/web && npm run dev

# 2. Rust Unit Tests (Always use -j 2)
cargo test -p paperpilot-pdf --lib -j 2
cargo test -p paperpilot-mcp --lib -j 2
cargo test -p paperpilot-storage -j 2

# 3. SLM Dataset Generator Tests
pytest tools/slm-training/test_dataset_generator.py -v

# 4. Web Portal Build & Quality Check
pnpm --dir apps/web run build

# 5. Knowledge Graph Refresh
graphify update .
```
