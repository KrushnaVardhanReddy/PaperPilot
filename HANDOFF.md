# PaperPilot — Session Handoff Guide

> **Single Source of Truth** for resuming work on PaperPilot.  
> Start any new agent or human session by reading this document.  
> **Date of Last Update:** October 25, 2026 (Evening EDT)  
> **Active Target:** Wave 3 & 4 Core Operations + Web/SLM Parity + Continuous Tooling (75+ Tools Total)

---

## 🧭 Executive Context & Active In-Flight Sessions

### 1. Active Status Overview
- **Completed & Merged in this Session**:
  - **Task 84C (Wave 3 SLM Dataset Generator `[Spec 084]`, PR #260)**: Expanded `tools/slm-training` to 45 tools and 10% negative guardrails for ChatML finetuning, 6/6 tests passing.
  - **Task 113A (Web Portal Mobile Handoff QR Code & Share Modal `[Spec 113]`, Task 5.9.82)**: Built `<ShareModal />` with 1-click URL copy, native `navigator.share`, and verified static SVG asset (`apps/web/public/qr_code.svg`).
  - **Task 114A (Web Portal Dynamic Releases Download Counter Badge `[Spec 114]`, Task 5.9.83)**: Built `<DownloadBadge />` with 15-minute `sessionStorage` caching, summing releases assets dynamically. Configured behind feature flag `ENABLE_DOWNLOAD_COUNT_BADGE = true` with auto-threshold `MIN_DOWNLOAD_COUNT_THRESHOLD = 1000` in `apps/web/src/constants.ts`.
  - **Task 110A (Wave 4 Office Modernization Real E2E Tests `[Spec 110]`, PR #256)**: Merged unmocked tests in `paperpilot-mcp/tests/wave4_mcp_tests.rs` and documented in `wiki/33-Wave-4-Office-Penta-Testing.md`.
  - **Task 109A (Invert PDF Colors & Dark Mode `[Spec 109]`, PR #258)**: Merged pure-Rust color inversion engine (`pdf_invert_colors`) across all 5 surfaces.
  - **Task 84B (Wave 2 SLM Dataset Generator `[Spec 084]`, PR #259)**: Expanded SLM generator to 30 layout and organization tools.
  - **Task 107A (Continuous Canvas Stitching `[Spec 107]`, PR #257)**: Merged continuous vertical/horizontal canvas stitching engine (`pdf_stitch_canvas`) across all surfaces.
  - **Task 108A (PDF Portfolio Attachments Engine `[Spec 108]`)**: Merged pure-Rust ISO 32000-1 `/EmbeddedFiles` Name Tree engine (`pdf_attachments`).
  - **SEO & Schema.org Enhancement**: Injected JSON-LD microdata (`SoftwareApplication`, `ItemList`) and OpenGraph/Twitter Card metadata into `apps/web/index.html`.
  - **Competitive Landscape Audit & Phase 5.3 Gap Backlog**: Added 5 competitive operations (`docx_to_pdf`, `epub_to_pdf`, `pdf_merge_alternate`, `pdf_split_by_outline`, `pdf_posterize`) to `PHASES.md` and `ROADMAP.md`.
- **Workspace Health**: All crates (`paperpilot-storage`, `paperpilot-pdf`, `paperpilot-cli`, `paperpilot-gateway`, `paperpilot-mcp`, `paperpilot-wasm`, `desktop`, `web`) compiling cleanly with zero errors.

### 2. Dispatched In-Flight Jules Cloud Sessions (Zero-Conflict Fast Track)
The following sessions are actively executing in the cloud on isolated, non-conflicting branches from `main`:

| Task ID | Spec | Feature Name | Starting Branch | Jules Session Link | Status |
|---|:---:|---|---|---|:---:|
| **Task 84C** | `[Spec 084]` | Dataset Generator Wave 3: Conversions, Forensics & Guards | `main` | [Session 11707325928057949102](https://jules.google.com/session/11707325928057949102) | ✅ Completed & Merged (PR #260) |
| **Task 111A** | `[Spec 111]` | Modify PDF Permissions (`pdf_change_permissions`) | `main` | [Session 8073581921523239798](https://jules.google.com/session/8073581921523239798) | 🚀 Active In-Flight |
| **Task 112A** | `[Spec 112]` | Scanned Dual-Page Book Center Split (`pdf_split_scanned_book`) | `main` | [Session 2005125894355067826](https://jules.google.com/session/2005125894355067826) | 🚀 Active In-Flight |

---

## 🏆 Recently Merged Milestones

1. **Task 84C (Wave 3 SLM Dataset Generator `[Spec 084]`, PR #260)**:
   - Expanded `tools/slm-training/tool_registry.py` and `templates.py` to **45 tools**.
   - Added 10% negative guardrail refusal prompts for out-of-scope/safety queries.
   - 6/6 tests passing in pytest (`test_dataset_generator.py`).

2. **Task 113A (Web Portal Mobile Handoff QR Code & Share Modal `[Spec 113]`, Task 5.9.82)**:
   - Added `<ShareModal />` in web header.
   - Clean, verified SVG QR code asset (`apps/web/public/qr_code.svg`) for instant mobile handoff.
   - 1-click clipboard copy + native `navigator.share` fallback.

3. **Task 114A (Web Portal Dynamic Releases Download Counter Badge `[Spec 114]`, Task 5.9.83)**:
   - Added `<DownloadBadge />` in `apps/web/src/components/DownloadBadge.svelte`.
   - Aggregates live asset downloads from GitHub Releases API with 15-minute `sessionStorage` caching.
   - Guarded by `ENABLE_DOWNLOAD_COUNT_BADGE = true` and `MIN_DOWNLOAD_COUNT_THRESHOLD = 1000` to prevent negative social proof before reaching 1,000 downloads.

4. **Task 109A (Invert PDF Colors / Dark Mode `[Spec 109]`, PR #258)**:
   - Pure-Rust `/ExtGState` `Difference` inversion and text luminance transformation.
   - Penta-interface parity across Core, CLI, MCP, Gateway, and WASM.

5. **Task 110A (Wave 4 Office Penta Testing Suite `[Spec 110]`, PR #256)**:
   - Real, unmocked tests covering `pdf_to_docx`, `pdf_to_xlsx`, `pdf_to_pptx`, `office_to_markdown`, `office_extract_text`, `office_modernize_legacy`.
   - Magic byte verification (`PK\x03\x04`, `\xD0\xCF\x11\xE0`).

---

## 📋 Resuming Checklist for Next Session

When starting the next session, perform the following sequence:

1. **Check Status of In-Flight Jules Sessions**:
   - Run `git fetch origin` and check for PRs on:
     - **Task 111A (`pdf_change_permissions`)**: [Session 8073581921523239798](https://jules.google.com/session/8073581921523239798)
     - **Task 112A (`pdf_split_scanned_book`)**: [Session 2005125894355067826](https://jules.google.com/session/2005125894355067826)
   - When PRs open:
     - Check out branch, test with `cargo test -p paperpilot-pdf -j 2`, merge to `main`, and push.
     - Update `PHASES.md` and `HANDOFF.md`.
     - Run `graphify update .`.

2. **Next Backlog Features to Dispatch or Implement**:
   - **Task 5.3.22 (`docx_to_pdf` / `office_to_pdf`)**: Direct pure-Rust/WASM document rendering to PDF.
   - **Task 5.3.23 (`epub_to_pdf`)**: EPUB e-book rendering and pagination into standard PDF.
   - **Task 5.3.24 (`pdf_merge_alternate`)**: Alternating collation for duplex scanner splits.
   - **Task 5.3.25 (`pdf_split_by_outline`)**: Bookmark/outline-based chapter splitting.
   - **Task 5.3.26 (`pdf_posterize`)**: Multi-sheet poster tile slicing with overlap margins.

---

## ⚖️ Rules of Engagement & System Constraints

1. **System Resource Constraint (`-j 2` is MANDATORY on local machine):**
   - **ALWAYS** pass `-j 2` for all local `cargo test`, `cargo build`, or `cargo clippy` commands (e.g. `cargo test -p paperpilot-pdf -j 2`).
   - Unbounded cargo compilations will freeze the host machine.

2. **The Single Core Runner + 5 Wrappers Architecture Pattern:**
   - Every tool operation MUST have exactly **ONE** core in-memory engine implementation in `paperpilot-pdf`:
     ```rust
     pub fn run_operation_bytes(input_bytes: &[u8], config: &Config) -> Result<Vec<u8>, OperationError>
     ```
   - 5 Thin Interface Wrappers: CLI (`paperpilot-cli`), MCP (`paperpilot-mcp`), REST Gateway (`paperpilot-gateway`), WASM Engine (`paperpilot-wasm`), and Desktop/Embed (`apps/desktop`, `apps/embed`).

3. **Workspace Safety Rules:**
   - **NEVER** execute `git reset --hard`, `git checkout -- .`, `git clean -fd`, or `git restore`.
   - Zero-Mock Policy: All tests must use real disk fixtures and real in-memory byte buffers (`fixtures/`, `tests/e2e_fixtures/real/`).

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
