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

### 4.1 — Local AI Agent & LLM Multi-Step Planning (100% Free & Open Source — Apache 2.0) `[PARALLEL]`

> Enables complex, multi-step conversational agent planning (e.g. "Merge invoice1 and invoice2, strip page 3, rotate page 2 clockwise, and compress under 2MB") running 100% locally on-device.
> **Designated Local Offline SLM:** Mozilla `llamafile` standalone executable (`SmolLM-135M-Instruct` or `Qwen2.5-Coder-1.5B/7B`) with zero external installation, private local execution, and in-app model downloading.
> **Universal Endpoint & BYOK:** Connect to ANY local or network-hosted server (Ollama, LM Studio, vLLM GPU cluster) or bring-your-own-key (BYOK) OpenAI / Gemini / Claude / Groq API stored in native OS Keyring.
> Implements the same `NlpResolver` trait from 4.0.2 — drop-in swap.
> **Licensing & Monetization:** **100% Free & Open Source (Apache 2.0)** in Community tier. Advanced background daemons (Watch folders, Cron automation), shared team recipe libraries, and enterprise IT fleet governance (SAML/SCIM/Air-gap GPO) power Pro/Teams/Enterprise tiers.

| # | Task | Notes |
|---|---|---|
| 4.1.1 | Create `paperpilot-ai` crate ✅ | Merged (PR #170, commit `6dffd85`, Session 7564763815279840049) — Scaffolding standalone crate in workspace; depends on `paperpilot-nlp` for shared trait (`reports/PAPERPILOT_AI_CRATE_REPORT.md`, `wiki/29-PaperPilot-AI-Engine.md`) |
| 4.1.2 | `LlmNlpResolver` struct ✅ | Merged (PR #170, commit `6dffd85`, Session 7564763815279840049) — Implements `NlpResolver` with dynamic runtime switching and automatic fallback to `OfflineNlpResolver` |
| 4.1.3 | Universal OpenAI-Compatible Client (`endpoint_url` + `api_key` + `model`) ✅ | Merged (PR #170, commit `6dffd85`, Session 7564763815279840049) — Connects to **ANY** local or remote OpenAI-compatible server (local Ollama, LM Studio, vLLM/GPU cluster, Groq, DeepSeek, Together, OpenRouter, Azure OpenAI) with wiremock tests |
| 4.1.4 | Mozilla `llamafile` Process Supervisor & 1-Click Offline Runner ✅ | Merged (PR #171, commit `53c1f5c`, Session 3282682283884732598) — Subprocess manager in Rust: boots, monitors, and terminates `.llamafile` standalone executable binaries (`--server --jinja --port 8080`) for zero-install, zero-Docker, air-gapped execution (`reports/LLAMAFILE_SUPERVISOR_REPORT.md`, `wiki/30-Llamafile-Supervisor.md`) |
| 4.1.5 | In-App Curated Model Downloader & Manager ✅ | Merged (PR #172, commit `b064797`, Session 17474141107561700970) — In-app UI download manager with progress bars streaming tested GGUF / llamafile models (`Qwen2.5-Coder-1.5B`, `Llama-3.2-3B`) to OS app data folders (`~/.local/share/paperpilot/models/`) (`reports/LOCAL_AI_SETTINGS_UI_REPORT.md`, `wiki/31-Local-AI-Settings-And-Downloader.md`) |
| 4.1.6 | BYOK & Endpoint Settings UI with OS Keyring Security ✅ | Merged (PR #172, commit `b064797`, Session 17474141107561700970) — Settings drawer with 4-way mode toggle (Offline NLP / Local Llamafile / Universal Endpoint / BYOK Cloud), endpoint latency testing, and secure OS Keychain / Stronghold storage |
| 4.1.7 | Multi-Tier AI Agent Supervisor & Dynamic LLM Router Integration `[Spec 051]` ✅ | **Completed & Merged (PR #290, commit `cb2d594`, Jules Session [`7744176619131778344`](https://jules.google.com/session/7744176619131778344))** — Wires configured AI mode (Offline NLP, Local Llamafile, Universal Endpoint, BYOK Cloud) from `AiSettingsModal` into the desktop natural language resolution pipeline (`resolve_natural_language`), connecting `paperpilot-ai`'s `DynamicRouter`, `LlamafileSupervisor`, and `LlmNlpResolver` with seamless fallback and interactive UI status pills in `PdfChatPanel.svelte` (`specs/051-multi-tier-ai-agent-supervisor/spec.md`). |
| 4.1.8 | Embedded Documentation RAG (`sqlite-vec`) ✅ | Merged (Session 6788529762952435381, PR #136) — In-memory/embedded SQLite vector store indexing all living documentation (`TRI_INTERFACE_E2E_AND_DOCS.md`, user guides, tool specs). Answers user questions directly in the chat with zero hallucinations, exact copy-pasteable CLI/API snippets, and sub-millisecond retrieval |
| 4.1.9 | On-Device Document SLM Dataset Generator Engine — Wave 1: Core 15 Tools (`tools/slm-training`) `[Spec 084 / Task 84A]` ✅ | **Completed & Merged (PR #255, commit `072a7a2`, Session 17682577633163496827)** — Synthetic dataset generator in Python creating ChatML training pairs across 5 linguistic tiers (direct, casual, legal/finance jargon, typos, compound workflows) for top 15 core tools with 5/5 pytest passing (`specs/084-on-device-document-slm-qwen-finetuning/task_84a.md`). |
| 4.1.10 | On-Device Document SLM Dataset Generator — Wave 2: Organization & Layout Tools `[Spec 084 / Task 84B]` ✅ | Completed (Merged PR #259, commit `5dd96f5`) — Expand `tool_registry.py` and `templates.py` with 15 layout & page tools: `pdf_nup`, `pdf_booklet`, `pdf_scale_pages`, `pdf_crop`, `pdf_reorder`, `pdf_delete_pages`, `pdf_reverse`, `pdf_stamp`, `pdf_overlay_underlay`, `pdf_stitch_canvas`, `pdf_attachments`, `pdf_validate_signatures`, `pdf_remove_signatures`, `pdf_flatten_annotations`, `pdf_header_footer` (`specs/084-on-device-document-slm-qwen-finetuning/task_84b.md`). |
| 4.1.11 | On-Device Document SLM Dataset Generator — Wave 3: Conversions, Forensics & Negative Guards `[Spec 084 / Task 84C]` ✅ | **Completed & Merged (PR #260)** — Expanded `tool_registry.py` and `templates.py` to 45 tools, added 10% negative guardrail refusal prompts, validated ChatML schemas and multi-step pipeline workflows, passing all unit tests (`specs/084-on-device-document-slm-qwen-finetuning/task_84c.md`). |

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
| 4.3.9 | Actionable RAG Workspace & Bi-Directional Citation Canvas `[Spec 116]` ✅ | **Completed & Merged (PR #263, Session 14413569953505972234)** — Bridges document chat with canvas viewer: clicking citations jumps directly to page/bounding-box with visual highlight pulse. Allows chat prompts to directly propose and trigger execution plans on active document (`specs/116-actionable-rag-workspace-citation-canvas/spec.md`). |
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
| 5.9.1 | Desktop Release Binary Size Optimization & Pure-Rust Inference (`rten`) `[Spec 028]` ✅ | Merged (`45a1566`) — Configured canonical root workspace release profile (`strip = true`, `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`), eliminated C++ `ort` (ONNX Runtime) in favor of pure-Rust `rten` SIMD engine with embedded `tinybert.rten.zst`, dropping standalone desktop binary from 107MB to 35MB (~67% reduction). Verified 100% pass across `paperpilot-nlp`, tri-interface E2E (132/132 assertions), and desktop Playwright AI chat pipeline (`reports/DESKTOP_BINARY_OPTIMIZATION_REPORT.md`, `wiki/22-Desktop-Binary-Optimization.md`). |
| 5.9.2 | Pure-Rust Jules Submitter CLI Tool (`tools/jules-submit`) `[Spec 029]` ✅ | Merged (PR #164, commit `1e673bc`, Session 8537116514657265561) — Standalone pure-Rust CLI tool in `tools/jules-submit/` (`reqwest`, `clap`, `dotenvy`) replacing Python submitter for 100% zero-Python developer workflow (`reports/JULES_SUBMIT_RUST_MIGRATION_REPORT.md`, `wiki/23-Pure-Rust-Jules-Submitter.md`). |

| 5.9.3 | Embed Widget Config & Web WASM Batch Playwright Unit Tests `[Spec 030]` ✅ | Merged (PR #156, commit `504b247`, Session 236392176878863235) — Authored and verified Playwright test suites for interactive embed widget parameter configuration (`apps/embed/tests/widget_batch_and_config.spec.ts`) and web WASM multi-file batch queue processing (`apps/web/tests/operations_batch_processing.spec.ts`), documented in `reports/WEB_AND_EMBED_BATCH_UNIT_TESTS_REPORT.md` and `wiki/24-Web-And-Embed-Batch-Testing.md`. |
| 5.9.4A | Rust-Native Penta-Interface Real Semantic Assertions: Foundation & Page Ops (9 Tools, 180 Tests) `[Spec 031]` ✅ | Merged (PR #161, commit `472ae69`, Session 10267474871153892002) — Executed 4 tiers across all 5 interfaces for 9 page operations in `tools/penta-interface-e2e/src/cases/page_ops.rs`, documented in `reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4A.md`. |
| 5.9.4B | Rust-Native Penta-Interface Real Semantic Assertions: Security, Stamping & Forms (14 Tools, 280 Tests) `[Spec 031]` ✅ | Merged (PR #159, commit `95eae07`, Session 11331400509707880947) — Executed 4 tiers across all 5 interfaces for 14 security & forms tools in `tools/penta-interface-e2e/src/cases/security_forms.rs`, documented in `reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4B.md`. |
| 5.9.4C | Rust-Native Penta-Interface Real Semantic Assertions: Extraction, Analysis & Optimization (13 Tools, 260 Tests) `[Spec 031]` ✅ | Merged (PR #160, commit `ba79af0`, Session 14159183327554136937) — Executed 4 tiers across all 5 interfaces for 13 analysis & extraction tools in `tools/penta-interface-e2e/src/cases/analysis.rs`, documented in `reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4C.md`. |
| 5.9.4D | Rust-Native Penta-Interface Real Semantic Assertions: Conversions & Master 880-Test Report (8 Tools, 160 Tests) `[Spec 031]` ✅ | Merged (PR #158, commit `f0607a3`, Session 898803185132269358) — Executed 4 tiers across all 5 interfaces for 8 conversion tools (HTML, Markdown, Excel, Images, DOCX, XLSX, PPTX, PDF/A) in `tools/penta-interface-e2e/src/cases/conversions.rs`, documented in `reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4D.md`. |
| 5.9.5 | Penta-Interface Edge Cases & Deep Boundary Verification Suite `[Spec 031]` ✅ | Merged (PR #162, commit `3a71c36`, Session 16355790137239811249) — Comprehensive 44-tool parameter & boundary matrix testing: compression quality (`low`/`medium`/`high`/numeric), discontinuous & reverse page selections (`1,3,5`, `5-1`), full-bleed & micro-crops, multi-byte UTF-8/emoji watermarks & metadata, complex passwords with shell escapes, AcroForm edge cases, corrupt PDF headers, Makefile integration (`make test-penta-edge-cases`), verified 100% pass (385/385 tests) in `reports/PENTA_INTERFACE_E2E_EDGE_CASES_REPORT.md`. |
| 5.9.6 | Penta-Interface High-Throughput & Concurrency Performance Benchmark `[Spec 031]` ✅ | Merged (PR #165, commit `a8de2ae`, Session 10557618272357635464) — Concurrency stress testing (10, 25, 50 workers), p50/p95/p99 latency profiling across all 5 surfaces, memory leak checks (<35MB RSS), Makefile integration (`make bench-penta-e2e`), generating `reports/PENTA_INTERFACE_PERFORMANCE_BENCHMARK_REPORT.md`. |

| 5.9.7 | Multimodal PDF-to-JSON with Image Extraction & Base64 Inlining `[Spec 032]` ✅ | Merged (PR #166, commit `2604a8e`, Session 7215646411441515522) — Implemented `pdf_to_json` with `--image-mode <none|files|base64>` across all 5 surfaces (CLI, MCP, REST, WASM, Edge), base64 inline image embedding, multimodal test fixture generation, 20 E2E assertions, latency benchmarks, and documentation (`reports/PDF_TO_JSON_MULTIMODAL_REPORT.md`, `wiki/08-Multimodal-PDF-to-JSON.md`). |
| 5.9.8 | Human-Friendly UI Error Normalization & Action Hints `[Spec 033]` ✅ | Merged (PR #163, commit `3886288`, Session 11507040602700534548) — Centralized `errorFormatter.ts`, category taxonomy (passwords, corruption, page ranges, locks, missing files), enhanced toasts with headlines, actionable hints, and expandable technical details with clipboard copy for power users. Verified 100% pass across 11 test suites (37/37 tests) in `reports/HUMAN_FRIENDLY_UI_ERRORS_REPORT.md`. |
| 5.9.9 | Automated PDF Structural & Conformance Verification (`qpdf` & `veraPDF`) `[Spec 034]` ✅ | Merged (PR #167, commit `0d61c0f`, Session 12877595132953884388) — Automated structural and conformance verification harness with `qpdf --check` and `veraPDF` ISO PDF/A-1b audits across all core outputs (`repair`, `linearize`, `compress`, `merge`, `split`, `pdf-a`), integrated into `scripts/verify_pdf_conformance.sh`, `Makefile` (`make test-conformance`), and `.github/workflows/rust.yml` CI runner (`reports/PDF_CONFORMANCE_VERIFICATION_REPORT.md`). |
| 5.9.10 | Local AI Agent Legal Due Diligence & Discovery Demo Harness `[Spec 035]` ✅ | Merged (PR #168, commit `8150fff`, Session 3245014174250604924) — Complete offline demo harness showcasing autonomous agent PDF manipulation (`pdf_rotate`, `pdf_redact`, `pdf_bates`, `pdf_merge`, `pdf_watermark`, `pdf_linearize`, `pdf_hash`), pure-Rust fixture generator `demo-legal-fixtures`, `scripts/run_ai_legal_demo.sh`, `make demo-legal-discovery`, and documentation (`demo/legal_discovery/README.md`, `reports/LOCAL_AI_AGENT_LEGAL_DISCOVERY_DEMO_REPORT.md`). |
| 5.9.11 | Bidirectional JSON-to-PDF Synthesis with Multimodal Images `[Spec 036]` ✅ | Merged (PR #169, commit `3688ba3`, Session 5118308487879675704) — Implemented pure-Rust `json_to_pdf` synthesis via `lopdf` + `image` (Approach 1: Native Engine Layout), 5-interface mandate (CLI, MCP, REST, WASM, Edge), round-trip fidelity testing, and 20 E2E assertions (`reports/JSON_TO_PDF_SYNTHESIS_REPORT.md`, `wiki/036-Json-To-Pdf-Synthesis.md`). |
| 5.9.12 | Diagonal, Semi-Transparent, and Configurable Watermarks `[Spec 037]` ✅ | Merged (PR #176, Session 11421227794067448625) — Upgraded `pdf_watermark` across all 5 deployment surfaces (CLI, MCP, REST, WASM, Edge) and Desktop UI. Defaults to 45° diagonal angle with semi-transparent opacity (0.2) and configurable font size, color, opacity, angle, and target pages. Documented in `reports/DIAGONAL_CONFIGURABLE_WATERMARKS_REPORT.md`. |
| 5.9.13 | High-Fidelity Pure-Rust PDF-to-Word (DOCX) Layout & Style Reconstruction `[Spec 038]` ✅ | Merged (PR #177, commit `6cfa5ae`, Session 2163210904964205811) — Upgraded `PdfToDocxOperation` with heading hierarchy inference, bold/italic font styling, RGB text color extraction, center/right alignment calculation, embedded image extraction, and native OpenXML page breaks via `docx-rs`. Included standalone WASM/Edge compatibility and fully verified across all 5 Penta-Interface surfaces (`reports/PDF_TO_DOCX_LAYOUT_RECONSTRUCTION_REPORT.md`). |
| 5.9.14 | Advanced Multi-Column & Table Document Layout Analysis via `pdf-oxide` `[Spec 039]` ✅ | Merged (PR #182, commit `a5036b8`, Session 7944320664313422740) — Integrating `pdf_oxide` spatial span clustering, XY-Cut reading-order tree, and table boundary detection into `paperpilot-pdf` and `paperpilot-wasm` to generate structured multi-column flows and native OpenXML tables (`specs/039-pdf-oxide-layout-analysis/spec.md`, `reports/PDF_OXIDE_LAYOUT_ANALYSIS_REPORT.md`). |
| 5.9.15 | Universal `pdf_oxide` Core Engine Migration & High-Performance Extraction Suite `[Spec 040]` ✅ | Completed — Migrated `pdf_convert_markdown`, first-class `pdf_to_html` (with Desktop UI card & responsive styling), `pdf_extract_text`, `pdf_search`, and zero-Python RAG chunking (`pdf_rag_chunks`) to `pdf_oxide` (v0.3.78) for sub-1ms speed, perfect word spacing, structured table/column extraction, and spatial bounding boxes (`specs/040-pdf-oxide-universal-migration/spec.md`). |
| 5.9.16 | Pure-Rust Chromium-Free URL-to-PDF Archival Engine via `browser_oxide` `[Spec 041]` ❌ | Omitted / Closed (PR #188 closed) — Evaluated and rejected due to excessive binary footprint and build bloat (+95,000 LOC, vendored headless browser with v8/skia). PaperPilot retains lean HTML-to-PDF synthesis via `pdf_convert_html` (fulgur/krilla). |
| 5.9.17 | Agentic Code Quality & Dead-Scaffolding Pruning via `fossil-mcp` `[Spec 042]` ⏳ | Planned — Integrating `fossil-mcp` code quality toolchain to audit and prune dead functions, duplicate clones, and orphan scaffolding across all 8 Rust workspace crates and Svelte apps (`specs/042-agentic-code-quality-fossil-mcp/spec.md`). |
| 5.9.18 | High-Speed Office Document Engine (DOCX, XLSX, PPTX & Legacy Binary) via `office_oxide` `[Spec 043]` ✅ | Completed — Integrating `office_oxide` into `paperpilot-pdf` and `paperpilot-wasm` for pure-Rust, ultra-fast reading, editing, and Markdown/text conversion of OOXML (`.docx`, `.xlsx`, `.pptx`) and legacy binary formats (`.doc`, `.xls`, `.ppt`) (`specs/043-pure-rust-office-engine-office-oxide/spec.md`, `reports/OFFICE_OXIDE_INTEGRATION_REPORT.md`). |
| 5.9.19 | Pure-Rust High-Precision OCR Engine via `rusto-rs` (RapidOCR / PaddleOCR + RTen) `[Spec 044]` ✅ | Completed — Migrated `paperpilot-pdf/src/operations/ocr.rs` from `ocrs` to `rusto-rs` (PaddleOCR v4 + RTen) for sub-second page inference, DBNet polygon word detection, exact bounding box spatial positioning, and 100% mouse-drag cursor selectable PDF overlays (`reports/RUSTO_RS_OCR_MIGRATION_REPORT.md`). |
| 5.9.20 | Desktop UX: Office + PDF Unified Intake, Run Button Feedback & Context-Aware Tool Navigation `[Spec 045]` ✅ | Merged (PR #181, commit `42730fd`, Session 10047595857438929348) — Add spinner/elapsed-timer loading state to all Run buttons, expand DropZone to accept PDF + Office files (`.docx`, `.xlsx`, `.pptx`, `.doc`, `.xls`, `.ppt`) with dynamic icons and multi-format Tauri dialog, and add a context-aware tool switcher (PDF Tools / Office Tools / AI Tools) to the Operations Panel (`specs/045-desktop-ux-office-and-pdf-integration/spec.md`, `reports/DESKTOP_UX_OFFICE_PDF_INTEGRATION_REPORT.md`). |
| 5.9.21 | Structured Spreadsheet Ingestion & Tabular JSON/CSV Agent Tooling via `office_oxide` `[Spec 046]` ✅ | Completed — Native `.xlsx` / `.xls` sheet extraction exposing structured JSON arrays and CSV tables directly to AI agents via `paperpilot-mcp` (`excel_to_json`, `excel_to_csv`), CLI commands, and Gateway REST endpoints with zero external dependencies. See `reports/SPREADSHEET_JSON_CSV_INGESTION_REPORT.md`. |
| 5.9.22 | Multi-Format Document Workspace Search (`Ctrl+Shift+F` Across PDF & Office) `[Spec 050]` ✅ | Merged (PR #189, commit `dd6dce3`, Session 1398070672745823199) — Unified in-memory and disk text extraction across `.pdf`, `.docx`, `.xlsx`, and `.pptx` workspaces, powering rapid search match highlighting, sentence snippets, and cross-document navigation in the desktop app. See `specs/050-multi-format-workspace-search/spec.md`. |
| 5.9.23 | Enterprise Legacy Office Watchdog & Batch Modernization Pipeline ⏳ | Planned — Headless directory watcher and batch CLI tool (`paperpilot office modernize-dir /input /output`) that automatically migrates legacy binary `.doc`, `.xls`, and `.ppt` archives into standard `.docx`, `.xlsx`, and `.pptx` files with corruption tolerance. |
| 5.9.24 | Presentation Slide Handout & Executive Brief Synthesis (`pptx-to-pdf`) ⏳ | Planned — Extract slide hierarchies, tables, and speaker notes from PowerPoint decks via `office_oxide` to synthesize compact executive summary handouts and meeting briefs in PDF. |
| 5.9.25 | Desktop OperationsPanel Modularization & Refactoring (`handleRunOperation`) `[Spec 048]` ✅ | Merged (PR #186, commit `e82ddbd`, Session 13180940037992674741) — Refactored the 500+ line monolithic `handleRunOperation` function into modular services (`toolRegistry.ts`, `operationArgsBuilder.ts`, `operationValidator.ts`) with 19 passing unit tests and clean Svelte component lifecycle. |
| 5.9.26 | Portable App Architecture & Modular Feature Pack Onboarding `[Spec 049]` ✅ | Merged (commit `95939b2`) — Implemented `./data/` discovery engine (`storage.rs`), Feature Pack Registry (`pack_commands.rs`), and Svelte 5 First-Run Modular Onboarding Wizard (`ModularSetupModal.svelte`). See `specs/049-portable-app-modular-installer-and-feature-packs/spec.md`. |
| 5.9.27 | Modular OCR Model Downloader and Operations Runtime History Badge `[Spec 047]` ✅ | Merged (PR #185, Session 5371306461973413988) — Multi-tier OCR model manager (PP-OCRv6 Tiny, Small, Medium, Mobile) in `ocr_commands.rs`, download progress events, and execution latency & last-task history badge in Desktop UI (`specs/047-modular-ocr-model-downloader-and-history-badge/spec.md`, `reports/MODULAR_OCR_DOWNLOADER_AND_HISTORY_REPORT.md`). |
| 5.9.28 | Penta-Interface E2E Suite Expansion (5 New Tools) & Unmocked Playwright Desktop Verification `[Spec 052]` ✅ | Merged (PR #191, commit `2d71840`, Session 9530387047429935978) — Added 100 new Penta-Interface assertions across CLI, MCP, REST, WASM, and Edge for `excel_to_json`, `excel_to_csv`, `pdf_rag_chunks`, `pdf_to_html`, and `workspace_search`. Implemented and verified unmocked real-engine Desktop Playwright test suite (`tests/e2e_real_desktop_execution.spec.ts`) for all Top 12 core launch tools with 100% PASS (`reports/PENTA_INTERFACE_EXPANSION_AND_REAL_PLAYWRIGHT_REPORT.md`). |
| 5.9.29 | Adaptive Multi-Layout Workspace & Studio Hub Unification `[Spec 053]` ✅ | Merged (PR #192, commit `5f67cb2`, Session 18170722690646801084) — Modernized Desktop Home into an enterprise Studio Hub with live system status pills (`100% On-Device`, `Port 7823 REST Gateway`), compact drag-drop intake, loaded document queue shelf, and categorized full-width operations cards matrix. Refactored tool catalogs into `toolList.ts` and `tools.ts` (`specs/053-adaptive-multi-layout-workspace/spec.md`). |
| 5.9.30 | Context-Aware Dynamic Tool Dispatcher & Feature Pack Filtering `[Spec 054]` ✅ | Merged (PR #194, commit `2ae374f`, Session 1673891404179339327) — Intelligent, context-aware tool recommendation engine adapting available/spotlighted tools based on uploaded document types (Excel, single PDF, multi-PDF) and installed Feature Packs, with contextual filter chip bars (`Recommended`, `All Tools`, `PDF`, `Office`, `AI & OCR`) in `OperationsPanel.svelte` and Studio Hub (`specs/054-context-aware-dynamic-tool-dispatcher/spec.md`). |
| 5.9.31 | Comprehensive Unmocked Playwright Desktop Verification for All 50+ Tools `[Spec 055]` 🔄 | In Progress (Jules Session 2996870268221322258, commit `9c6f2f2`) — Expand unmocked real-engine desktop Playwright test harness to all 50+ tools (`apps/desktop/tests/e2e_all_tools_real_execution.spec.ts`), testing parameters, UI interactions, and verifying real disk outputs (`specs/055-all-tools-real-playwright-desktop-e2e/spec.md`, `reports/ALL_50_TOOLS_REAL_DESKTOP_E2E_SCORECARD.md`). |
| 5.9.32 | Penta-Interface Conversions Parity & Gateway Route Completion `[Spec 056]` ✅ | Merged (commit `2fca4f5`, Session 623008379935493326) — Completed REST route handlers in `paperpilot-gateway` (`office-to-markdown`, `office-extract-text`, `rag-chunks`), synced CLI subcommands (`extract` alias), generated legacy `test.xls` fixture, wired `workspace_search`, and achieved 100% authentic pass rate (301/301 conversions, 1041/1041 master suite) in `reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT.md`. |
| 5.9.33 | Penta-Interface In-Memory WASM Parity & Sandboxed E2E Hardening `[Spec 057]` ✅ | Merged (PR #199, commit `1e6df92`, Session 3241956896051261198) — Sandboxed output directory cleanup per run, true in-memory buffer execution for WASM/Edge conversion tools (replacing simulated markers with real byte assertions), and deep Playwright semantic validation across 50+ tools (`specs/057-penta-wasm-parity-and-e2e-hardening/spec.md`). |
| 5.9.34 | Independent Multi-Interface Packaging & Modular Distribution `[Spec 058]` ✅ | Merged (PR #196, commit `5f574ed`, Session 14868958832212048254) — Independent standalone release packaging for `paperpilot-cli`, `paperpilot-mcp`, `paperpilot-gateway`, and `paperpilot-all`, with shell completions (`paperpilot completions <SHELL>`) and cargo feature flags for minimal lean builds (`specs/058-modular-packaging-and-distribution/spec.md`). |
| 5.9.35 | Real-World User Feedback Polish & Engine Hardening `[Spec 059]` ✅ | Merged (PR #200, commit `537550c`, Session 6713598024074314673) — Fixed desktop missing output paths (`excel_to_json`), enforced `.csv` extensions, added split "every N pages", added raster image downsampling for compression, OCR model availability guidance, embedded images in DOCX conversions, and preserved Penta-interface + NLP/LLM integrity (`specs/059-user-feedback-polish-and-hardening/spec.md`). |
| 5.9.36 | Pure-Rust 1-Hour Sustained Soak & Endurance Benchmark Harness `[Spec 060]` ✅ | Merged (PR #205, commit `a2b7c9d`, Session 1504835173939937428) — Executed full 1-hour sustained endurance benchmark (3,600.0s, 340,834 cycles, 8,180,016 total operations executed at 2,272.2 ops/sec sustained throughput) with 100.00% success rate, 0 panics, zero memory leaks (stable at 142.45 MB RSS across all 60 minutes), and detailed per-operation latency breakdown in `reports/ONE_HOUR_SUSTAINED_BENCHMARK_REPORT.md` and `wiki/19-One-Hour-Endurance-Benchmark.md`. |
| 5.9.37 | Clean-Room E2E Verification & Audit for Top 12 Core Launch Tools `[Spec 061]` ✅ | Merged (PR #202, commit `d24edb5`, Session 2246403833456447715) — Clean-room isolated sandbox audit for Top 12 Core Launch Tools in `paperpilot-cli/tests/top12_cleanroom_audit.rs`, verified 100% PASS with zero overrides (`reports/TOP_12_CORE_TOOLS_AUDIT_REPORT.md`). |
| 5.9.38 | Tauri 2 Native Mobile Backend Scaffolding & Native Engine Integration `[Spec 062]` ✅ | Merged (PR #203, commit `07c4a56`, Session 15776658825085896715) — Tauri 2 Android/iOS project scaffolding, lean mobile feature-gating (stripping `paperpilot-gateway` and `paperpilot-mcp` on mobile via `desktop-features` flag), capabilities configuration, and in-memory byte buffer bridge (`process_mobile_buffer`) for sandboxed mobile storage (`specs/062-tauri-mobile-backend-scaffolding/spec.md`, `reports/TAURI_MOBILE_BACKEND_SCAFFOLDING_REPORT.md`, `docs/MOBILE_SETUP.md`). |
| 5.9.39 | Mobile Touch UI, Camera Scanner with Auto-OCR & Adaptive Layout (`apps/desktop`) `[Spec 063]` 🔄 | In Progress (Jules Session 5037301362112166263) — Mobile touch-first Svelte 5 + Tailwind v4 layout adaptation, unified single-UI architecture without dual codebases, on-device Camera Document Scanner with sequential multi-page capture and on-device Auto-OCR (`images_to_pdf` + `ocr`), bottom navigation bar (Documents, Scan, Tools, Settings) with PaperPilot logo, swipeable bottom sheet for operations panel with ≥48px tap targets, canvas pinch/swipe gestures, safe-area inset protection, in-memory mobile document intake, and APK release size optimization (`specs/063-mobile-touch-ui-adaptation/spec.md`). |
| 5.9.41 | 50-Tool Complete Clean-Room Isolated Sandbox Verification & Audit `[Spec 064]` ✅ | Completed (Session 13530573138717224797) — Comprehensive automated clean-room verification of all 50 PaperPilot tools in an isolated timestamped sandbox directory (`paperpilot-cli/tests/all50_cleanroom_audit.rs`, `make test-cleanroom-all50`), recording exact inputs, outputs, latencies, and deep semantic assertions in `reports/ALL_50_TOOLS_CLEANROOM_AUDIT_REPORT.md` (50/50 100% PASS). |
| 5.9.42 | High-Fidelity Responsive Flow Layout & Inline Image Reconstruction for PDF-to-DOCX `[Spec 065]` ✅ | Merged (PR #209, commit `91b7beb`, Jules Session 11237800863675110882) — Upgraded `PdfToDocxOperation` in `paperpilot-pdf` with font subset prefix sanitization (`sanitize_font_name`, eliminating tofu `□□□□` boxes), 2-column borderless OpenXML tables for side-by-side icon/text pairing, inline image extraction (`docx-rs::Pic`), and compact page margin geometry keeping single-page documents to strictly 1 page (`specs/065-pdf-to-docx-flow-layout-images/spec.md`). |
| 5.9.43 | Tool Disambiguation & Explicit Input/Output Format Badges Across Desktop Studio `[Spec 066]` ✅ | Merged (PR #207, commit `42b6f29`, Jules Session 14117039404412235313) — Disambiguated overlapping tool titles (PDF to Plain Text vs Office to Plain Text vs Extract PDF Pages) and added explicit `INPUT ➔ OUTPUT` format badges (`inputFormat`, `outputFormat`) across all 50+ tool cards in `OperationsPanel.svelte`, `toolList.ts`, and Studio Hub, and standardized format boundaries across MCP, CLI, and REST Gateway (`specs/066-tool-format-badges-and-disambiguation/spec.md`). |
| 5.9.44 | Production-Grade PDF Compression Optimization (Image Resampling & Stream Flate Compaction) `[Spec 067]` ✅ | Merged (PR #208, Jules Session 6600304163605773570) — Upgraded `CompressOperation` in `paperpilot-pdf` with preset-driven image downsampling, JPEG quality levels (low/medium/high), non-image stream Flate encoding (`flate2`), dead/orphan object pruning (`prune_objects`), and file size non-expansion guards with all 5 unit tests passing cleanly (`specs/067-production-pdf-compression/spec.md`). |
| 5.9.45 | PDF-to-DOCX High-Fidelity Semantic Layout & Side-by-Side Table Reconstruction `[Spec 068]` ⏹️ | Superseded — Evaluated in Session 4642962178811848175; superseded by merged Task 5.9.47 / Spec 070. **Pending for Wave 3**: Vector Form XObject (`/Fm0`–`/Fm4`) bezier path rasterization. In marketing brochures like `Smallpdf.pdf`, card illustrations are raw bezier operator paths (`m`, `l`, `c`, `f`) rather than raster `/Subtype /Image` streams. To achieve full pixel-level fidelity: use `hayro`/`tiny-skia` or `pdfium-render` to rasterize the bounding boxes of Form XObjects into PNG bytes and embed them via `docx_rs::Pic::new(&png_bytes)`. |
| 5.9.46 | Universal HTML/Markdown AST Bridge for High-Fidelity PDF-to-DOCX `[Spec 069]` ⏹️ | Superseded — Evaluated in Session 8784878538674900730 (PR #211); superseded by clean Task 5.9.47 / Spec 070. **Pending for Wave 3**: AST image bridge integration. While `pdf_oxide` captures text hierarchy (`h1`, `h3`, `h4`, `p`), markdown AST lacks spatial bounding boxes for non-raster vector graphics. To achieve this: pair the AST parser with spatial graphics bounding box detectors or SVG/DrawingML shape synthesis (`<w:drawing>`). |
| 5.9.47 | PDF-to-DOCX Conversion via `unpdf` Recursive XY-Cut Layout Analysis `[Spec 070]` ✅ | Merged (PR #212, Jules Session 7824070929951783545) — Integrated `unpdf v0.46.0` recursive XY-Cut layout analysis into `PdfToDocxOperation`, reconstructing 4 side-by-side borderless OpenXML tables (`<w:tbl>`), centered headers, and compact 0.5-inch margins for strictly 1-page geometry in LibreOffice/Word (`specs/070-pdf-to-docx-unpdf-bridge/spec.md`). |
| 5.9.48 | Recursive XY-Cut Multi-Column Text Extraction via `unpdf` `[Spec 071]` 📋 | Planned (Wave 2/3) — Upgrade `PdfExtractTextOperation` in `paperpilot-pdf` to use `unpdf::to_text` and recursive XY-Cut column boundary detection, eliminating column-scrambling bugs on scientific papers and brochures. |
| 5.9.49 | High-Fidelity PDF-to-JSON Document Model via `unpdf` Layout Tree `[Spec 072]` 📋 | Planned (Wave 3) — Enhance `PdfToJsonOperation` using `unpdf::to_json` and `unpdf::model` to provide structural bounding boxes, reading order, and semantic blocks for AI RAG ingestion. |
| 5.9.50 | Layout-Aware Borderless Table Detection for PDF-to-Excel via `unpdf` `[Spec 073]` 📋 | Planned (Wave 4) — Integrate `unpdf::detect` gridline and table boundary detection into `PdfToXlsxOperation` to accurately map complex borderless tables into multi-cell Excel workbooks. |
| 5.9.51 | Full 50-Tool Penta-Interface Verification & Documentation Parity `[Spec 064]` | Completed (PR #214) — Comprehensive end-to-end verification across all 5 interfaces (CLI, MCP, REST Gateway, WASM, Edge), updating `50-tools-handbook.md` with verified, copy-pasteable snippets across CLI, REST, and MCP for all 50 tools (`prompts/tasks/done/phase5/P5_9_51_all_50_tools_penta_audit_and_docs_sync.txt`). |
| 5.9.52 | PDF-to-DOCX High-Fidelity Layout Synthesis & Form XObject Vector Image Extraction `[Spec 074]` | Completed (PR #215) — Resolves vector Form XObject (`/Fm0`..`/Fm4`) graphic extraction in `PdfToDocxOperation`, producing alternating 2-column borderless OpenXML tables with visual illustration containers matching `fixtures/Get_Started_With_Smallpdf_EXPECTED.docx` (`specs/074-pdf-to-docx-high-fidelity-layout-and-images/spec.md`). |
| 5.9.53 | Generic Dynamic Layout, N-Column Table Generation & Vector Illustration Rasterization for PDF-to-DOCX `[Spec 075]` 📋 | Planned (Stage 2) — Generalizes `PdfToDocxOperation` to adaptively compile Markdown/HTML into DOCX (arbitrary N-column tables via `pulldown-cmark`, standard flowing paragraphs, and real PNG rasterization/embedding). |
| 5.9.54 | High-Fidelity PDF-to-Markdown & PDF-to-HTML with Embedded Media `[Spec 076]` ✅ | Merged (PR #216, commit `66534e7`) — Stage 1 of the decoupled conversion pipeline: Upgraded `PdfToMarkdownOperation` and `PdfToHtmlOperation` to extract both raster photos (`/Subtype /Image`) and vector Form XObjects (`/Subtype /Form` via `hayro` 150 DPI rasterization), embedding visual assets cleanly into Markdown and responsive HTML as base64 data URIs. |
| 5.9.55 | Decoupled Markdown-to-DOCX Compiler with DrawingML Media & Layout Synthesis `[Spec 077]` ✅ | Completed (commit `8ddc7ff`, Jules Session `378425988505835118`) — Stage 2: Consumes Stage 1 Markdown, parses via `pulldown_cmark`, decodes base64 data URIs, scales to DrawingML EMUs, and emits genuine `docx_rs::Pic` image parts inside `.docx` archives, eliminating all placeholder text. |
| 5.9.56 | Tool Discovery & Registry Endpoints Across All 5 Surfaces (`list-tools`) ✅ | Completed (commit `897b902`) — Implemented `paperpilot list-tools` CLI command, Axum Gateway `GET /api/v1/tools` discovery endpoint, updated OpenAPI 3.1 & Swagger UI schemas, and `WasmPdfEngine.list_tools()` for in-browser tool capability discovery across all 52 tools. |
| 5.9.58 | Structured Tabular Data Extraction API (`pdf_extract_tables`) `[Spec 090]` ✅ | Completed (PR #234, Jules Session [`334056780263731633`](https://jules.google.com/session/334056780263731633)) — Pure-Rust table extraction engine using bounding box proximity and alignment clustering; generates structured cell matrices, headers, JSON coordinates, and RFC 4180 CSV across all 5 surfaces (CLI, MCP, REST, WASM, Edge) with 10/10 (100%) unmocked Penta-Interface assertions (`reports/PENTA_INTERFACE_E2E_TABLE_REPORT.md`). |
| 5.9.60 | PII & Sensitive Information Scanner (`document_pii_scan`) `[Spec 086]` ✅ | Completed (Merged PR #232) — Automated pure-Rust pattern detector for SSNs, credit cards (Luhn validated), emails, phone numbers, and IBANs (MOD 97-10 validated) returning masked values and structured audit reports across all 5 surfaces (CLI, MCP, REST Gateway, WASM, Cloudflare Edge) with 10/10 (100%) unmocked Penta E2E test verification (`reports/PENTA_INTERFACE_E2E_PII_SCAN_REPORT.md`). |
| 5.9.61 | Digital Signature Verification Engine (`pdf_validate_signatures`) ✅ | **Completed** (PR #238, commit `a6e68c6`) — Cryptographic verification of PKCS#7 / CMS digital signatures, `/ByteRange` integrity checks, signer identity, and post-signing tampering detection complementing `pdf_sign`. 10/10 unmocked Penta-Interface E2E tests passing. |
| 5.9.62 | Accessibility & PDF/UA Compliance Verifier (`pdf_validate_accessibility`) 📋 | Planned (Phase 1 Priority) — Audit document tagged structure tree, reading order, language tags (`/Lang`), and image alt text (`/Alt`) for Section 508 and European Accessibility Act compliance. |
| 5.9.63 | Semantic Legal & Clause Diffing (`pdf_compare_semantic`) 📋 | Planned (Phase 1 Priority) — Meaning-based clause and sentence comparison for contracts, terms of service, and policies, outputting structured Markdown changelogs alongside visual diffs. |
| 5.9.64 | Declarative Multi-Step Document Workflow Runner (`document_workflow`) 📋 | Planned (Phase 2 Priority) — Chain multiple tools into reusable pipelines (e.g. `deskew ➔ sanitize ➔ compress ➔ watermark ➔ convert`) without intermediate disk roundtrips. |
| 5.9.65 | Resilient Batch Processing & Failure Reporting (`document_batch_process`) 📋 | Planned (Phase 2 Priority) — Batch directory processing engine with concurrent task workers, automatic retries, progress telemetry, and structured `failure_report.json`. |
| 5.9.66 | Convenience Multi-Page Image Exporter (`pdf_to_images`) 📋 | Planned (Phase 2 Priority) — One-call convenience exporter rendering all pages of a document to a directory as PNG, JPEG, or WebP. |
| 5.9.67 | Mobile Touch-First WASM Engine & Embed Demonstration `[Spec 087]` ✅ | Completed (PR #231) — Pure in-memory WASM execution for mobile emergency operations (Camera Scan, Sign, Merge, Rotate, Sanitize, Compress) with touch-first UI (≥48px targets, bottom-sheet parameter drawer, safe-area protection, and native `navigator.share`) bypassing native mobile cross-compilation and scoped storage friction (`specs/087-mobile-wasm-touch-first-embed/spec.md`). |
| 5.9.68 | Universal Pipeline Parity & Comprehensive Parameter Configuration `[Spec 079]` ✅ | Completed (PR #221, commit `b31bc2c`) — Universal tool parity, full parameter configuration schemas across Desktop and Embed, and intermediate split fan-out resolution. |
| 5.9.69A | Pipeline Flow State, Local Persistence & Industry Presets `[Spec 081]` ✅ | Completed (PR #220, commit `460ec1d`) — Data model (`SavedPipeline`), 3 industry default presets (`preset-legal-discovery`, `preset-clean-scan`, `preset-redact-protect`), `localStorage` persistence, JSON serialization & 100% Vitest unit tests in `apps/desktop/src/lib/state/pipeline.svelte.ts`. |
| 5.9.69B | Visual Pipeline Flow Selector Toolbar & Save/Import Dialogs `[Spec 081]` ✅ | Completed (PR #222, commit `adf293c`) — `FlowSelector.svelte`, `SaveFlowModal.svelte`, Embed preset prop mapping in `Widget.svelte`, JSON export/import buttons, and 100% passing Playwright E2E verification (`e2e_pipeline_flow_management.spec.ts`). |
| 5.9.69C | Penta-Interface Declarative Workflow Engine & Recipe Execution `[Spec 081]` ✅ | Completed (Session `3461133562467863809` / `15691603918731845147`) — Full job lifecycle (list, inspect, execute) using the Single Core Runner (`run_workflow_bytes`) + 5 Thin Wrappers pattern across CLI, MCP, REST Gateway, WASM Engine, Edge Worker, and 20/20 (100%) passing unmocked Penta-Interface E2E suite (`reports/PENTA_INTERFACE_E2E_WORKFLOW_REPORT.md`). |
| 5.9.69D | Visual Pipeline "Export as Code / API / MCP Prompt" Modal `[Spec 081]` ✅ | Completed (PR #223, commit `5b4436b`, Jules Session `4310793047227771998`) — Developer & AI ergonomics modal in Desktop UI (`FlowSelector.svelte`), generating copyable snippets for cURL, CLI command, MCP Agent prompt, and Embed JS API (`ExportCodeModal.svelte`, with 100% passing Playwright E2E verification). |
| 5.9.70 | Wave 2 Penta-Interface E2E & Unmocked Playwright Testing Suite (6 Tools) ✅ | Completed (PR #221, commit `b7ff7c3`) — Comprehensive unmocked testing across all 5 interfaces (CLI, MCP, REST Gateway, WASM Playwright, Desktop) for Wave 2 tools (`img_to_pdf`, `extract_images`, `remove_blank`, `page_numbers`, `burst`, `search`), with 100% test passes across `wave2_mcp_tests`, CLI tests, Gateway tests, and `wiki/09-Wave-2-Penta-Testing.md`. |
| 5.9.71 | Universal "Export as Code / API / MCP Prompt" Workbench (All Tools) `[Spec 082]` ✅ | Completed (PR #224, commit `95fcd5a`, Jules Session `17427853372924290951`) — Centralized headless `snippetGenerator.ts`, unified `ExportCodeModal.svelte` for both pipelines and individual action cards, added `</> Code / API` button to `EditableActionCard.svelte`, 6/6 unit tests passing, and Playwright verification. |
| 5.9.73 | On-Device Document SLM (PaperPilot-Qwen-0.5B) Dataset Generator & Local QLoRA Pipeline `[Spec 084]` ✅ | **Completed & Verified** — Python synthetic ChatML dataset generator (5,000+ pairs across 45+ tools with 5 linguistic noise tiers), 4-bit QLoRA fine-tuning engine (`tools/slm-training/train_local.py`) on local NVIDIA RTX 4070 (846 steps, train loss dropped from ~2.1 to 0.47, eval loss 0.496, 90.4% token accuracy), and automated test bench (`tools/slm-training/infer_local.py`) verifying 100% exact JSON tool call generation (`specs/084-on-device-document-slm-qwen-finetuning/spec.md`). |
| 5.9.74 | HTML & Markdown to PDF Full-Fidelity Image Rendering via `AssetBundle` `[Spec 088]` ✅ | Completed (PR #230, commit `f276a78`) — Full-fidelity inline image rendering for HTML & Markdown to PDF synthesis via `fulgur::asset::AssetBundle`. Parses both local filesystem paths and base64 data URIs (`data:image/*;base64,...`), decodes into raw buffers in-memory, maps asset names, and passes directly into `fulgur::Engine`, with 100% unit tests passing (`test_html_to_pdf_with_base64_image`, `test_md_to_pdf_with_base64_image`). |
| 5.9.75 | Mobile Lean Architecture, Full 26+ WASM Tool Suite & On-Demand OCR Engine `[Spec 089]` ✅ | Completed (Merged commit `df8c673`) — Cut Android/iOS debug APK from 82 MB to $\le$ 22 MB by moving heavy server sidecars behind `desktop-features` in `src-tauri/Cargo.toml`; enabled the full 26+ WASM tool catalog on mobile with a touch-friendly **Quick Actions + Categorized Catalog** UI in `OperationsPanel.svelte`; and implemented cross-platform on-demand download of `ppocrv6-tiny` model weights (~6 MB) on first OCR execution (`specs/089-mobile-lean-architecture-and-on-demand-ocr/spec.md`). |
| 5.9.76 | Enterprise SIEM Compliance & Tamper-Evident Audit Logging `[Spec 091]` 📋 | Planned (Phase 1 B2B Priority) — Structured immutable audit events (JSON format) capturing tool executions, SHA-256 file fingerprints, redactions, signatures, and timestamps; streaming endpoints for Splunk, Datadog, CloudWatch, and local `audit.log` compliance trails. |
| 5.9.77 | Air-Gapped Ed25519 Cryptographic License Key Validator `[Spec 092]` 📋 | Planned (Phase 1 B2B Priority) — Offline signed license file parser (`license.key` via Ed25519) enforcing seat limits, tier capabilities, and expiration timestamps without any phone-home or outbound network connection for classified SCIFs and defense environments. |
| 5.9.78 | Universal Remote PDF URL Ingestion Across Penta-Interface `[Spec 093]` ✅ | Completed (Merged PR #236, commit `d826eb4`) — Ingestion of remote PDF documents directly via HTTP/HTTPS URLs (e.g. S3 presigned links, public documents, webhooks) with SSRF protection, magic byte verification, and zero-upload cURL/API support across Edge, Desktop UI, CLI, and MCP (`specs/093-universal-remote-pdf-url-ingestion/spec.md`, `wiki/URL-Ingestion.md`). |
| 5.9.79 | Pure-Rust Penta-Interface Tool Scaffolding Generator `[Spec 094]` ✅ | Completed (commit `5fd9155`) — Standalone pure-Rust CLI tool in `tools/penta-scaffold` that generates 100% compiling boilerplate across all 5 surfaces (Core Rust, CLI, MCP Server, REST Gateway, WASM, Cloudflare Edge, Desktop UI, and Penta E2E Suite) from a single command (`specs/094-pure-rust-tool-scaffolding-generator/spec.md`, `wiki/ToolScaffolding.md`). |
| 5.9.80 | Penta-Scaffold UI Automation & Universal Audit Report Visualizer `[Spec 100]` ✅ | Completed (PR #240, commit `5ca9e5c`) — Automatically injects `operationArgsBuilder.ts` match arms and Vitest unit tests on tool scaffolding, retrofits `pdf_validate_signatures`, `document_pii_scan`, and `pdf_extract_tables`, and renders an interactive `<AuditReportViewer />` card for structured JSON audit reports (`specs/100-penta-scaffold-ui-automation/spec.md`). |
| 5.9.81 | Wave 4 (Office Modernization & Spreadsheets) Unmocked E2E & Real Testing Suite `[Spec 110]` 🚀 | ✅ **Completed & Merged (PR #256)** — Comprehensive unmocked testing suite (`paperpilot-mcp/tests/wave4_mcp_tests.rs`) covering all 6 Wave 4 operations (`pdf_to_docx`, `pdf_to_xlsx`, `pdf_to_pptx`, `office_to_markdown`, `office_extract_text`, `office_modernize_legacy`), magic byte assertions, and tester documentation (`specs/110-wave4-office-real-test-suite/spec.md`, `wiki/33-Wave-4-Office-Penta-Testing.md`). |
| 5.9.82 | Web Portal Mobile Handoff QR Code & Share Modal (`<ShareModal />`) `[Spec 113]` ✅ | **Completed** — Client-side verified SVG QR code asset (`apps/web/public/qr_code.svg`), 1-click clipboard copy, and native `navigator.share` integration in web portal header navigation (`specs/113-web-share-qr-modal/spec.md`). |
| 5.9.83 | Web Portal Dynamic GitHub Releases Download Counter Badge `[Spec 114]` ✅ | **Completed** — Svelte 5 component (`apps/web/src/components/DownloadBadge.svelte`) querying releases download metrics with `sessionStorage` caching (15m TTL), protected behind `ENABLE_DOWNLOAD_COUNT_BADGE` feature flag and `MIN_DOWNLOAD_COUNT_THRESHOLD` in `apps/web/src/constants.ts` (`specs/114-web-dynamic-download-badge/spec.md`). |
| 5.9.84 | Web Portal Use Cases Showcase & Interactive Scenario Cards `[Spec 115]` ✅ | **Completed** — Interactive Svelte 5 component (`apps/web/src/components/UseCasesShowcase.svelte`) highlighting 6 primary personas (Legal, Healthcare/HIPAA, Finance, Academic, Print, Developers) with problem/solution descriptions and deep-links to WASM tools (`specs/115-web-use-cases-showcase/spec.md`). |
| 5.9.85 | Scanned Dual-Page Book Center Spine Split (`pdf_split_scanned_book`) `[Spec 112]` ✅ | **Completed & Merged (PR #261)** — Pure-Rust dual-page center spine splitting engine supporting horizontal/vertical splits, configurable gutter overlap margins, and LTR/RTL page ordering across Core, CLI, MCP, and Gateway (`specs/112-pdf-split-scanned-book/spec.md`). |
| 5.9.86 | Modify PDF Permissions & Rights Management (`pdf_change_permissions`) `[Spec 111]` ✅ | **Completed & Merged (PR #262)** — Standard ISO 32000-1 `/P` permission flag management via `lopdf::EncryptionState` and `EncryptionVersion::V2`, supporting toggles for printing, copying, modifications, annotations, form filling, and accessibility across Core, CLI, MCP, and Gateway (`specs/111-pdf-change-permissions/spec.md`). |
| 5.9.87 | Interleave / Alternate PDF Collation (`pdf_merge_alternate`) `[Spec 117]` ✅ | **Completed & Merged into `feature/dev` (PR #268)** — Pure-Rust alternating two-document PDF page collator supporting configurable step cadences (`step`, `step_b`) and page reversal (`reverse_a`, `reverse_b`) across Core, CLI, MCP, Gateway, WASM, Edge, and Desktop (`specs/117-pdf-merge-alternate/spec.md`). |
| 5.9.88 | Split PDF by Outline / Bookmarks (`pdf_split_by_outline`) `[Spec 118]` ✅ | **Completed & Merged (PR #264)** — Recursively parses document `/Outlines` hierarchy and emits a segmented ZIP archive with individual chapters across Core, CLI, MCP, Gateway, WASM, Edge, and Desktop (`specs/118-pdf-split-by-outline/spec.md`). |
| 5.9.89 | Multi-Sheet Poster Tile Slicing Engine (`pdf_posterize`) `[Spec 119]` ✅ | **Completed & Merged into `feature/dev` (PR #270)** — Slices oversized or scaled documents across standard printable grid sheets (A4, Letter) with glue gutters (`overlap_pt`) and cut trim marks across Core, CLI, MCP, Gateway, and Desktop (`specs/119-pdf-posterize/spec.md`). |
| 5.9.90 | Mobile Touch Gestures, Viewport Panning & Dual-State Zoom Engine `[Spec 121]` ✅ | **Completed & Merged into `feature/dev` (PR #269)** — Isolates zoom transform to `.preview-content` in `PdfPreview.svelte`, implements double-tap toggle (1.0x <-> 2.5x), 2D bounded viewport panning when zoomed in, page navigation swipe when zoomed out, floating dock clearance padding, and 5/5 Vitest gesture tests passing cleanly (`specs/121-mobile-touch-gestures-and-viewport/spec.md`). |
| 5.9.91 | Lossless Object & Stream Deduplication & CMYK-Safe Compression `[Spec 122]` ✅ | **Completed & Merged into `feature/dev` (PR #275)** — Inspired by `presse` (`SimonBure/presse`): stream hashing pass to collapse duplicate images/fonts/ICC-profiles across pages into single indirect references (`pdf_dedup_streams`), plus CMYK color-space safety guards in `paperpilot-pdf/src/operations/compress.rs`. Tested across all 5 interfaces with 10/10 Penta-Interface E2E suite passing (`specs/122-lossless-stream-deduplication-and-cmyk-safety/spec.md`, `specs/122-lossless-stream-deduplication-and-cmyk-safety/task_122a.md`). |
| 5.9.92 | Production Security Hardening & Vulnerability Remediation `[Spec 123]` ✅ | **Completed & Merged into `feature/dev` (PR #271 & Local Gateway Gate)** — Remediates Semgrep/Trivy findings and community audit: Stored XSS snippet sanitization + CSP in `apps/desktop`, alphanumeric regex on `pack_id`, SSRF redirect/DNS lockdown in `paperpilot-core`, Dockerfile non-root user (PR #271), and CORS lockdown + body size limit in `paperpilot-gateway` (`specs/123-production-security-hardening/spec.md`, `specs/123-production-security-hardening/task_123a.md`). |
| 5.9.94 | Single-File Offline Self-Contained HTML Sandbox `[Spec 125]` ✅ | **Completed & Verified (Task 125A)** — Zero-dependency, air-gapped single-file `paperpilot-portable.html` distribution with inline base64-encoded WASM engine and responsive offline UI. Verified with Playwright emitting strictly 0 network requests and working PDF operations (`specs/125-single-file-offline-html-sandbox/spec.md`, `specs/125-single-file-offline-html-sandbox/task_125a.md`). |
| 5.9.95 | Open-WebUI & Apache Tika Replacement Ingestion Adapter `[Spec 126]` ✅ | **Completed & Merged into `feature/dev` (PR #273)** — Raw PDF upload endpoints for Open-WebUI's Tika 3 (`/tika/text`) and Tika 4 (`/tika/json/md`) loaders, root metadata array aliases, shared in-memory Rust extraction, and optional shared-folder custom tool. 5 core + 21 gateway tests and real HTTP/MCP smoke checks passed; small-fixture HTTP median 2.419 ms ([report](reports/TIKA_INGESTION_ADAPTER_REPORT.md), [setup](docs/integrations/open_webui.md)). |
| 5.9.96 | PDF-to-EPUB Semantic Engine via Intermediate AST `[Spec 127]` ✅ | **Completed & Merged (Task 127A / PR #286)** — Decompiles structured PDF documents into valid EPUB 3.0 e-books with reconstructed headings, paragraphs, and navigation outline (requested by `Shorel` on HN) across Core, CLI, MCP, Gateway, WASM, and Desktop (`specs/127-pdf-to-epub-semantic-engine/spec.md`). |
| 5.9.97 | CI Supply Chain Hardening & Action SHA Pinning `[Spec 128]` ✅ | **Completed & Merged into `feature/dev` (PR #274)** — Security audit remediation: Pinned all GitHub Actions across `.github/workflows/` to immutable 40-char commit SHAs, replaced unverified `wasm-pack` curl installer with `cargo install --locked`, and configured `.github/dependabot.yml` for automated weekly SHA tracking (`specs/128-ci-supply-chain-sha-pinning/spec.md`, `specs/128-ci-supply-chain-sha-pinning/task_128a.md`). |
| 5.9.98 | Automated Security Regression & Fuzzing Harness for Malformed PDFs `[Spec 129]` ✅ | **Completed (Task 129A / PR #276)** — Synthetic malformed PDF corpus (circular page trees, corrupt xref tables, giant stream lengths) ensuring all operations exit cleanly via `Result::Err` without panics or memory exhaustion (`tests/malformed_regression.rs`, `wiki/129-Fuzzing-Harness.md`). |
| 5.9.99 | Conversational SLM Multi-Step Pipeline Generator & Visual Editor `[Spec 084 / Task 84D]` ✅ | **Completed & Verified** — Python synthetic ChatML generator with parameter chaining (`/tmp/inter_1.pdf` ➔ `/tmp/inter_2.pdf` ➔ final output), 15 curated industry recipes, dynamic arbitrary tool sampling, sequential/inverted/pipe phrasing, and 8/8 pytest unit tests passing cleanly in `tools/slm-training/test_dataset_generator.py` (`specs/084-on-device-document-slm-qwen-finetuning/task_84d.md`). |
| 5.9.100 | OpenSSF Scorecard Supply Chain Hardening & Security Compliance `[Spec 133 / Task 133A]` ✅ | **Completed (PR #285 / `5ae175e`)** — Root `SECURITY.md` vulnerability reporting policy, workflow token permissions scoped to `permissions: read-all`, 40-char SHA pinning, and automated OpenSSF Scorecard SARIF workflow (`specs/133-openssf-scorecard-supply-chain-hardening/spec.md`, `specs/133-openssf-scorecard-supply-chain-hardening/task_133a.md`). Scorecard jumped from 4.7 to 8.2 / 10. |
| 5.9.101 | GGUF Quantization & Fast Local Execution for PaperPilot SLM (0.5B) `[Spec 132]` ✅ | **Completed & Published (Task 132A)** — Standalone, memory-mapped `Q4_K_M` GGUF quantization (397 MB, ~60% reduction from 988 MB FP16) for on-device tool calling and compound workflows. Published to Hugging Face Hub under `kpakkiragari/PaperPilot-Qwen-0.5B-Instruct` and registered in `models/registry.json` (`specs/132-gguf-slm-quantization/spec.md`). |




