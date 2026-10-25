# PaperPilot — Session Handoff Guide

> **Single Source of Truth** for resuming work on PaperPilot.  
> Start any new agent or human session by reading this document.  
> **Date of Last Update:** October 24, 2026 (Afternoon EDT)  
> **Active Target:** Wave 2 (`v0.7.1`) Core Operations Sprint + Penta-Interface Parity (66 Tools Total)

---

## 🧭 Executive Context & Active In-Flight Sessions

### 1. Active Status Overview
- **Completed & Merged in this Session**:
  - **Task 107A (Continuous Canvas Stitching `[Spec 107]`)**: Merged pure-Rust continuous vertical/horizontal scrollable single-canvas stitching engine (`pdf_stitch_canvas`) with 3/3 unit tests, 75 MCP tools, 14/14 CLI tests passing.
  - **Task 84A (On-Device Document SLM Dataset Generator Engine `[Spec 084]`)**: Merged Python synthetic dataset generation engine (`tools/slm-training`) producing ChatML training pairs across 5 linguistic tiers (direct, casual, legal/finance jargon, typos, compound workflows) for top 15 core tools with 5/5 pytest passing.
  - **Task 108A (PDF Portfolio Attachments Engine `[Spec 108]`)**: Merged pure-Rust ISO 32000-1 `/EmbeddedFiles` Name Tree inspection, binary stream extraction, and FlateDecode attachment injection (`pdf_attachments`) with 5/5 unit tests, 76 MCP tools, 14 CLI tests, and REST Gateway endpoints passing.
  - **Task 107A (Continuous Canvas Stitching `[Spec 107]`)**: Merged pure-Rust continuous vertical/horizontal scrollable single-canvas stitching engine (`pdf_stitch_canvas`) with 3/3 unit tests, 75 MCP tools, 14/14 CLI tests passing.
  - **OpenAPI Synchronization**: Fully audited and updated `docs/api/openapi.json` and `paperpilot-gateway/docs/api/openapi.json` to register omitted endpoints (`/api/v1/pdf/pii-scan`, `/api/v1/pdf/sanitize`, and `/api/v1/pdf/booklet`).
  - **PR #253 (Embed Studio Experience & Portal Parity - Spec 106 / Task 106A)**: Upgraded `apps/embed` to deliver full macOS/Studio window parity (`● ● ●`, categorized functional tabs, search filtering, 2-column studio layout, active canvas, and parameter controls) with dual layout modes (`studio` vs `compact`), updated `/embed-test.html`, automated build synchronization, and 24/24 Playwright tests passing.
  - **PR #251 (Desktop UI libSQL Storage & Vector Search Integration - Spec 103 / Task 103C)**: Integrated `paperpilot-storage` into Tauri desktop app. Implemented IPC commands (`get_jobs_history`, `get_audit_events`, `search_document_vectors`, `store_document_chunks`), added slide-out `<JobQueueDrawer />` with real-time status monitoring, badge counter in `<GlobalCommandBar />`, and semantic vector search toggle in `<WorkspaceSearchModal />`.
  - **PR #250 (Baseline Pre-Sign Tamper Detection - Spec 101 / Task 5.9.90)**: Enhanced `pdf_validate_signatures` with dual-layer forensics (cryptographic PKCS#7 digest verification + content stream / page text baseline comparison) and 10/10 unmocked Penta-Interface E2E coverage.
  - **PR #249 (Web WASM Portal Parity & Categorized Navigation - Spec 104 / Task 104A)**: Expanded Web Worker bridge (`pdfWorker.ts`, `WasmPdfClient`) with dynamic categorized tool navigation (`pages`, `edit`, `optimize`, `security`, `convert`), real-time search, and dedicated parameter panels in `apps/web`.
  - **PR #248 (Embed Widget Tool Parity - Spec 105 / Task 105A)**: Wave 2 tools (`pdf_nup`, `pdf_booklet`, `pdf_scale_pages`, `pdf_auto_rotate`, `pdf_stamp`, `pdf_overlay_underlay`, `pdf_validate_signatures`, `pdf_remove_signatures`), category filtering, parameter controls, and upgraded `/embed-test.html` sandbox with 26/26 Playwright tests passing.
  - **PR #247 (Vector Store & Semantic Search - Spec 103 / Task 103B)**: Pure-Rust cosine similarity vector search and libSQL document chunk storage engine with 7/7 unit tests in `paperpilot-storage`.
  - **PR #245 (`pdf_auto_rotate` - Spec 102 / Task 5.3.10)**: Pure-Rust automated page uprighting engine, validated with 3/3 unit tests and 10/10 unmocked Penta-Interface E2E tests (`reports/PENTA_INTERFACE_E2E_PDF_AUTO_ROTATE_REPORT.md`).
  - **PR #246 (`paperpilot-storage` core - Spec 103 / Task 103A)**: Embedded Turso / libSQL storage engine with connection pooling and automated schema migrations (`jobs`, `webhooks`, `document_chunks`, `audit_logs`).
- **Active Scaffolding**:
  - **Task 84C (Wave 3: Conversions, Forensics & Negative Guards - Task 4.1.11)**: Scheduled to complete the dataset catalog.
- **Workspace Health**: All crates (`paperpilot-storage`, `paperpilot-pdf`, `paperpilot-cli`, `paperpilot-gateway`, `paperpilot-mcp`, `paperpilot-wasm`, `desktop`) compiling cleanly with zero errors.

### 2. Dispatched In-Flight Jules Cloud Sessions (Zero-Conflict Fast Track)
The following sessions are actively executing in the cloud on isolated, non-conflicting branches from `main`:

| Task ID | Spec | Feature Name | Starting Branch | Jules Session Link | Status |
|---|:---:|---|---|---|:---:|
| **Task 108A** | `[Spec 108]` | PDF Attachments & Portfolio Engine (`pdf_attachments`) | `main` | [Session 17693334025127438627](https://jules.google.com/session/17693334025127438627) | ✅ Completed & Merged |
| **Task 84A** | `[Spec 084]` | Synthetic Dataset Generator Engine (Wave 1: Core 15 Tools) | `main` | [Session 17682577633163496827](https://jules.google.com/session/17682577633163496827) | ✅ Completed & Merged |
| **Task 84B** | `[Spec 084]` | Dataset Generator Wave 2: Organization & Layout Tools | `main` | [Session 11055015021161649810](https://jules.google.com/session/11055015021161649810) | ✅ Completed & Merged (PR #259) |
| **Task 109A** | `[Spec 109]` | Invert PDF Colors & Dark Mode (`pdf_invert_colors`) | `main` | [Session 11658717195560231927](https://jules.google.com/session/11658717195560231927) | ✅ Completed & Merged (PR #258) |
| **Task 110A** | `[Spec 110]` | Wave 4 (Office) Unmocked E2E & Real Testing Suite | `main` | [Session 2052459974142158496](https://jules.google.com/session/2052459974142158496) | ✅ Completed & Merged (PR #256) |
| **Task 84C** | `[Spec 084]` | Dataset Generator Wave 3: Conversions, Forensics & Guards | `main` | [Session 11707325928057949102](https://jules.google.com/session/11707325928057949102) | 🚀 Active In-Flight |

---

## 🏆 Recently Merged Milestones (Wave 2 Core Sprint)

1. **Task 106A (Embed Studio Experience & Portal Parity `[Spec 106]`)**:
   - Merged (PR #253).
   - Re-architected `Widget.svelte` into a macOS-style studio frame (`● ● ●`, categorized tabs `📑 Pages`, `🎨 Edit`, `🔒 Security`, search bar, active canvas).
   - Supported dual layout modes: `data-layout="studio"` vs `data-layout="compact"`.
   - Upgraded `apps/web/public/embed-test.html` featuring all 4 demonstration modes.
   - Configured auto-sync build copying `dist/embed.js` to `apps/web/public/embed.js`.
   - 24/24 Playwright tests passing, 0 Svelte diagnostics errors.

2. **Task 103C (Desktop UI libSQL Storage & Async Jobs Drawer `[Spec 103]`)**:
   - Merged (PR #251).
   - Added Tauri IPC commands: `get_jobs_history`, `clear_jobs_history`, `get_audit_events`, `search_document_vectors`, `store_document_chunks` wired to embedded `StorageEngine`.
   - Created `<JobQueueDrawer.svelte>` slide-out tray with status filtering (`All`, `Running`, `Completed`, `Failed`), live progress bars, polling, and history clearing.
   - Added background job indicator and counter button in `<GlobalCommandBar.svelte>`.
   - Added Semantic Vector Search mode toggle and cosine score badges in `<WorkspaceSearchModal.svelte>`.
   - 17/17 tests passing in `desktop`, 8/8 tests passing in `paperpilot-storage`, and Svelte typecheck passing with 0 errors.

3. **Task 5.9.90 (Baseline Pre-Sign Tamper Detection `[Spec 101]`)**:
   - Merged (PR #250).
   - Enhanced `pdf_validate_signatures` with optional `--baseline <PATH>` or `baseline_bytes` audit.
   - Dual-layer forensics: cryptographic PKCS#7 digest verification + content stream / page text baseline comparison.
   - 6/6 unit tests passed, 10/10 (100.0%) unmocked Penta-Interface E2E assertions passed in `reports/PENTA_INTERFACE_E2E_PDF_VALIDATE_SIGNATURES_REPORT.md` across CLI, MCP, REST Gateway, WASM, and Cloudflare Edge.

2. **Task 104A (Web WASM Portal Parity & Categorized Navigation `[Spec 104]`)**:
   - Merged (PR #249).
   - Expanded Web Worker bridge in `apps/web/src/lib/wasm/` (`pdfWorker.ts`, `index.ts`) for all new in-browser WASM tools.
   - Built category-based navigation (`pages`, `edit`, `optimize`, `security`, `convert`) with live search in `OperationsView.svelte`.
   - Added interactive parameter panes for N-Up grid, booklet imposition, dimensional scaling, auto-rotate, stamp, overlay, sanitization, and baseline contract forensics.
   - Verified clean production build (`pnpm --dir apps/web run build`).

2. **Task 105A (Embed Widget Tool Parity & Categorized Sandboxes `[Spec 105]`)**:
   - Merged (PR #248).
   - Added 8 Wave 2 tools to `Widget.svelte` (`pdf_nup`, `pdf_booklet`, `pdf_scale_pages`, `pdf_auto_rotate`, `pdf_stamp`, `pdf_overlay_underlay`, `pdf_validate_signatures`, `pdf_remove_signatures`).
   - Categorized embedding with `category` prop (`pages`, `edit`, `optimize`, `security`, `convert`, `all`).
   - Upgraded `apps/web/public/embed-test.html` showcasing Imposition, Contract Forensics, Classic Page Editing, and Multi-Step Pipelines.
   - 26/26 Playwright test suite passing in `apps/embed`.

2. **Task 103B (Vector Store & Cosine Semantic Search `[Spec 103]`)**:
   - Merged (PR #247).
   - Fast, pure-Rust cosine similarity vector calculation in `paperpilot_storage::vector`.
   - `StorageEngine` methods: `store_chunks`, `search_chunks`, `delete_chunks_by_document`, `count_chunks_by_document`.
   - Zero external embedding API overhead; embeddings stored as JSON strings in embedded libSQL SQLite.
   - Verified with 7/7 unit tests in `paperpilot-storage`.
   - Documented in `wiki/103-Vector-Store-Engine.md` and `apps/docs/src/content/docs/reference/tools/semantic_search.md`.

3. **Task 103A (Embedded libSQL Core Storage Engine & Migrations `[Spec 103]`)**:
   - Merged (PR #246).
   - Dedicated workspace crate `paperpilot-storage` with `libsql 0.10.0`.
   - Embedded zero-setup local SQLite file in `get_app_data_root()` and in-memory execution.
   - Automated DDL schema migrations: `jobs`, `webhooks`, `document_chunks` (vector embeddings), and `audit_logs`.
   - Documented in `wiki/32-Embedded-libSQL-Core-Storage.md`.

2. **Task 5.3.10 (Automated Page Orientation Engine `pdf_auto_rotate` `[Spec 102]`)**:
   - Merged (PR #245).
   - Pure-Rust automated page orientation and uprighting engine (effective aspect-ratio calculation via MediaBox & current Rotate dictionary entries, 90°/180°/270° orientation uprighting, page filtering).
   - 10/10 (100.0%) unmocked Penta-Interface E2E tests passing in `reports/PENTA_INTERFACE_E2E_PDF_AUTO_ROTATE_REPORT.md` across CLI, MCP, REST Gateway, WASM, and Cloudflare Edge.

3. **Task 5.3.8 (Stationery Overlay & Underlay `pdf_overlay_underlay`)**:
   - Merged (PR #244).
   - Pure-Rust stationery and letterhead imposition engine (template form XObject import, underlay/overlay stream ordering, page filtering, opacity control).
   - 10/10 unmocked Penta-Interface E2E tests passing in `reports/PENTA_INTERFACE_E2E_PDF_OVERLAY_UNDERLAY_REPORT.md`.

4. **Task 5.3.3 & 5.3.4 (Dimensional Scaling `pdf_scale_pages` & Image Watermark Stamp `pdf_stamp`)**:
   - Merged (PR #243).
   - Page scaling engine supporting standard formats (A4, Letter, Legal, A3) and percentage factors with centering.
   - Coordinate-exact PNG/JPEG placement, `/Image` insertion, `/ExtGState` opacity control, FlateDecode compression.

5. **Task 5.3.13 (Digital Signature Removal `pdf_remove_signatures` `[Spec 099]`)**:
   - Merged (PR #242).
   - Pure-Rust digital signature removal engine stripping `/AcroForm` signature fields, `/ByteRange` cryptographic locks, and widget annotations.

6. **Task 5.3.2 (Multi-Page N-Up Imposition Engine `pdf_nup` `[Spec 097]`)**:
   - Merged (PR #241).
   - Pure-Rust N-Up grid imposition engine (2, 4, 9, 16 pages per sheet), horizontal/vertical ordering, margins, spacing, and hairline borders.

---

## 🗺️ Architectural Parity: Desktop vs. WASM Tools

The PaperPilot engine maintains strict boundary definitions between client-side WebAssembly execution and native desktop capabilities:

### Available in Both Desktop & In-Browser WASM (41 Tools)
- **Pages & Imposition**: `merge`, `split`, `rotate`, `delete_pages`, `extract_pages`, `reorder_pages`, `crop`, `pdf_nup`, `pdf_booklet`, `pdf_scale_pages`, `pdf_auto_rotate`.
- **Edit & Stationery**: `watermark`, `page_numbers`, `header_footer`, `flatten`, `set_metadata`, `pdf_stamp`, `pdf_overlay_underlay`.
- **Optimization & Inspection**: `compress`, `pdf_hash`, `pdf_info`.
- **Security & Forensics**: `encrypt`, `decrypt`, `sanitize`, `pii_scan`, `pdf_validate_signatures`, `pdf_remove_signatures`.
- **Conversions**: `pdf_to_docx`, `images_to_pdf`, `extract_images`, `extract_text`, `render_page`, `json_to_pdf`, `ocr` (`ocrs` via pure-Rust neural runtime).

### Desktop-Only Tools (15 Tools Requiring Native OS / Filesystem Access)
- **Office Processing**: `office_modernize_legacy` (`.doc`/`.xls`/`.ppt`), `excel_to_csv`, `excel_to_json`, `office_to_markdown`, `office_extract_text`.
- **Heavy Document Converters**: `pdf_to_xlsx`, `pdf_extract_tables`, `pdf_to_pptx`, `pdf_to_pdf_a` (requires VeraPDF & ICC profiles).
- **Persistent AI & Storage**: `search` (persistent cross-document vector search via `paperpilot-storage`), `classify_type` (local SLM weights in AppData).
- **Network / External Subprocesses**: `pdf_timestamp` (outbound RFC 3161 TSA HTTP requests blocked by browser CORS), `repair` (deep recovery via native binary handles).

---

## 📋 Resuming Checklist for Next Session

When starting the next session, perform the following sequence:

1. **Check Status of In-Flight Jules Sessions**:
   - Run `git fetch origin` and check for merged PRs or incoming branches:
     - `Task 103B`: libSQL Vector Store & Semantic Search.
     - `Task 5.9.90`: Baseline Tamper Forensics (`pdf_validate_signatures --baseline`).
     - `Task 104A`: Web WASM Portal Parity & Categorized Navigation.
     - `Task 105A`: Embed Widget Parity & `/embed-test` Sandbox Upgrade.
   - For completed sessions, validate unit tests and Penta-Interface E2E suite, then merge into `main`.

2. **Next Operations to Pick After In-Flight Tasks**:
   - **Continuous Canvas Stitch (`pdf_stitch_canvas`, Task 5.3.11)**: Pure-Rust vertical and horizontal multi-page continuous canvas stitch. (Already pre-scaffolded across all 5 surfaces).
   - **PDF Attachments & Portfolios (`pdf_attachments`, Task 5.3.12)**: Embedded `/EmbeddedFiles` trees inspection, extraction, and embedding.
   - **Enterprise Async Processing (`Task 103C` / Section 6.8.1)**: Asynchronous job worker integration using `paperpilot-storage` for background gateway tasks.

---

## ⚖️ Rules of Engagement & System Constraints

1. **System Resource Constraint (`-j 2` is MANDATORY on local machine):**
   - **ALWAYS** pass `-j 2` for all local `cargo test`, `cargo build`, or `cargo clippy` commands (e.g. `cargo test -p paperpilot-storage -j 2`).
   - Unbounded cargo compilations will freeze the host machine.

2. **The Single Core Runner + 5 Wrappers Architecture Pattern:**
   - Every tool operation MUST have exactly **ONE** core in-memory engine implementation in `paperpilot-pdf`:
     ```rust
     pub fn run_operation_bytes(input_bytes: &[u8], config: &Config) -> Result<Vec<u8>, OperationError>
     ```
   - 5 Thin Interface Wrappers: CLI (`paperpilot-cli`), MCP (`paperpilot-mcp`), REST Gateway (`paperpilot-gateway`), WASM Engine (`paperpilot-wasm`), and Desktop/Embed (`apps/desktop`, `apps/embed`).
   - **Zero Intermediate Disk I/O**: Pipeline steps chain memory buffers in RAM without temporary disk roundtrips.

3. **Workspace Safety Rules:**
   - **NEVER** execute `git reset --hard`, `git checkout -- .`, `git clean -fd`, or `git restore`.
   - Zero-Mock Policy: All tests must use real disk fixtures and real in-memory byte buffers (`fixtures/`, `tests/e2e_fixtures/real/`).

---

## 🛠️ Quick Command Reference

```bash
# 1. Dev Server (Desktop Tauri + Svelte 5)
make dev

# 2. Rust Unit Tests (Always use -j 2)
cargo test -p paperpilot-storage -j 2
cargo test -p paperpilot-pdf --lib -j 2
cargo test -p paperpilot-mcp --lib -j 2
cargo test -p paperpilot-gateway --lib -j 2

# 3. Penta-Interface Real E2E Test Suite
cargo run -p penta-interface-e2e -- --group all

# 4. Web Portal & Embed Build / Check
pnpm --dir apps/web run build
pnpm --dir apps/embed run build
pnpm --dir apps/desktop run check

# 5. Mobile Android Build (Dynamic SDK/NDK detection)
make build-android
make install-android
make log-android

# 6. Dispatch Jules Tasks via CLI
cargo run -p jules-submit -- --file prompts/tasks/phase6/<prompt-file>.txt
```
