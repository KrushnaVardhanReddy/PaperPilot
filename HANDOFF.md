# PaperPilot — Session Handoff Guide
 
> **Single Source of Truth** for resuming work on PaperPilot.  
> Start any new agent or human session by reading this document.  
> **Date of Last Update:** November 19, 2026
>
> **Active Target (Milestone: v0.8.0):** 
> 1. **100/100 Tools Penta-Interface Complete & 100% Passing**: Waves 1–22 QA complete across CLI, MCP, REST Gateway, Desktop UI, and Penta-Interface E2E suite (100 verified tools).
> 2. **Spec 170 Native Image Intelligence & Animated Showcase Engine**: 8 new pure-Rust tools added (`pdf_to_gif`, `showcase_gif`, `images_to_gif`, `img_deskew`, `img_crop_to_content`, `img_redact`, `img_optimize_for_llm`, `img_convert`).
> 3. **REST Gateway Semantic Aliases & Route Namespacing**: Introduced clean `/api/v1/image/*` and `/api/v1/office/*` primary endpoints while maintaining backward-compatible `/api/v1/pdf/*` legacy aliases.
> 4. **Phase 0 Desktop Real E2E Setup**: WebdriverIO + `@wdio/tauri-service` + `tauri-driver` with native WebKitGTK and `xvfb-run` integration verified with passing smoke tests.
> 5. **Penta-Scaffold Engine Upgraded**: `penta-scaffold` now scaffolds full Desktop UI wiring, WDIO desktop E2E tests, documentation, and OpenAPI schemas in a single pass.
> 6. **Active Branch**: `feature/native-showcase-gif-engine` (synced to origin, base `feature/dev`).

---

## 🧭 Executive Summary & Current Repository State

- **Current Working Branch**: `feature/native-showcase-gif-engine` (clean, synchronized with `origin/feature/native-showcase-gif-engine`).
- **Base Integration Branch**: `feature/dev`.
- **System Stability & Audits**:
  - **100/100 Tools Penta-Interface Parity**: **100% PASS** across CLI, MCP, REST Gateway API, Desktop UI, and Core Rust.
  - **Spec 170 Complete**: Pure-Rust NeuQuant GIF encoding, animated PDF/image sequencer, projection profile baseline deskew, bounding box content crop, token-budgeted LLM vision optimizer, coordinate redact, and image transcoding.
  - **Real Desktop E2E Automation**: WebdriverIO suite initialized in `apps/desktop/test/specs/smoke.e2e.ts` running against real Linux Tauri v2 / Wry v0.57 (passing in 1.9s).
  - **Desktop Vitest Suite**: 54/54 tests passing in `operationArgsBuilder.test.ts` across all 100 tools.
  - **Documentation Suite**: Starlight / Astro documentation updated to 100 tools with `v0.8.0+` badges across all reference pages and `tools-handbook.md`. Build passing cleanly (116 pages indexed, 0 broken links).
  - **Cargo Workspace**: Compiles cleanly with `-j 4` concurrency limit.

---

## 🏆 Recently Merged Milestones

1. **Spec 170: Native Image Intelligence & Animated Showcase Engine**:
   - 8 new pure-Rust tools (`pdf_to_gif`, `showcase_gif`, `images_to_gif`, `img_deskew`, `img_crop_to_content`, `img_redact`, `img_optimize_for_llm`, `img_convert`). Zero external runtime dependencies (no Python, ffmpeg, or ImageMagick).
   - Document-first showcase card generator with dropshadow, scan-sweep line, and multi-page cross-dissolve page-turns.
   - Text baseline deskew via horizontal projection profile variance optimization (-45° to +45° with 0.1° resolution).

2. **Semantic REST Gateway Namespacing & Aliases**:
   - Published clean primary routes: `/api/v1/image/deskew`, `/api/v1/image/crop-to-content`, `/api/v1/image/redact`, `/api/v1/image/optimize-for-llm`, `/api/v1/image/convert`, `/api/v1/image/to-gif`, `/api/v1/office/word-to-markdown`, `/api/v1/office/extract-text`.
   - Preserved all `/api/v1/pdf/*` routes as active backward-compatible aliases.

3. **Phase 0 Desktop Real E2E Setup (@wdio/tauri-service)**:
   - Configured `apps/desktop/wdio.conf.ts` for Linux WebKitGTK and `tauri-driver`.
   - Wired `data-testid` test hooks into Svelte 5 desktop layout.
   - Verified initial smoke test passing in 1.9s.

4. **Penta-Scaffold Full-Stack Generator Upgrade**:
   - Upgraded `tools/penta-scaffold/src/generator.rs` to automatically generate:
     - Full Desktop UI wiring (`toolList.ts`, `toolRegistry.ts`, `operationArgsBuilder.ts`, `OperationsPanel.svelte`).
     - WebdriverIO E2E test case in `apps/desktop/test/specs/<tool>.e2e.ts`.
     - Tool documentation in `apps/docs/src/content/docs/reference/tools/<tool>.md` and `tools-handbook.md`.
     - Gateway handler with OpenAPI `#[utoipa::path(...)]` annotations and semantic route namespacing.


---

## 🎯 Prioritized Next Tasks & Execution Roadmap

```
[Tier 1: Distribution & DX] ➔ [Tier 2: Agent Moat & Scalability] ➔ [Tier 3: Browser Extension] ➔ [Tier 4: Enterprise Moats]
```

### 🥇 Tier 1: Distribution & Developer Onboarding
*Goal: Remove all friction so any developer can install PaperPilot in 10 seconds and connect it to Claude, Cursor, or Python.*

1. **Spec 161: Package Distribution & One-Line Installers (`specs/161-package-distribution-and-installers/`)** — ✅ **100% MERGED** (PR #339)
   - POSIX `scripts/install.sh`, Windows PowerShell `scripts/install.ps1`, Homebrew formula `packaging/homebrew/paperpilot.rb`.
2. **Spec 162: AI Framework Integration Recipes (`specs/162-framework-integration-recipes/`)** — ✅ **100% MERGED** (PR #338)
   - Turnkey Python adapters for LangChain & LlamaIndex, Claude/Cursor MCP configs, and n8n invoice auto-OCR pipeline.
3. **Spec 160: System Diagnostics Engine (`paperpilot doctor`) (`specs/160-paperpilot-doctor-diagnostics/`)** — ✅ **100% MERGED** (Commit `db12ffd`)
   - Scope: `paperpilot-core::diagnostics` probing Gateway port 7823, temp directory permissions, OCR model storage, and disk space. CLI `paperpilot doctor` (`--json`), REST `GET /health` & `GET /api/v1/doctor`, Tauri command `run_system_doctor`, and Settings UI `SystemDoctorModal.svelte`.

---

### 🥈 Tier 2: Agent Performance & Scalability Moat (Next Priority)
*Goal: Solidify PaperPilot as the highest-performance local document engine for AI agents and RAG pipelines.*

4. **Spec 159: Semantic Chunking Engine (`pdf_chunk`) (`specs/159-agent-semantic-chunking/`)** — ✅ **100% MERGED**
   - Implemented multi-level structure-aware boundary detection (`#`, `##`, `\n\n`, tables `|...|`), configurable `target_tokens` and `overlap_tokens`, and BPE estimation.
   - Built boilerplate suppression removing recurring headers/footers across $\ge 3$ consecutive pages.
   - Zero-copy `DocumentSource` integration natively in `paperpilot-pdf`.
   - Full Penta-Interface coverage: CLI (`paperpilot pdf-chunk`), MCP tool (`pdf_chunk`), REST Gateway (`POST /api/v1/pdf/chunk`), Desktop Svelte 5 UI with sliders in `OperationsPanel.svelte`, and Vitest + Rust integration test suites (180/180 Vitest passing, 6/6 Rust chunk tests passing).
5. **Spec 163: Zero-Copy Memory-Mapped Document Handling (`specs/163-zero-copy-mmap-large-doc-handling/`)** — ✅ **100% MERGED** (Commit `5663728`)
   - Implemented `DocumentSource` using `memmap2` for files $\ge 10\text{ MB}$ to keep resident memory flat (< 100 MB) during search, chunking, and PII scanning of huge PDFs.
   - Implemented `PageBatchIterator` for streaming batch execution without loading entire documents into memory.
   - Guarded with `#[cfg(not(target_arch = "wasm32"))]` preserving 100% clean WASM compilation. Verified with unit, e2e, and doctor tests.

---

### 🥉 Tier 3: Growth & Virality — Browser Companion Extension (Phase 11)
*Goal: Bring PaperPilot directly into everyday user workflows inside Chrome, Edge, and Firefox tabs.*

6. **Phase 11: Manifest V3 Companion Browser Extension (`apps/extension`)** — ✅ **100% IMPLEMENTED & VERIFIED**
   - **WASM-First Zero-Install Architecture**: Bundles `paperpilot-wasm` for instant in-tab document manipulation (split, merge, rotate, watermark, metadata inspection, text extraction) requiring 0 native binary installation.
   - **Optional Desktop Gateway Bridge**: Seamlessly detects local PaperPilot daemon (`localhost:7823`) for heavy neural OCR (`ppocrv4`), zero-copy mmap, and full conversions when available; falls back gracefully to standalone WASM mode when offline.
   - **In-Tab Context Menus**: Right-click on any web PDF (arXiv, Stripe invoices, receipts) ➔ *"PaperPilot: Clean Text / Chunk / Redact / Convert"*.
   - **Web Clean-Article Exporter**: Injects `articleReader` to strip clutter, ads, navigation, and tracking, exporting clean Markdown or PDF.
   - **Svelte 5 Popup UI**: Live connection badge (`[⚡ Desktop Connected (7823)]` / `[🌐 Standalone WASM Mode]`), drag-and-drop file processing, and quick action buttons.
   - **Tests & Builds**: `pnpm --dir apps/extension test` (9/9 passing), `pnpm run build:extension` (< 1s clean build to `dist/`), 0 workspace compilation errors, and `paperpilot doctor` diagnostics 100% healthy. Note: IDE workflows are officially served via `paperpilot-mcp` (Spec 162).

---

### ⏸️ Tier 4: Long-Term Enterprise Moats (Post-Launch)
* **Phase 10: Universal Document Operating System (`DocAST`)**
  - Full AST conversion matrix: `DOCX ⇄ MD ⇄ HTML ⇄ PDF ⇄ EPUB ⇄ XLSX ⇄ CSV`.
* **Phase 6.7: Advanced Document SDK Parity (Apryse / PDFTron Gap)**
  - Stylus pressure-ink SVG annotations, dynamic XFA form calculations, and certified PAdES LTV digital signatures.

---

## 📋 Resuming Checklist for Next Session

When resuming a new session, follow these steps:

1. **Verify Branch Alignment**:
   - Ensure local branch is `feature/dev` and up to date with `origin/feature/dev`.
   - Verify `git status` is clean.

2. **Run Resource-Safe Compilation (`-j 4` MANDATORY)**:
   - Check workspace: `cargo check --workspace --all-targets --all-features -j 4`
   - Run tests: `cargo test --workspace --no-run -j 4`
   - Run desktop Vitest component tests: `pnpm --dir apps/desktop test:unit`

3. **Active Next Tasks**:
   - **Tier 1 (Immediate)**: Package Distribution & Installers (Spec 161), System Diagnostics `doctor` (Spec 160), AI Integration Recipes (Spec 162).
   - **Tier 2 (Agent Performance)**: Semantic Chunking `pdf_chunk` (Spec 159), Zero-Copy mmap Engine (Spec 163).
   - **Tier 3 (Browser Growth)**: Manifest V3 Browser Extension (Phase 11).

---

## ⚖️ Rules of Engagement & Production Constraints

> ⚠️ **CRITICAL**: PaperPilot is a live open-source product. All development must preserve production stability.

1. **System Resource Constraint (`-j 4` is MANDATORY on local machine):**
   - **ALWAYS** pass `-j 4` for all local `cargo test`, `cargo build`, or `cargo check` commands (e.g. `cargo test -p paperpilot-pdf -j 4`).
   - Unbounded cargo compilations will exhaust memory and freeze the host machine / log off the system.

2. **Feature Branch Protocol (No Direct Pushes to `main`):**
   - All development, bug fixes, and experiments must occur on dedicated feature branches or reviewed into `feature/dev`. Never push directly to `main`.

3. **The Single Core Runner + 5 Wrappers Architecture Pattern:**
   - Every tool operation MUST have exactly **ONE** core in-memory engine implementation in `paperpilot-pdf`:
     ```rust
     pub fn run_operation_bytes(input_bytes: &[u8], config: &Config) -> Result<Vec<u8>, OperationError>
     ```
   - 5 Thin Interface Wrappers: CLI (`paperpilot-cli`), MCP (`paperpilot-mcp`), REST Gateway (`paperpilot-gateway`), WASM Engine (`paperpilot-wasm`), and Desktop/Embed (`apps/desktop`, `apps/embed`).

4. **Zero-Mock Policy on Tests:**
   - Test fixtures must be real disk fixtures or synthesized byte buffers (`fixtures/`, `tests/e2e_fixtures/real/`). Never mock PDF bytes or operation responses.

5. **WASM Isolation Guarantee:**
   - Never add OS-specific filesystem watchers or native network sockets (`mio`, `tokio/net`, `notify`) to `paperpilot-core` or common dependencies of `paperpilot-wasm`.

6. **Knowledge Graph Maintenance:**
   - After modifying project code files, run `/home/krushna/.cargo/bin/graphify-rs build` to keep the knowledge graph synchronized.

---

## 🛠️ Quick Command Reference

```bash
# 1. WASM Build & Tests (Memory-Safe)
wasm-pack build --target web --release paperpilot-wasm -- --jobs 4
cargo test -p paperpilot-wasm -j 4

# 2. Rust Unit Tests (Always use -j 4)
cargo test -p paperpilot-pdf --lib -j 4
cargo test -p paperpilot-mcp --lib -j 4
cargo test -p paperpilot-gateway -j 4
cargo test -p paperpilot-cli -j 4

# 3. Penta-Interface E2E Audit Suites
make test-wave1
make test-wave2
make test-wave3
make test-wave4
make test-wave5
make test-wave6
make test-wave7
make test-wave8

# 4. System Benchmark Suite
python3 scripts/run_system_benchmark.py --quick

# 5. Desktop Vitest Tests
pnpm --dir apps/desktop test

# 6. Knowledge Graph Refresh
/home/krushna/.cargo/bin/graphify-rs build
```
