# PaperPilot v1.0 Launch Plan — Community Tier & Release Waves

> **Focus:** The **100% Free / Community Tier** ($0 Forever, 100% Local Processing, Zero Cloud Dependency, Zero-Chrome pure Rust runtime).

---

## 🎯 Release Wave 1: The "Top 12 Core Launch Suite" (v1.0 Priority Scope)

These 12 operations are frozen as the **v1.0 Core Suite**. They undergo **unmocked real-disk Playwright execution** and full **Penta-Interface automated testing** before public launch:

| # | Tool ID | Interface Title | Category | Format Support | Primary Use Case |
|---|---|---|---|---|---|
| 1 | `pdf_merge` | Merge Documents | Organize | `.pdf` | Combine multi-part documents & invoices |
| 2 | `pdf_split` | Split PDF | Organize | `.pdf` | Separate chapters & single-page extractions |
| 3 | `pdf_rotate` | Rotate Pages | Organize | `.pdf` | Fix orientation for scanned documents (90°/180°/270°) |
| 4 | `pdf_delete_pages` | Delete Pages | Organize | `.pdf` | Remove unwanted or blank pages |
| 5 | `pdf_reorder_pages` | Reorder Pages | Organize | `.pdf` | Rearrange page sequences |
| 6 | `pdf_compress` | Compress PDF | Optimize | `.pdf` | Reduce file size via high-ratio image recompression |
| 7 | `pdf_watermark` | Add Watermark | Security | `.pdf` | Semi-transparent diagonal watermarks |
| 8 | `pdf_ocr` | OCR Text Recognition | AI / OCR | `.pdf`, Images | Pure-Rust PaddleOCR v4 / RTen OCR overlays |
| 9 | `pdf_extract_text` | Extract Plain Text | Extract | `.pdf` | High-accuracy layout-aware text extraction |
| 10 | `pdf_to_docx` | PDF to Word (DOCX) | Convert | `.pdf` $\to$ `.docx` | High-fidelity editable Word reconstruction |
| 11 | `excel_to_json` | Excel to JSON | Office Data | `.xlsx`, `.xls` $\to$ `.json` | Structured spreadsheet ingestion for AI agents |
| 12 | `excel_to_csv` | Excel to CSV | Office Data | `.xlsx`, `.xls` $\to$ `.csv` | High-speed tabular CSV data export |

---

## 📅 Post-Launch Progressive Release Schedule (Weekly / Bi-Weekly Waves)

Following v1.0 release, each subsequent batch of 4–5 tools will undergo dedicated unmocked E2E testing, bug hardening, and official rollout every 1–2 weeks:

### 🌊 Wave 2: Web & AI Synthesis Suite (Week 2)
Focuses on advanced modern conversions, AI ingestion, and spatial search:
| # | Tool ID | Interface Title | Category | Target Value |
|---|---|---|---|---|
| 13 | `pdf_to_html` | PDF to HTML | Convert | Semantic, responsive HTML5 web document synthesis |
| 14 | `pdf_to_markdown` | PDF to Markdown | Convert | LLM-ready clean layout markdown for AI RAG |
| 15 | `html_to_pdf` | HTML to PDF | Convert | Chromium-free HTML synthesis to styled PDF (`fulgur`) |
| 16 | `md_to_pdf` | Markdown to PDF | Convert | Fast developer documentation to PDF generator |
| 17 | `pdf_to_json` | PDF to JSON | AI / Convert | Multimodal layout & text JSON extraction with bounding boxes & base64 images |
| 18 | `json_to_pdf` | JSON to PDF | AI / Convert | Bidirectional synthesis of publication-ready PDFs from structured JSON |
| 19 | `search` | Search Text & BBoxes | AI / Search | Spatial word search with bounding-box highlights |

### 🌊 Wave 3: Office Modernization & Spreadsheets (Week 3)
Focuses on full Microsoft Office suite interoperability:
| # | Tool ID | Interface Title | Category | Target Value |
|---|---|---|---|---|
| 20 | `pdf_to_xlsx` | PDF to Excel (XLSX) | Convert | Tabular extraction into structured Excel workbooks |
| 21 | `pdf_to_pptx` | PDF to PowerPoint (PPTX) | Convert | Slide deck presentation reconstruction |
| 22 | `office_to_markdown` | Office to Markdown | Convert | One-click Word, Excel, PPT conversion to Markdown |
| 23 | `office_extract_text` | Office Plain Text | Extract | Fast raw text dump from `.docx`, `.xlsx`, `.pptx` |
| 24 | `office_modernize_legacy` | Modernize Legacy Office | Convert | Upgrade legacy binary `.doc`/`.xls`/`.ppt` to modern OpenXML |

### 🌊 Wave 4: Security, Signing & Compliance (Week 4)
Focuses on enterprise compliance and legal document workflows:
| # | Tool ID | Interface Title | Category | Target Value |
|---|---|---|---|---|
| 25 | `sign` | Sign Document | Security | Offline cryptographic digital signatures & certs |
| 26 | `redact` | Redact Text & Areas | Security | Permanent PII / sensitive data blackouts |
| 27 | `encrypt` | Encrypt with Password | Security | AES-256 standard permission protection |
| 28 | `decrypt` | Remove Password | Security | Strip restrictions from unlocked documents |
| 29 | `pdf_to_pdf_a` | Convert to PDF/A | Compliance | ISO 19005 archival standards validation |
| 30 | `hash` | Generate Hash | Security | Document integrity checksum calculation (MD5, SHA-256) |

### 🌊 Wave 5: Page Geometry, Cleaning & Forms (Week 5)
Focuses on document cleanup, page manipulation, and interactive forms:
| # | Tool ID | Interface Title | Category | Target Value |
|---|---|---|---|---|
| 31 | `extract_pages` | Extract Pages | Organize | Pull specific pages into a new document |
| 32 | `remove_blank` | Remove Blank Pages | Organize | Scan and strip accidental blank pages |
| 33 | `page_numbers` | Add Page Numbers | Edit | Dynamic header/footer pagination |
| 34 | `crop` | Crop Pages | Organize | Precision rectangular margin trimming |
| 35 | `burst` | Burst PDF | Organize | Explode document into individual 1-page files |
| 36 | `fill_form` | Fill Form Data | Forms | Interactive AcroForm automated field population |

### 🌊 Wave 6: Advanced Layout, Forms, Outlines & Archival (Week 6)
Focuses on metadata, outlines, annotations, and forensic repair:
| # | Tool ID | Interface Title | Category | Target Value |
|---|---|---|---|---|
| 37 | `read_form` | Read Form Fields | Forms | JSON dump of all form names, values, and flags |
| 38 | `create_form_field` | Add Form Field | Forms | Programmatically append new interactive form inputs |
| 39 | `bookmarks` | Bookmarks & Outline | Navigation | Extract document outline and Table of Contents bookmarks |
| 40 | `annotate` | Add Annotations | Edit | Programmatic highlight, underline, and stamp annotations |
| 41 | `flatten` | Flatten Annotations | Security | Bake forms & marks into uneditable vector streams |
| 42 | `metadata` | Sanitize Metadata | Privacy | Inspect & clean author, GPS, and timestamp tags |
| 43 | `repair` | Repair Corrupted PDF | Maintenance | Rebuild broken xref tables and unreadable catalogs |

### 🌊 Wave 7: Visual Media, Stamping, Comparison & Diagnostic Utilities (Week 7)
Completes the full 50+ tool catalog:
| # | Tool ID | Interface Title | Category | Target Value |
|---|---|---|---|---|
| 44 | `img_to_pdf` | Images to PDF | Convert | Stitch JPG/PNG/WebP photo albums into PDF |
| 45 | `extract_images` | Extract Embedded Images | Extract | Dump all embedded graphics to directory |
| 46 | `render` | Render Page to Image | Convert | High-res PNG/JPEG page rasterization (`hayro`) |
| 47 | `compare` | Compare PDFs | Quality / Diff | Side-by-side visual and structural difference detection |
| 48 | `classify_type` | Classify Document | AI / Taxo | Automated document taxonomy & type classification |
| 49 | `bates` | Bates Numbering | Legal | Consecutive legal litigation numbering |
| 50 | `header_footer` | Header & Footer | Edit | Custom margins, timestamps, and page branding |
| 51 | `linearize` | Fast Web View | Optimize | Byte-serving linearization for instant web load |
| 52 | `validate` | Structural Validator | Diagnostic | Diagnostic check of PDF cross-references |

---

## 🚧 Active In-Flight Verification & Release Gates

| # | Task | Area | Spec | Target Outcome / Deliverables | Status |
|---|---|---|---|---|---|
| **5.9.31** | Full Catalog Penta-Interface & Unmocked Desktop UI Parity Suite (Deep Semantic Validation) | Desktop & Engine E2E | [Spec 055](specs/055-all-tools-real-playwright-desktop-e2e/spec.md) | Extend real-engine desktop Playwright UI execution and Rust Penta-interface suite across all tools with deep semantic assertions. | ⏳ **In Progress** (Jules Session [`12242990113169294559`](https://jules.google.com/session/12242990113169294559)) |

---

## 🏆 Recently Completed Pre-Launch Milestones

| # | Task | Area | Spec | Highlights | Status |
|---|---|---|---|---|---|
| **5.9.30** | Context-Aware Dynamic Tool Dispatcher & Feature Pack Filtering | Desktop Ergonomics & Tool Discovery | [Spec 054](specs/054-context-aware-dynamic-tool-dispatcher/spec.md) | Intelligent tool ranking based on uploaded file types (Excel spotlight, Multi-File + Batch operations on $\ge 2$ PDFs) and 1-click Feature Pack enabling. | ✅ **Completed** (PR #193, `9c51606`) |
| **5.9.29** | Adaptive Multi-Layout Workspace & Studio Hub Unification | Desktop UX & Layout | [Spec 053](specs/053-adaptive-multi-layout-workspace/spec.md) | Upgraded Home view into a widescreen Studio Hub with system status pills, document shelf, and responsive operations cards matrix. | ✅ **Completed** (PR #192, `5f67cb2`) |
| **5.9.28** | Penta-Interface E2E Suite Expansion & Unmocked Playwright Verification | End-to-End Test Automation | [Spec 052](specs/052-penta-interface-expansion-and-unmocked-playwright/spec.md) | 100 new Penta-Interface assertions across 5 surfaces + 12/12 real unmocked Desktop Playwright tests passed with zero mocks. | ✅ **Completed** (PR #191, `2d71840`) |
| **5.9.15** | Universal `pdf_oxide` Core Engine Migration & High-Performance Extraction Suite | Core Engine & Markdown/HTML | [Spec 040](specs/040-pdf-oxide-universal-migration/spec.md) | Migrated Markdown, HTML, Plain Text, Search, and RAG chunking to `pdf_oxide` with Desktop UI integration. | ✅ **Completed** (Commits `70e8424`, `fc66840`) |
| **5.9.7** | Multimodal PDF-to-JSON with Image Extraction & Base64 Inlining | Multimodal AI & 5-Interface Parity | [Spec 032](specs/032-pdf-to-json-multimodal-images/spec.md) | `pdf_to_json` with `--image-mode <none|files|base64>`, 5-interface mandate (CLI, MCP, REST, WASM, Edge). | ✅ **Completed** (PR #166, `2604a8e`) |
