# PaperPilot — Development Phases & Tasks

> Tasks marked **[PARALLEL]** can be worked on simultaneously.
> Tasks marked **[BLOCKED BY]** cannot start until the listed task is complete.
> Each phase has a clear exit condition — don't move to the next phase until it's met.

---

## Licensing Model (LOCKED)

**Decision: Open Core** — like GitLab, HashiCorp Vault, and Outline.

| Tier | License | What's included |
|---|---|---|
| **Community (Free)** | Apache 2.0 | Core engine, CLI, MCP server, Desktop app, all PDF operations |
| **Enterprise (Paid)** | Commercial | SSO (SAML/OIDC), RBAC, Audit logs, Multi-org, License key management, Group Policy |
| **Cloud (Paid)** | Commercial | Managed hosting, Cloud MCP gateway, Billing, SLA monitoring |

**Rules:**
- Everything in Phases 1–5 is **Apache 2.0** — always free, always open.
- Everything in Phase 6 (Enterprise) and Phase 7 (Cloud) is **Commercial** — gated behind a license key.
- The boundary must never move. Enterprise features must never sneak into the Apache-licensed core.

---

## Phase 1 — Rust Core Engine

**Goal:** A reliable, well-tested PDF manipulation engine with a working CLI.
No AI. No GUI. Just a solid foundation everything else will build on.

**Exit condition:** All PDF operations listed below work correctly on real-world PDFs, have passing tests, and are accessible via the CLI.

---

### 1.1 — Project Setup `[PARALLEL]`

These can all be done on day one simultaneously.

| # | Task | Notes |
|---|---|---|
| ~~1.1.1~~ ✓ | ~~Initialize Cargo workspace~~ | Root `Cargo.toml` with workspace members |
| ~~1.1.2~~ ✓ | ~~Create crate stubs~~ | `paperpilot-core`, `paperpilot-pdf`, `paperpilot-cli` empty crates |
| ~~1.1.3~~ ✓ | ~~Set up CI pipeline~~ | Completed by Jules (Session 367767207543843406) |
| 1.1.4 | Set up test fixture corpus ✅ | Merged (Session 4769317803713179485) |
| ~~1.1.5~~ ✓ | ~~Choose and document PDF library decision~~ | Completed by Jules (Session 901308598875262977) |
| ~~1.1.6~~ ✓ | ~~Set up linting and formatting~~ | Completed by Jules (Session 367767207543843406) |

---

### 1.2 — Core Traits and Abstractions

**[BLOCKED BY 1.1.1, 1.1.2]**

Design the internal interfaces before implementing anything. This prevents tight coupling to a specific PDF library.

| # | Task | Notes |
|---|---|---|
| ~~1.2.1~~ ✓ | ~~Define `PdfDocument` trait~~ | Completed by Jules (Session 803802133789731818) |
| ~~1.2.2~~ ✓ | ~~Define `PdfOperation` trait~~ | Completed by Jules (Session 803802133789731818) |
| ~~1.2.3~~ ✓ | ~~Define `OperationResult` and error types~~ | Completed by Jules (Session 6267567952319900834) |
| ~~1.2.4~~ ✓ | ~~Define `JobProgress` event type~~ | Completed by Jules (Session 6267567952319900834) |

---

### 1.3 — PDF Operations `[PARALLEL within group]`

**[BLOCKED BY 1.2]**

All operations are independent of each other and can be built in parallel once the traits are in place.

#### Group A — Page Manipulation
| # | Task | Notes |
|---|---|---|
| 1.3.1 | Merge ✅ | Merged (Session 2375117738102953150) |
| 1.3.2 | Split ✅ | Merged (Session 2375117738102953150) |
| 1.3.3 | Extract pages ✅ | Merged (Session 1757891711996002541) |
| 1.3.4 | Delete pages ✅ | Merged (Session 1757891711996002541) |
| 1.3.5 | Reorder pages ✅ | Merged (Session 1757891711996002541) |
| 1.3.6 | Rotate pages ✅ | Merged (Session 12203497870162599469) |
| 1.3.7 | Crop pages ✅ | Merged (Session 12203497870162599469) |
| 1.3.8 | Burst ✅ | Merged (Session 2375117738102953150) |

#### Group B — Document Operations
| # | Task | Notes |
|---|---|---|
| 1.3.9 | Compress ✅ | Merged (Session 6923800200121241243) |
| 1.3.10 | Repair ✅ | Merged (Session 6923800200121241243) |
| 1.3.11 | Metadata read/write ✅ | Merged (Session 2384599268814762867) |
| 1.3.12 | Encrypt ✅ | Merged (Session 2384599268814762867) |
| 1.3.13 | Decrypt ✅ | Merged (Session 2384599268814762867) |
| 1.3.14 | Watermark ✅ | Merged (Session 8470929282963705394) |
| 1.3.15 | E-Signature ✅ | Merged (Session 12850003959773306225) |
| 1.3.16 | Redact ✅ | Merged (Session 12850003959773306225) |
| 1.3.17 | Linearize ✅ | Merged (Session 6923800200121241243) |
| 1.3.18 | Flatten ✅ | Merged (Session 12850003959773306225) |
| 1.3.19 | PDF/A conversion ✅ | Merged (Session 12850003959773306225) |
| 1.3.20 | Header/Footer ✅ | Merged (Session 8470929282963705394) |
| 1.3.21 | Bates numbering ✅ | Merged (Session 12785657976803298608) |
| 1.3.22 | Integrity hash ✅ | Merged (Session 10906031722199501492) |

#### Group C — Extraction & Search
| # | Task | Notes |
|---|---|---|
| 1.3.22 | Extract text ✅ | Merged (Session 2668342604059292265) |
| 1.3.23 | Extract images ✅ | Merged (Session 10564378390682319461) |
| 1.3.24 | Render pages ✅ | Merged (Session 12785657976803298608) |
| 1.3.25 | Images → PDF ✅ | Merged (Session 10564378390682319461) |
| 1.3.26 | Search ✅ | Merged (Session 2668342604059292265) |
| 1.3.27 | Compare ✅ | Merged (Session 12785657976803298608) |
| 1.3.28 | Bookmarks ✅ | Merged (Session 2668342604059292265) |
| 1.3.29 | OCR ✅ | Merged (Session 12785657976803298608) |

#### Group D — Conversion
| # | Task | Notes |
|---|---|---|
| 1.3.30 | PDF → Word (DOCX) ✅ | Merged (Session 8203167166999327475) |
| 1.3.31 | PDF → Excel (XLSX) ✅ | Merged (Session 8203167166999327475) |
| 1.3.32 | PDF → PowerPoint (PPTX) ✅ | Merged (Session 8203167166999327475) |
| 1.3.33 | PDF → HTML ✅ | Merged (Session 10906031722199501492) |
| 1.3.34 | PDF → Markdown ✅ | Merged (Session 10906031722199501492) |
| 1.3.38 | PDF → LLM-Ready Export ✅ | Merged (Session 10906031722199501492) |
| 1.3.39 | Document type classifier ✅ | Merged (Session 8203167166999327475) |
| 1.3.46 | PDF → Structured JSON ✅ | Merged (Session 10906031722199501492) |

#### Group E — Forms
| # | Task | Notes |
|---|---|---|
| 1.3.35 | AcroForm read ✅ | Merged (Session 996394326205831326) |
| 1.3.36 | AcroForm fill ✅ | Merged (Session 996394326205831326) |
| 1.3.37 | Form field creation ✅ | Merged (Session 996394326205831326) |

#### Group F — PDF Creation (from scratch)
| # | Task | Notes |
|---|---|---|
| 1.3.40 | Markdown → PDF ✅ | Merged |
| 1.3.41 | HTML → PDF ✅ | Merged |
| 1.3.42 | Template engine ✅ | Merged |
| 1.3.43 | Built-in template library ✅ | Merged |
| 1.3.44 | Template → PDF CLI ✅ | Merged |
| 1.3.45 | Custom page layout ✅ | Merged |

---

#### Group G — Smart Scan Compression (DjVu-inspired)
> Inspired by DjVu's 3-layer decomposition (background + foreground text mask + OCR). Target: 80–90% size reduction on scanned PDFs vs. standard Flate/JPEG. See `INSPIRATION.md §1`.

| # | Task | Notes |
|---|---|---|
| 1.3.50 | Scanned page detector | Heuristic: if >80% of page content stream is raster image data with no vector paths → classify as "scanned page" |
| 1.3.51 | Background layer extraction | Downsample page raster to ~72 DPI; apply IW44-style wavelet compression (Rust `image` crate wavelets) for paper texture / colour wash |
| 1.3.52 | Foreground text mask | Binarise the full-res page raster (adaptive threshold); detect connected components corresponding to text glyphs; encode as JBIG2 or CCITT Group 4 bitonal stream |
| 1.3.53 | Hidden text layer injection | After OCR (Phase 5.1), inject the text layer as a `ActualText` / invisible-text overlay within the compressed page for copy-paste and search |
| 1.3.54 | 3-layer page reassembly | Combine background (low-res colour) + foreground (bitonal hi-res mask) + hidden text into a single lopdf page object; result opens normally in any PDF viewer |
| 1.3.55 | `SmartCompressOperation` | Wraps tasks 1.3.50–1.3.54; called by `pdf_compress` when `--mode smart-scan` is passed; falls back to standard Flate for non-scanned pages |
| 1.3.56 | Compression ratio benchmarks | Automated test comparing output size vs. standard `pdf_compress --quality low/medium` on a fixture set of 10 scanned PDFs; must achieve ≥70% reduction |

---

### 1.4 — Testing `[PARALLEL with 1.3]`

Write tests alongside each operation, not after.

| # | Task | Notes |
|---|---|---|
| 1.4.1 | Unit tests per operation ✅ | Written inline in every operation (`#[cfg(test)]`) |
| 1.4.2 | Round-trip tests ✅ | Merged (`fixture_tests_group_a/b/c.rs`) |
| 1.4.3 | Fixture-based integration tests ✅ | Merged (`fixture_tests_group_a/b/c.rs`) |
| 1.4.4 | Property-based tests ✅ | Merged |
| 1.4.5 | Error handling tests ✅ | Merged |
| 1.4.6 | PDF Validation layer ✅ | Merged |

---

### 1.5 — CLI `[BLOCKED BY 1.3]`

**[BLOCKED BY 1.3 — each command blocked only by its own operation]**

| # | Task | Notes |
|---|---|---|
| 1.5.1 | CLI scaffold ✅ | Merged |
| 1.5.2 | `merge` command ✅ | Merged |
| 1.5.3 | `split` command ✅ | Merged |
| 1.5.4 | `extract` command ✅ | Merged |
| 1.5.5 | `remove` command ✅ | Merged |
| 1.5.6 | `reorder` command ✅ | Merged |
| 1.5.7 | `rotate` command ✅ | Merged |
| 1.5.8 | `compress` command ✅ | Merged |
| 1.5.9 | `extract-text` command ✅ | Merged |
| 1.5.10 | `extract-images` command ✅ | Merged |
| 1.5.11 | `watermark` command ✅ | Merged |
| 1.5.12 | `encrypt` / `decrypt` commands ✅ | Merged |
| 1.5.13 | `search` command ✅ | Merged |
| 1.5.14 | `validate` command ✅ | Merged |
| 1.5.15 | Progress output ✅ | Merged |
| 1.5.16 | JSON output mode ✅ | Merged |
| 1.5.17 | Webhook support ✅ | Merged |
| 1.5.18 | Webhook HMAC signing ✅ | Merged (Session 3236745347341611735) |
| 1.5.19 | Webhook filter flags ✅ | Merged (Session 3236745347341611735) |
| 1.5.20 | `hash` command ✅ | Merged (Session 3236745347341611735) |
| 1.5.21 | `verify` command ✅ | Merged (Session 3236745347341611735) |
| 1.5.22 | CLI integration tests ✅ | Merged (Session 3236745347341611735) |
| 1.5.23 | E2E CLI pipeline tests ✅ | Merged (Session 3236745347341611735) |
| 1.5.24 | `extract-text --format json` flag ✅ | Merged (Session 3236745347341611735) |

---

## Phase 2 — MCP Server

**Goal:** PaperPilot exposes all PDF operations as strongly typed MCP tools that AI agents can call.

**Exit condition:** An external AI agent (Claude, GPT, etc.) can use PaperPilot via MCP to merge, split, compress, and extract text from PDFs without any custom integration code.

**[Phase 1 exit condition: ✅ MET — all operations implemented, tested, and CLI working]**

---

### 2.1 — MCP Foundation `[PARALLEL]`

| # | Task | Notes |
|---|---|---|
| 2.1.1 | Add `rmcp` crate dependency ✅ | Merged (Session 8555100062784875086) |
| 2.1.2 | Create `paperpilot-mcp` crate ✅ | Merged (Session 8555100062784875086) |
| 2.1.3 | Design tool schema conventions ✅ | Merged (Session 8555100062784875086) |

---

### 2.2 — Tool Implementations `[PARALLEL within group, BLOCKED BY 2.1]`

Each MCP tool wraps the corresponding Phase 1 Rust operation.

| # | Task | Notes |
|---|---|---|
| 2.2.1 | `pdf_merge` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.2 | `pdf_split` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.3 | `pdf_extract_pages` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.4 | `pdf_delete_pages` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.5 | `pdf_reorder_pages` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.6 | `pdf_rotate` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.7 | `pdf_crop` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.8 | `pdf_burst` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.9 | `pdf_compress` tool ✅ | Merged (Session 5942899190760144396) |
| 2.2.10 | `pdf_extract_text` tool ✅ | Merged (Session 5942899190760144396) |
| 2.2.11 | `pdf_extract_images` tool ✅ | Merged (Session 5942899190760144396) |
| 2.2.12 | `pdf_search` tool ✅ | Merged (Session 5942899190760144396) |
| 2.2.13 | `pdf_sign` tool ✅ | Merged |
| 2.2.14 | `pdf_watermark` tool ✅ | Merged (Session 5942899190760144396) |
| 2.2.15 | `pdf_encrypt` / `pdf_decrypt` tools ✅ | Merged (Session 5942899190760144396) |
| 2.2.16 | `pdf_metadata` tool ✅ | Merged (Session 5942899190760144396) |

---

### 2.4 — MCP Tool Backfill `[PARALLEL with 2.2, BLOCKED BY respective Phase 1 ops]`

> These tools were added during Phase 1 sessions **after** 2.2 was originally written.
> They are split into groups matching who will implement the MCP wiring.

#### 2.4a — Group B Missing Tools ✅
| # | Task | Notes |
|---|---|---|
| 2.4.1 | `pdf_repair` tool ✅ | Merged |
| 2.4.2 | `pdf_linearize` tool ✅ | Merged |
| 2.4.3 | `pdf_redact` tool ✅ | Merged |
| 2.4.4 | `pdf_flatten` tool ✅ | Merged |
| 2.4.5 | `pdf_to_pdf_a` tool ✅ | Merged |
| 2.4.6 | `pdf_header_footer` tool ✅ | Merged |
| 2.4.7 | `pdf_bates` tool ✅ | Merged |
| 2.4.8 | `pdf_integrity_hash` tool ✅ | Merged |

#### 2.4b — Group C Missing Tools ✅
| # | Task | Notes |
|---|---|---|
| 2.4.9 | `pdf_render` tool ✅ | Merged |
| 2.4.10 | `pdf_images_to_pdf` tool ✅ | Merged |
| 2.4.11 | `pdf_compare` tool ✅ | Merged |
| 2.4.12 | `pdf_bookmarks` tool ✅ | Merged |
| 2.4.13 | `pdf_ocr` tool ✅ | Merged |

#### 2.4c — Group F (PDF Creation) Missing Tools (needs new Jules session)
| # | Task | Notes |
|---|---|---|
| 2.4.14 | `pdf_markdown_to_pdf` tool | Wraps `MarkdownToPdf` from 1.3.40 |
| 2.4.15 | `pdf_html_to_pdf` tool | Wraps `HtmlToPdf` from 1.3.41 |
| 2.4.16 | `pdf_create_from_template` tool | Wraps template engine from 1.3.42/1.3.43 |

#### 2.4d — Group D Conversion Tools
| # | Task | Notes |
|---|---|---|
| 2.4.17 | `pdf_to_html` tool 🚧 | Already registered in PR #37 (Session 10906031722199501492) |
| 2.4.18 | `pdf_to_markdown` tool 🚧 | Already registered in PR #37 (Session 10906031722199501492) |
| 2.4.19 | `pdf_to_json` tool 🚧 | Already registered in PR #37 (Session 10906031722199501492) |
| 2.4.20 | `pdf_to_llm_export` tool 🚧 | Already registered in PR #37 (Session 10906031722199501492) |
| 2.4.21 | `pdf_to_docx` tool ✅ | Merged (Session 8203167166999327475) |
| 2.4.22 | `pdf_to_xlsx` tool ✅ | Merged (Session 8203167166999327475) |
| 2.4.23 | `pdf_to_pptx` tool ✅ | Merged (Session 8203167166999327475) |
| 2.4.24 | `pdf_classify_type` tool ✅ | Merged (Session 8203167166999327475) |

#### 2.4e — Group E Forms Tools (wired by active Jules session 996394326205831326)
| # | Task | Notes |
|---|---|---|
| 2.4.25 | `pdf_read_form` tool ✅ | Merged (Session 996394326205831326) |
| 2.4.26 | `pdf_fill_form` tool ✅ | Merged (Session 996394326205831326) |
| 2.4.27 | `pdf_create_form_field` tool ✅ | Merged (Session 996394326205831326) |

---

### 2.3 — MCP Server Infrastructure `[PARALLEL with 2.2]`

| # | Task | Notes |
|---|---|---|
| 2.3.1 | Input validation layer ✅ | Merged (Session 9931121574944478405) |
| 2.3.2 | Error response format ✅ | Merged (Session 9931121574944478405) |
| 2.3.3 | File path / temp file handling ✅ | Merged (Session 9931121574944478405) |
| 2.3.4 | MCP server binary ✅ | Merged (Session 9931121574944478405) |
| 2.3.5 | MCP schema tests ✅ | Merged (Session 9931121574944478405) |
| 2.3.6 | End-to-end MCP tests ✅ | Merged (Session 9931121574944478405) |
| 2.3.7 | E2E MCP tests (Strict) ✅ | Merged (Session 9931121574944478405) |
| 2.3.8 | SDK Architecture Lock (`rmcp` vs `pmcp`) ✅ | **Architectural Decision (Locked):** Retain official Anthropic `rmcp` 3.5.0 SDK for guaranteed host compatibility (Claude Desktop, Cursor, Goose). Evaluated `pmcp` (zero-cost alternative); deferred to Phase 7.1 Cloud gateway where high-concurrency microsecond serialization matters. |

---

### 2.5 — OpenAPI 3.1 & Embedded Swagger UI for Gateway

> **Stirling-PDF Parity & Developer Integration**: Exposes interactive Swagger UI at `/swagger-ui` and the OpenAPI 3.1 JSON specification at `/api-docs/openapi.json` for all REST endpoints and MCP dispatchers.

| # | Task | Notes |
|---|---|---|
| 2.5.1 | `utoipa` & `utoipa-swagger-ui` Integration | Add dependencies to `paperpilot-gateway` and define root `ApiDoc` |
| 2.5.2 | REST & MCP Endpoint Annotations | Annotate all 14+ endpoints (`/health`, `/merge`, `/split`, `/compress`, `/tools/{tool_name}`) with `#[utoipa::path]` |
| 2.5.3 | Schema Models Definition | Add `#[derive(ToSchema)]` to all request/response payload structs |
| 2.5.4 | Embedded Swagger UI Route | Mount `/swagger-ui` and `/api-docs/openapi.json` in Axum router |
| 2.5.5 | OpenAPI JSON Spec Export | Export static `docs/api/openapi.json` for external client SDK generation |

---

## Phase 3 — Desktop + Mobile Application (Tauri 2.0)

**Goal:** A polished, cross-platform Tauri 2.0 app for PDF manipulation — running natively on Windows, macOS, Linux, iOS, and Android from a single codebase.

**Technology Decision (LOCKED):** Tauri 2.0 + Svelte 5 + TypeScript.
- **Why Tauri 2.0:** Official iOS and Android support landed in v2.0. The Rust backend (`paperpilot-core`) is shared across all targets with zero changes — only the UI shell differs. Apache 2.0 licensed.
- **Why Svelte:** Minimal bundle size (critical for mobile), no runtime overhead, compiles to vanilla JS.

**Exit condition:** A user with no technical knowledge can open the app on any platform (desktop or mobile), drop in PDFs, perform all basic operations, and get output files — without touching the CLI.

**[BLOCKED BY Phase 1 exit condition]**
**[Can start in parallel with Phase 2]**

---

### 3.1 — Project Setup `[PARALLEL]`

| # | Task | Notes |
|---|---|---|
| 3.1.1 | Initialize Tauri 2.0 project ✅ | `apps/desktop/src-tauri/` fully scaffolded |
| 3.1.2 | Configure Svelte 5 + TypeScript + Vite ✅ | Svelte 5.56, TypeScript 6.0, Vite 8 |
| 3.1.3 | Set up Tauri ↔ Rust command bindings ✅ | `invoke_mcp_tool` in `src-tauri/src/lib.rs` (Phase 3.5) |
| 3.1.4 | Set up component library / design system ✅ | Custom vanilla CSS design system in `app.css` |
| 3.1.5 | Define Tauri IPC schema ✅ | `jobs.svelte.ts` + `serde_json::Value`-typed commands |
| 3.1.6 | Set up frontend tests ✅ | Playwright E2E & Vitest component unit tests merged |
| 3.1.7 | Configure iOS target | Requires macOS (Xcode) to run `cargo tauri ios init` |
| 3.1.8 | Configure Android target ✅ | `apps/desktop/src-tauri/gen/android` scaffolded |

---

### 3.2 — Core UI Shell `[PARALLEL within group, BLOCKED BY 3.1]`

| # | Task | Notes |
|---|---|---|
| 3.2.1 | App layout and navigation ✅ | Merged (Session 6096383570980483046) |
| 3.2.2 | File drop zone ✅ | Merged (Session 6096383570980483046) |
| 3.2.3 | Document list ✅ | Merged (Session 11119733351266944393) |
| 3.2.4 | Settings panel ✅ | Merged (Session 11119733351266944393) |
| 3.2.5 | Notification / toast system ✅ | Merged (Session 6096383570980483046) |

---

### 3.3 — PDF Preview `[PARALLEL with 3.2, BLOCKED BY 3.1]`

| # | Task | Notes |
|---|---|---|
| 3.3.1 | Page renderer ✅ | Merged (Session 10488084682857299204) |
| 3.3.2 | Page thumbnail strip ✅ | Merged (Session 10488084682857299204) |
| 3.3.3 | Page selection ✅ | Merged (Session 10488084682857299204) |
| 3.3.4 | Page reorder via drag ✅ | Merged (Session 6565731315931125703) |
| 3.3.5 | Page rotation controls ✅ | Merged (Session 6565731315931125703) |
| 3.3.6 | Page deletion UI ✅ | Merged (Session 6565731315931125703) |

---

### 3.4 — Operations UI `[BLOCKED BY 3.2]`

| # | Task | Notes |
|---|---|---|
| 3.4.1 | Operation panel ✅ | Merged (Session 11119733351266944393) |
| 3.4.2 | Merge UI ✅ | Merged (Session 11119733351266944393) |
| 3.4.3 | Split UI ✅ | Merged (Session 10488084682857299204) |
| 3.4.4 | Compress UI ✅ | Merged (Session 10488084682857299204) |
| 3.4.5 | Rotate UI ✅ | Merged (Session 10488084682857299204) |
| 3.4.6 | Watermark UI ✅ | Merged (Session 10488084682857299204) |
| 3.4.7 | Encrypt/Decrypt UI ✅ | Merged (Session 10488084682857299204) |
| 3.4.8 | Extract UI ✅ | Merged (Session 10488084682857299204) |
| 3.4.9 | Output file picker ✅ | Merged (Session 10488084682857299204) |
| 3.4.10 | **Visual Pipeline Builder (Basic)** ✅ | Merged |
| 3.4.11 | **Conditional branching UI** | Add IF/ELSE nodes (e.g. "if scanned → OCR, else extract text directly") to the pipeline canvas |
| 3.4.12 | **Pipeline trigger config** | Choose trigger: manual, watch folder, webhook, or schedule |

---

### 3.5 — Job System `[BLOCKED BY 3.1, PARALLEL with 3.2]`

| # | Task | Notes |
|---|---|---|
| 3.5.1 | Rust async job runner ✅ | Merged (Session 8999512547197594932) |
| 3.5.2 | Tauri event bridge ✅ | Merged (Session 8999512547197594932) |
| 3.5.3 | Progress bar component ✅ | Merged |
| 3.5.4 | Job cancel support ✅ | Merged |
| 3.5.5 | Job history panel ✅ | Merged (Session 8999512547197594932) |
| 3.5.6 | Responsive mobile layout ✅ | Merged (Session 13229007488227647984) |
| 3.5.7 | E2E Desktop tests ✅ | Merged (Session 617520399890233763) |
| 3.5.8 | E2E Mobile tests (Simulator) | iOS Simulator + Android Emulator automated tests via Appium or Detox (No Mocking) |

---

### 3.6 — Release / Distribution `[BLOCKED BY 3.2, 3.4]`

| # | Task | Notes |
|---|---|---|
| 3.6.1 | Portable Windows Build (`.exe`) ✅ | Merged |
| 3.6.2 | Portable Linux Build (`AppImage`) ✅ | Merged |
| 3.6.3 | Portable macOS Build (`.app` / `.dmg`) ✅ | Merged |
| 3.6.4 | MSI / MDM Installers (Enterprise) | Create traditional MSI installers for IT deployment (Intune, SCCM). |

---

## Phase 4 — AI Layer

**Goal:** Users can describe what they want in natural language. The AI produces a structured plan. The user confirms. Rust executes.

**Two-Mode Strategy:**
- **Offline NLP (Free Tier):** A fast, embedded intent classifier that runs 100% locally with zero API key. Handles single-intent commands (~80% of real user needs). Ships by default.
- **LLM Mode (Pro/Teams):** Full natural language reasoning for complex multi-step commands. Requires an API key. Powered by a pluggable `NlpResolver` trait — OpenAI, Ollama, Gemini, or local llamafile.

**Exit condition:** A free-tier user with no API key can type "compress this PDF" and get the correct output. A Pro user can type "merge these files, remove blank pages, and compress the result" and get the full chain executed.

**[BLOCKED BY Phase 3]**

---

### 4.0 — Offline NLP Mode (Free Personal Tier: TinyBERT-4L-312D) `[PARALLEL]`

> Ships bundled with the Personal (Free) tier. No API key, no network, no external download required.
> **Designated Model:** `TinyBERT-4L-312D` (INT8 ONNX, ~14MB on disk, ~25MB active RAM, ~2ms CPU latency).
> Handles instantaneous single-intent commands & parameter slot-filling across arbitrary human phrasing (e.g. "squish this pdf", "chop first 5 pages").
> Total desktop package remains under ~50MB (well below the 100MB budget).
> **Licensing: Apache 2.0 (Community)**

| # | Task | Notes |
|---|---|---|
| 4.0.1 | Create `paperpilot-nlp` crate | Separate crate for offline NLP; no LLM dependencies |
| 4.0.2 | Define `NlpResolver` trait | `fn resolve(input: &str) -> OperationPlan` — the single abstraction all modes implement |
| 4.0.3 | Intent vocabulary definition ✅ | Merged (Session 7343236324684096962) |
| 4.0.4 | Layer 1 — Keyword & regex rule engine ✅ | Merged (Session 283097227045968085) |
| 4.0.5 | Layer 2 — Embedded ONNX Intent & Entity Classifier | ✅ Completed (PR #127) — `TinyBERT-4L-312D` INT8 ONNX compressed with `zstd` (~9.6MB) embedded via `include_bytes!` and executed in-memory via `ort` (<2ms CPU latency, `reports/NLP_ONNX_VERIFIED.md`). Zero external dependencies. |
| 4.0.6 | Entity extractor ✅ | Merged (Session 4945848464737601940) |
| 4.0.7 | Ambiguity resolver ✅ | Merged (Session 14288794462066623856) |
| 4.0.8 | Offline NLP → `OperationPlan` output ✅ | Merged (Session 14288794462066623856) |
| 4.0.9 | Offline NLP unit tests ✅ | Merged (Session 8279024761787825031) |
| 4.0.10 | Offline NLP E2E tests ✅ | Merged (Session 15569134686656851401) |

---

### 4.E2E — System Validation Testing (All Surfaces)

| # | Task | Notes |
|---|---|---|
| E.1 | CLI End-to-End Testing & Validation Report ✅ | Merged (Session 15082388319906953095). Bugs fixed locally. |
| E.2 | MCP End-to-End Testing & Validation Report ✅ | Merged. 100% Green (32/32 Ops tested). |
| E.3 | Svelte UI End-to-End Testing & Validation Report ✅ | Merged (Session 8851318901150698695). 100% Green. |
| E.4 | Overall System Performance & Latency Benchmarking ✅ | Merged (Session 3687747131412548067) |

---

### 4.F — PDF Viewer, Annotations & Native Menu *(Pre-Release Blocker)*

> **Why this is required before release:** PaperPilot without a PDF viewer and basic annotation support is incomplete as a product. Users cannot preview what they are operating on, cannot annotate documents, and cannot fill forms — all baseline expectations for any PDF tool. Without these, users still need Acrobat alongside PaperPilot.

#### 4.F.1 — Embedded PDF Viewer

| # | Task | Notes |
|---|---|---|
| F.1.1 | Integrate `pdfjs-dist` into Svelte app | ✅ Add `pdfjs-dist` npm package; configure the worker in `apps/desktop/` |
| F.1.2 | PDF Viewer component (`PdfViewer.svelte`) | ✅ Render pages on `<canvas>` elements with scroll, zoom in/out, fit-to-width |
| F.1.3 | Page thumbnail strip | ✅ Vertical scrollable strip showing page thumbnails for navigation |
| F.1.4 | Document info panel | ✅ Show page count, file size, title, author, PDF version |
| F.1.5 | Viewer ↔ DropZone integration | ✅ Selecting a file in the DocumentList opens it in the viewer pane |

#### 4.F.2 — Annotations

| # | Task | Notes |
|---|---|---|
| F.2.1 | Highlight tool | ✅ Select text on a page and apply a coloured highlight (yellow, green, pink) |
| F.2.2 | Underline & Strikethrough | ✅ Text markup tools matching standard annotation conventions |
| F.2.3 | Sticky note / Comment | ✅ Click anywhere on a page to attach a floating comment bubble |
| F.2.4 | Free-draw tool | ✅ Draw freehand lines or shapes (pen tool) on top of a page |
| F.2.5 | Annotation panel | ✅ Side panel listing all annotations in the document with page references |
| F.2.6 | Save annotations to PDF | ✅ Flatten or embed annotations into the PDF via `paperpilot-pdf` |
| F.2.7 | Annotation unit tests | ✅ Verify annotation add/remove/serialize round-trips correctly |

#### 4.F.3 — Form Filling

| # | Task | Notes |
|---|---|---|
| F.3.1 | Detect interactive form fields | ✅ Use `pdfjs` to find `AcroForm` fields (text, checkbox, radio, dropdown) |
| F.3.2 | Render editable form overlays | ✅ Show native HTML `<input>` / `<select>` overlays on top of field positions |
| F.3.3 | Save filled form to PDF | ✅ Write field values back into the PDF file via `paperpilot-pdf` |
| F.3.4 | Form fill E2E test | ✅ Load a form PDF, fill all fields, save, reload and verify field values persist |

#### 4.F.4 — Custom HTML Titlebar

| # | Task | Notes |
|---|---|---|
| F.4.1 | Disable Native Decorations | ✅ Set `decorations: false` in `tauri.conf.json` |
| F.4.2 | Remove Native Menu | ✅ Remove Rust `menu::build_menu` from `lib.rs` |
| F.4.3 | `Titlebar.svelte` Component | ✅ Custom HTML/CSS component with drag regions and controls |
| F.4.4 | File/Edit/View/Window Menus | ✅ HTML dropdowns replicating old native menu |
| F.4.5 | Layout Integration | ✅ Mount `<Titlebar />` globally in `+layout.svelte` |

#### 4.F.5 — End-to-End Testing (Phase 4.F)

| # | Task | Notes |
|---|---|---|
| F.5.1 | Viewer E2E Test | ✅ Completed |
| F.5.2 | Annotations E2E Test | ✅ Completed |
| F.5.3 | Form Filling E2E Test | ✅ Form tests are fully implemented and verified in F.3 |
| F.5.4 | Menu / Tauri IPC Test | ✅ Completed |

#### 4.F.6 — PDF Viewer UX Improvements

| # | Task | Notes |
|---|---|---|
| F.6.1 | View Button on Uploads | ✅ Completed (PR #73) |
| F.6.2 | Collapsible Sidebars & Viewer Panels | ✅ Completed (PR #75, PR #76) |
| F.6.3 | Native "File -> Open" & Window Permissions | ✅ Completed (PR #74) |

#### 4.F.7 — Multi-Document Workspace & Toolbar Precision `[COMPLETED]`

| # | Task | Notes |
|---|---|---|
| F.7.1 | Editable Page Jump Input + Zoom Presets | ✅ Completed (PR #77) — Type page number + Enter; quick zoom presets |
| F.7.2 | Multi-Document Tab Bar `[Tab1][Tab2][+]` | ✅ Completed (PR #78) — Tab bar, tab switching, close tabs, and `+` native picker |
| F.7.3 | Right-Docked Operations Panel | ✅ Completed (PR #78) — Docked inspector side-rail, remove bottom wrapping |

#### 4.F.8 — Desktop Instant Open & Power UX `[COMPLETED]`

| # | Task | Notes |
|---|---|---|
| F.8.1 | Instant Document Open on Drop/Select | ✅ Completed (PR #79) — Dropping or selecting a PDF immediately opens it in viewer |
| F.8.2 | Raycast Command Palette (`Ctrl+K` / `⌘K`) | ✅ Completed (PR #79) — Floating fuzzy search overlay over operations, documents, and actions |
| F.8.3 | Floating Canvas-First Annotation Toolbar | ✅ Completed (PR #79) — Floating pill toolbar over PDF canvas with backdrop blur |
| F.8.4 | Resizable Right Panel & Titlebar Anchor Fix | ✅ Completed (PR #79) — Draggable split-handle (240px-600px) and anchored window controls |
#### 4.F.9 — Canvas Search & Direct Thumbnail Page Management `[COMPLETED]`

| # | Task | Notes |
|---|---|---|
| F.9.1 | Canvas Text Search Bar (`Ctrl+F` / `⌘F`) | ✅ Completed (PR #81) — Floating search bar over canvas with match navigation (`1 of 12`, `‹ / ›`), Titlebar window controls anchor fix |
| F.9.2 | Direct Thumbnail Page Reordering | ✅ Completed (PR #80) — Drag-and-drop page reordering directly in thumbnail sidebar |
| F.9.3 | Thumbnail Quick Actions (Rotate & Delete Page) | ✅ Completed (PR #80) — Hover quick buttons on thumbnail cards to rotate or delete individual pages |

#### 4.F.10 — Stirling-Style Categorized Tools Dock `[COMPLETED]`

| # | Task | Notes |
|---|---|---|
| F.10.1 | Searchable Tool Directory & Filtering | ✅ Completed (PR #82) — `🔍 Search tools...` input with instant tag & name filtering |
| F.10.2 | Categorized Tool Accordions | ✅ Completed (PR #82) — Quick Actions, Page Mgmt, Edit, Optimize, Security, Convert, AI |
| F.10.3 | Slide-in Inspector & Param Config | ✅ Completed (PR #82) — Inline parameter inspector with `‹ Back` navigation |

#### 4.F.11 — Visual Split-Screen Pixel PDF Diff Slider `[COMPLETED]`

> **Core Basic / Free Feature**: Bypasses Adobe Acrobat Pro's $239/yr paywall and Stirling PDF's text-only limitation.

| # | Task | Notes |
|---|---|---|
| F.11.1 | Visual Pixel Diff Engine | ✅ Completed (PR #83) — HTML5 2D pixel comparison (`getImageData()`), red/green discrepancy map |
| F.11.2 | Interactive Split Slider | ✅ Completed (PR #83) — Draggable divider handle across dual canvas layers (before/after) |
| F.11.3 | Synchronized Paging & Navigation | ✅ Completed (PR #83) — Multi-document selection, side-by-side mode, keyboard shortcut (`Ctrl+D`) |

#### 4.F.12 — Custom CSS Injection for HTML/Markdown/Excel → PDF `[COMPLETED]`

> **Core Basic / Free Feature**: Overcomes Stirling PDF's unstyled conversions and Adobe Acrobat Pro's costly enterprise layout paywalls.

| # | Task | Notes |
|---|---|---|
| F.12.1 | CSS Preset Library (5 Themes) | ✅ Completed (PR #84) — `github`, `elegant`, `minimal`, `branded`, `compact` print-optimized stylesheets |
| F.12.2 | Custom CSS File & Inline Injection | ✅ Completed (PR #84) — Desktop `.css` file upload + inline textarea + CLI `--css` flags |
| F.12.3 | Excel → Styled Semantic Table Pipeline | ✅ Completed (PR #84) — Spreadsheet `.xlsx`/`.csv` parsed into semantic HTML `<table>` with CSS styling |
| F.12.4 | Standalone Svelte 5 `CssInjectionPanel` | ✅ Completed (PR #84) — Modern Svelte 5 runes component for tool inspector integration |



#### 4.F.13 — Stirling Parity: Category Filter Pills & Extended Tools `[COMPLETED]`

> **Core Basic / Free Feature**: Enhances Operations Dock ergonomics with category filter pills, instant 1-click filtering, and Stirling tools parity.

| # | Task | Notes |
|---|---|---|
| F.13.1 | Horizontal Category Filter Pills Bar | ✅ Completed (PR #87, Session 12318669239840057258) — `[All]`, `[⚡ Quick]`, `[📑 Pages]`, `[✍️ Edit]`, `[🔒 Security]` with 1-click category switching |
| F.13.2 | To-PDF Conversions in Dock | ✅ Completed (PR #87, Session 12318669239840057258) — Markdown → PDF, HTML → PDF, Images → PDF with CSS presets |
| F.13.3 | Page Numbers & Bates Stamping | ✅ Completed (PR #87, Session 12318669239840057258) — Dynamic page numbering, position selector, margin controls |
| F.13.4 | Auto Remove Blank Pages & Text Redaction | ✅ Completed (PR #87, Session 12318669239840057258) — Blank page stream sensitivity detection and removal |

#### 4.F.14 — Native Save-As Dialog & Editable Output Path for Operations

> **Core UX Fix**: Users currently have no visibility into where output files are written. This closes that gap with a native save dialog for Merge/Split and an editable output path field for all other tools.

| # | Task | Notes |
|---|---|---|
| F.14.1 | Native "Save As" dialog for Merge & Split | ✅ Completed — Show `@tauri-apps/plugin-dialog` `save()` before running Merge or Split; user picks filename & destination; cancelling aborts the operation |
| F.14.2 | Editable output path field for all other tools | ✅ Completed — Inspector shows a pre-filled, editable "Output file:" path input (auto-suggested from source dir) that the user can override before running |
| F.14.3 | Full absolute path in success toast | ✅ Completed — Success notification always displays the exact absolute path where the output was written |

#### 4.F.15 — High-Volume File Ingestion: Lazy Byte Loading & Tab Limit Management

> **Core Stability & Scalability Fix**: Loading 50–100 files simultaneously currently triggers out-of-memory crashes due to eager byte reads, and crashes tab ergonomics. Introduces lightweight metadata ingestion, lazy on-demand byte reading for the canvas viewer, and an 8-tab ceiling.

| # | Task | Notes |
|---|---|---|
| F.15.1 | Lightweight File Metadata Command (`get_file_metadata`) | ✅ Completed — Rust Tauri command querying file path, name, and size via `std::fs::metadata` without allocating file payload |
| F.15.2 | Lazy Ingestion in DropZone | ✅ Completed — Instant multi-file browsing via lightweight placeholders; zero upfront byte allocation |
| F.15.3 | Tab Bar Limit & Library Separation | ✅ Completed — Decouple Documents Library list from open tabs; enforce max 8 active tabs with auto-rotation |
| F.15.4 | On-Demand Viewer Byte Loading | ✅ Completed — Load raw file bytes and blob URLs only when a document is actively viewed in the canvas; revoke URLs on switch |

#### 4.F.16 — Operation Panel Tool Audits: Correct File Extensions, Creation Workflows & Compare Integration

> **Core Ergonomics & Accuracy Fix**: Audit and fix tool outputs (e.g. Word exports ending in `.pdf`), connect Compare directly to `PdfVisualDiff`, and allow PDF creation tools (`md_to_pdf`, `html_to_pdf`, `img_to_pdf`) without requiring a pre-opened PDF.

| # | Task | Notes |
|---|---|---|
| F.16.1 | Output Extension Mapping | ✅ Completed — Auto-suggest correct extensions (`.docx`, `.xlsx`, `.md`, `.pptx`, directory for images) instead of fallback `.pdf` |
| F.16.2 | Direct Compare / Diff Wiring | ✅ Completed — Wire "Compare PDFs" card directly to `appState.toggleDiffView(true)` with multi-document validation |
| F.16.3 | PDF Creation Tool Unblocking | ✅ Completed — Allow `md_to_pdf`, `html_to_pdf`, and `img_to_pdf` without requiring an already opened PDF; integrate input file pickers |

#### 4.F.17 — Swagger / OpenAPI Interactive Documentation Tab

> **Interactive Developer Experience**: Dedicated native OpenAPI/Swagger explorer tab inside the desktop app with live schema navigation, endpoint documentation, curl code generators, and local gateway health checking.

| # | Task | Notes |
|---|---|---|
| F.17.1 | API Docs Component (`ApiDocsView.svelte`) | ✅ Completed (PR #107) — Interactive Swagger/OpenAPI documentation browser with endpoint search, method filtering, and try-it-out capabilities |
| F.17.2 | Sidebar & Tab Navigation Integration | ✅ Completed (PR #107) — Added API Docs tab to desktop navigation rail (`⚡`) and application state |
| F.17.3 | OpenAPI Spec Bundling & Schema Ingestion | ✅ Completed (PR #107) — Static spec JSON ingestion from `docs/api/openapi.json` with gateway ping check |

#### 4.F.18 — Developer Mode (Local REST API Sidecar)

> **Strategic Growth Feature**: Allows developers to instantly run local automations via `localhost:7823` from Python, n8n, etc., without requiring them to install Docker or the standalone CLI. Keeps the free tier insanely competitive vs Stirling PDF.

| # | Task | Notes |
|---|---|---|
| F.18.1 | Bundle `paperpilot-gateway` Sidecar | ✅ Completed (PR #109) — Integrate `paperpilot-gateway` crate into `apps/desktop/src-tauri` workspace dependency |
| F.18.2 | "Developer Mode" Settings Toggle | ✅ Completed (PR #109) — Added `Developer Mode` toggle with live status, port indicator, and Swagger quick-link in `SettingsPanel.svelte` |
| F.18.3 | Sidecar Tauri IPC Spawning | ✅ Completed (PR #109) — Implemented `get_gateway_status`, `start_gateway`, and `stop_gateway` Tauri commands with tokio async lifecycle |
| F.18.4 | API Docs UI Integration | ✅ Completed (PR #109) — Bound Developer Mode toggle to AppState, updating sidebar gateway badge and API Docs live connectivity |
| F.18.5 | Sidecar Lifecycle Management | ✅ Completed (PR #109) — Handled graceful termination on toggle disable and Tauri application exit hooks |

#### 4.F.19 — Interactive Page Organizer & Thumbnail Context Menu (`PdfThumbnails.svelte`)

> **Core Ergonomics & Usability**: Elevates thumbnail page management into a first-class visual workstation with explicit drag grip affordance, drop insertion indicators, persistent quick action buttons, and a rich right-click context menu (`Rotate 90/180/270`, `Delete`, `Duplicate`, `Extract`).

| # | Task | Notes |
|---|---|---|
| F.19.1 | Drag Grips & Insertion Indicators | ✅ Completed (PR #131) — Visible `⋮⋮` drag handle, active grab cursors, and blue insertion drop-indicator lines between pages (`reports/UI_THUMBNAIL_ORGANIZER_REPORT.md`) |
| F.19.2 | High-Contrast Thumbnail Action Bar | ✅ Completed (PR #131) — Quick action buttons on each thumbnail with tooltip cues for 1-click Rotate (`🔄`) and Delete (`🗑️`) |
| F.19.3 | Thumbnail Right-Click Context Menu | ✅ Completed (PR #131) — Custom floating context menu with Rotate 90°/180°/270°, Duplicate page, Delete page, and Extract page to new document |
| F.19.4 | In-Place Document Refresh & Undo Toast | ✅ Completed (PR #131) — Seamless background IPC execution (`pdf_rotate`, `pdf_delete_pages`, `pdf_reorder_pages`, `pdf_extract_pages`) with instant canvas reload (`refreshCurrentDocument()`) |

---

### 4.E2E Round 2 — Full System Re-Validation (Post Viewer & Annotations)

> **Trigger condition:** All of Phase 4.F must be merged before Round 2 begins. These tests must re-run the full surface area including the new Viewer, Annotation, Form Fill, and Menu features.

| # | Task | Notes |
|---|---|---|
| R2.E1 | CLI End-to-End Re-Validation | ✅ Completed |
| R2.E2 | MCP End-to-End Re-Validation | ✅ Completed |
| R2.E3 | UI Full Re-Validation (Viewer + Annotations + Phase 4.F Features) | ✅ Completed (PR #85, Session 11778243636880036911) — Playwright E2E suite covering Tabs, Canvas Search, Tools Dock, Diff Slider, CSS Injection |
| R2.E4 | System Performance Re-Benchmark | Re-run all benchmarks; add viewer render latency (ms/page), annotation save time |
| R2.E5 | Autonomous Agent QA Testing Suite (Vercel Agent Browser / Stagehand) | ✅ Completed — QA-1 through QA-8 fully verified and merged (PR #100 - PR #108) |

---

### 4.E2E Round 3 — Tri-Interface E2E Verification & Interactive Living Documentation (CLI + MCP + REST API)

> **The Unified Reference & Verification Suite**: Tests all 44 tools across all three developer interfaces (CLI, MCP, and REST API). Takes pre- and post-execution PDF snapshots (file size, page counts, integrity checks), generates copy-pasteable working examples for each tool, and compiles a comprehensive Markdown reference document (`reports/TRI_INTERFACE_E2E_AND_DOCS.md`). If any tool fails on an interface, records root cause and failure details without masking.

| # | Task | Notes |
|---|---|---|
| R3.E1 | Group 1: Structural & Page Operations Tri-Interface Test (10 tools) | ✅ Completed (PR #110) — Verified CLI, MCP, and REST API across tools 1–10 |
| R3.E2 | Group 2: Security & Integrity Operations Tri-Interface Test (7 tools) | ✅ Completed (PR #112) — Verified CLI, MCP, and REST API across security operations |
| R3.E3 | Group 3: Content Transformation & OCR Tri-Interface Test (9 tools) | ✅ Completed (PR #112 & #113) — Verified CLI, MCP, and REST API across content extraction and OCR operations |
| R3.E4 | Group 4: Forms, Metadata & Optimization Tri-Interface Test (8 tools) | ✅ Completed (PR #111 & #113) — Verified CLI, MCP, and REST API across forms, metadata, and PDF optimization |
| R3.E5 | Group 5: Advanced Conversion & Intelligence Tri-Interface Test (10 tools) | ✅ Completed (PR #111) — Verified CLI, MCP, and REST API across format conversions and AI helpers |
| R3.E6 | Tri-Interface Automated Master Runner & Living Documentation Generator | ✅ Completed (PR #110–#113) — Living Documentation consolidated at `reports/TRI_INTERFACE_E2E_AND_DOCS.md` |
| R3.FIX.1 | Crate Isolated Fix: CLI Argument & Flag Normalization | ✅ Completed (PR #114) — `paperpilot-cli` |
| R3.FIX.2 | Crate Isolated Fix: Gateway MCP/API Parameter Mapping | ✅ Completed (PR #115) — `paperpilot-gateway` |
| R3.FIX.3A | Core Engine Operation Fixes (Decrypt, Split, Burst) | ✅ Completed (PR #117) — `paperpilot-pdf` |
| R3.FIX.3B | MCP & Gateway E2E Stability & Param Resolution | ✅ Completed (PR #116) — `paperpilot-mcp` & `paperpilot-gateway` |
| R3.FIX.4A | CLI Polish: Inline JSON Annotate, Burst Dir Creation & Sign Alias | ✅ Completed (PR #119) — `paperpilot-cli` |
| R3.FIX.4B | Gateway REST API: Format-to-Tool JSON Mapping for `/convert` | ✅ Completed (PR #118) — `paperpilot-gateway` |
| R3.FIX.4C | Master Tri-Interface E2E Suite: All 44 Tools, 100% Parity | ✅ Completed (PR #120) — 100% Parity (44/44 CLI, 44/44 MCP, 44/44 API) verified at `reports/TRI_INTERFACE_E2E_100_VERIFIED.md` |
| R3.FE.1 | OperationsPanel: Register All 44 Operations & Parameter Cards | ✅ Completed (PR #121) — `apps/desktop/src/lib/components/layout/OperationsPanel.svelte` (`reports/UI_44_TOOLS_PANEL_REPORT.md`) |
| R3.FE.2 | Playwright E2E Suite: Data-Driven 44-Operations Parity Test | ✅ Completed (PR #123) — `apps/desktop/tests/e2e_44_operations_parity.spec.ts` (`reports/UI_44_OPERATIONS_E2E_SCORECARD.md` 44/44 PASS) |
| R3.FE.3 | Canvas Interactive E2E: Sticky Notes, Markup & Visual Diff Slider | ✅ Completed (PR #122) — `apps/desktop/tests/e2e_canvas_viewer_features.spec.ts` (`reports/UI_CANVAS_ANNOTATIONS_DIFF_REPORT.md`) |





---

### 4.1 — LLM & Advanced Local SLM Infrastructure (Pro & Teams Tier) `[PARALLEL]`

> Powers the Pro/Teams tier for complex, multi-step conversational agent planning (e.g. "Merge invoice1 and invoice2, strip page 3, rotate page 2 clockwise, and compress under 2MB").
> **Designated Local SLM:** `SmolLM-135M-Instruct` (Q4 GGUF, ~75MB on disk, ~120MB active RAM, ~15ms latency). Zero server dependencies, private local execution.
> **Cloud / External Options:** Local Ollama (2–8GB models) or Bring-Your-Own-Key (BYOK) OpenAI / Gemini / Claude API.
> Implements the same `NlpResolver` trait from 4.0.2 — drop-in swap.
> **Licensing: Apache 2.0 (Community) — key/provider config is user-supplied**

| # | Task | Notes |
|---|---|---|
| 4.1.1 | Create `paperpilot-ai` crate | LLM-specific implementations; depends on `paperpilot-nlp` for the shared trait |
| 4.1.2 | `LlmNlpResolver` struct | Implements `NlpResolver` via LLM API call |
| 4.1.3 | `OllamaNlpResolver` struct | Implements `NlpResolver` via local Ollama endpoint (offline but heavier, ~2–8 GB) |
| 4.1.4 | Embedded Local SLM (`SmolLM-135M` / llamafile) | Built-in offline small language model (~75MB Q4 GGUF) for conversational, multi-step pipeline planning without external servers |
| 4.1.5 | OpenAI-compatible API client | Works with OpenAI, Gemini, Groq, LM Studio, etc. (BYOK - Bring Your Own Key) |
| 4.1.6 | AI provider config in settings | User picks resolver: Offline / Ollama / OpenAI-compatible; enters endpoint + key |
| 4.1.8 | Embedded Documentation RAG (`sqlite-vec`) ✅ | Merged (Session 6788529762952435381, PR #136) — In-memory/embedded SQLite vector store indexing all living documentation (`TRI_INTERFACE_E2E_AND_DOCS.md`, user guides, tool specs). Answers user questions directly in the chat with zero hallucinations, exact copy-pasteable CLI/API snippets, and sub-millisecond retrieval |

---

### 4.2 — Operation Planning `[BLOCKED BY 4.0, 4.1]`

| # | Task | Notes |
|---|---|---|
| 4.2.1 | Define `OperationPlan` type | Ordered list of typed operations — shared between offline and LLM resolvers |
| 4.2.2 | System prompt design (LLM mode) | Instruct LLM to output only structured `OperationPlan` JSON |
| 4.2.3 | Natural language → `OperationPlan` parser | Route to offline or LLM resolver based on user config |
| 4.2.4 | Plan validation | Validate plan against available operations before showing to user |
| 4.2.5 | Plan serialization | JSON schema for plans, used by UI and MCP |
| 4.2.6 | Mode indicator in UI | Show a small badge: "Offline NLP" or "AI (GPT-4o)" so users know which mode ran |

---

### 4.3 — Natural Language UI `[BLOCKED BY 4.2, PARALLEL with 4.4]`

| # | Task | Notes |
|---|---|---|
| 4.3.1 | Command box component | Text input in main UI |
| 4.3.2 | "Thinking" state | Show spinner while AI/NLP processes |
| 4.3.3 | Plan display component | Show numbered list of interpreted operations |
| 4.3.4 | Plan edit UI | Allow user to add/remove/reorder steps before running |
| 4.3.5 | Voice input | Microphone → speech-to-text → feeds the NLP command box (integrates with Phase 2 MCP) |
| 4.3.6 | NLP mode toggle in settings | Simple switch: "Use Offline NLP" / "Use AI (requires key)" |
| 4.3.7 | Multi-turn Chat Panel (`PdfChatPanel.svelte`) | ✅ Completed (PR #124) — Collapsible side-drawer in `ViewerRightPanel.svelte` with natural language command prompt, quick suggestion chips, offline NLP IPC resolver bridge (`resolve_natural_language`), and interactive action execution cards |
| 4.3.8 | Developer Mode: Tri-Interface Action Inspector & CodeGen (`ActionInspector.svelte`) | Toggleable Dev Mode in Desktop GUI (Ctrl+Shift+I). Reactive inspector showing live code generation across 4 tabs: CLI command (with batch loop toggle), cURL REST API, native MCP tool call payload JSON, and Python/Node/Rust scripts. Includes session action log and "Export Workflow as Pipeline" script generation |
| 4.3.9 | Actionable RAG Workspace & Bi-Directional Citation Canvas | Bridges document chat with canvas viewer: clicking citations jumps directly to page/bounding-box with visual highlight pulse. Allows chat prompts to directly propose and trigger execution plans on active document (beating Adobe Acrobat AI Assistant with 100% offline privacy and zero subscription) |
| 4.3.10 | Context-Aware NLP Resolution Bridge | ✅ Completed (PR #128) — Enhances `resolve_natural_language` IPC and `paperpilot-nlp` with `ResolverContext`: automatically binds active viewer document path to operations (fixing "split pages 1 to 2 requires file path" error, `reports/CONTEXT_AWARE_NLP_REPORT.md`) |
| 4.3.11 | Multi-Document Omnibar (`GlobalCommandBar.svelte`) | ✅ Completed (PR #134) — Universal keyboard-first command bar docked at bottom of Documents view (`Ctrl+K` / `Cmd+K`) with `@filename` autocomplete dropdown popover, quick action chips, offline NLP plan resolution, and inline blueprint preview (`reports/UI_GLOBAL_COMMAND_BAR_REPORT.md`) |
| 4.3.12 | Master E2E AI Chat Pipeline Test Suite (All 44 Tools) | ✅ Completed (PR #133) — Playwright E2E suite (`tests/e2e_ai_chat_real_pipeline.spec.ts`) verifying AI Chat panel across all 44 PDF operations with 100% pass rate (`reports/UI_CHAT_44_OPERATIONS_E2E_SCORECARD.md` 44/44 PASS, `reports/AI_CHAT_REAL_PIPELINE_E2E_REPORT.md`) |
| 4.3.13 | Searchable Command Help & Cheat Sheet Drawer (`ChatCheatSheet.svelte`) & Adjustable Right Panel | ✅ Completed (PR #129) — Searchable command palette and cheat sheet drawer inside `PdfChatPanel.svelte` covering all 44 PDF operations, hotkey `?` / `💡 Examples` button, one-click prompt insertion, plus dynamic width scaling (240px–800px) and auto-comfort chat width in `ViewerRightPanel.svelte` (`reports/UI_CHAT_CHEATSHEET_REPORT.md`) |
| 4.3.14 | Editable Action Blueprint Cards & 44-Tool Execution Dispatch (`PdfChatPanel.svelte`) | ✅ Completed (PR #130) — Interactive, editable action cards in chat stream (`EditableActionCard.svelte`): target pages selector (`all`, `current`, custom `1, 4`), contextual hints, format badges, and complete 44-tool MCP execution mapping with fallback paths (`reports/UI_EDITABLE_CHAT_ACTION_CARDS_REPORT.md`) |

---

### 4.4 — Confirmation and Execution `[BLOCKED BY 4.2, PARALLEL with 4.3]`

| # | Task | Notes |
|---|---|---|
| 4.4.1 | Confirmation dialog | Show plan with Cancel / Run buttons |
| 4.4.2 | Plan → job execution | Convert confirmed plan into sequential job run |
| 4.4.3 | Error recovery UI | If one step fails, show which step and allow retry |
| 4.4.4 | MCP-based execution path | Both NLP modes produce plans executed via MCP tools |

---

### 4.5 — CLI AI Integration `[BLOCKED BY 4.2, PARALLEL with 4.3]`

| # | Task | Notes |
|---|---|---|
| 4.5.1 | `paperpilot ask "..."` command | Sends natural language to configured resolver (offline or LLM), shows plan, confirms, runs |
| 4.5.2 | Non-interactive mode | `--yes` flag to skip confirmation for scripting |
| 4.5.3 | `--offline` flag | Force offline NLP even if LLM key is configured |

---

### 4.6 — AI PDF Creation `[BLOCKED BY 4.2, 1.3.40–1.3.45]`

**Goal:** A user or AI agent describes what they want and PaperPilot generates a professional PDF from scratch — no design skills, no templates to configure manually.

| # | Task | Notes |
|---|---|---|
| 4.6.1 | Natural language → PDF content | AI generates structured Markdown content from a prompt (e.g. "create a consulting invoice for Acme Corp, $5,000, due in 30 days") |
| 4.6.2 | AI template selection | AI picks the best built-in template (invoice, contract, report) based on the user's intent |
| 4.6.3 | AI fills template variables | AI extracts structured data from the prompt and populates all template fields |
| 4.6.4 | Review & edit before rendering | Show AI-filled template in editor before generating the PDF |
| 4.6.5 | `pdf_create` MCP tool | AI agents call `pdf_create` with a prompt — returns a ready PDF; no human in the loop needed |
| 4.6.6 | `paperpilot create "..."` CLI | `paperpilot create "monthly report for Q3 sales" --template report -o q3.pdf` |
| 4.6.7 | Multi-turn creation (chat mode) | User refines the PDF via follow-up prompts: "change the due date to Oct 31" → PDF updates live |
| 4.6.8 | E2E AI creation tests | Generate real PDFs from prompts and verify structure and content (No Mocking) |

---

### 4.7 — Automation Engine `[BLOCKED BY 4.4, PARALLEL with 4.5, 4.6]`

**Goal:** Turn PaperPilot into a headless automation backend that runs document workflows without human interaction — triggered by file system events, scheduled runs, or CLI/MCP invocations.

**Exit condition:** A folder watcher daemon can be configured to auto-OCR, watermark, and compress any PDF dropped into a watched directory. A CSV + template can generate a batch of 1,000 personalized PDFs unattended. All batch jobs emit a machine-readable execution receipt.

---

| # | Task | Notes |
|---|---|---|
| 4.7.1 | Hot Folder Watcher daemon | Background service watching a configured directory; triggers a named recipe when a `.pdf` file is created or moved in. Config via `paperpilot watch --dir ~/inbox --recipe ocr-and-compress`. Uses `notify-rs` (cross-platform filesystem events). |
| 4.7.2 | Recipe / Pipeline definition format | Simple TOML config file defining ordered steps: `[[step]] tool = "pdf_ocr"`, `[[step]] tool = "pdf_compress"`. Referenced by the watcher and CLI. |
| 4.7.3 | `paperpilot run-recipe` CLI command | `paperpilot run-recipe --recipe pipeline.toml --input file.pdf --output out/` — headless, exit-code-clean, scriptable. |
| 4.7.4 | CSV/JSON Bulk Fill (Mail Merge) | `paperpilot fill-batch --template form.pdf --data records.csv --output-dir ./out/` — iterates rows, fills AcroForm fields, flattens, saves as `{row.name}_{date}.pdf`. |
| 4.7.5 | Dynamic token naming | String interpolation for output paths: `{input_name}`, `{date:YYYY-MM-DD}`, `{index:04}`, `{hash:8}`, `{metadata:title}`. Used in batch fill and watcher recipes. |
| 4.7.6 | Conditional logic in recipes | TOML-defined conditions: `if_scanned = true → run ocr first`, `if_page_count_gt = 50 → compression = aggressive`. Evaluated at runtime before each step. |
| 4.7.7 | Execution Receipt / Audit Log | After every batch job or recipe run, emit `execution_receipt.json` alongside output: `job_id`, `timestamp`, `input_hash` (SHA-256), `output_hash`, `operations_applied[]`, `execution_time_ms`, `errors[]`. |
| 4.7.8 | Desktop UI for Automation | "Automations" tab in desktop app: list configured watchers, enable/disable, view last execution receipt per watcher, manually trigger a recipe against open document. |
| 4.7.9 | MCP tools for automation | `recipe_run(recipe_toml, input)`, `batch_fill(template, data_json)`, `watcher_start(dir, recipe)`, `watcher_stop(id)` — allows AI agents to orchestrate headless document automation. |

---

### 4.8 — Client-Side WebAssembly (WASM) & Edge Engine (`paperpilot-wasm`) `[VIRAL GROWTH HOOK]`

> Delivers zero-install client-side browser execution (100% privacy, $0 server cost) to massively accelerate user adoption, paired with Cloudflare Workers Wasm edge execution. (Spec 015)

| # | Task | Notes |
|---|---|---|
| 4.8.1 | `paperpilot-wasm` Complete Client-Side Engine & Web Worker Bridge ✅ | Merged (Session 14466591561004330997, PR #135) — Complete `paperpilot-wasm` crate with `wasm-bindgen`, in-memory `Uint8Array` buffer adapters for 6 core operations (`merge`, `split`, `rotate`, `compress`, `encrypt`, `watermark`), and TypeScript Web Worker bridge (`pdfWorker.ts`) |
| 4.8.2 | Zero-Install Web App Demo ✅ | Merged (Session 10211896095989692590, PR #137) — Standalone web application in `apps/web/` (Vite + Svelte 5 / TS) powered by `paperpilot-wasm` Web Worker bridge. Supports 6 core ops in-browser, privacy badge, and high-converting Desktop download funnel CTA. |
| 4.8.4 | Expand WASM Engine & Web Suite to 12 Pure-Rust Tools ✅ | Merged (Session 16017685921024489423, PR #138) — Implemented 6 additional pure-memory operations (`delete_pages`, `extract_pages`, `reorder_pages`, `crop`, `flatten`, `set_metadata`) in `paperpilot-wasm`, updated Web Worker bridge, and added full interactive UI cards in `apps/web/`. |
| 4.8.5 | WASM 12 Tools Playwright E2E Suite & Scorecard ✅ | Merged (Session 10562587771207472872, PR #139) — Automated in-browser Playwright E2E suite (`apps/web/tests/e2e_wasm_12_tools.spec.ts`), pure-Rust in-memory encryption, living scorecard at `reports/WASM_12_TOOLS_E2E_SCORECARD.md` (12/12 PASS), and documentation in `wiki/08-Wasm-E2E-Testing.md`. |
| 4.8.3 | Cloudflare Workers Edge Microservice ✅ | Merged (Session 803293663318829942, PR #140) — Deployed `paperpilot-wasm` into Cloudflare Workers standalone service in `apps/edge/` with 12 sub-10ms memory REST endpoints, Vitest suite (14/14 PASS), `reports/EDGE_MICROSERVICE_REPORT.md`, and `wiki/10-Cloudflare-Edge-Service.md`. |

---

### 4.9 — Embedded Web Distribution: Drop-In Widget & CMS Plugins `[DISTRIBUTION FLYWHEEL]`

> Turns PaperPilot into a drop-in embeddable tool for any website. Website owners paste 2 lines of HTML and get a full, branded, client-side PDF portal — powered by `paperpilot-wasm` with zero server load and zero hosting cost. (Spec 019)

**[v1.0 CO-LAUNCH TARGET]** — Released simultaneously with PaperPilot Free Desktop and Cloudflare Edge. Powered by `paperpilot-wasm` (expanded via Phase 5.1.1 / 5.5.1).

| # | Task | Notes |
|---|---|---|
| 4.9.1 | `embed.js` Universal Drop-In CDN Script | Tiny JS loader (< 5KB gzip) served from Cloudflare CDN. Reads `data-tools`, `data-theme`, `data-brand-color`, and `data-logo-url` attributes from the host `<div>`. Mounts a Shadow DOM container to isolate styles from the host site. Lazy-loads the `paperpilot_wasm_bg.wasm` bundle and the Svelte 5 widget. |
| 4.9.2 | Brandable Svelte 5 Embed Widget (`apps/embed/`) | Configurable, responsive, iframe-safe Svelte 5 UI component. Accepts theme tokens from `embed.js`. Renders the selected tool subset (e.g. only `merge,compress`). Includes the "⚡ Powered by PaperPilot" viral badge (removable in Pro tier). |
| 4.9.3 | Shadow DOM Isolation & CSS Theme Tokens | Widget renders inside a Shadow DOM root so host site CSS never bleeds in. Brand colors, font size, and border-radius exposed as `--pp-brand-color`, `--pp-radius` CSS custom properties, settable via `data-` attributes or JS API. |
| 4.9.4 | JavaScript Embed API (`window.PaperPilot`) | Programmatic API for advanced users: `PaperPilot.mount('#target', { tools: ['merge'], theme: 'light' })`, `PaperPilot.on('complete', callback)`, `PaperPilot.unmount()`. Enables headless integration into React/Vue/Angular apps. |
| 4.9.5 | WordPress Plugin (`paperpilot-wp`) — Gutenberg Block | PHP plugin published to `wordpress.org` registry. Registers a Gutenberg Editor block (`PaperPilot PDF Portal`) and a legacy shortcode `[paperpilot_tools]`. Block settings panel exposes tool selection, theme, and brand color. Covers 43% of the web (800M+ sites). |
| 4.9.6 | WordPress Plugin — WooCommerce Digital Downloads Integration | On WooCommerce order completion webhook, call Cloudflare Edge API to auto-watermark (`Licensed to {email} — Order #{id}`) and optionally encrypt purchased PDF products. All server-side via `apps/edge/`, no customer data stored. |
| 4.9.7 | Webflow App & Framer Component | Package embed widget as a verified Webflow App and Framer Community Component. Agencies can install on unlimited client sites (lawyers, accountants, HR portals). |
| 4.9.8 | Shopify App ("Secure PDF Delivery") | Shopify App Bridge integration. Intercepts digital product fulfillment webhooks to auto-stamp and encrypt PDF downloads with order metadata via the Cloudflare Edge API (< 10ms). Submit to Shopify App Store. |
| 4.9.9 | Embed E2E Playwright Test Suite | Playwright tests that inject the `embed.js` script into a bare HTML fixture page, exercise all configurable `data-` attributes, run a PDF merge end-to-end through the widget, and assert the download is a valid `%PDF-` binary. |
| 4.9.10 | Embed Analytics & Upgrade Funnel | Anonymous, privacy-safe usage telemetry (tool name, file size bucket, success/error — no file content). Powers the "⚡ Powered by PaperPilot" badge click-through funnel to `paperpilot.app` for Desktop or Cloud Pro conversion. |
| 4.9.11 | `usepaperpilot.com` Official Web Portal & Playground | Official marketing and playground portal deployed to Cloudflare Pages. Features live Hero Embed Playground (dogfooding), Interactive Embed Code Generator for agencies, and Swagger/OpenAPI explorer for developers (`docs/OFFICIAL_WEBSITE_STRATEGY.md`). |
| 4.9.12 | `docs.usepaperpilot.com` Official Documentation Hub | Comprehensive 4-pillar docs portal: Getting Started, 44-tool Tri-Interface reference (CLI/MCP/API), `embed.js` integration guide, and interactive Swagger UI / Claude Desktop MCP setup (`docs/DOCUMENTATION_PORTAL_STRATEGY.md`). |

### Phase 5.5 — WASM Re-Export & Edge Sync Post-Migration
| # | Task | Notes |
|---|---|---|
| 5.2.1 | Pure-Rust OCR Engine Migration (`ocrs` via `rten`) ✅ | Merged (Session 10581530668658369493, PR #141) — Eliminated host C++ Tesseract/Leptonica dependencies in `paperpilot-pdf/src/operations/ocr.rs`. Powered by Robert Knight's `ocrs` and `rten` SIMD neural runtime with graceful fallback, `wiki/09-Pure-Rust-OCR.md`, and `reports/OCR_PURE_RUST_REPORT.md` (100% parity maintained). |
| 5.3.1 | Pure-Rust PDF Rasterization & Rendering (`hayro` + `tiny-skia`) ✅ | Merged (Session 15128032333344162811, PR #142) — Eliminated Google PDFium (`libpdfium.so`, `pdfium-render`) dependency in `paperpilot-pdf/src/operations/render.rs` using pure-Rust `hayro` vector interpreter and `tiny-skia` 2D rasterizer, `wiki/12-Pure-Rust-Rendering.md`, and `reports/RENDERING_PURE_RUST_REPORT.md` (100% parity maintained). |
| 5.1.1 | Pure-Rust Image & Hash WASM Expansion (15 Tools Suite) ✅ | Merged (Session 15762776336095880921, PR #143) — Expanded `paperpilot-wasm` and `apps/web/` to 15 tools with in-memory `images_to_pdf`, `extract_images`, and `pdf_hash` via `image` and `sha2`, `wiki/11-Wasm-15-Tools-Suite.md`, and `reports/WASM_15_TOOLS_E2E_SCORECARD.md` (100% parity verified). |
| 5.5.1 | Pure-Rust WASM 22-Tools Suite Expansion (Render, OCR, Text Extraction, Security) `[Spec 020]` ✅ | Completed (Session 8898129496130614008) — Re-exported pure-Rust `hayro` render, `ocrs` OCR, and added `extract_text`, `decrypt`, `page_numbers`, `header_footer`, and `pdf_info` into `paperpilot-wasm`. Rebuilt `pkg/` and verified 100% pass across all 22 operations in Playwright (`e2e_wasm_22_tools.spec.ts`) and Rust unit tests (`wasm_tests.rs`). |
| 5.6.1 | Pure-Rust High-Speed Office & Document Conversions (Zero-Chrome) `[Spec 024]` ✅ | Merged (PR #150, commit `9c3f5cc`) — Eliminated `headless_chrome` dependency and browser spawning across HTML, Markdown, and Excel to PDF conversions using pure-Rust `fulgur` engine. Latency dropped from ~2,500ms to sub-60ms. Validated 100% pass across tri-interface suite (132/132 assertions), documented in `wiki/18-Pure-Rust-Office-Conversions.md` and `reports/PURE_RUST_OFFICE_CONVERSIONS_REPORT.md`. |
| 5.6.2 | Universal High-Ratio PDF Compression Engine `[Spec 026]` ✅ | Merged (commit `16f8ede`) — Replaced no-op stream compression with true image traversal, JPEG recompression, downsampling, and `--quality` control across CLI, MCP, Gateway API, WASM, and Desktop UI slider. Verified image size reductions in `reports/COMPRESSION_RATIO_VERIFICATION_REPORT.md` and `wiki/20-High-Ratio-PDF-Compression.md`. |
| 5.7.1 | 1-Hour Sustained Soak & Endurance Benchmark (Post Zero-Chrome) `[Spec 025]` ✅ | Merged (PR #151, commit `987cc0d`) — Ran 60-minute continuous soak test across all 44 tools (353,892 operations, 98.3 ops/sec sustained throughput, 100.00% success rate, 0 panics/crashes, bounded ~25 MB RSS memory with zero leaks). Documented in `wiki/19-One-Hour-Endurance-Benchmark.md` and `reports/ONE_HOUR_SUSTAINED_BENCHMARK_REPORT.md`. |
| 5.9.1 | Desktop Release Binary Size Optimization & Pure-Rust Inference (`rten`) `[Spec 028]` ⏳ | Dispatched (Session 11361744507474308005) — Configure root workspace release profile (`strip = true`, `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`), eliminate C++ `ort` (ONNX Runtime) in favor of pure-Rust `rten` SIMD inference engine, shrinking binary from 107MB to <75MB with mandatory 44-tools multi-interface verification round. |
| 5.9.3 | Embed Widget Config & Web WASM Batch Playwright Unit Tests `[Spec 030]` ⏳ | Dispatched (Session 236392176878863235) — Write and validate Playwright unit tests for embed widget config panel, page ranges, and web batch processing. |

## Phase 5 — Advanced Intelligence

**Goal:** PaperPilot understands document content, not just structure.

**Exit condition:** Users can make scanned PDFs searchable, search semantically, extract tables, and ask questions about document content.

**[BLOCKED BY Phase 4]**

---

### 5.1 — OCR `[PARALLEL with 5.2]`

| # | Task | Notes |
|---|---|---|
| 5.1.1 | Evaluate and Migrate to Pure-Rust OCR (`ocrs` via `rten`) ✅ | Merged (Session 10581530668658369493, PR #141) — Eliminated host C++ Tesseract/Leptonica dependencies in `paperpilot-pdf/src/operations/ocr.rs`. Powered by Robert Knight's `ocrs` and `rten` SIMD neural runtime with graceful fallback, `wiki/09-Pure-Rust-OCR.md`, and `reports/OCR_PURE_RUST_REPORT.md` (100% parity maintained). |
| 5.1.3 | Page-level OCR pipeline | Render page → OCR → embed text layer |
| 5.1.4 | OCR progress reporting | Per-page progress events |
| 5.1.5 | `pdf_ocr` MCP tool | |
| 5.1.6 | `paperpilot ocr` CLI command | |
| 5.1.7 | OCR UI in desktop | Button to make scanned PDF searchable |
| 5.1.8 | Language selection | Support multi-language OCR |

---

### 5.2 — Document Understanding & Local Vector Engine `[PARALLEL with 5.1]`

| # | Task | Notes |
|---|---|---|
| 5.2.1 | Local Embedded Vector Engine | Microsecond in-memory vector index in Rust via `ort` (ONNX Runtime) using NeuML's `bert-hash-nano-embeddings` (<1M params, ~1.1MB INT8 ONNX, 128-dim vectors) — 100% offline RAG & semantic search |
| 5.2.2 | Layout-Aware Markdown & Table Parser | MinerU/Marker-style structured text extraction preserving multi-columns, LaTeX math, and Markdown tables |
| 5.2.3 | Semantic Search & Highlighting | Search document by intent/meaning rather than exact keywords, jumping directly to target sentences |
| 5.2.4 | Table Extraction to CSV/JSON/Parquet | Detect table boundaries and export structured data |
| 5.2.5 | Automatic PII Redaction (SSN, Cards, Names) | Local regex + NER entity classifier to automatically detect and structurally redact sensitive PII |
| 5.2.6 | Single-Document Contextual Q&A | Grounded Q&A against document content with page source citations |
| 5.2.7 | Multi-Document Synthesis | Query 2–50 documents simultaneously (e.g. "compare indemnity caps across all 3 vendor agreements") |
| 5.2.8 | Layout-Preserving Translation | Translate text blocks in-place while keeping columns, tables, and typography intact |
| 5.2.9 | High-Scale Memory-Mapped Streaming (`memmap2`) | Lazy page streaming and multi-core Rayon processing for fast opening of 1,000+ page documents |
| 5.2.10 | E2E Intelligence tests | Real embeddings and pipeline execution on test corpus (No Mocking) |

---

### 5.3 — Advanced Layout, Imposition & Security Tools `[PARALLEL with 5.1 and 5.2]`

> Closes the competitive gaps identified against Stirling-PDF and Adobe Acrobat Pro.

| # | Task | Notes |
|---|---|---|
| 5.3.1 | Booklet Imposition | Reorder and layout pages for 2-sided booklet printing |
| 5.3.2 | Multi-Page Layout (N-Up) | Layout 2, 4, 9 pages per physical sheet |
| 5.3.3 | Scale / Resize Page Dimensions | Uniformly scale pages to target standards (A4, Letter, Legal) |
| 5.3.4 | Image / Logo Watermark Stamp | Place transparent PNG/JPEG stamps at exact coordinates on pages |
| 5.3.5 | Validate Digital Signatures | Verify cryptographic signature validity, certificate chain, and tamper hashes |
| 5.3.6 | PDF Sanitization | Strip embedded JavaScript, external launch links, and dangerous metadata |
| 5.3.7 | Remove Annotations (Batch) | Programmatically strip or flatten all annotations across pages |
| 5.3.8 | Overlay / Underlay PDFs | Stamp one PDF on top of or behind another (letterhead/watermark layer) |
| 5.3.9 | Visual Pixel PDF Comparison Slider | Side-by-side & overlay dual-canvas pixel-diff tool in Rust (`image` crate) with grayscale identical background and red/green visual shift highlights |

---

### 5.4 — Document Intelligence & Auto-Organization `[PARALLEL with 5.3]`

| # | Task | Notes |
|---|---|---|
| 5.4.1 | Content-Based Auto-Rename | Automatically rename files based on extracted invoice numbers, dates, or vendor titles |
| 5.4.2 | Fake Scanner Effect | Apply subtle skew, grain, and contrast filter to emulate physical scanner output |
| 5.4.3 | Embedded JavaScript Inspector | Inspect and extract embedded script payloads for security review |
| 5.4.4 | Color Inversion / Dark Mode Filter | Invert document colors or strip colored backgrounds for paper/ink conservation |
| 5.4.5 | Change PDF Permissions (Security Flags) | Modify document user/owner permissions (allow/disallow printing, copying text, form filling, annotations) without changing master encryption |
| 5.4.6 | Adjust Image Contrast / Clean Scan | Enhance contrast, brightness, and thresholding on scanned PDFs to whiten backgrounds and sharpen text before OCR |
| 5.4.7 | Scanner Image Split (Dual-Page Book Split) | Automatically detect center spine and split scanned two-page book spreads into individual portrait pages |
| 5.4.8 | Comic Book Archive Conversion (PDF ↔ CBR/CBZ) | Convert PDFs to/from comic archive formats (`.cbz` zip and `.cbr` rar) preserving page sequence and metadata |
| 5.4.9 | Selective Color Replacement / Background Removal | Replace specific document CMYK/RGB colors (e.g. replace colored header bars or tinted paper backgrounds with pure white) |

---

### 5.5 — Headless Automation & Streamable MCP Transport `[PARALLEL with 5.4]`

> Bridges the UI pipeline to headless scripts and remote AI agents.

| # | Task | Notes |
|---|---|---|
| 5.5.1 | CLI Headless Pipeline Runner (`paperpilot pipeline run`) | Execute visual pipeline JSON workflows directly from CLI without opening GUI |
| 5.5.2 | Pipeline Recipe Export in Desktop UI | One-click button in `PipelineCanvas.svelte` to copy bash CLI command or export JSON recipe |
| 5.5.3 | Streamable HTTP MCP Server (`paperpilot-mcp --port 8080 --http`) | Expose Model Context Protocol over streamable HTTP for remote agents (LangChain, n8n, AutoGen) alongside stdio |
| 5.5.4 | Chained Pipeline MCP Tool (`pdf_run_pipeline`) | Allow AI agents over MCP to trigger an entire multi-step recipe in a single atomic RPC call |

---

### 5.6 — In-Place Content & Typography Editor (Adobe Acrobat Pro Parity) `[PARALLEL with 5.5]`

> Directly addresses Adobe Acrobat Pro's core competitive advantage: true inline typographic text and object manipulation without re-exporting.

| # | Task | Notes |
|---|---|---|
| 5.6.1 | Text Block & Paragraph Boundary Detection | Segment raw content stream operators (`BT...ET`, `Tj`, `TJ`) into selectable paragraphs and text bounding blocks |
| 5.6.2 | In-Place Text Editing & Content Stream Rewriting | Allow user to click any existing text, edit typos, and rewrite font encoding and content streams via `lopdf` |
| 5.6.3 | Font Metric Matching & Fallback Synthesis | Analyze embedded font descriptors to match typeface metrics, font weight, and kerning on edits |
| 5.6.4 | Object-Level Image & Shape Transformation | Move, resize, replace, or delete existing images and vector graphics directly on the canvas |
| 5.6.5 | Print Preflight & PDF/X Verification | Verify color profiles (RGB/CMYK), embedded fonts, image DPI (>300 DPI for print), and PDF/X-1a / PDF/X-4 compliance |

---

### 5.7 — Two-Tier Hybrid Cloud Engine: Edge Wasm (Cloudflare Workers) & Ephemeral Sandboxes (Kubernetes) `[PARALLEL with 5.6]`

> Delivers the high-margin, dual-tier Cloud SaaS processing architecture: lightweight tasks execute at the edge with zero disk footprint via Cloudflare Workers (Wasm), while heavy compute pipelines run on-demand in isolated ephemeral Kubernetes Pods.

| # | Task | Notes |
|---|---|---|
| 5.7.1 | `paperpilot-wasm` Crate & Bindings | Compile core `paperpilot-pdf` algorithms to `wasm32-unknown-unknown` / `wasm32-wasi` via `wasm-bindgen` |
| 5.7.2 | Client-Side In-Browser Operations | Run Merge, Split, Rotate, Compress, Encrypt, and Redact 100% locally in browser memory without network requests |
| 5.7.3 | Web Worker Threading (`wasm-bindgen-rayon`) | Multi-threaded page processing in web browsers using Web Workers and SharedArrayBuffer |
| 5.7.4 | Cloudflare Workers Edge Microservice | Deploy `paperpilot-wasm` to Cloudflare Workers for sub-10ms, memory-only edge processing (0ms cold start, zero disk, $0.005/run) |
| 5.7.5 | Ephemeral Kubernetes Sandbox Worker | On-demand K8s Job / KEDA runner (`emptyDir: { medium: "Memory" }`) for heavy OCR, multi-gigabyte documents, and complex pipelines that auto-terminates and wipes RAM upon job completion |
| 5.7.6 | Smart Hybrid Dispatch Router | Gateway router that routes light operations (merge, split, rotate, stamp) to Cloudflare Workers and routes heavy compute (OCR, large rendering) to Ephemeral K8s Pods |
| 5.7.7 | Zero-Knowledge Audit & Ephemeral Proof | Cryptographic verification asserting zero files touch persistent storage and in-memory streams are purged upon delivery |

---

### 5.8 — AI Document Parsing & LLM/RAG Extraction Engine (Zero-Python, Pure-Rust) `[PARALLEL with 5.7]`

> Directly captures the high-growth AI Engineering & RAG pipeline market (displacing bloated Python stacks like Docling, Marker, PyMuPDF4LLM, and Camelot) with sub-10ms, pure-Rust, on-device parsing.

| # | Task | Notes |
|---|---|---|
| 5.8.1 | RAG-Ready Markdown Extractor (`pdf_to_markdown` / `pdf_extract_structure`) | Extract document visual hierarchy into clean Markdown (`# H1`, `## H2`, bullet lists, code blocks, bold/italics) preserving reading order via pure-Rust font heuristics and spatial clustering (< 10ms/page) |
| 5.8.2 | Vector Table Extractor to CSV/JSON (`pdf_extract_tables`) | Pure-Rust table grid and cell line detector traversing `lopdf` path drawing operators (`re`, `m`, `l`) to extract financial & tabular data directly to JSON/CSV (Camelot/pdfplumber killer) |
| 5.8.3 | Layout-Aware Semantic Chunker (`pdf_chunk_rag`) | Chunk documents by natural semantic boundaries (headers, paragraphs, callout boxes) rather than arbitrary character splits, outputting token-counted JSON chunks ready for vector DB insertion (Pinecone, Qdrant, Chroma) |
| 5.8.4 | Key-Value & Form Entity Extraction (`pdf_extract_kv`) | Heuristic and layout-based key-value pair extractor for invoices, receipts, tax forms, and W-2/1099 documents without cloud LLM dependencies |
| 5.8.5 | Local Embedding & Vector Export (`pdf_embed`) | Offline document vectorization via pure-Rust embedding crates (`candle` / `fastembed-rs`) generating 384/768-dim float arrays locally on CPU/WASM |
| 5.8.6 | Tri-Interface & Python SDK Bindings (`paperpilot-py` via PyO3) | Expose fast Rust document parsers across CLI, MCP, REST Gateway, and a lightweight zero-dependency Python wheel (`pip install paperpilot`) to easily capture Python AI engineers |

## Phase 6 — Enterprise

**Goal:** Organizations can deploy and govern PaperPilot at scale.

**Exit condition:** An IT admin can deploy PaperPilot, set up SSO, configure AI policies, and review audit logs — without touching source code.

**[BLOCKED BY Phase 4]**
**[Can run in parallel with Phase 5]**

---

### 6.1 — Identity `[PARALLEL within group]`

| # | Task | Notes |
|---|---|---|
| 6.1.1 | Organization model | Multi-org data model |
| 6.1.2 | User and team management | CRUD for users, teams, roles |
| 6.1.3 | RBAC | Role definitions, permission checks |
| 6.1.4 | SSO — SAML 2.0 | |
| 6.1.5 | SSO — OIDC | |
| 6.1.6 | SCIM provisioning | Automated user lifecycle from IdP |

---

### 6.2 — Security and Policy `[PARALLEL with 6.1, BLOCKED BY 6.1.3]`

| # | Task | Notes |
|---|---|---|
| 6.2.1 | AI provider policy engine | Allow/block specific providers per org |
| 6.2.2 | Data processing policies | Local-only, region restrictions |
| 6.2.3 | Document retention policies | Auto-delete rules |
| 6.2.4 | Encryption configuration | At-rest and in-transit options |
| 6.2.5 | Central configuration management | Push config to all instances |

---

### 6.3 — Audit `[PARALLEL with 6.1 and 6.2]`

| # | Task | Notes |
|---|---|---|
| 6.3.1 | Audit event schema | Typed events: who, what, when, result |
| 6.3.2 | Audit log storage | Append-only, tamper-evident |
| 6.3.3 | Audit log UI | Filterable, exportable |
| 6.3.4 | Configurable verbosity | What gets logged vs. what doesn't |

---

### 6.4 — Enterprise Deployment `[PARALLEL with 6.1, 6.2, 6.3]`

| # | Task | Notes |
|---|---|---|
| 6.4.1 | Docker image | Production-ready Dockerfile |
| 6.4.2 | Docker Compose setup | For easy self-hosted deployment |
| 6.4.3 | Kubernetes manifests | Helm chart or plain manifests |
| 6.4.4 | Air-gapped deployment guide | No external network dependencies |
| 6.4.5 | Upgrade / migration tooling | Safe schema migrations, zero-downtime updates |

---

### 6.5 — License & EULA Enforcement `[PARALLEL with 6.1]`

**Goal:** Prevent unauthorized commercial use by individuals without requiring server calls for the free tier.

| # | Task | Notes |
|---|---|---|
| 6.5.1 | EULA gate at first launch | Show Personal vs. Commercial choice on first run; log acceptance |
| 6.5.2 | License key validation | Lightweight offline-first key check (cryptographic signature, no phone-home required) |
| 6.5.3 | License key backend API | Simple serverless function (Cloudflare Worker) to issue and revoke keys |
| 6.5.4 | Enterprise feature gating | MSI/MDM, SSO, Audit Logs, Group Policy features locked behind valid commercial key |
| 6.5.5 | License dashboard | Admin UI to manage seats, assign/revoke users per organization |
| 6.5.6 | Grace period & offline mode | License valid offline for 30 days before requiring re-validation |
| 6.5.7 | License expiry notifications | Warn admins 30/14/7 days before renewal |
| 6.5.8 | E2E Enterprise integration tests | Automated deployment and SSO workflow testing (No Mocking) |

---

### 6.6 — Enterprise DRM, Anti-Brute-Force & Memory-Burn Encryption `[PARALLEL with 6.2]`

> High-security defense against unauthorized data exfiltration, GPU brute-forcing, and dictionary cracking. Prioritizes zero-server offline cryptographic memory-hardness.

| # | Task | Notes |
|---|---|---|
| 6.6.1 | Pure Offline Argon2id Memory-Burn (Primary Moat) | Configurable memory-hard key derivation (1–2 GB RAM + 2s compute per guess). Neutralizes offline GPU rack brute-forcing (Hashcat) with zero server dependencies or network connectivity |
| 6.6.2 | Exponential Backoff & Local Lockout Enforcement | Enforce progressive delay penalties on repeated incorrect decryption attempts across Desktop, CLI, and REST Gateway |
| 6.6.3 | Coercion / Duress Password & Honeypot Decoy | Secondary password triggers silent decoy mode: opens sanitized mock PDF while quietly wiping sensitive memory/cache |
| 6.6.4 | Optional Enterprise Remote Burn Switch (Key Escrow / KMS) | Optional cloud/VPC policy: Master key $K_{doc}$ stored in Vault/HSM; after $N$ failed verification attempts or revoke signal, purges key rendering copies unreadable |
| 6.6.5 | Exfiltration & Brute-Force Alerting Webhooks | Fire real-time SIEM alerts (PagerDuty, Slack, Syslog) when repeated decryption failures indicate brute-force reconnaissance |

---

### 6.7 — Enterprise Document Engine Parity (Apryse / PDFTron Gap Closure) `[POST-v1.0 ROADMAP]`

> Closes the capability gap with heavy enterprise document SDKs (Apryse / formerly PDFTron) while maintaining PaperPilot's lightweight pure-Rust speed and client-side privacy. (Reference: `docs/COMPETITIVE_GAP_ANALYSIS_APRYSE.md`)

| # | Task | Notes |
|---|---|---|
| 6.7.1 | Interactive PDF Form Engine (AcroForms & XFA) | Interactive HTML5/canvas input overlay over `hayro` mapped to `/AcroForm` and `/Annots` dictionaries in `lopdf`. Read, fill, validate, and serialize field values back to binary without third-party plugins. |
| 6.7.2 | Collaborative SVG Annotation & Markup Layer | Freehand ink drawing with stylus pressure sensitivity, comment threads, callouts, text highlights, and sticky notes. Export/import via Adobe-standard XFDF (XML) and permanent burn-in support. |
| 6.7.3 | Certified Cryptographic Signatures (PAdES / PKCS#7) | X.509 digital certificate signing using Rust cryptographic crates (`rsa`, `ed25519-dalek`, `x509-parser`). Adobe-compliant ByteRange cryptographic hashing, LTV (Long-Term Validation), and TSA timestamping. |
| 6.7.4 | Semantic Glyph & Vector Redaction Engine | Parse `/Contents` streams via `hayro_syntax` and `lopdf` to completely excise intersected glyphs, vector paths, and metadata under redaction bounding boxes (guaranteed zero text extraction leakage). |
| 6.7.5 | Client-Side MS Office Document Viewer (`.docx` / `.xlsx` to PDF) | In-browser pure-Rust OpenXML parser (`docx-rs`) or sandboxed micro-WASM converter enabling direct drag-and-drop conversion of Office files to PDF without Microsoft Office or server dependencies. |
| 6.7.6 | 2D CAD & BIM Blueprint Vector Viewer (`.dxf` / `.dwg`) | Pure-Rust CAD entity parser (`dxf` crate) converting blueprint layers and 2D vector primitives directly into `tiny-skia` paths for high-precision architectural viewing and measurement. |

---

## Phase 7 — Managed Cloud

**Goal:** Organizations that don't want to self-host can use PaperPilot as a service.

**Exit condition:** A company can sign up, connect their team, and use PaperPilot Cloud without operating any infrastructure.

**[BLOCKED BY Phase 6]**

---

### 7.1 — Cloud Infrastructure `[PARALLEL within group]`

| # | Task | Notes |
|---|---|---|
| 7.1.1 | Multi-tenant architecture | Org isolation at data and compute layer |
| 7.1.2 | Document processing pipeline | Async, scalable job processing |
| 7.1.3 | Cloud MCP gateway | Hosted MCP endpoint per org; evaluate `pmcp` zero-cost SDK for high-throughput multi-tenant streaming |
| 7.1.4 | Monitoring and alerting | Uptime, error rates, job queue depth |
| 7.1.5 | Autoscaling | Scale workers based on job queue |

---

### 7.2 — Cloud-Specific Features `[BLOCKED BY 7.1]`

| # | Task | Notes |
|---|---|---|
| 7.2.1 | Billing and usage tracking | Per-org metering |
| 7.2.2 | Admin dashboard | Org management, usage, billing |
| 7.2.3 | Onboarding flow | Signup, org setup, first document |
| 7.2.4 | SLA monitoring | Track and report uptime against commitments |
| 7.2.5 | Enterprise support tooling | Ticket system, escalation paths |
| 7.2.6 | E2E Cloud tenant tests | End-to-end multi-tenant isolation and billing workflow tests (No Mocking) |

---

## Parallel Execution Summary

This is the high-level view of what can run simultaneously across phases:

```text
Phase 1
├── 1.1 (Setup)          ──────────── all parallel
├── 1.2 (Traits)         ──────────── after 1.1
├── 1.3 (Operations)     ──────────── all parallel with each other, after 1.2
├── 1.4 (Tests)          ──────────── parallel with 1.3
└── 1.5 (CLI)            ──────────── each command unblocked when its op is done

Phase 2 ──── starts after Phase 1
├── 2.1 (MCP setup)      ──────────── parallel
├── 2.2 (Tools)          ──────────── all parallel, after 2.1
└── 2.3 (Infrastructure) ──────────── parallel with 2.2

Phase 3 ──── starts after Phase 1 (PARALLEL WITH Phase 2)
├── 3.1 (Setup)          ──────────── parallel
├── 3.2 (Shell UI)       ──────────── parallel within group, after 3.1
├── 3.3 (Preview)        ──────────── parallel with 3.2, after 3.1
├── 3.4 (Operations UI)  ──────────── after 3.2
└── 3.5 (Job System)     ──────────── parallel with 3.2, after 3.1

Phase 4 ──── starts after Phase 3
├── 4.0 (Offline NLP)    ──────────── parallel [FREE TIER, no key needed]
├── 4.1 (LLM infra)      ──────────── parallel [PRO TIER, API key required]
├── 4.2 (Planning)       ──────────── after 4.0 + 4.1
├── 4.3 (NL UI)          ──────────── parallel with 4.4, after 4.2
├── 4.4 (Execution)      ──────────── parallel with 4.3, after 4.2
└── 4.5 (CLI AI)         ──────────── parallel with 4.3, after 4.2

Phase 5 ──── starts after Phase 4 (PARALLEL WITH Phase 6)
├── 5.1 (OCR)            ──────────── parallel with 5.2
└── 5.2 (Understanding)  ──────────── parallel with 5.1

Phase 6 ──── starts after Phase 4 (PARALLEL WITH Phase 5)
├── 6.1 (Identity)       ──────────── parallel within group
├── 6.2 (Security)       ──────────── parallel with 6.1 (RBAC needed first)
├── 6.3 (Audit)          ──────────── parallel with 6.1 and 6.2
└── 6.4 (Deployment)     ──────────── parallel with all of 6.x

Phase 7 ──── starts after Phase 6
├── 7.1 (Infrastructure) ──────────── parallel within group
└── 7.2 (Features)       ──────────── after 7.1
```

---

## Critical Path

The sequence that determines the earliest possible ship date for each milestone:

```text
1.1 Setup
  → 1.2 Traits
    → 1.3 Operations (all parallel)
      → 1.5 CLI
        → Phase 1 complete ✓

        → 2.1 MCP setup
          → 2.2 + 2.3 (parallel)
            → Phase 2 complete ✓

        → 3.1 Desktop setup (starts same time as 2.1)
          → 3.2 + 3.3 + 3.5 (parallel)
            → 3.4 Operations UI
              → Phase 3 complete ✓

              → 4.0 Offline NLP + 4.1 LLM infra (parallel)
                → 4.2 Planning (unified, resolver-agnostic)
                  → 4.3 + 4.4 + 4.5 (parallel)
                    → Phase 4 complete ✓

                    → Phase 5 + Phase 6 (parallel)
                      → Phase 7
```

---

## Recommended Start

If working solo or as a small team, this is the suggested order for the first 4 weeks:

| Week | Focus |
|---|---|
| 1 | 1.1 Setup + 1.2 Traits + PDF library evaluation |
| 2 | 1.3 Group A (Merge, Split, Extract, Delete, Reorder, Rotate) |
| 3 | 1.3 Group B + C (Compress, Repair, Metadata, Encrypt, Extract text/images) |
| 4 | 1.4 Tests + 1.5 CLI — ship a working `paperpilot` binary |

After week 4, you have something real to show. Start Phase 2 (MCP) and Phase 3 (Desktop) in parallel from week 5.

---

## Phase 8 — Growth Platform

**Goal:** Transform PaperPilot from a standalone tool into a community-driven ecosystem with a plugin registry, workflow builder, watch folder daemon, and SDK libraries.

**[BLOCKED BY Phase 3 — needs working desktop UI]**
**[Can run in parallel with Phase 6 and Phase 7]**

---

### 8.1 — Plugin System `[PARALLEL within group]`

| # | Task | Notes |
|---|---|---|
| 8.1.1 | Define `PdfPlugin` trait | Plugins are just `PdfOperation` implementors with metadata |
| 8.1.2 | Plugin discovery & loading | Load `.so`/`.dylib`/`.dll` plugins at runtime from `~/.paperpilot/plugins/` |
| 8.1.3 | Plugin registry website | Searchable public registry of community plugins |
| 8.1.4 | Plugin packaging CLI | `paperpilot plugin pack` and `paperpilot plugin publish` commands |
| 8.1.5 | Plugin sandboxing | Restrict filesystem and network access per plugin policy |

---

### 8.2 — Pipeline Orchestrator `[BLOCKED BY Phase 3, Phase 8.3]`

**Goal:** Let users define multi-step PDF processing pipelines (recipes) that run automatically, triggered by file system events, webhooks, or schedules. This is the feature that replaces expensive Enterprise Content Management (ECM) platforms.

---

#### 🏗️ Pipeline Format Decision (2026-10-01) — `.ppflow` TOML-first

> **Reference:** Stirling PDF pipeline docs were reviewed at https://docs.stirlingpdf.com/Configuration/Automation/Pipeline/

**Decision: Primary format is `.ppflow` (TOML). Secondary format is `.ppflow.json` (JSON) for machine exchange.**

**Why NOT pure JSON like Stirling:**
- Stirling's JSON pipeline format is a serialized batch of HTTP calls — operations are literal REST endpoint paths (`/api/v1/misc/compress-pdf`). Tightly coupled to their Java/Spring web server.
- JSON has no comment support — you can't annotate why a stage exists, reference a ticket number, or leave a warning.
- Stirling has **silent failure modes** — invalid ops return `200 OK` with an empty body; errors disappear into logs.
- Stirling has **no real branching** — filters silently drop files; there's no IF/ELSE. No retry. No per-stage error handling.
- Stirling has **two incompatible JSON formats** (UI format vs folder-scan format) — causes user confusion.

**Why TOML for authoring:**
| | Stirling (JSON) | PaperPilot (TOML) |
|---|---|---|
| Comments | ❌ | ✅ `# Process invoices nightly` |
| Multi-line strings | ❌ | ✅ Triple-quote `"""` blocks |
| Readability | Verbose | Clean key=value sections |
| Git diffs | Noisy | One value per line |
| Env var injection | ❌ | ✅ `"${env:MY_SECRET}"` |
| Machine generation | Easy | Easy |

**Why JSON as the export/API format:**
- REST API request body, MCP tool args, CLI `--json` flag all emit/accept JSON.
- Desktop "Export Recipe" button produces `.ppflow.json` (importable by other tools).
- CLI and Desktop accept **both formats transparently** at runtime.

**Automation Interface Strategy: CLI vs. API vs. MCP (Both CLI and API are supported):**
| Interface | Target Audience | Tier / Deployment | Primary Use Cases |
|---|---|---|---|
| **CLI (`paperpilot run ...`)** | Developers, Sysadmins, DevOps, Power Desktop Users | **Free / Community** (Local binary) | Shell scripts, cron jobs, local folder watchers, zero-network CI/CD pipelines, offline workstation automation |
| **REST API (`POST /pipelines/...`)** | Enterprise Teams, SaaS Integrations, Cloud Microservices | **Pro / Enterprise** (Headless Docker/Server) | Low-code orchestrators (n8n, Zapier, Make), batch cloud S3/GCS processing, webhook callbacks, multi-tenant scheduling |
| **MCP Tool (`pdf_run_pipeline`)** | AI Agents & AI Engineers | **Pro / Enterprise** (Desktop & Headless Server) | Autonomous LLM agents (Claude Desktop, Cursor, LangChain) executing entire multi-step recipes in a single atomic invocation |

*Architecture Note:* CLI, REST API, Desktop UI, and MCP all act as thin presentation layers dispatching to the same unified `paperpilot-core` pipeline runner and `PdfOperation` trait implementations.

**Trigger support matrix (vs Stirling):**
| Trigger | Stirling | PaperPilot |
|---|---|---|
| Manual (UI) | ✅ | ✅ Desktop GUI canvas |
| CLI file arg | Via REST only | ✅ `paperpilot run invoice.ppflow input.pdf` |
| Watch folder | ✅ | ✅ `type = "watch_folder"` |
| Cron schedule | ❌ | ✅ `type = "schedule", cron = "0 2 * * 1"` |
| Webhook | ❌ | ✅ `type = "webhook"` (Phase 8.2b) |
| MCP agent call | ❌ | ✅ `pdf_run_pipeline` MCP tool (Phase 5.5.4) |

**Reference `.ppflow` schema example:**
```toml
# invoice-processing.ppflow
name        = "Invoice Processing"
description = "Monthly AR invoice batch: OCR → watermark → encrypt → archive"
version     = "1"

[trigger]
type        = "watch_folder"
path        = "/mnt/invoices/incoming"
extensions  = ["pdf"]

[output]
dir         = "/mnt/invoices/processed/{date}"
filename    = "{stem}-processed-{date}"
on_conflict = "rename"   # "overwrite" | "rename" | "skip"

[[stage]]
id          = "ocr"
op          = "ocr"
[stage.params]
languages   = ["eng"]
skip_text   = true

[[stage]]
id          = "filter_large"
op          = "filter_page_count"
condition   = { comparator = "Greater", value = 3 }
on_no_match = "archive_original"   # "skip" | "archive_original" | "error"

[[stage]]
id          = "watermark"
op          = "watermark"
depends_on  = ["ocr"]            # DAG dependency — not just linear serial
[stage.params]
text        = "PROCESSED {date}"
opacity     = 0.4

[[stage]]
id          = "compress"
op          = "compress"
retry       = 2                   # per-stage retry on failure
[stage.params]
level       = "medium"

[[stage]]
id          = "encrypt"
op          = "encrypt"
[stage.params]
password    = "${env:INVOICE_PDF_PASSWORD}"  # env var injection — no hardcoded secrets
permissions = { print = true, edit = false }
```

**Key advantages over Stirling's JSON:**
1. `depends_on` — proper DAG dependency model; stages can depend on specific prior stages (not just serial)
2. `on_no_match` — explicit, named behavior when filters don't match (not silent empty response)
3. `retry = N` — per-stage retry count
4. `${env:VAR}` — environment variable injection for secrets
5. `[trigger]` section — first-class typed trigger config (folder, schedule, webhook, MCP)
6. `[output]` section — named output routing and file naming with template variables
7. Comments — every stage can explain *why* it exists, not just *what* it does

---

#### 8.2a — Pipeline Engine (Core)

| # | Task | Notes |
|---|---|---|
| 8.2.1 | `.ppflow` pipeline schema | **TOML primary, JSON secondary** — see design decision above. Fields: `name`, `version`, `[trigger]`, `[output]`, `[[stage]]` with `op`, `depends_on`, `condition`, `on_no_match`, `retry`, `[stage.params]`. `${env:VAR}` injection for secrets. |
| 8.2.2 | CLI pipeline execution | `paperpilot run invoice.ppflow input.pdf` — accepts `.ppflow` (TOML) or `.ppflow.json` (JSON) transparently |
| 8.2.3 | Conditional branching engine | IF/ELSE via `condition` field per stage + `on_no_match` routing (e.g., "if scanned → OCR branch, else extract text") |
| 8.2.4 | Stage retry & error handling | Per-stage `retry = N`, quarantine folder on final failure, optional Slack/webhook failure alert |
| 8.2.5 | Pipeline sharing & community library | Export/import `.ppflow` files; public community registry of pipeline templates |
| 8.2.6 | Enterprise private pipelines | Org-scoped pipeline libraries, access controlled via RBAC |
| 8.2.7 | Pipeline audit trail | Every execution logged: which file, which stage, result, duration, who triggered it |
| 8.2.8 | Per-stage integrity hashing | SHA-256 the file before and after every pipeline stage; mismatch = automatic abort + alert |
| 8.2.9 | E2E Orchestrator tests | Run real pipelines against test PDF corpus end-to-end (No Mocking) |

---

#### 8.2b — Orchestrator API `[BLOCKED BY 8.2a, ENTERPRISE FEATURE]`

**Goal:** Enterprises can define, trigger, and monitor pipelines programmatically via a REST/WebSocket API — no desktop app required. This is how large-scale automated document processing works (e.g. processing 50,000 PDFs a night from S3).

| # | Task | Notes |
|---|---|---|
| 8.2b.1 | REST API for pipeline management | CRUD endpoints: create, update, delete, list pipelines |
| 8.2b.2 | Pipeline trigger API | `POST /pipelines/{id}/run` with file payload or S3/GCS URL |
| 8.2b.3 | Real-time execution status (WebSocket) | Stream stage-by-stage progress to caller in real-time |
| 8.2b.4 | Pipeline execution history API | Query past runs, filter by status/date/file |
| 8.2b.5 | Webhook callbacks | Fire a webhook at each stage completion or final success/failure |
| 8.2b.6 | S3 / GCS / Azure Blob input sources | Pull input files directly from cloud storage buckets |
| 8.2b.7 | Output routing | Route processed files back to S3/GCS/local/SFTP |
| 8.2b.8 | API key management | Issue, rotate, and revoke API keys per org/team |
| 8.2b.9 | Rate limiting & quotas | Per-org throttling to prevent abuse on cloud tier |
| 8.2b.10 | Load testing & concurrency profiling | Benchmark orchestrator with 10,000+ concurrent pipeline runs to guarantee zero deadlocks and predictable memory usage |

---

### 8.3 — Watch Folder Daemon `[BLOCKED BY 1.5 CLI]`

| # | Task | Notes |
|---|---|---|
| 8.3.1 | `paperpilot watch` command | Monitor directory for new files using `notify` crate |
| 8.3.2 | Auto-apply recipe on new files | Trigger a recipe automatically on each new file |
| 8.3.3 | Daemon mode (`--daemon`) | Run as background service / system daemon |
| 8.3.4 | systemd / launchd / Windows Service support | Native OS service integration |
| 8.3.5 | Watch folder + webhook | Fire webhook on each processed file |

---

### 8.4 — SDK Libraries `[BLOCKED BY Phase 2 MCP]`

#### 8.4a — Core SDKs

| # | Task | Notes |
|---|---|---|
| 8.4.1 | Python SDK (`paperpilot-py`) | PyPI package: pythonic wrappers for all 29 operations, async support (`asyncio`), type hints, Pydantic models for all inputs/outputs |
| 8.4.2 | JavaScript / Node SDK (`paperpilot-js`) | npm package with TypeScript types |
| 8.4.3 | Go SDK (`paperpilot-go`) | Go module, includes `VerifyWebhookSignature()` crypto utility |
| 8.4.4 | SDK documentation site | Auto-generated API docs for all SDKs |

---

#### 8.4b — AI Framework Integrations `[BLOCKED BY 8.4a]`

**Goal:** AI engineers should be able to drop PaperPilot into their existing agent pipelines in under 5 minutes. Target the most popular Python AI frameworks used in production today.

| # | Task | Notes |
|---|---|---|
| 8.4b.1 | LangChain Tools integration | Ship `paperpilot-langchain` — a set of `BaseTool` subclasses (e.g. `MergePdfTool`, `ExtractTextTool`) that plug directly into any LangChain agent or chain |
| 8.4b.2 | LlamaIndex Reader + Tool integration | Ship `paperpilot-llamaindex` — `PaperPilotReader` (load/parse PDFs as LlamaIndex Documents) and `PaperPilotToolSpec` (expose operations as LlamaIndex tools) |
| 8.4b.3 | CrewAI Tool integration | Ship `paperpilot-crewai` — PaperPilot operations as CrewAI `BaseTool` instances, ready for multi-agent task delegation |
| 8.4b.4 | LangGraph node integration | Ship `paperpilot-langgraph` — PaperPilot operations as stateful LangGraph nodes with typed `State` schemas |
| 8.4b.5 | AutoGen integration | Tool wrappers for Microsoft AutoGen multi-agent framework |
| 8.4b.6 | Haystack integration | `PaperPilotConverter` component for the Haystack document AI pipeline |
| 8.4b.7 | AI framework integration tests | End-to-end tests: spawn a real LangChain agent, have it call PaperPilot tools on real PDFs, verify outputs (No Mocking) |

---

### 8.5 — No-Code Integrations `[BLOCKED BY 8.4]`

| # | Task | Notes |
|---|---|---|
| 8.5.1 | Zapier integration | Official PaperPilot app in Zapier marketplace |
| 8.5.2 | Make.com (Integromat) integration | Official module |
| 8.5.3 | n8n community node | Self-hostable automation integration |
| 8.5.4 | Enterprise Workload Automation | Official guides & plugins for Control-M, JAMS, and AutoSys (using REST + Webhook callbacks) |

---

### 8.6 — Contributor Revenue Sharing System `[BLOCKED BY Phase 7]`

| # | Task | Notes |
|---|---|---|
| 8.6.1 | Contribution scoring algorithm | Weight PRs, reviews, docs, plugin publishes |
| 8.6.2 | Contributor registration portal | GitHub OAuth + payment info (Stripe / Wise) + tax form |
| 8.6.3 | Quarterly automated payouts | Calculate scores, split pool, pay out via Stripe Connect |
| 8.6.4 | Public transparency dashboard | Show pool size, scores, and payout amounts publicly |
| 8.6.5 | Plugin usage attribution | Track plugin downloads by Enterprise customers → plugin author earns |

---

### 8.7 — Document Intelligence Dashboard `[BLOCKED BY Phase 6]`

**Goal:** Give Enterprise admins a full-picture view of how PaperPilot is being used across their org.

| # | Task | Notes |
|---|---|---|
| 8.7.1 | Job history log | Searchable, filterable table of all operations (who, what, when, result) |
| 8.7.2 | Operation analytics | Bar/line charts: most-used operations, busiest hours, file sizes |
| 8.7.3 | Error rate tracking | Failure heatmaps, top error types, most problematic file types |
| 8.7.4 | User activity reports | Per-user operation volume (for compliance audits) |
| 8.7.5 | Dashboard export | Export reports to PDF or CSV on a schedule |

---

### 8.8 — Browser Extension `[BLOCKED BY Phase 2 MCP]`

**Goal:** Capture PDFs directly from the web browser and send them to PaperPilot for processing.

| # | Task | Notes |
|---|---|---|
| 8.8.1 | Chrome / Edge extension | Right-click any PDF link → "Open in PaperPilot" |
| 8.8.2 | Firefox extension | Same functionality |
| 8.8.3 | Deep link to desktop app | Extension calls `paperpilot://open?url=...` to launch the desktop app |
| 8.8.4 | "Save & Process" flow | Download PDF from web → apply recipe → save locally |

---

### 8.9 — Mobile App `[BLOCKED BY Phase 3 iOS/Android targets]`

**Goal:** Full-featured iOS and Android PaperPilot app built from the same Tauri 2.0 + Svelte codebase as the desktop app — not a separate project. All PDF processing runs locally on-device via the same `paperpilot-core` Rust library.

| # | Task | Notes |
|---|---|---|
| 8.9.1 | Ship iOS app to App Store | Archive + submit via Xcode with `cargo tauri ios build` |
| 8.9.2 | Ship Android app to Play Store | Sign + submit APK/AAB via `cargo tauri android build` |
| 8.9.3 | Push notifications | Get notified when a long job completes or fails |
| 8.9.4 | Document approval flow | Review and approve or reject a signed PDF from mobile |
| 8.9.5 | Quick scan → PDF | Capture a physical document with camera and send to PaperPilot for OCR |
| 8.9.6 | Mobile-specific gestures | Pinch-to-zoom on preview, swipe to delete pages |
| 8.9.7 | Share sheet integration | iOS Share Sheet / Android Intent support to open PDFs directly from other apps |
| 8.9.8 | E2E Mobile tests | iOS Simulator + Android Emulator automated tests via Appium or Detox (No Mocking) |

---

### 8.10 — Audio & Multimedia Generation `[BLOCKED BY Phase 5]`

**Goal:** Break PDFs out of their static format into dynamic multimedia — following the emerging trend of AI-generated audio summaries and presentation generation from documents.

| # | Task | Notes |
|---|---|---|
| 8.10.1 | PDF → Audio summary | Generate an AI-voiced conversational audio summary (MP3/WAV) from a PDF using a pluggable TTS engine (ElevenLabs / Kokoro / local TTS) |
| 8.10.2 | PDF → Podcast format | Two-voice AI conversational podcast generated from a PDF document (NotebookLM-style) |
| 8.10.3 | PDF → Presentation (AI-structured) | AI maps the structural outline of a PDF and generates a professionally structured PPTX slide deck — not a raw conversion, but an AI-authored presentation |
| 8.10.4 | Audio summary MCP tool | `pdf_audio_summary` MCP tool so AI agents can generate audio from any PDF |
| 8.10.5 | Audio summary CLI command | `paperpilot audio-summary <file> --voice kokoro --output summary.mp3` |
| 8.10.6 | E2E Multimedia tests | Generate real audio and presentation outputs from test PDFs and verify output quality (No Mocking) |

---

### 8.11 — Physical Document Ingestion `[BLOCKED BY Phase 5.1 OCR, PARALLEL with 8.9]`

**Goal:** Make PaperPilot the automatic destination for every scan that leaves a physical printer or phone camera — without requiring the user to change their existing scan workflow. Every scan that touches PaperPilot immediately becomes a first-class `.ppdoc` document: deskewed, compressed (DjVu 3-layer, Phase 1.3.50–1.3.56), OCR'd, and searchable.

**Exit condition:** A user can walk to any network printer, select "Scan to Computer → PaperPilot" on the printer's touchscreen, and the scanned document arrives as a fully processed `.ppdoc` on their desktop with OCR and AI embeddings — without installing any printer driver.

---

| # | Task | Notes |
|---|---|---|
| 8.11.1 | Hot folder scan-ingest recipe | Extend Phase 4.7.1 watcher to detect `.pdf`/`.jpg`/`.png` drops and auto-run `scan-ingest` recipe: deskew → smart compress (1.3.55) → OCR (Phase 5.1) → save `.ppdoc`. No user action needed after initial setup. |
| 8.11.2 | eSCL/AirScan local server | Implement an eSCL (Mopria/AirScan) HTTP server that advertises itself on mDNS/Bonjour as `PaperPilot on <hostname>`. Any modern network printer's "Scan to Computer" menu discovers and shows it as a destination. Scanner streams raw TIFF/JPEG directly to PaperPilot over HTTP — no driver install, zero user config beyond first enable. Works with every eSCL-compliant printer (HP, Epson, Canon, Brother, Ricoh all support eSCL). |
| 8.11.3 | Virtual CUPS printer (Linux/macOS) | Register PaperPilot as a CUPS print queue with a custom PPD. When a scanner's "Print to PDF" or "Scan to Application" sends a PostScript/PWG Raster job to this queue, PaperPilot intercepts the stream and converts to `.ppdoc`. Replaces the role of PDF24 / PDFCreator but adds OCR and full PaperPilot processing. |
| 8.11.4 | Windows WIA virtual scanner | On Windows, register a WIA (Windows Image Acquisition) minidriver so PaperPilot appears in "Windows Fax and Scan" and any WIA-aware scanner application as a scan destination. The WIA driver returns processed `.ppdoc` data rather than raw TIFF. |
| 8.11.5 | Mobile camera scan (native) | In the PaperPilot mobile app (8.9), expose a native document camera scanner with: real-time perspective correction guide overlay, auto-capture on document detection, multi-page session (keep scanning until user taps "Done"), and immediate DjVu-style compression + OCR pipeline on-device. |
| 8.11.6 | Mobile Share Extension (any scanner app) | Register a system Share Extension on iOS and an Intent filter on Android. User scans in **any** third-party scanner app (Apple Continuity Camera, Microsoft Lens, Adobe Scan) and taps Share → PaperPilot. The JPEG/PDF is ingested, processed, and saved as `.ppdoc`. No PaperPilot scanner UI required. |
| 8.11.7 | Scan-to-email receiver | Run a local SMTP listener (bound to `localhost:2525`). User configures their printer's "Scan to Email" to send to `scan@paperpilot.local` (DNS resolved via mDNS). PaperPilot receives the email, extracts the PDF/JPEG attachment, and runs the scan-ingest pipeline. Works with every enterprise MFP that supports scan-to-email. |
| 8.11.8 | Print-with-return-address (steganographic fingerprint) | When PaperPilot **prints** a `.ppdoc` document, embed two invisible markers in the printed output: (a) a 2×2 mm corner micro-QR code encoding `{doc_id, version_hash, ppdoc_server}` and (b) a DCT-domain steganographic watermark that survives the print→scan round-trip. When the physical document is later scanned back into PaperPilot, it detects the markers and: links the scan to the original `.ppdoc` source; overlays hand-written annotations as a new annotation layer on the original editable document; shows a diff "Original vs. annotated physical copy." Enables paper-based annotation workflows to flow back into the digital document. |
| 8.11.9 | Round-trip annotation merge UI | Desktop UI for reviewing a scanned-back document: side-by-side view of original `.ppdoc` vs. inbound scan; detected handwritten annotations shown as a new `annotation` layer; one-click "Accept all" merges annotations into the canonical document. |
| 8.11.10 | E2E Physical Ingestion tests | Automated tests: spin up a mock eSCL HTTP client, POST a test TIFF scan, verify PaperPilot produces a valid `.ppdoc` with OCR text and DjVu compression applied. Print-with-return-address: embed watermark, simulate scan (Gaussian blur + noise filter to simulate print/scan degradation), verify fingerprint is recovered and doc_id decoded correctly. (No Mocking of the compression or OCR pipeline.) |

---

## Phase 9 — Advanced Editing & Enterprise Features

**Goal:** Close the most common gaps vs. Adobe Acrobat with achievable, well-defined capabilities. WYSIWYG text reflow is deliberately deferred to Phase 9.3 (Moonshot) due to extreme complexity.

### 9.1 — Annotation & Markup (Achievable)
| # | Task | Notes |
|---|---|---|
| 9.1.1 | Highlight Tool | Select text regions and apply colour highlight overlays |
| 9.1.2 | Sticky Notes / Comments | Attach pop-up comment annotations to any page location |
| 9.1.3 | Free-Draw Markup | Freehand pen/pencil drawing over pages (stored as SVG overlay) |
| 9.1.4 | Redaction Tool | Select text/areas, burn a black box over them, and permanently strip underlying content stream data |

### 9.2 — Forms & E-Signatures (Achievable)
| # | Task | Notes |
|---|---|---|
| 9.2.1 | Form Field Builder | Drag-and-drop text boxes, checkboxes, and radio buttons to create fillable PDFs |
| 9.2.2 | Data Extraction | Extract filled form data to JSON/CSV |
| 9.2.3 | Cryptographic Signing UI | Select a certificate, draw a signature, and apply a cryptographic seal |
| 9.2.4 | Signature Verification | Automatically verify document integrity and show a green/red seal indicator |

### 9.3 — WYSIWYG Canvas Editing (Moonshot)
> [!WARNING]
> This is the hardest unsolved problem in PDF engineering. PDFs have no reflow model — text is stored as absolute `x,y` positioned glyphs. True in-place editing requires parsing content streams, having access to embedded font subsets, and re-encoding glyph data. Even Adobe Acrobat struggles with this on some PDFs. Estimated scope: 12–24 engineering months.

| # | Task | Notes |
|---|---|---|
| 9.3.1 | Content Stream Parser | Parse PDF page content streams to extract text bounding boxes and glyph sequences |
| 9.3.2 | Font Subsetting Engine | Expand embedded font subsets to include any new characters the user wants to type |
| 9.3.3 | In-Place Text Edit | Click a text run, edit it, re-encode back into the content stream |
| 9.3.4 | Image Drag & Replace | Click an embedded image, drag to reposition, or replace with a new file |

---

## Phase 10 — WebAssembly (WASM) Target

**Goal:** Compile `paperpilot-core` and `paperpilot-pdf` to WASM so PDF processing can run entirely in-browser — enabling a zero-install web app, powerful browser extensions, and embeddable SDKs for third-party websites.

**Why WASM matters for PaperPilot:**
- Our Rust core is already platform-agnostic — WASM is a natural compilation target.
- Enables a `paperpilot.app` web version that processes files 100% locally in the browser (zero upload = perfect privacy).
- The Browser Extension (Phase 8.8) can run the full PDF engine in-page without needing the native app installed.
- Third-party developers can embed `paperpilot-wasm` in their own web apps as a drop-in library.

**[BLOCKED BY Phase 3 / Phase 2 MCP]**

### 10.1 — WASM Core Build
| # | Task | Notes |
|---|---|---|
| 10.1.1 | Audit `paperpilot-core` for WASM compatibility | Replace any `std::fs` / `std::path` calls with WASM-safe equivalents (e.g., in-memory byte slices) |
| 10.1.2 | Add `wasm32-unknown-unknown` target | Add to workspace `Cargo.toml` and CI matrix |
| 10.1.3 | Create `paperpilot-wasm` crate | Thin `wasm-bindgen` wrapper that exposes core operations as JS-callable functions |
| 10.1.4 | Build WASM bundle | `wasm-pack build --target web` — outputs `paperpilot_wasm.js` + `paperpilot_wasm_bg.wasm` |
| 10.1.5 | Publish to npm as `@paperpilot/wasm` | Versioned npm package for easy consumption by web projects |

### 10.2 — Web App (paperpilot.app)
| # | Task | Notes |
|---|---|---|
| 10.2.1 | New SvelteKit web app | `apps/web` — same component library as the desktop, but without Tauri IPC |
| 10.2.2 | File drag-and-drop via browser File API | Accept PDF uploads into in-memory `Uint8Array` |
| 10.2.3 | WASM-based operations | All PDF processing via `@paperpilot/wasm` — zero server upload |
| 10.2.4 | Progressive Web App (PWA) | Add service worker + manifest so users can "install" paperpilot.app to their desktop |
| 10.2.5 | File System Access API integration | Directly read/write to local disk on Chrome/Edge without downloading |

### 10.3 — WASM CI & Testing
| # | Task | Notes |
|---|---|---|
| 10.3.1 | WASM unit tests | `wasm-pack test --headless --chrome` for all core operations |
| 10.3.2 | Playwright WASM E2E | Spin up the web app and run the same Playwright tests as the desktop |
| 10.3.3 | Bundle size tracking | Fail CI if WASM bundle exceeds 5 MB (keep it fast to load) |

---

## Architecture Improvements — Hexagonal / Ports & Adapters

**Goal:** Harden the Hexagonal Architecture foundations already present in PaperPilot so the codebase stays infrastructure-agnostic as we scale to cloud and WASM.

**Strategy:** These are incremental, non-breaking additions to `paperpilot-core`. No operation code needs to change. Adapters are added alongside the existing direct-write pattern.

---

### A.1 — StoragePort (Output Adapter)

| # | Task | Notes |
|---|---|---|
| A.1.1 | `StoragePort` trait ✅ | Defined in `paperpilot-core/src/ports.rs` — `write`, `read`, `exists` |
| A.1.2 | `LocalFileStorage` adapter ✅ | Default impl — wraps `std::fs`; used in CLI and desktop |
| A.1.3 | `MemoryStorage` adapter ✅ | In-memory map — use in unit tests, eliminates all temp file setup |
| A.1.4 | Wire `StoragePort` into CLI commands | Replace `std::fs::write` calls in `paperpilot-cli` with `LocalFileStorage` |
| A.1.5 | Wire `StoragePort` into MCP execute arms | Replace direct file writes in `paperpilot-mcp/src/server.rs` with injected port |
| A.1.6 | `S3Storage` adapter | Implements `StoragePort` via `aws-sdk-s3`; used in Cloud phase (Phase 9) |
| A.1.7 | Refactor all operation tests to use `MemoryStorage` | Eliminates all `tempfile` usage from tests |

---

### A.2 — PdfBackend Port (Input Adapter)

| # | Task | Notes |
|---|---|---|
| A.2.1 | `PdfBackend` trait ✅ | Defined in `paperpilot-core/src/ports.rs` — `load_from_bytes`, `load_from_path` |
| A.2.2 | `LopdfBackend` adapter | Implement `PdfBackend` for `lopdf` in `paperpilot-pdf` — wraps `LopdfDocument::load` |
| A.2.3 | Wire `PdfBackend` into MCP server | Replace `LopdfDocument::load(...)` hardcoded calls with injected `PdfBackend` trait object |
| A.2.4 | `PdfiumBackend` adapter (Future) | Swap in `pdfium-render` for higher-fidelity rendering, no operation changes needed |
| A.2.5 | `MuPdfBackend` adapter (Future) | Use MuPDF for OCR-heavy workflows via FFI |

---

### A.3 — Dependency Inversion in CLI & MCP

| # | Task | Notes |
|---|---|---|
| A.3.1 | `AppContext` struct | Single struct holding `Box<dyn StoragePort>` + `Box<dyn PdfBackend>` — injected at startup |
| A.3.2 | CLI uses `AppContext` | `paperpilot-cli/src/main.rs` builds `AppContext` and passes to all command handlers |
| A.3.3 | MCP uses `AppContext` | `PaperPilotMcpServer` holds `Arc<AppContext>` — all tools use it instead of hardcoded paths |
| A.3.4 | Integration test harness | Boot the full app with `MemoryStorage` + `LopdfBackend` — true end-to-end, no file I/O |

---

## Phase 8 — `.ppdoc` Native Document Format

**Goal:** Design and implement PaperPilot's own next-generation document format that supersedes PDF as an *authoring* format while keeping PDF as a *render/export target*. This turns PaperPilot from a "PDF tool" into a "document platform."

**Spec:** [`specs/ppdoc-format/spec.md`](specs/ppdoc-format/spec.md)

**Strategic rationale:** The same move that made Figma beat Sketch (`.fig` as source → export anything), LaTeX dominate academia (`.tex` → PDF), and Pandoc indispensable (Markdown → 40 formats). PaperPilot would be the first tool to offer this with native PDF import, offline AI embeddings, and Rust-speed processing.

**Exit condition:** A `.ppdoc` file can be created, edited, round-tripped to JSON, and exported to PDF and HTML with full content fidelity. `pdf2ppdoc` can import a real-world PDF with ≥ 80% structural accuracy.

---

### 8.1 — Format Core (`ppdoc-rs` crate) `[PARALLEL]`

| # | Task | Notes |
|---|---|---|
| 8.1.1 | Define `PpdocNode` enum in Rust | All node types: `Document`, `Section`, `Paragraph`, `Table`, `Figure`, `List`, `CodeBlock`, `Formula`, `Callout`, `Interactive`, `FormField`, `PageBreak` |
| 8.1.2 | Implement ZIP container read/write | Use `zip` crate — manifest, content, layout, assets, metadata, history directories |
| 8.1.3 | `manifest.json` validation | Format version check, file registry, SHA-256 checksums |
| 8.1.4 | JSON serialization via `serde` | `#[derive(Serialize, Deserialize)]` on all node types; stable UUID `id` generation |
| 8.1.5 | `PpdocDocument::query(selector)` | XPath-style node lookup by `id`, `type`, `section`, or custom attributes |
| 8.1.6 | Round-trip test suite | Parse → serialize → parse — content tree must be byte-identical |

---

### 8.2 — Layout Engine (`pplayout`)

| # | Task | Notes |
|---|---|---|
| 8.2.1 | `PplayoutRules` struct | Page size, margins, typography scale, flow mode (`Reflowable` / `Fixed`) |
| 8.2.2 | Breakpoint resolver | Select correct layout rules for `screen` / `mobile` / `print` contexts |
| 8.2.3 | Default layout pack | Ship `default.pplayout` and `print.pplayout` as embedded defaults |
| 8.2.4 | Custom layout loading | Load user-provided `.pplayout` from the document archive |

---

### 8.3 — Export Pipeline

| # | Task | Notes |
|---|---|---|
| 8.3.1 | `ppdoc → PDF` exporter | Walk content tree, emit `lopdf` objects using layout rules; reuse Phase 1 PDF engine |
| 8.3.2 | `ppdoc → HTML` exporter | Emit semantic HTML5 with ARIA roles; inline CSS from layout rules |
| 8.3.3 | `ppdoc → EPUB 3` exporter | Package HTML export into EPUB container; WASM widgets → static fallback images |
| 8.3.4 | `ppdoc → Markdown` exporter | Best-effort flat text; preserve tables (GFM), images (relative paths), headings |
| 8.3.5 | `ppdoc → Plain Text` exporter | Reading-order extraction with clean whitespace |
| 8.3.6 | Export CLI subcommand | `paperpilot ppdoc export --format pdf --input doc.ppdoc --output doc.pdf` |
| 8.3.7 | Hybrid PDF Export (`--embed-source`) | Inspired by LibreOffice Hybrid PDF but **visible**: attach the `.ppdoc` ZIP bundle into the PDF `/EmbeddedFiles` dictionary; inject structured XMP metadata (`ppdoc_version`, `generator`); render a small tasteful "PaperPilot Document" badge in the PDF margin (opt-out flag `--no-badge`). When any resulting `.pdf` is dropped back into PaperPilot, the embedded `.ppdoc` is extracted and opened with full edit history, AI embeddings, and vector source intact. **Viral distribution mechanism** — every shared PDF is a soft funnel for PaperPilot adoption. See `INSPIRATION.md §4`. |

---

### 8.4 — Import Pipeline

| # | Task | Notes |
|---|---|---|
| 8.4.1 | `pdf2ppdoc` converter | Use `lopdf` + heuristics to reconstruct semantic tree from PDF content streams |
| 8.4.2 | Heading detection | Font-size clustering → assign `heading` level 1–6 |
| 8.4.3 | Table detection | Adjacent text-block alignment → `table` node reconstruction |
| 8.4.4 | Figure extraction | Raster image objects → `figure` nodes with extracted alt-text via OCR |
| 8.4.5 | `md2ppdoc` converter | Markdown AST (via `pulldown-cmark`) → `.ppdoc` content tree (lossless for flat docs) |
| 8.4.6 | Import accuracy benchmark | Target: ≥ 80% structural accuracy on a corpus of 100 real-world PDFs |

---

### 8.5 — AI Integration

| # | Task | Notes |
|---|---|---|
| 8.5.1 | `ppdoc-embed` tool | Walk all `section` nodes → run `neuml/bert-hash-nano-embeddings` → write `metadata/embeddings.json` |
| 8.5.2 | Named entity extraction | Run NER pass during embedding → store `named_entities[]` per section |
| 8.5.3 | Instant RAG on open | When PaperPilot opens a `.ppdoc` with `embeddings.json`, load vectors into in-memory index immediately — no re-indexing |
| 8.5.4 | MCP tool: `ppdoc_query` | `ppdoc_query(file, question)` → semantic search over embedded sections → return top-K chunks |

---

### 8.6 — Cryptographic Signatures

| # | Task | Notes |
|---|---|---|
| 8.6.1 | Per-section SHA-256 hashing | Hash canonical JSON of each `section` node → store in `signatures.json` |
| 8.6.2 | Merkle tree construction | Build root hash from section hashes → enables partial verification |
| 8.6.3 | Ed25519 signing | `ppdoc-sign sign --section sec-id --key private.pem` |
| 8.6.4 | Signature verification CLI | `ppdoc-sign verify --input doc.ppdoc` → reports which sections are signed and by whom |
| 8.6.5 | Selective disclosure proof | Verify a single section's signature without reading the full document |

---

### 8.7 — Change History

| # | Task | Notes |
|---|---|---|
| 8.7.1 | NDJSON change log writer | Append `{"ts","op","author","node_id","prev","next"}` entries on every edit |
| 8.7.2 | `ppdoc-diff` CLI | `ppdoc-diff doc_v1.ppdoc doc_v2.ppdoc` → structural diff output (human-readable) |
| 8.7.3 | History replay | Reconstruct any past version of the document from the change log |
| 8.7.4 | History viewer in Desktop app | Timeline UI showing who changed what and when (Phase 4 integration) |

---

### 8.8 — WASM Interactive Widgets

| # | Task | Notes |
|---|---|---|
| 8.8.1 | Widget sandbox spec | WASI Preview 2 (`wasi:io`, `wasi:filesystem` blocked, `wasi:random` allowed) |
| 8.8.2 | Widget host in desktop app | Load `.wasm` from `assets/wasm/`, call `render(props_json) → svg_string` |
| 8.8.3 | Reference widget: bar chart | Pure-Rust WASM widget that renders a bar chart from `table` node data |
| 8.8.4 | Static fallback generation | On export (PDF/EPUB), call widget → capture SVG → embed as static image |
| 8.8.5 | Widget SDK docs | Document the `render` ABI so third parties can build `.ppdoc` widgets |

