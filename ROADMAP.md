# PaperPilot Master Roadmap & Comprehensive Task Tracker

> **Architecture Mandate:** 100% Pure-Rust engine, Zero-Python/Zero-C++ runtime, Penta-Interface Parity (Core Engine ➔ CLI ➔ MCP Server ➔ REST Gateway ➔ WASM/Edge).

---

## 🎯 Global Architecture & Wave Dependency Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ WAVE 1: FOUNDATIONS & GENERATORS                                            │
│ • 5.9.79: Pure-Rust Tool Scaffolding Generator (`penta-scaffold`)           │
│ • 5.9.78: Universal Remote PDF URL Ingestion                                │
│ • 5.9.58: Pure-Rust Spatial Table Extraction (Stage 2 - Merged ✅)          │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ WAVE 2: CORE DOCUMENT OPERATIONS SPRINT (Parity with Acrobat Pro & Stirling)│
│ • 5.9.61: Digital Signature Verification Engine (`pdf_validate_signatures`) │
│ • 5.9.62: Accessibility & PDF/UA Compliance (`pdf_validate_accessibility`)  │
│ • 5.9.63: Semantic Legal Diffing (`pdf_compare_semantic`)                   │
│ • 5.9.66: Multi-Page Image Exporter (`pdf_to_images`)                       │
│ • 5.3.1:  Booklet Imposition (`pdf_booklet`)                                │
│ • 5.3.2:  Multi-Page N-Up Layout (`pdf_nup`)                                │
│ • 5.3.3:  Page Dimension Scaling (`pdf_scale_pages`)                        │
│ • 5.3.4:  Logo Watermark Stamping at Exact Coordinates (`pdf_stamp`)        │
│ • 5.3.8:  Overlay / Underlay Letterhead (`pdf_overlay_underlay`)            │
│ • 5.3.11: PDF to Single Continuous Canvas (`pdf_stitch_canvas`)              │
│ • 5.3.12: Embedded Portfolios & Attachments (`pdf_attachments`)             │
│ • 5.3.13: Strip Certificate Signatures (`pdf_remove_signatures`)            │
│ • 5.3.14: RFC 3161 TSA Cryptographic Timestamps (`pdf_timestamp`)           │
│ • 5.4.5:  PDF User/Owner Permission Flags (`pdf_change_permissions`)        │
│ • 5.4.7:  Scanned Dual-Page Book Center Spine Split                         │
│ • 5.4.8:  Comic Book Archive Conversion (`pdf_to_cbz` / `cbz_to_pdf`)       │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ WAVE 3: AI DOCUMENT PARSING & ZERO-PYTHON RAG ENGINE (Phase 5.8 & 5.2)       │
│ • 5.8.1 / 5.2.2: Layout-Aware Structured Markdown Extractor                 │
│ • 5.8.3 / 5.2.3: Layout-Aware Semantic Chunker (`pdf_chunk_rag`)            │
│ • 5.8.4: Heuristic Key-Value & Form Entity Extractor (`pdf_extract_kv`)     │
│ • 5.8.5 / 5.2.1: Local Embedded Vector Indexing (`candle`/`rten` nano)      │
│ • 5.8.6: Zero-Dependency Python Wheel (`paperpilot-py` via PyO3)            │
│ • 5.2.6: Single-Document Contextual Grounded Q&A                            │
│ • 5.2.7: Multi-Document Synthesis & Comparison                              │
│ • 5.2.8: Layout-Preserving Document Translation Block Replacer              │
│ • 5.2.9: Memory-Mapped Streaming (`memmap2`) for 1000+ Page Documents      │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ WAVE 4: OCR & SCAN INTELLIGENCE (Phase 5.1 & 5.4)                            │
│ • 5.1.3: Page-Level OCR Pipeline (Render ➔ Detect ➔ Embed Text Layer)       │
│ • 5.1.4: Per-Page OCR Progress Telemetry Events                             │
│ • 5.1.8: Multi-Language OCR Model Selection                                 │
│ • 5.4.1: Content-Based Automated Document Renaming (Invoices/Dates)          │
│ • 5.4.2: Emulated Scanner Skew & Grain Filter (Photocopy Effect)            │
│ • 5.4.6: High-Contrast Scan Cleanup & Thresholding                          │
│ • 5.4.10: Flatbed Scanner Multi-Photo Segmentation                          │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ WAVE 5: CLOUDFLARE EDGE & HYBRID CLOUD ORCHESTRATION (Phase 5.7 & 5.5)       │
│ • 5.7.4: Cloudflare Workers Edge Microservice                               │
│ • 5.5.3: Remote Streamable MCP Server over SSE (`/mcp/sse`)                 │
│ • 5.7.3: In-Browser Web Worker Threading (`wasm-bindgen-rayon`)             │
│ • 5.7.5: Ephemeral Kubernetes Sandbox Worker for Massive Batch Jobs         │
│ • 5.7.6: Smart Hybrid Dispatch Router (Edge WASM ↔ K8s)                     │
│ • 5.7.7: Zero-Knowledge Audit & In-Memory Cryptographic Purge Proof         │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ WAVE 6: RESILIENT WORKFLOWS & HEADLESS AUTOMATION (Phase 5.5 & 5.9)          │
│ • 5.9.64: Declarative Multi-Step Document Workflow Runner                   │
│ • 5.9.65: Resilient Batch Directory Processor with Retry & Error Reports    │
│ • 5.5.1:  CLI Headless Pipeline Runner (`paperpilot pipeline run`)          │
│ • 5.5.4:  Chained Pipeline MCP Tool (`pdf_run_pipeline`)                    │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ WAVE 7: ENTERPRISE B2B GOVERNANCE & SECURITY (Phase 6)                       │
│ • 5.9.76 / 6.3.1–6.3.4: SIEM Compliance & Tamper-Evident Audit Logging      │
│ • 5.9.77 / 6.5.2: Air-Gapped Ed25519 Cryptographic License Key Validator    │
│ • 6.1.1–6.1.3: Multi-Tenant Org Boundaries & Enterprise RBAC                │
│ • 6.1.4: SSO — SAML 2.0 (Okta, Azure AD, PingIdentity)                      │
│ • 6.1.5: SSO — OIDC (Google Workspace, Microsoft Entra)                     │
│ • 6.1.6: Automated SCIM User Provisioning & Deprovisioning                  │
│ • 6.2.1–6.2.5: Enterprise AI Policies, Retention, and Central Config       │
│ • 6.4.1–6.4.5: Production Docker, K8s Manifests, Zero-Downtime Migrations   │
│ • 6.6.1: Pure Offline Argon2id Memory-Hard Anti-Brute-Force Encryption      │
│ • 6.6.2–6.6.5: Decoy Honeypots, Progressive Lockout, and SIEM Alert Webhooks│
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ WAVE 8: ADOBE ACROBAT PRO INLINE TYPOGRAPHY EDITOR (Phase 5.6)               │
│ • 5.6.1: Text Block & Paragraph Boundary Detection                          │
│ • 5.6.2: In-Place Text Typo Editing & Content Stream Rewriting              │
│ • 5.6.3: Font Metric Matching & Kerning Synthesis                           │
│ • 5.6.4: Object-Level Image & Shape Direct Transformation                    │
│ • 5.6.5: Print Preflight & PDF/X Compliance Verifier                        │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ WAVE 9: ADVANCED DOCUMENT SDK PARITY (Apryse / PDFTron Gap - Phase 6.7)     │
│ • 6.7.1: Interactive AcroForms & XFA Dynamic Form Engine                     │
│ • 6.7.2: Collaborative SVG Freehand Annotation, Stylus Ink, & XFDF Sync     │
│ • 6.7.3: Certified Cryptographic Signatures (PAdES / PKCS#7 / LTV)          │
│ • 6.7.4: Guaranteed Zero-Leak Vector Glyph Redaction Engine                 │
│ • 6.7.6: 2D CAD & BIM Blueprint Vector Viewer (`.dxf` / `.dwg` via Skia)    │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ WAVE 10: UNIVERSAL DOCUMENT OPERATING SYSTEM (Phase 10)                      │
│ • 10.1: Universal Document Abstract Syntax Tree (`DocAST`)                  │
│ • 10.2: Any-to-Any Format Conversion Matrix (PDF ⇄ DOCX ⇄ XLSX ⇄ HTML ⇄ MD) │
│ • 10.3: Native Non-PDF Hygiene (`docx_strip_tracking`, `xlsx_deduplicate`)  │
│ • 10.4: Multi-Format Penta-Interface Parity                                  │
│ • 10.5: Universal AI Document Gateway                                       │
│ • 10.6: Enterprise Universal Document Studio UI                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 📋 Comprehensive Section-by-Section Task Index

### 1. In-Flight Tasks (Active Jules Sessions)
| Task # | Feature Name | Spec / Crate | Jules Session ID / Status |
|---|---|---|---|
| **5.9.58** | Pure-Rust Spatial Table Extraction (Stage 2) | Spec 090 / `table.rs` | ✅ Merged to `main` (commit `5d1f0ce`) |
| **5.9.78** | Universal Remote PDF URL Ingestion | Spec 093 / `url_fetch.rs` | ⏳ In-Flight ([`6136209794630860269`](https://jules.google.com/session/6136209794630860269)) |
| **5.9.79** | Pure-Rust Tool Scaffolding Generator (`penta-scaffold`) | Spec 094 / `tools/penta-scaffold` | ⏳ In-Flight ([`8915713543148419332`](https://jules.google.com/session/8915713543148419332)) |

---

### 2. High-Priority Operations Sprint (Wave 2)
*Blocked only by `5.9.79` (`penta-scaffold` generates all boilerplate across all 5 surfaces).*

| Task # | Feature Name | Core Functionality | Impact / Market Need |
|---|---|---|---|
| **5.9.61** | `pdf_validate_signatures` | PKCS#7 / CMS cryptographic signature & cert chain check | Legal, Banking, Government |
| **5.9.62** | `pdf_validate_accessibility` | PDF/UA, Section 508, `/Lang`, and `/Alt` tags validation | Higher Ed & Enterprise Procurement |
| **5.9.63** | `pdf_compare_semantic` | Legal clause & sentence meaning diffing | Contract Review & M&A Due Diligence |
| **5.9.66** | `pdf_to_images` | Batch page rendering to PNG/JPEG/WebP directories | Archival & Web Previews |
| **5.3.1** | `pdf_booklet` | 2-sided booklet signature imposition | Commercial & Office Printing |
| **5.3.2** | `pdf_nup` | Multi-page layout (2, 4, 9, 16 pages per sheet) | Paper Reduction & Handouts |
| **5.3.3** | `pdf_scale_pages` | Dimensional scaling (A4 $\leftrightarrow$ Letter) with aspect preservation | Multi-region Normalization |
| **5.3.4** | `pdf_stamp` | Coordinate-exact logo/image placement | Corporate Invoicing & Branding |
| **5.3.8** | `pdf_overlay_underlay` | Digital letterhead & background stamping | Letterhead & Stationary |
| **5.3.10** | `pdf_auto_rotate` | Heuristic font baseline + OCR automatic page uprighting | Scanned Document Intake |
| **5.3.11** | `pdf_stitch_canvas` | Continuous single-canvas stitch for receipts/blueprints | Architecture & Invoicing |
| **5.3.12** | `pdf_attachments` | Inspect, extract, or embed PDF Portfolio attachments | Legal & Case Files |
| **5.3.13** | `pdf_remove_signatures`| Strip `/ByteRange` signature dictionaries for editing | Document Re-authoring |
| **5.3.14** | `pdf_timestamp` | RFC 3161 TSA cryptographic timestamp stamping | Non-repudiation Auditing |
| **5.4.5** | `pdf_change_permissions`| Modify user/owner permission flags (print, copy, edit) | Security Configuration |
| **5.4.7** | `pdf_split_scanned_book`| Detect center spine and split dual-page scans to single pages | Book Archival & Libraries |
| **5.4.8** | `pdf_comic_archive` | Convert PDF $\leftrightarrow$ CBR/CBZ preserving sequence | Digital Media & Publishing |

---

### 3. AI Document Parsing & LLM/RAG Extraction Engine (Phase 5.8 & 5.2)
*Captures the AI Engineering and RAG ingestion pipeline market with zero Python dependencies.*

| Task # | Feature Name | Core Functionality | Impact / Market Need |
|---|---|---|---|
| **5.8.1 / 5.2.2**| Structured Markdown Extractor | Preserves reading order, `# H1`, `## H2`, tables, and math (< 10ms/page) | Replaces Marker & Docling |
| **5.8.3 / 5.2.3**| Layout-Aware Semantic Chunker | Chunk by natural semantic boundaries with token counts | Pinecone, Qdrant, Chroma RAG |
| **5.8.4** | Key-Value & Form Extractor | Heuristic invoice, receipt, and tax form data extraction | Financial & ERP Automation |
| **5.8.5 / 5.2.1**| Local Embedded Vector Engine | Microsecond in-memory vector index via `candle`/`rten` nano | 100% Offline Semantic Search |
| **5.8.6** | Python SDK (`paperpilot-py`) | Zero-dependency pure-Rust PyO3 Python wheel | Capture Python AI Engineers |
| **5.2.6** | Single-Document Grounded Q&A | In-memory Q&A with exact page/coordinate citations | Document Assistants |
| **5.2.7** | Multi-Document Synthesis | Compare terms across 2–50 documents simultaneously | Enterprise Legal Due Diligence |
| **5.2.8** | Layout-Preserving Translation | In-place text block translation preserving typography | Global Enterprise Publishing |
| **5.2.9** | Memory-Mapped Streaming (`memmap2`)| Instant opening and multi-core Rayon processing of 1,000+ pages | High-Volume Government Docs |

---

### 4. OCR & Scan Intelligence (Phase 5.1 & 5.4)
*Complete scanned paper document pipeline.*

| Task # | Feature Name | Core Functionality | Impact / Market Need |
|---|---|---|---|
| **5.1.3** | Page-Level OCR Pipeline | Render page $\to$ OCR text recognition $\to$ embed invisible text layer | Searchable Scans |
| **5.1.4** | OCR Progress Telemetry | Per-page progress events over SSE / Tauri channels | UI Loading Bars |
| **5.1.8** | Multi-Language Selection | Support for European, CJK, and Arabic model packs | International Users |
| **5.4.1** | Content Auto-Rename | Rename files based on extracted invoice numbers/dates | Accounting File Automation |
| **5.4.2** | Fake Scanner Effect | Subtle skew, grain, and contrast filter for digital PDFs | Bureaucracy Emulation |
| **5.4.6** | Clean Scanned Documents | Contrast enhancement, thresholding, and deskewing | Pre-OCR Optimization |
| **5.4.10** | Scanned Photo Segmentation | Flatbed CV segmentation to separate multiple photos into files | Archival Photo Digitization |

---

### 5. Cloudflare Edge & Hybrid Cloud Orchestration (Phase 5.7 & 5.5)
*Low-cost serverless execution at 300+ edge locations.*

| Task # | Feature Name | Core Functionality | Impact / Market Need |
|---|---|---|---|
| **5.7.4** | Cloudflare Workers Edge Microservice | Memory-only edge processing via `paperpilot-wasm` ($0.005/run) with Bearer token / Cloudflare Access auth guard | Infinite Serverless Scale |
| **5.5.3** | Streamable Remote MCP over SSE | Expose MCP tools over `/mcp/sse` for remote AI agents | Zero-Setup Agent Tools |
| **5.7.3** | In-Browser Web Worker Threading | Multi-threaded client WASM via `wasm-bindgen-rayon` | Fast Browser Operations |
| **5.7.5** | Ephemeral Kubernetes Sandbox Worker | On-demand K8s Pods with RAM-only disks for massive 1GB+ files | Heavy Enterprise Workloads |
| **5.7.6** | Smart Hybrid Dispatch Router | Auto-routes light tasks to Edge and heavy tasks to K8s | Cost & Performance Optimizer |
| **5.7.7** | Zero-Knowledge Cryptographic Proof | Verified assurance that zero bytes touch disk and RAM is purged | HIPAA & Defense Compliance |

---

### 6. Resilient Workflows & Headless Automation (Phase 5.5 & 5.9)
*Chain multi-step pipelines without disk overhead.*

| Task # | Feature Name | Core Functionality | Impact / Market Need |
|---|---|---|---|
| **5.9.64** | Multi-Step Declarative Workflow Runner | Chain tools in-memory (`deskew ➔ sanitize ➔ compress ➔ watermark`) | Zero Disk IO Overhead |
| **5.9.65** | Resilient Batch Directory Processor | Multi-worker directory engine with retries & `failure_report.json` | High-Volume Ingestion |
| **5.5.1** | CLI Headless Pipeline Runner | `paperpilot pipeline run recipe.json` | CI/CD & Cron Automation |
| **5.5.4** | Chained Pipeline MCP Tool | Trigger multi-step recipes from AI agents in a single RPC call | Autonomous AI Workflows |

---

### 7. Enterprise B2B Governance, Security & Deployment (Phase 6)
*Mandatory prerequisites for selling enterprise licenses and defense deployments.*

| Task # | Feature Name | Core Functionality | Impact / Market Need |
|---|---|---|---|
| **5.9.76 / 6.3.1–4**| SIEM Compliance & Audit Logging | Immutable JSON audit logs streaming to Splunk, Datadog, Syslog | SOC2, ISO27001, HIPAA |
| **5.9.77 / 6.5.2**| Air-Gapped Ed25519 License Validator | Offline cryptographically signed `license.key` validation | SCIF / Defense / Air-Gap |
| **6.1.1–6.1.3** | Multi-Tenant Org & RBAC | Data isolation boundaries and role-based permission checks | Enterprise Multi-Tenancy |
| **6.1.4** | SSO — SAML 2.0 | Okta, Azure AD, PingIdentity enterprise SAML login | Corporate IT Standard |
| **6.1.5** | SSO — OIDC | Google Workspace, Microsoft Entra OIDC login | Modern Cloud Identity |
| **6.1.6** | SCIM Automated User Provisioning | Real-time employee lifecycle sync from Okta / IdP | IT Automation |
| **6.2.1–6.2.5** | AI & Retention Policies | Local-only processing policies and automated deletion schedules | Corporate Compliance |
| **6.4.1–6.4.5** | Production Docker & K8s Manifests | Official Helm charts, zero-downtime database migrations | DevOps & Self-Hosting |
| **6.6.1** | Offline Argon2id Memory-Burn Encryption| 1–2 GB RAM + 2s compute per guess to defeat GPU brute-forcing | Military-Grade Document Security |
| **6.6.2–6.6.5** | Decoy Honeypots & Lockouts | Coercion decoy passcodes and progressive delay penalties | Physical Anti-Tamper Security |

---

### 8. Inline Typography & In-Place Content Editor (Phase 5.6)
*Acrobat Pro inline document authoring parity.*

| Task # | Feature Name | Core Functionality | Impact / Market Need |
|---|---|---|---|
| **5.6.1** | Text Block & Paragraph Detection | Segment raw `BT...ET` operator streams into paragraphs | Inline Click-to-Edit |
| **5.6.2** | In-Place Text Editing & Stream Rewrite | Click existing text, edit typos, and rewrite content streams | Typo Fixes Without Re-export |
| **5.6.3** | Font Metric Matching & Synthesis | Analyze embedded font descriptors to match metrics and kerning | Seamless Font Replacement |
| **5.6.4** | Object-Level Image & Shape Transform | Move, resize, replace, or delete existing images on canvas | Visual Layout Corrections |
| **5.6.5** | Print Preflight & PDF/X Verification | Verify CMYK profiles, embedded fonts, image DPI (>300 DPI) | Commercial Print Prep |

---

### 9. Advanced Document SDK Parity (Phase 6.7 - Apryse / PDFTron Gap)
*Closes the competitive moat against heavy enterprise document SDKs.*

| Task # | Feature Name | Core Functionality | Impact / Market Need |
|---|---|---|---|
| **6.7.1** | Interactive AcroForms & XFA Engine | In-browser dynamic form rendering, calculation, and serialization | Government & Insurance Forms |
| **6.7.2** | Collaborative SVG & Stylus Annotations | Stylus pressure ink, sticky notes, and XFDF XML synchronization | Real-Time Contract Review |
| **6.7.3** | Certified Signatures (PAdES / LTV) | X.509 Adobe-compliant ByteRange digital signing with LTV | Legal Non-Repudiation |
| **6.7.4** | Vector Glyph Redaction Engine | Guaranteed complete removal of intersected glyphs from streams | Zero-Leak National Security |
| **6.7.6** | 2D CAD & BIM Blueprint Vector Viewer | Pure-Rust `.dxf` / `.dwg` layer rendering via `tiny-skia` | Architecture & Engineering |

---

### 10. Universal Document Operating System (Phase 10)
*Expands PaperPilot beyond PDF to a cross-format document manipulation platform.*

| Task # | Feature Name | Core Functionality | Impact / Market Need |
|---|---|---|---|
| **10.1** | Universal Document AST (`DocAST`) | Unified DOM for spans, layout, tables, vectors, and media | Single Cross-Format Kernel |
| **10.2** | Any-to-Any Format Conversion Matrix | Pure-Rust `DOCX ⇄ MD ⇄ HTML ⇄ PDF ⇄ EPUB ⇄ XLSX ⇄ CSV` | Universal Document Converter |
| **10.3** | Native Non-PDF Hygiene | `docx_strip_tracking`, `xlsx_deduplicate`, `markdown_lint` | Office File Sanitization |
| **10.4** | Multi-Format Penta-Interface Parity | Expose all document formats over CLI, MCP, REST, and Edge | Universal API & AI Gateway |
| **10.5** | Universal AI Document Gateway | Multimodal asset extraction & normalization for LLM agents | Default AI Document Engine |
| **10.6** | Enterprise Universal Studio UI | Side-by-side preview, edit, annotate, split, and merge any format | Next-Gen Office Workstation |
