# PaperPilot — Session Handoff Guide

> **Single Source of Truth** for resuming work on PaperPilot.  
> Start any new agent or human session by reading this document.  
> **Date of Last Update:** October 23, 2026 (Night EDT)  
> **Active Target:** Wave 2 (`v0.7.1`) Core Operations Sprint + Penta-Interface Parity (66 Tools Total)

---

## 🧭 Executive Context & Active In-Flight Sessions

### 1. Active Status & Nightly Dispatch
- **Merged PR #242 (`pdf_remove_signatures` - Spec 099)**: Fully merged to `main`, validated with live real-fixture CLI execution and 3/3 unit tests passing, 0 stray files.
- **Merged PR #241 (`pdf_nup` - Spec 097)**: Fully merged to `main`, clean verified with 3/3 tests passing.
- **Batch Scaffolding (11 Tools)**: Pre-scaffolded across all 5 surfaces (CLI, MCP, Gateway, WASM, Desktop Svelte, E2E harness).
- **Tool Registry**: Scaled from 55 to **66 tools** registered in `paperpilot-cli`!

### 2. Dispatched Overnight Jules Sessions (Zero-Conflict Fast Track)
| Task ID | Feature Name | Starting Branch | Jules Session Link | Status |
|---|---|---|---|---|
| **Task 5.3.3** | Page Dimensional Scaling (`pdf_scale_pages` `[Spec 098]`) | `main` | [Session 11505281124052234531](https://jules.google.com/session/11505281124052234531) | ✅ Merged (PR #243) |
| **Task 5.3.4** | Image & Logo Stamp (`pdf_stamp`) | `main` | [Session 8384172369592473139](https://jules.google.com/session/8384172369592473139) | ✅ Merged (PR #243 / #244 prep) |
| **Task 5.3.8** | Overlay & Underlay Stationary (`pdf_overlay_underlay`) | `main` | [Session 16199328015910459749](https://jules.google.com/session/16199328015910459749) | ✅ Merged (PR #244) |
| **Task 5.3.13** | Digital Signature Removal (`pdf_remove_signatures` `[Spec 099]`) | `main` | [Session 1595313133805116906](https://jules.google.com/session/1595313133805116906) | ✅ Merged (PR #242) |
| **Task 5.9.90** | Baseline Tamper Forensics (`pdf_validate_signatures --baseline` `[Spec 101]`) | `main` | [Session 9563501665020719300](https://jules.google.com/session/9563501665020719300) | 🚀 Active In-Flight |

### 3. Immediate Next Roadmap Target (Selected: Option A + Strategy Option 1)
**Enterprise Asynchronous Processing, Webhooks & Embedded Database Engine (Section 6.8 / Wave 7)**:
- **Repo Strategy Decision (Option 1):** Develop modularly within this Cargo workspace first (`paperpilot-enterprise` crate / `--features enterprise`). This gives maximum development velocity and zero cross-repo dependency friction during rapid development, while keeping a clean boundary that can be extracted to `PaperPilot-Enterprise` in 15 minutes prior to commercial release.
- **Phase 1 (6.8.3):** Embedded **Turso / libSQL** (`libsql` crate) database integration with zero-setup local SQLite file in `get_app_data_root()` and automatic schema migrations (`jobs`, `webhooks`, `audit_logs`).
- **Phase 2 (6.8.1):** Asynchronous Job Queue & HTTP Callback Engine (`POST /api/v1/jobs/async` returning `202 Accepted`, HMAC-SHA256 signature verification, startup recovery query for interrupted jobs).
- **Phase 3 (6.8.2):** Outbound Webhook Subscriptions (`diff.completed`, `ocr.finished`, `signature.verified`) with retry queues & exponential backoff.

---

## 🏆 Recently Completed Milestones

1. **Task 5.3.2 (Multi-Page N-Up Imposition Engine `pdf_nup` `[Spec 097]`)**:
   - Merged (PR #241, commit `5f37009`).
   - Pure-Rust N-Up grid imposition engine (2, 4, 9, 16 pages per sheet), horizontal/vertical ordering, margins, spacing, and hairline borders.
   - Verified across Desktop UI controls, CLI, and Penta-Interface E2E harness.
2. **Batch Pre-Scaffolding for 11 Operations Sprint Tools**:
   - `pdf_scale_pages` (Spec 098), `pdf_remove_signatures` (Spec 099), `pdf_stamp`, `pdf_overlay_underlay`, `pdf_auto_rotate`, `pdf_stitch_canvas`, `pdf_attachments`, `pdf_timestamp`, `pdf_change_permissions`, `pdf_split_scanned_book`, `pdf_comic_archive`.
   - Wired in `paperpilot-pdf`, `paperpilot-cli`, `paperpilot-gateway`, `paperpilot-mcp`, `paperpilot-wasm`, `tools/penta-interface-e2e`, and `apps/desktop`.
   - All 104 desktop tests and workspace `cargo check` passing cleanly.
   - Merged (PR #240, commit `5ca9e5c`).
   - Automatically injects desktop `operationArgsBuilder.ts` match arms and unit tests on tool scaffolding.
   - Retroactively wired `pdf_validate_signatures`, `document_pii_scan`, and `pdf_extract_tables`.
   - Built `<AuditReportViewer />` component in Desktop UI to automatically render structured findings, copy JSON, and inspect reports for all `.JSON` tools.
   - All 92/92 Vitest unit tests and `penta-scaffold` CLI tests passing cleanly.
2. **Task 5.3.1 (Booklet Imposition Engine `pdf_booklet` `[Spec 096]`)**:
   - Pure-Rust booklet imposition algorithm calculating duplex page sequences (`[N, 1, 2, N-1, ...]`) with automatic page padding $N \pmod 4 = 0$.
   - 10/10 (100.0%) unmocked Penta-Interface E2E assertions passing in `reports/PENTA_INTERFACE_E2E_PDF_BOOKLET_REPORT.md` across CLI, MCP, REST Gateway, WASM, and Cloudflare Edge.
3. **Task 5.9.61 (Cryptographic Digital Signature Verification Engine `pdf_validate_signatures` `[Spec 095]`)**:
   - Merged (PR #238, commit `a6e68c6`).
   - Pure-Rust digital signature extraction & `/ByteRange` verification engine traversing `/AcroForm` and page widgets.
   - Detects signer identity, signing timestamp, PKCS#7 / CMS subfilters (`adbe.pkcs7.detached`), byte boundaries, and post-signing tampering (extra bytes, structural changes).
   - 10/10 (100.0%) unmocked real Penta-Interface E2E tests passing in `reports/PENTA_INTERFACE_E2E_PDF_VALIDATE_SIGNATURES_REPORT.md`.
2. **Task 5.9.79 (Pure-Rust Tool Scaffolding Generator `penta-scaffold` `[Spec 094]`)**:
   - Merged (commit `5fd9155` / `e411d41` & bugfix `dc0c089`).
   - CLI generator creating 1-click zero-boilerplate wiring across all 5 surfaces (Core Rust, CLI, MCP, REST Gateway, WASM, Desktop UI, E2E harness).
3. **Task 5.9.78 (Universal Remote PDF URL Ingestion `url_fetch` `[Spec 093]`)**:
   - Merged (commit `d826eb4` / `6e3a019`).
   - Native pure-Rust streaming URL fetcher via `reqwest`.
4. **Task 5.9.60 (PII & Sensitive Information Scanner `document_pii_scan` `[Spec 086]`)**:
   - Merged (PR #232).
   - Pure-Rust automated regex detector & validator for SSNs, credit cards (Luhn algorithm validated), emails, phone numbers, and IBANs (MOD 97-10 check validated).
   - Single Core Runner in `paperpilot-pdf/src/operations/pii.rs` (`scan_pii_bytes`).
   - 5 thin wrappers across CLI (`paperpilot pii-scan`), MCP (`document_pii_scan`), REST Gateway (`/api/v1/pdf/document-pii-scan`), WASM (`WasmPdfEngine.document_pii_scan`), and Cloudflare Edge (`/api/v1/pii-scan`).
   - 10/10 (100.0%) unmocked Penta-Interface E2E assertions passing in `reports/PENTA_INTERFACE_E2E_PII_SCAN_REPORT.md`.
5. **Task 5.9.67 (Mobile Touch-First WASM Engine & Embed Demonstration `[Spec 087]`)**:
   - Merged (PR #231, commit `024d957`).
   - Deprecated and removed bloated `process_mobile_buffer` Tauri bridge from `apps/desktop/src-tauri/src/mobile_bridge.rs`.
   - Wired `OperationsPanel.svelte` and `CameraScannerModal.svelte` to execute directly via in-memory `WasmPdfClientImpl` and `WasmPdfEngine.images_to_pdf`.
   - Enhanced `Widget.svelte` with mobile touch drawer (<640px bottom-sheet, $\ge$48px touch targets, `safe-area-inset-bottom` protection).
   - Created `apps/embed/playwright.config.ts` and verified 26/26 passing Playwright mobile tests, 88/88 passing desktop unit tests, and clean `svelte-check`.
6. **Task 5.9.74 (HTML & Markdown to PDF Full-Fidelity Image Rendering via `AssetBundle` `[Spec 088]`)**:
   - Merged (PR #230, commit `f276a78`).
   - Implemented `fulgur::asset::AssetBundle` integration in `HtmlToPdfOperation` and `MdToPdfOperation` (`paperpilot-pdf/src/operations/conversion.rs`).
   - Parses `<img src="...">` and Markdown links, decodes inline base64 image data URIs (`data:image/*;base64,...`), reads local relative/absolute disk images, registers them as in-memory asset buffers, and re-maps sources.
   - End-to-end verified via `pdfimages -list` (embedded 200×200 RGB image streams confirmed) and 21 unit tests passing (`test_html_to_pdf_with_base64_image`, `test_md_to_pdf_with_base64_image`, `test_html_to_pdf_graceful_missing_image`).
7. **Task 5.9.59 (Privacy & Deep Security Sanitizer `pdf_sanitize` `[Spec 085]`)**:
   - Merged (PR #228, commit `00c2f67`).
   - Implemented Single Core Runner `sanitize_pdf` in `paperpilot-pdf/src/operations/sanitize.rs` using pure in-memory `lopdf` manipulation to strip `/Info`, XML `/Metadata`, `/EmbeddedFiles`, `/AF`, active scripts (`/JavaScript`, `/OpenAction`, `/AA`), `/Thumb`, and private editing data (`/PieceInfo`).
   - Built 5 thin wrappers across Native CLI (`paperpilot sanitize`), MCP (`pdf_sanitize`), REST Gateway (`/api/v1/pdf/sanitize`), WASM engine (`WasmPdfEngine.sanitize`), and Cloudflare Edge worker (`/api/v1/sanitize`).
   - 10/10 (100.0%) unmocked real Penta-Interface semantic tests passing in `reports/PENTA_INTERFACE_E2E_SANITIZE_REPORT.md`.
8. **Task 5.9.69C (Penta-Interface Declarative Workflow Engine & Recipe Execution `[Spec 081]`)**:
   - Implemented Single Core Runner `run_workflow_bytes` in `paperpilot-pdf/src/operations/workflow.rs`.
   - Built 5 thin wrappers across Native CLI (`paperpilot workflow`), MCP (`list_workflows`, `document_workflow`), REST Gateway (`/api/v1/workflow/*`), WASM engine (`workflow`), and Cloudflare Edge worker.
   - 20/20 (100.0%) unmocked Penta-Interface E2E tests passing in `reports/PENTA_INTERFACE_E2E_WORKFLOW_REPORT.md`.
9. **Task 5.9.71 (Universal "Export as Code / API / MCP Prompt" Workbench `[Spec 082]`)**:
   - Centralized snippet generation engine in `apps/desktop/src/lib/utils/snippetGenerator.ts`.
   - Integrated live `</> Code / API` button and dynamic modal in **both** the Visual Pipeline Builder (`FlowSelector.svelte`) and Workspace Operations Panel (`OperationsPanel.svelte`, `EditableActionCard.svelte`).
10. **Task 5.9.72 (Embed Pipeline 1-to-1 Desktop Parity `[Spec 083]`)**:
   - Replaced hidden file input with prominent browse button & file chip summary.
   - Added interactive step stepper with dynamic tool picker (`+ Add Step`), deletion, presets dropdown, and flow persistence.
11. **Task 5.9.55 (Decoupled Markdown-to-DOCX Compiler with DrawingML Images `[Spec 077]`)**:
   - High-fidelity conversion decoding inline base64 images into true DrawingML `.docx` image parts with 2-column card layout synthesis.
12. **Spec 084 Authored (On-Device Document SLM Fine-Tuning)**:
   - Authored specification for `PaperPilot-Qwen-0.6B` (~350 MB GGUF, sub-50ms offline inference) and training roadmap.

---

## 🧠 Knowledge Graph & Token-Saving Architecture (`graphify-out/` & Obsidian)

> ⚡ **CRITICAL TOKEN-SAVING INSTRUCTION FOR AGENTS (Antigravity & Jules):**
> This repository is fully indexed with an AST Knowledge Graph at `graphify-out/graph.json` (3,937 nodes, 7,369 edges, 296 communities).
> - **DO NOT scan dozens of raw source files** when exploring functions, callers, traits, or dependencies.
> - **Query the Knowledge Graph First:** Use `graphify query "<question>"` (CLI) or inspect `graphify-out/.graphify_analysis.json` to immediately find the exact file and line.
> - **Obsidian Vault:** The entire codebase graph and architectural docs are exported to `wiki/vault/` (with `graph.canvas` and 4,233 interconnected notes). Run `flatpak run md.obsidian.Obsidian` to explore.
> - **Keep Current:** After adding or modifying core tools, run `graphify update .` (local AST, zero cost) to update the graph.

---

## 🏗️ Architecture & Key Crate Boundaries

```
PaperPilot/
├── paperpilot-core/        # Core traits (PdfOperation, PdfDocument), error & result types
├── paperpilot-pdf/         # All 50+ PDF operations (lopdf, pdf_oxide, fulgur, docx-rs, rusto-rs)
├── paperpilot-nlp/         # Offline Intent Classification & NLP Rule Engine
├── paperpilot-ai/          # Universal LLM client & router (Ollama, Llamafile, BYOK)
├── paperpilot-mcp/         # Model Context Protocol JSON-RPC server (52 tools)
├── paperpilot-gateway/     # Axum HTTP REST server & MCP SSE gateway (port 7823)
├── paperpilot-cli/         # Native CLI binary (paperpilot merge, split, burst, search, etc.)
├── paperpilot-wasm/        # In-memory WebAssembly engine for client-side browser execution
├── apps/desktop/           # Tauri 2.0 + Svelte 5 + Tailwind desktop app (with visual pipeline runner)
├── apps/embed/             # Embeddable zero-config JS widget & automation pipeline
├── apps/web/               # In-browser WASM client portal
├── apps/docs/              # Astro Starlight documentation portal (docs.usepaperpilot.com)
├── tools/penta-interface-e2e/ # Pure-Rust Penta-Interface E2E suite across all 5 surfaces
├── tools/jules-submit/     # Pure-Rust CLI for dispatching Jules tasks
├── reports/                # E2E test reports, benchmarks, and scorecards
└── specs/                  # Architecture, product specs, and verification checklists
```

---

## ⚖️ Rules of Engagement & System Constraints

1. **System Resource Constraint (`-j 2` is MANDATORY on local machine):**
   - **ALWAYS** pass `-j 2` for all local `cargo test`, `cargo build`, or `cargo clippy` commands (e.g. `cargo test -p paperpilot-mcp -j 2`).
   - Unbounded cargo compilations freeze the host machine.

2. **The Penta-Interface Parity Mandate:**
   - 💻 **CLI (`paperpilot-cli`)**: Kebab-case subcommand with `--json` support.
   - 🤖 **MCP (`paperpilot-mcp`)**: JSON-RPC 2.0 tool declaration with required input schema and execution match arm.
   - 🌐 **REST API (`paperpilot-gateway`)**: POST route `/api/v1/pdf/...` with JSON payload.
   - ⚡ **WASM (`paperpilot-wasm`)**: In-memory byte processing without filesystem IO.
   - 📱 **Desktop / Embed API (`apps/desktop`, `apps/embed`)**: Visual cards, parameter schemas, and pipeline execution.

3. **The Single Core Runner + Interface Wrappers Architecture Pattern (MANDATORY FOR ALL NEW TOOLS & WORKFLOWS):**
   - **Problem It Solves:** Prevents logic duplication and fragile implementations across the 5 surfaces (CLI, MCP, REST Gateway, WASM, Desktop/Embed).
   - **The Rule:** Every tool or workflow operation MUST have exactly **ONE** core in-memory engine implementation:
     ```rust
     // Pure in-memory: bytes in -> bytes out (in paperpilot-pdf)
     pub fn run_operation_bytes(input_bytes: &[u8], config: &Config) -> Result<Vec<u8>, OperationError>
     ```
   - **The 5 Thin Interface Wrappers (1–5 lines each):**
     1. 💻 **CLI (`paperpilot-cli`)**: `let bytes = fs::read(&input)?; let out = run_operation_bytes(&bytes, &cfg)?; fs::write(&output, out)?;`
     2. 🤖 **MCP (`paperpilot-mcp`)**: Reads file path $\to$ calls `run_operation_bytes` $\to$ writes output path $\to$ returns JSON summary.
     3. 🌐 **REST Gateway (`paperpilot-gateway`)**: Receives multipart upload $\to$ calls `run_operation_bytes` $\to$ streams PDF binary response.
     4. ⚡ **WASM Engine (`paperpilot-wasm`)**: Receives `Uint8Array` $\to$ calls `run_operation_bytes` $\to$ returns `Uint8Array`.
     5. 📱 **Web Embed & Desktop (`apps/embed`, `apps/desktop`)**:
        - In Embed/Web WASM: Calls the WASM wrapper directly in-memory via `window.PaperPilot` / WebWorker.
        - In Desktop: Calls either the native Rust sidecar via Tauri IPC or the Axum Gateway.
   - **Zero Intermediate Disk I/O:** Pipelines chain output bytes from step $N$ directly into step $N+1$ in RAM without temporary disk roundtrips.

4. **Workspace Safety Rules:**
   - **NEVER** execute `git reset --hard`, `git checkout -- .`, `git clean -fd`, or `git restore` under any circumstance.
   - Always commit or stash working progress.

5. **Zero-Mock Policy:**
   - All tests must use real disk fixtures and real in-memory byte buffers (`fixtures/`, `tests/e2e_fixtures/real/`). No dummy `{ success: true }` echo stubs.

---

## 📋 Resuming Checklist for Next Session

### Step 1: Pending Tool Roadmap (Wave 2 Core Operations)
The following tools are actively targeted in Wave 2 to reach full feature completion:

| # | Tool ID | Phase Task | Interface Title | Status / Scope & Objectives |
|---|---|---|---|---|
| 1 | `pdf_nup` | **Task 5.3.2** | Multi-Page N-Up Imposition | 📋 **Spec 097 Authored** — 2-up, 4-up, 9-up grid layouts with auto-scaling, borders, and margins. |
| 2 | `pdf_scale_pages` | **Task 5.3.3** | Page Dimension Rescaling | 📋 **Spec 098 Authored** — Scale to standard paper sizes (A4, Letter, A3) or custom factor with centering. |
| 3 | `pdf_remove_signatures` | **Task 5.3.13** | Strip Digital Signatures | 📋 **Spec 099 Authored** — Strip signature dictionaries and `/ByteRange` locks to restore editability. |
| 4 | `pdf_validate_accessibility` | **Task 5.9.62** | PDF/UA Accessibility Verifier | 📋 Planned — Section 508 / EAA structure tree audit, reading order, `/Lang` tags, and image `/Alt` validation. |
| 5 | `pdf_compare_semantic` | **Task 5.9.63** | Semantic Legal & Clause Diffing | 📋 Planned — Meaning-based sentence & clause comparison for contracts, generating structured Markdown changelogs. |
| 6 | `document_batch_process` | **Task 5.9.65** | Resilient Batch Directory Processor | 📋 Planned — Multi-threaded directory processing engine with retry telemetry and structured `failure_report.json`. |
| 7 | `pdf_to_images` | **Task 5.9.66** | Convenience Multi-Page Exporter | 📋 Planned — One-call exporter dumping all pages of a PDF to a folder as PNG, JPEG, or WebP. |

---

### Step 2: Mobile Scoped Features & Lean Architecture Plan

#### 📱 Mobile Scoped Feature Set (Emergency On-The-Go Operations)
Under **Spec 087**, mobile devices (Android APK & PWA/Embed) focus **exclusively** on rapid, high-utility emergency tasks via the in-memory `paperpilot-wasm` engine:
1. **Camera Scan to PDF (`images_to_pdf`)**: Take camera snapshots via `<input capture="camera">` or `CameraScannerModal.svelte`, crop, and convert to clean PDF in RAM.
2. **Emergency Signing (`pdf_sign`)**: Draw touch signature or stamp pre-saved signature onto contracts.
3. **Document Sanitization (`pdf_sanitize` & `document_pii_scan`)**: Strip metadata, private editing tags, and audit SSN/Credit Card leaks before sharing.
4. **Organize (`pdf_merge`, `pdf_rotate`, `pdf_delete_pages`, `pdf_reorder_pages`)**: Fix orientation, remove bad pages, combine attachments.
5. **Compression (`pdf_compress`)**: Shrink documents (<2 MB) before sending via WhatsApp, Email, or Slack.
6. **Native Sharing (`navigator.share()`)**: Push processed PDF bytes straight into OS intents without downloading to storage.

#### ⚖️ Lean Mobile APK Optimization (Task 5.9.75 / Spec 089)
- **Problem**: Current debug APK is **82 MB** because `apps/desktop/src-tauri` still compiles the native gateway/MCP sidecars and full desktop engines into `libdesktop_lib.so` (~32 MB), and bundles heavy dependencies.
- **Specification**: Authored [Spec 089](specs/089-mobile-lean-architecture-and-on-demand-ocr/spec.md) and [Task 5.9.75](specs/089-mobile-lean-architecture-and-on-demand-ocr/task.md).
- **Solution to implement**:
  - In `apps/desktop/src-tauri/Cargo.toml`, move `paperpilot-gateway` and `paperpilot-mcp` behind `desktop-features` flag (or `not(target_os = "android")`).
  - Keep `src-tauri` on Android as a lightweight Tauri shell (~3 MB) that only handles WebView initialization and OS dialogs.
  - Implement mobile curated UX in `OperationsPanel.svelte`: **Quick Actions row** (4 hero tools: `Scan`, `Sign`, `Compress`, `Sanitize`) followed by the full **26+ WASM tool catalog** organized into clean collapsible category accordions (`Organize`, `Security`, `Conversions`, `Enhance`).
  - Integrate cross-platform on-demand `ppocrv6-tiny` model download (~6 MB) on first OCR run instead of bundling models in APK/IPA assets.
  - Reduces final APK size from 82 MB to **~15–20 MB** and cuts build times by **70%**.

---

## 🛠️ Quick Command Reference

```bash
# 1. Dev Server (Desktop Tauri + Svelte 5)
make dev

# 2. Rust Unit Tests (Always use -j 2)
cargo test -p paperpilot-pdf --lib -j 2
cargo test -p paperpilot-mcp --lib -j 2
cargo test -p paperpilot-gateway --lib -j 2

# 3. Mobile Android Build & Install (Dynamic SDK/NDK detection)
make build-android       # Builds APK to apps/desktop/src-tauri/gen/android/app/build/outputs/apk/
make install-android     # Builds and installs APK to connected adb device/emulator
make log-android         # Streams filtered mobile logcat

# 4. Clean-Room Audits
make test-cleanroom-top12
make test-cleanroom-all50

# 5. Playwright E2E Suites
pnpm --dir apps/desktop exec playwright test
pnpm --dir apps/embed exec playwright test

# 6. Dispatch Jules Tasks
./scripts/jules_submit.sh --file prompts/tasks/phase5/<prompt-file>.txt
```
