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
| 5.2.1 | Local Embedded Vector Engine | Microsecond in-memory vector index in Rust via `ort`/`rten` using nano embeddings — 100% offline RAG & semantic search across both PDFs and Office (`.docx`, `.xlsx`, `.pptx` via `office_oxide`) |
| 5.2.2 | Layout-Aware Markdown & Table Parser | MinerU/Marker-style structured text extraction preserving multi-columns, LaTeX math, and Markdown tables |
| 5.2.3 | Semantic Search & Highlighting | Search document by intent/meaning rather than exact keywords, jumping directly to target sentences |
| 5.2.4 | Table Extraction to CSV/JSON/Parquet | Detect and extract tables across PDF pages and Office spreadsheets (`.xlsx`/`.xls`) directly to structured data |
| 5.2.5 | Automatic PII Redaction (SSN, Cards, Names) | Local regex + NER entity classifier to automatically detect and structurally redact sensitive PII across PDF and Office documents |
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
| 5.3.1 | Booklet Imposition (`pdf_booklet`) `[Spec 096]` ✅ | **Completed** — Pure-Rust booklet imposition algorithm (duplex sheet ordering, automatic page padding $N \pmod 4 = 0$, center fold line), verified across all 5 surfaces (CLI `paperpilot pdf-booklet`, MCP `pdf_booklet`, REST Gateway `POST /api/v1/pdf/booklet`, WASM, Cloudflare Edge) with 10/10 unmocked E2E assertions passing (`reports/PENTA_INTERFACE_E2E_PDF_BOOKLET_REPORT.md`, `wiki/PDF-Booklet.md`). |
| 5.3.2 | Multi-Page Layout (N-Up) (`pdf_nup`) `[Spec 097]` ✅ | **Merged to main (PR #241)** — Pure-Rust N-Up grid imposition engine (2, 4, 9, 16 pages per sheet), horizontal/vertical ordering, configurable margin & spacing, optional hairline grid borders, and Desktop UI controls (`reports/PENTA_INTERFACE_E2E_PDF_NUP_REPORT.md`, `wiki/097-pdf-nup.md`). |
| 5.3.3 | Scale / Resize Page Dimensions (`pdf_scale_pages`) `[Spec 098]` ✅ | **Completed (PR #243)** — Pure-Rust page scaling engine supporting standard formats (A4, Letter, Legal, A3) and percentage factors with aspect ratio preservation and centering, verified across all 5 surfaces (`reports/PENTA_INTERFACE_E2E_PDF_SCALE_PAGES_REPORT.md`, `apps/docs/src/content/docs/reference/tools/pdf_scale_pages.md`). |
| 5.3.4 | Image / Logo Watermark Stamp (`pdf_stamp`) ✅ | **Completed** — Coordinate-exact PNG/JPEG placement, XObject `/Image` insertion, `/ExtGState` opacity control, FlateDecode compression, and page stream transformation across all 5 surfaces (`reports/PENTA_INTERFACE_E2E_PDF_STAMP_REPORT.md`, `apps/docs/src/content/docs/reference/tools/pdf_stamp.md`, `wiki/Pdf-Stamp.md`). |
| 5.3.5 | Validate Digital Signatures (`pdf_validate_signatures`) `[Spec 095 & Spec 101]` ✅ | **Completed (PR #250)** — Cryptographically verify PKCS#7 / CMS digital signatures, certificate chains, document integrity hashes, and pre-sign baseline tamper forensics (`--baseline <original.pdf>`) using 3-tier SHA-256 + token diffing, verified across all 5 surfaces (`reports/PENTA_INTERFACE_E2E_PDF_VALIDATE_SIGNATURES_REPORT.md`, `wiki/101-Baseline-Integrity.md`). |
| 5.3.6 | PDF Sanitization (`sanitize`) `[Spec 085]` ✅ | **Completed** — Deep security sanitizer stripping embedded JavaScript, launch actions, external URI hooks, and dangerous metadata. |
| 5.3.7 | Remove Annotations (Batch) ✅ | **Completed** — Programmatically strip or flatten annotations across pages (`pdf_flatten`). |
| 5.3.8 | Overlay / Underlay PDFs (`pdf_overlay_underlay`) ✅ | **Completed (PR #244)** — Pure-Rust stationery and letterhead imposition engine (template form XObject import, underlay/overlay stream ordering, page filtering, opacity control) verified across all 5 surfaces (`reports/PENTA_INTERFACE_E2E_PDF_OVERLAY_UNDERLAY_REPORT.md`, `apps/docs/src/content/docs/reference/tools/pdf_overlay_underlay.md`, `wiki/101-Pdf-Overlay-Underlay.md`). |
| 5.3.9 | Visual Pixel PDF Comparison Slider (`pdf_compare`) ✅ | **Completed** — Side-by-side split slider, overlay diffing, and GitHub-style line addition/deletion diff (`🌿 Git Diff`). |
| 5.3.10 | Auto-Rotate (`pdf_auto_rotate`) `[Spec 102]` ✅ | **Completed (PR #245)** — Pure-Rust automated page orientation and uprighting engine (aspect-ratio & vector text baseline orientation analysis, 90°/180°/270° uprighting, coordinate transformation) verified across all 5 surfaces (`reports/PENTA_INTERFACE_E2E_PDF_AUTO_ROTATE_REPORT.md`, `apps/docs/src/content/docs/reference/tools/pdf_auto_rotate.md`, `wiki/102-Pdf-Auto-Rotate.md`). |
| 5.3.11 | PDF Continuous Canvas Stitch (`pdf_stitch_canvas`) `[Spec 107]` ✅ | Completed (Merged PR #257, commit `c64356c`) — Pure-Rust vertical and horizontal continuous single-sheet canvas stitch, margin & spacing calculation, Form XObject encapsulation, and Penta-Interface options (`wiki/107-Pdf-Stitch-Canvas.md`, `apps/docs/src/content/docs/reference/tools/pdf_stitch_canvas.md`). |
| 5.3.12 | Manage Attachments & PDF Portfolios (`pdf_attachments`) `[Spec 108]` ✅ | **Completed & Merged (PR #254, commit `c3ad50c`, Session 17693334025127438627)** — ISO 32000-1 `/EmbeddedFiles` Name Tree inspection, binary stream extraction, and FlateDecode attachment injection with 5/5 unit tests and full CLI, MCP (76 tools), and REST Gateway integration (`specs/108-pdf-attachments/spec.md`, `wiki/108-Pdf-Attachments.md`). |
| 5.3.13 | Remove Digital Signatures (`pdf_remove_signatures`) `[Spec 099]` ✅ | **Completed (PR #242)** — Pure-Rust digital signature removal engine stripping `/AcroForm` signature fields, `/ByteRange` cryptographic locks, and widget annotations (`reports/PENTA_INTERFACE_E2E_PDF_REMOVE_SIGNATURES_REPORT.md`, `wiki/100-Pdf-Remove-Signatures.md`). |
| 5.3.14 | Timestamp PDF RFC 3161 TSA (`pdf_timestamp`) 🏗️ | **Scaffolded across all 5 surfaces** — Unified registration and REST/CLI endpoints for TSA timestamping. |
| 5.3.15 | Modify PDF Permissions (`pdf_change_permissions`) `[Spec 111 / Task 111A]` ✅ | **Completed & Merged (PR #262)** — Modify ISO 32000-1 `/Encrypt` `/P` integer bitmask flags (allow/disallow printing, text copying, form filling, annotations) without re-encrypting (`specs/111-pdf-change-permissions/spec.md`). |
| 5.3.16 | Split Scanned Book Spine (`pdf_split_scanned_book`) `[Spec 112 / Task 112A]` ✅ | **Completed & Merged (PR #261)** — Pure-Rust two-up center gutter scan splitting into single portrait pages with configurable reading direction (LTR/RTL) and spine overlap (`specs/112-pdf-split-scanned-book/spec.md`). |
| 5.3.17 | Comic Archive Conversion (`pdf_comic_archive`) 🏗️ | **Scaffolded across all 5 surfaces** — Unified registration for PDF $\leftrightarrow$ CBZ/CBR archive bidirectional synthesis. |
| 5.3.18 | Invert PDF Colors & Dark Mode (`pdf_invert_colors`) `[Spec 109 / Task 109A]` ✅ | Completed (Merged PR #258, commit `0f8e89c`) — Pure-Rust vector stream color inversion (dark mode, sepia, high-contrast) preserving selectable text layers and embedded typography (`specs/109-pdf-invert-colors/spec.md`, `wiki/109-Pdf-Invert-Colors.md`). |
| 5.3.19 | Steganographic PDF Fingerprinting (`pdf_fingerprint`) 💡 | **Planned** — Embed imperceptible zero-width unicode tokens & micro-metadata watermarks to trace source of confidential document leaks. |
| 5.3.20 | Exportable Redline Comparison Report (`pdf_compare_redline`) 💡 | **Planned** — Generate downloadable visual redline audit PDF (`.pdf` / `.json`) with strikethroughs, additions, and change metrics. |
| 5.3.21 | Cross-Format Comparison (`doc_compare_cross`) 💡 | **Planned** — Pure-Rust `PDF ⇄ DOCX` direct document diffing without manual conversions (Draftable Desktop parity). |
| 5.3.22 | Word Document to PDF Synthesis (`docx_to_pdf` / `office_to_pdf`) 💡 | **Planned** — Pure-Rust OpenXML `.docx` document parsing and layout engine rendering styled paragraphs, tables, and images directly into a paged PDF document. |
| 5.3.23 | EPUB eBook to PDF Conversion (`epub_to_pdf`) 💡 | **Planned** — Pure-Rust `.epub` container extraction, XHTML/CSS reflow parsing, and chapter-based pagination into printable PDF. |
| 5.3.24 | Interleave / Alternate PDF Collation (`pdf_merge_alternate`) `[Spec 117]` ✅ | **Completed & Merged into `feature/dev` (PR #268)** — Interleave pages from two documents alternating between odd and even pages (e.g. Doc A p1, Doc B p1, Doc A p2, Doc B p2) with reverse even option for double-sided scanner batch intake (`specs/117-pdf-merge-alternate/spec.md`). |
| 5.3.25 | Split PDF by Outline / Bookmarks (`pdf_split_by_outline`) `[Spec 118]` ✅ | **Completed & Merged (PR #264)** — Automatically traverse `/Outlines` hierarchy tree and split multi-chapter documents at named bookmark destinations into individual chapter PDFs (`specs/118-pdf-split-by-outline/spec.md`). |
| 5.3.26 | Posterize / Multi-Sheet Tile Print (`pdf_posterize`) `[Spec 119]` ✅ | **Completed & Merged (Task 119A, PR #270)** — Tile single large pages (A0, A1, A2, architectural blueprints) across standard grid of A4/Letter sheets with printable overlap crop marks (`specs/119-pdf-posterize/spec.md`). |

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
| 5.4.10 | Detect & Split Scanned Photos | Computer-vision flatbed segmentation detecting multiple physical photos placed on a single scanner bed and saving each as an individual image |

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
| 5.7.4 | Cloudflare Workers Edge Microservice | Deploy `paperpilot-wasm` to Cloudflare Workers for sub-10ms, memory-only edge processing (0ms cold start, zero disk, $0.005/run); gated by Bearer secret token / Cloudflare Access service token to prevent unauthenticated public execution |
| 5.7.5 | Ephemeral Kubernetes Sandbox Worker | On-demand K8s Job / KEDA runner (`emptyDir: { medium: "Memory" }`) for heavy OCR, multi-gigabyte documents, and complex pipelines that auto-terminates and wipes RAM upon job completion |
| 5.7.6 | Smart Hybrid Dispatch Router | Gateway router that routes light operations (merge, split, rotate, stamp) to Cloudflare Workers and routes heavy compute (OCR, large rendering) to Ephemeral K8s Pods |
| 5.7.7 | Zero-Knowledge Audit & Ephemeral Proof | Cryptographic verification asserting zero files touch persistent storage and in-memory streams are purged upon delivery |
| 5.7.8 | Zero-Dependency CI/CD PDF Test Assertions Library (`@paperpilot/wasm` / `@paperpilot/test-utils`) ✅ | Merged (PR #174, Session 2383638745133243635) — Sub-10ms in-memory PDF testing runner for external projects' CI pipelines (Jest, Vitest, Playwright, Bun, Node.js). Allows developers to assert generated invoices/reports (layout, text, forms, hashes) without Docker, Python, or system C-libraries (`reports/WASM_TEST_UTILS_REPORT.md`, `wiki/26-Wasm-Test-Utils.md`). |

---

### 5.8 — AI Document Parsing & LLM/RAG Extraction Engine (Zero-Python, Pure-Rust) `[PARALLEL with 5.7]`

> Directly captures the high-growth AI Engineering & RAG pipeline market (displacing bloated Python stacks like Docling, Marker, PyMuPDF4LLM, and Camelot) with sub-10ms, pure-Rust, on-device parsing.

| # | Task | Notes |
|---|---|---|
| 5.8.1 | RAG-Ready Markdown Extractor (`pdf_to_markdown` / `pdf_extract_structure`) | Extract document visual hierarchy into clean Markdown (`# H1`, `## H2`, bullet lists, code blocks, bold/italics) preserving reading order via pure-Rust font heuristics and spatial clustering (< 10ms/page) |
| 5.8.2 | Vector Table Extractor to CSV/JSON (`pdf_extract_tables`) | **Completed** — Pure-Rust table grid and cell line detector traversing `lopdf` path drawing operators to extract tabular data directly to JSON/CSV |
| 5.8.3 | Layout-Aware Semantic Chunker (`pdf_chunk_rag`) | Chunk documents by natural semantic boundaries (headers, paragraphs, callout boxes) rather than arbitrary character splits, outputting token-counted JSON chunks ready for vector DB insertion (Pinecone, Qdrant, Chroma) |
| 5.8.4 | Key-Value & Form Entity Extraction (`pdf_extract_kv`) | Heuristic and layout-based key-value pair extractor for invoices, receipts, tax forms, and W-2/1099 documents without cloud LLM dependencies |
| 5.8.5 | Local Embedding & Vector Export (`pdf_embed`) | Offline document vectorization via pure-Rust embedding crates (`candle` / `fastembed-rs`) generating 384/768-dim float arrays locally on CPU/WASM |
| 5.8.6 | Tri-Interface & Python SDK Bindings (`paperpilot-py` via PyO3) | Expose fast Rust document parsers across CLI, MCP, REST Gateway, and a lightweight zero-dependency Python wheel (`pip install paperpilot`) to easily capture Python AI engineers |

