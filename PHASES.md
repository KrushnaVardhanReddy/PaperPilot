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
| 1.1.4 | Set up test fixture corpus | Collect 20–30 real-world PDFs: normal, scanned, malformed, large, encrypted |
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
| 1.3.22 | Integrity hash | Compute SHA-256 / SHA-512 fingerprint of a PDF; output digest and file size for tamper detection |

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
| 1.3.30 | PDF → Word (DOCX) | Export PDF content to editable Word document |
| 1.3.31 | PDF → Excel (XLSX) | Export table content to spreadsheet |
| 1.3.32 | PDF → PowerPoint (PPTX) | Export PDF slides to PPTX |
| 1.3.33 | PDF → HTML | Export to HTML for web viewing |
| 1.3.34 | PDF → Markdown | Export to clean Markdown text |
| 1.3.38 | PDF → LLM-Ready Export | Structured export optimised for RAG pipelines: clean heading hierarchy, page numbers as metadata, tables as JSON, image captions — output as `{content}.md` + `{metadata}.json` sidecar pair |
| 1.3.39 | Document type classifier | Auto-detect document type (invoice, contract, research paper, form, report) using heuristics + AI; tag the output metadata accordingly |
| 1.3.46 | PDF → Structured JSON | Per-page structured text extraction: `{ document, pages: [{ page_number, text, word_count, has_images, tables[] }] }` — machine-readable format for AI agents, RAG pipelines, and enterprise integrations |

#### Group E — Forms
| # | Task | Notes |
|---|---|---|
| 1.3.35 | AcroForm read | Read form field values from interactive PDF forms |
| 1.3.36 | AcroForm fill | Fill and flatten PDF form fields programmatically |
| 1.3.37 | Form field creation | Add text/checkbox/dropdown form fields to any PDF |

#### Group F — PDF Creation (from scratch)
| # | Task | Notes |
|---|---|---|
| 1.3.40 | Markdown → PDF | Render a Markdown file into a pixel-perfect PDF (headings, tables, code blocks, images) |
| 1.3.41 | HTML → PDF | Render an HTML page/template into a PDF (full CSS support via headless renderer) |
| 1.3.42 | Template engine | Handlebars-style templates with variable substitution: `{{client_name}}`, `{{invoice_total}}` etc. |
| 1.3.43 | Built-in template library | Starter templates: Invoice, Contract, Report, Certificate, Letter, Resume |
| 1.3.44 | Template → PDF CLI | `paperpilot create --template invoice.hbs --data data.json -o invoice.pdf` |
| 1.3.45 | Custom page layout | Margin, paper size (A4/Letter/Legal), orientation, header/footer per template |

---

### 1.4 — Testing `[PARALLEL with 1.3]`

Write tests alongside each operation, not after.

| # | Task | Notes |
|---|---|---|
| 1.4.1 | Unit tests per operation ✅ | Written inline in every operation (`#[cfg(test)]`) |
| 1.4.2 | Round-trip tests ✅ | Merged (`fixture_tests_group_a/b/c.rs`) |
| 1.4.3 | Fixture-based integration tests ✅ | Merged (`fixture_tests_group_a/b/c.rs`) |
| 1.4.4 | Property-based tests | Use `proptest` for page range invariants |
| 1.4.5 | Error handling tests | Malformed PDFs, wrong passwords, empty inputs |
| 1.4.6 | PDF Validation layer | Pre-flight checks before every operation: magic bytes, encryption, page range bounds, file size, output path writability |

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
| 1.5.18 | Webhook HMAC signing | `--webhook-secret <key>` signs payload so receivers can verify authenticity |
| 1.5.19 | Webhook filter flags | `--webhook-on-success` and `--webhook-on-failure` for selective firing |
| 1.5.20 | `hash` command | `paperpilot hash <file>` — prints SHA-256 fingerprint of any PDF |
| 1.5.21 | `verify` command | `paperpilot verify <file> <expected-hash>` — exits 0 if match, 1 if tampered |
| 1.5.22 | CLI integration tests | Spawn binary, check outputs |
| 1.5.23 | E2E CLI pipeline tests | Full end-to-end testing against real PDFs using the compiled binary (No Mocking) |
| 1.5.24 | `extract-text --format json` flag | Outputs structured per-page JSON instead of flat `.txt`; distinct from `--json` mode which only wraps the operation result |

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
| 2.2.13 | `pdf_sign` tool | DocuSign / Local Signatures |
| 2.2.14 | `pdf_watermark` tool ✅ | Merged (Session 5942899190760144396) |
| 2.2.15 | `pdf_encrypt` / `pdf_decrypt` tools ✅ | Merged (Session 5942899190760144396) |
| 2.2.16 | `pdf_metadata` tool ✅ | Merged (Session 5942899190760144396) |

---

### 2.3 — MCP Server Infrastructure `[PARALLEL with 2.2]`

| # | Task | Notes |
|---|---|---|
| 2.3.1 | Input validation layer | Validate all tool inputs before passing to engine |
| 2.3.2 | Error response format | Structured, consistent error responses |
| 2.3.3 | File path / temp file handling | Safe handling of input/output file paths |
| 2.3.4 | MCP server binary | Standalone runnable server |
| 2.3.5 | MCP schema tests | Validate all tool schemas are well-formed |
| 2.3.6 | End-to-end MCP tests | Use MCP client to call tools, verify outputs |
| 2.3.7 | E2E MCP tests (Strict) | Spawn real MCP server and execute JSON-RPC calls against real PDFs (No Mocking) |

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
| 3.1.1 | Initialize Tauri 2.0 project | Under `apps/desktop/` using `cargo tauri init` |
| 3.1.2 | Configure Svelte 5 + TypeScript + Vite | |
| 3.1.3 | Set up Tauri ↔ Rust command bindings | Define initial `invoke` commands |
| 3.1.4 | Set up component library / design system | Pick: shadcn-svelte, bits-ui, or custom |
| 3.1.5 | Define Tauri IPC schema | Types for all commands and events |
| 3.1.6 | Set up frontend tests | Vitest + Testing Library |
| 3.1.7 | Configure iOS target | `cargo tauri ios init`, provision profile setup |
| 3.1.8 | Configure Android target | `cargo tauri android init`, NDK setup |

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
| 3.3.1 | Page renderer | Use Tauri to call Rust render-to-PNG, display in Svelte |
| 3.3.2 | Page thumbnail strip | Scrollable strip of all pages |
| 3.3.3 | Page selection | Click/shift-click to select pages |
| 3.3.4 | Page reorder via drag | Drag thumbnails to reorder |
| 3.3.5 | Page rotation controls | Per-page rotate buttons |
| 3.3.6 | Page deletion UI | Select + delete pages visually |

---

### 3.4 — Operations UI `[BLOCKED BY 3.2]`

| # | Task | Notes |
|---|---|---|
| 3.4.1 | Operation panel ✅ | Merged (Session 11119733351266944393) |
| 3.4.2 | Merge UI ✅ | Merged (Session 11119733351266944393) |
| 3.4.3 | Split UI | Choose split points or ranges |
| 3.4.4 | Compress UI | Quality/size tradeoff slider |
| 3.4.5 | Rotate UI | Select pages, choose angle |
| 3.4.6 | Watermark UI | Text/image, position, opacity |
| 3.4.7 | Encrypt/Decrypt UI | Password input, options |
| 3.4.8 | Extract UI | Pages, text, images — separate modes |
| 3.4.9 | Output file picker | Choose output path before running |
| 3.4.10 | **Visual Pipeline Builder (Basic)** | Drag-and-drop canvas to chain operations into a pipeline; save as a `.ppflow` recipe file |
| 3.4.11 | **Conditional branching UI** | Add IF/ELSE nodes (e.g. "if scanned → OCR, else extract text directly") to the pipeline canvas |
| 3.4.12 | **Pipeline trigger config** | Choose trigger: manual, watch folder, webhook, or schedule |

---

### 3.5 — Job System `[BLOCKED BY 3.1, PARALLEL with 3.2]`

| # | Task | Notes |
|---|---|---|
| 3.5.1 | Rust async job runner | Tokio-based, emits progress events |
| 3.5.2 | Tauri event bridge | Forward Rust progress events to Svelte |
| 3.5.3 | Progress bar component | Shows job name, %, current page |
| 3.5.4 | Job cancel support | Cancel button sends cancellation to Rust |
| 3.5.5 | Job history panel | List of completed/failed jobs with output paths |
| 3.5.6 | Responsive mobile layout | Adapt UI shell for small screens (bottom nav, touch targets) |
| 3.5.7 | E2E Desktop tests | Tauri WebDriver / Playwright automation driving the actual Rust backend (No Mocking) |
| 3.5.8 | E2E Mobile tests (Simulator) | iOS Simulator + Android Emulator automated tests via Appium or Detox (No Mocking) |

---

## Phase 4 — AI Layer

**Goal:** Users can describe what they want in natural language. The AI produces a structured plan. The user confirms. Rust executes.

**Exit condition:** A user can type "merge these files, remove blank pages, and compress the result" and get the correct output without manually configuring any operation.

**[BLOCKED BY Phase 3]**

---

### 4.1 — AI Infrastructure `[PARALLEL]`

| # | Task | Notes |
|---|---|---|
| 4.1.1 | Create `paperpilot-ai` crate | |
| 4.1.2 | Define `AiProvider` trait | Pluggable: local, OpenAI-compatible, others |
| 4.1.3 | llamafile integration | Bundle or point to local llamafile binary |
| 4.1.4 | OpenAI-compatible API client | Works with Ollama, LM Studio, OpenAI, etc. |
| 4.1.5 | AI provider config in settings | User picks provider, model, endpoint |

---

### 4.2 — Operation Planning `[BLOCKED BY 4.1]`

| # | Task | Notes |
|---|---|---|
| 4.2.1 | Define `OperationPlan` type | Ordered list of typed operations |
| 4.2.2 | System prompt design | Instruct AI to output only structured operation plans |
| 4.2.3 | Natural language → `OperationPlan` parser | AI call + parse/validate output |
| 4.2.4 | Plan validation | Validate plan against available operations before showing to user |
| 4.2.5 | Plan serialization | JSON schema for plans, used by UI and MCP |

---

### 4.3 — Natural Language UI `[BLOCKED BY 4.2, PARALLEL with 4.4]`

| # | Task | Notes |
|---|---|---|
| 4.3.1 | Command box component | Text input in main UI |
| 4.3.2 | "Thinking" state | Show spinner while AI processes |
| 4.3.3 | Plan display component | Show numbered list of interpreted operations |
| 4.3.4 | Plan edit UI | Allow user to add/remove/reorder steps before running |
| 4.3.5 | Voice input | Microphone → speech-to-text → feeds the NLP command box (integrates with Phase 2 MCP) |

---

### 4.4 — Confirmation and Execution `[BLOCKED BY 4.2, PARALLEL with 4.3]`

| # | Task | Notes |
|---|---|---|
| 4.4.1 | Confirmation dialog | Show plan with Cancel / Run buttons |
| 4.4.2 | Plan → job execution | Convert confirmed plan into sequential job run |
| 4.4.3 | Error recovery UI | If one step fails, show which step and allow retry |
| 4.4.4 | MCP-based execution path | AI plans via MCP tools, Rust executes |

---

### 4.5 — CLI AI Integration `[BLOCKED BY 4.2, PARALLEL with 4.3]`

| # | Task | Notes |
|---|---|---|
| 4.5.1 | `paperpilot ask "..."` command | Sends natural language to AI, shows plan, confirms, runs |
| 4.5.2 | Non-interactive mode | `--yes` flag to skip confirmation for scripting |

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

## Phase 5 — Advanced Intelligence

**Goal:** PaperPilot understands document content, not just structure.

**Exit condition:** Users can make scanned PDFs searchable, search semantically, extract tables, and ask questions about document content.

**[BLOCKED BY Phase 4]**

---

### 5.1 — OCR `[PARALLEL with 5.2]`

| # | Task | Notes |
|---|---|---|
| 5.1.1 | Evaluate OCR options | Tesseract (via `leptess`), `ocrs`, cloud fallback |
| 5.1.2 | Create `paperpilot-ocr` crate | |
| 5.1.3 | Page-level OCR pipeline | Render page → OCR → embed text layer |
| 5.1.4 | OCR progress reporting | Per-page progress events |
| 5.1.5 | `pdf_ocr` MCP tool | |
| 5.1.6 | `paperpilot ocr` CLI command | |
| 5.1.7 | OCR UI in desktop | Button to make scanned PDF searchable |
| 5.1.8 | Language selection | Support multi-language OCR |

---

### 5.2 — Document Understanding `[PARALLEL with 5.1]`

| # | Task | Notes |
|---|---|---|
| 5.2.1 | Semantic search | Embed document chunks, search by meaning |
| 5.2.2 | Table extraction | Detect and extract tables to CSV/JSON |
| 5.2.3 | Document comparison | Semantic diff, not just structural |
| 5.2.4 | Document summarization | AI-powered summary of document content |
| 5.2.5 | PDF Q&A (single doc) | Ask questions, get answers grounded in one document |
| 5.2.6 | Multi-document synthesis | Upload 2–50 PDFs; ask cross-document questions (e.g. "which contract has the lowest penalty clause?"); AI synthesises across all docs |
| 5.2.7 | Layout-preserving translation | Translate PDF text to any language while keeping the original visual layout intact (columns, tables, fonts) — powered by a pluggable translation API (DeepL / Google Translate / local LLM) |
| 5.2.8 | Batch workflows | Run operations across folders of documents |
| 5.2.9 | E2E Intelligence tests | Real embeddings and pipeline execution on test corpus (No Mocking) |

---

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
| 7.1.3 | Cloud MCP gateway | Hosted MCP endpoint per org |
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
├── 4.1 (AI infra)       ──────────── parallel
├── 4.2 (Planning)       ──────────── after 4.1
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

              → 4.1 AI infra
                → 4.2 Planning
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

#### 8.2a — Pipeline Engine (Core)

| # | Task | Notes |
|---|---|---|
| 8.2.1 | `.ppflow` pipeline schema | TOML/JSON format: trigger, stages, conditions, error handlers, output routing |
| 8.2.2 | CLI pipeline execution | `paperpilot run pipeline.ppflow --input file.pdf` |
| 8.2.3 | Conditional branching engine | IF/ELSE logic in pipelines (e.g., "if scanned → OCR branch, else text branch") |
| 8.2.4 | Stage retry & error handling | Per-stage retry count, quarantine folder on failure, Slack/webhook alert |
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


