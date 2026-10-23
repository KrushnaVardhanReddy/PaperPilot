# PaperPilot v0.5.0 Launch Plan — Community Tier & Release Waves

> **Focus:** The **100% Free / Community Tier** ($0 Forever, 100% Local Processing, Zero Cloud Dependency, Zero-Chrome pure Rust runtime).
> **Active Target:** `v0.5.0` Public Preview Release (`prerelease: true`).

---

## 🎯 Release Wave 1: The "Top 12 Core Launch Suite" (v0.5 Core Priority Scope)

These 12 operations are frozen as the **v0.5 Core Suite**. They undergo **unmocked real-disk Playwright execution** and full **Penta-Interface automated testing** before public launch:

| # | Tool ID | Interface Title | Category | Format Support | Primary Use Case | Release Status |
|---|---|---|---|---|---|---|
| 1 | `pdf_merge` | Merge Documents | Organize | `.pdf` | Combine multi-part documents & invoices | ✅ **v0.5 Core** |
| 2 | `pdf_split` | Split PDF | Organize | `.pdf` | Separate chapters & single-page extractions | ✅ **v0.5 Core** |
| 3 | `pdf_rotate` | Rotate Pages | Organize | `.pdf` | Fix orientation for scanned documents (90°/180°/270°) | ✅ **v0.5 Core** |
| 4 | `pdf_delete_pages` | Delete Pages | Organize | `.pdf` | Remove unwanted or blank pages | ✅ **v0.5 Core** |
| 5 | `pdf_reorder_pages` | Reorder Pages | Organize | `.pdf` | Rearrange page sequences | ✅ **v0.5 Core** |
| 6 | `pdf_compress` | Compress PDF | Optimize | `.pdf` | Reduce file size via high-ratio image recompression | ✅ **v0.5 Core** |
| 7 | `pdf_watermark` | Add Watermark | Security | `.pdf` | Semi-transparent diagonal watermarks | ✅ **v0.5 Core** |
| 8 | `pdf_ocr` | OCR Text Recognition | AI / OCR | `.pdf`, Images | Pure-Rust PaddleOCR v4 / RTen OCR overlays | ✅ **v0.5 Core** |
| 9 | `pdf_extract_text` | Extract Plain Text | Extract | `.pdf` | High-accuracy layout-aware text extraction | ✅ **v0.5 Core** |
| 10 | `extract_pages` | Extract PDF Pages | Organize | `.pdf` $\to$ `.pdf` | Pull specific page ranges into a new document | ✅ **v0.5 Core** |
| 11 | `excel_to_json` | Excel to JSON | Office Data | `.xlsx`, `.xls` $\to$ `.json` | Structured spreadsheet ingestion for AI agents | ✅ **v0.5 Core** |
| 12 | `excel_to_csv` | Excel to CSV | Office Data | `.xlsx`, `.xls` $\to$ `.csv` | High-speed tabular CSV data export | ✅ **v0.5 Core** |

---

## 📅 Progressive Release Schedule & SemVer Version Roadmap

Following the initial `v0.5.0` Public Preview release, each subsequent wave of tools will undergo dedicated unmocked E2E testing, bug hardening, and official rollout following a predictable SemVer cadence:

### 🌟 Wave 1: `v0.5.0` — Core Launch Suite (Top 12)
*Status: Production Ready / Released (`v0.5.0`)*
- Tools (12): `pdf_merge`, `pdf_split`, `pdf_rotate`, `pdf_delete_pages`, `pdf_reorder_pages`, `pdf_compress`, `pdf_watermark`, `pdf_ocr`, `pdf_extract_text`, `extract_pages`, `excel_to_json`, `excel_to_csv`.
- Target: Full Penta-Interface parity, unmocked Playwright tests, ~45–75ms latency SLA.

### 🌊 Waves 2 & 3: `v0.6.0` — Essential Media, Page Utilities & Web/AI Synthesis (Consolidated Release)
*Status: Production Ready / Released (`v0.6.0`)*
Focuses on simple high-demand utilities alongside text-to-document synthesis, Markdown, and AI ingestion pipelines:
| # | Tool ID | Interface Title | Category | Target Value | Release Milestone |
|---|---|---|---|---|---|
| 13 | `img_to_pdf` | Images to PDF | Convert | Stitch JPG/PNG/WebP photo albums into PDF without layout complexity | ✅ `v0.6.0` |
| 14 | `extract_images` | Extract Embedded Images | Extract | Dump all embedded graphics and photos directly to directory | ✅ `v0.6.0` |
| 15 | `remove_blank` | Remove Blank Pages | Organize | Scan and strip accidental blank pages from scans | ✅ `v0.6.0` |
| 16 | `page_numbers` | Add Page Numbers | Edit | Dynamic header/footer pagination with custom numbering | ✅ `v0.6.0` |
| 17 | `burst` | Burst PDF | Organize | Explode multi-page document into individual 1-page files | ✅ `v0.6.0` |
| 18 | `search` | Search Text & BBoxes | AI / Search | Spatial word search with bounding-box highlights | ✅ `v0.6.0` |
| 19 | `pdf_to_markdown` | PDF to Markdown | Convert | LLM-ready clean layout markdown for AI RAG | ✅ `v0.6.0` |
| 20 | `pdf_to_html` | PDF to HTML | Convert | Semantic, responsive HTML5 web document synthesis | ✅ `v0.6.0` |
| 21 | `html_to_pdf` | HTML to PDF | Convert | Chromium-free HTML synthesis to styled PDF (`fulgur`) | ✅ `v0.6.0` |
| 22 | `md_to_pdf` | Markdown to PDF | Convert | Fast developer documentation to PDF generator | ✅ `v0.6.0` |
| 23 | `pdf_to_json` | PDF to JSON | AI / Convert | Multimodal layout & text JSON extraction with bounding boxes & base64 images | ✅ `v0.6.0` |
| 24 | `json_to_pdf` | JSON to PDF | AI / Convert | Bidirectional synthesis of publication-ready PDFs from structured JSON | ✅ `v0.6.0` |

### 🌊 Wave 4: `v0.7.0` — Office Modernization & Spreadsheets (Upcoming)
Focuses on full Microsoft Office suite interoperability and deep vector layout reconstruction:
| # | Tool ID | Interface Title | Category | Target Value | Release Milestone |
|---|---|---|---|---|---|
| 25 | `pdf_to_docx` | PDF to Word (DOCX) | Convert | High-fidelity editable Word reconstruction with vector shape/icon rasterization | 🚀 `v0.8.0` |
| 26 | `pdf_to_xlsx` | PDF to Excel (XLSX) | Convert | Tabular extraction into structured Excel workbooks | 🚀 `v0.8.0` |
| 27 | `pdf_to_pptx` | PDF to PowerPoint (PPTX) | Convert | Slide deck presentation reconstruction | 🚀 `v0.8.0` |
| 28 | `office_to_markdown` | Office to Markdown | Convert | One-click Word, Excel, PPT conversion to Markdown | 🚀 `v0.8.0` |
| 29 | `office_extract_text` | Office Plain Text | Extract | Fast raw text dump from `.docx`, `.xlsx`, `.pptx` | 🚀 `v0.8.0` |
| 30 | `office_modernize_legacy` | Modernize Legacy Office | Convert | Upgrade legacy binary `.doc`/`.xls`/`.ppt` to modern OpenXML | 🚀 `v0.8.0` |

### 🌊 Wave 5: `v0.9.0` — Security, Signing & Cryptographic Compliance (Week 5)
Focuses on enterprise compliance and legal document workflows:
| # | Tool ID | Interface Title | Category | Target Value | Release Milestone |
|---|---|---|---|---|---|
| 31 | `sign` | Sign Document | Security | Offline cryptographic digital signatures & certs | 🚀 `v0.9.0` |
| 32 | `redact` | Redact Text & Areas | Security | Permanent PII / sensitive data blackouts | 🚀 `v0.9.0` |
| 33 | `encrypt` | Encrypt with Password | Security | AES-256 standard permission protection | 🚀 `v0.9.0` |
| 34 | `decrypt` | Remove Password | Security | Strip restrictions from unlocked documents | 🚀 `v0.9.0` |
| 35 | `pdf_to_pdf_a` | Convert to PDF/A | Compliance | ISO 19005 archival standards validation | 🚀 `v0.9.0` |
| 36 | `hash` | Generate Hash | Security | Document integrity checksum calculation (MD5, SHA-256) | 🚀 `v0.9.0` |

### 🌊 Wave 6: `v0.9.5` — Page Geometry, Cleaning & Interactive Forms (Week 6)
Focuses on document cleanup, page manipulation, and interactive forms:
| # | Tool ID | Interface Title | Category | Target Value | Release Milestone |
|---|---|---|---|---|---|
| 37 | `crop` | Crop Pages | Organize | Precision rectangular margin trimming | 🚀 `v0.9.5` |
| 38 | `fill_form` | Fill Form Data | Forms | Interactive AcroForm automated field population | 🚀 `v0.9.5` |
| 39 | `read_form` | Read Form Fields | Forms | JSON dump of all form names, values, and flags | 🚀 `v0.9.5` |
| 40 | `create_form_field` | Add Form Field | Forms | Programmatically append new interactive form inputs | 🚀 `v0.9.5` |
| 41 | `bookmarks` | Bookmarks & Outline | Navigation | Extract document outline and Table of Contents bookmarks | 🚀 `v0.9.5` |
| 42 | `annotate` | Add Annotations | Edit | Programmatic highlight, underline, and stamp annotations | 🚀 `v0.9.5` |

### 🏆 Wave 7: `v1.0.0 GA` — Visual Media, Forensics & General Availability (Week 7)
Completes the full 52-tool catalog for the official v1.0.0 General Availability release:
| # | Tool ID | Interface Title | Category | Target Value | Release Milestone |
|---|---|---|---|---|---|
| 43 | `render` | Render Page to Image | Convert | High-res PNG/JPEG page rasterization (`hayro`) | 🌟 `v1.0.0 GA` |
| 44 | `flatten` | Flatten Annotations | Security | Bake forms & marks into uneditable vector streams | 🌟 `v1.0.0 GA` |
| 45 | `metadata` | Sanitize Metadata | Privacy | Inspect & clean author, GPS, and timestamp tags | 🌟 `v1.0.0 GA` |
| 46 | `repair` | Repair Corrupted PDF | Maintenance | Rebuild broken xref tables and unreadable catalogs | 🌟 `v1.0.0 GA` |
| 47 | `compare` | Compare PDFs | Quality / Diff | Side-by-side visual and structural difference detection | 🌟 `v1.0.0 GA` |
| 48 | `classify_type` | Classify Document | AI / Taxo | Automated document taxonomy & type classification | 🌟 `v1.0.0 GA` |
| 49 | `bates` | Bates Numbering | Legal | Consecutive legal litigation numbering | 🌟 `v1.0.0 GA` |
| 50 | `header_footer` | Header & Footer | Edit | Custom margins, timestamps, and page branding | 🌟 `v1.0.0 GA` |
| 51 | `linearize` | Fast Web View | Optimize | Byte-serving linearization for instant web load | 🌟 `v1.0.0 GA` |
| 52 | `validate` | Structural Validator | Diagnostic | Diagnostic check of PDF cross-references | 🌟 `v1.0.0 GA` |

---

## 🚧 Active In-Flight Verification & Release Gates

| # | Task | Area | Spec | Target Outcome / Deliverables | Status |
|---|---|---|---|---|---|
| **5.9.70** | Wave 2 Penta-Interface E2E & Unmocked Playwright Testing Suite (6 Tools) | End-to-End Test Automation | [Spec 052](specs/052-penta-interface-expansion-and-unmocked-playwright/spec.md) | Unmocked E2E testing across all 5 interfaces (CLI, MCP, REST Gateway, WASM Browser Playwright, Desktop/Embed) for Wave 2 tools (`img_to_pdf`, `extract_images`, `remove_blank`, `page_numbers`, `burst`, `search`). | ⏳ **In Progress** (Jules Session [`4388001643990007173`](https://jules.google.com/session/4388001643990007173)) |
| **5.9.31** | Full Catalog Penta-Interface & Unmocked Desktop UI Parity Suite (Deep Semantic Validation) | Desktop & Engine E2E | [Spec 055](specs/055-all-tools-real-playwright-desktop-e2e/spec.md) | Extend real-engine desktop Playwright UI execution and Rust Penta-interface suite across all tools with deep semantic assertions. | ⏳ **In Progress** (Jules Session [`2996870268221322258`](https://jules.google.com/session/2996870268221322258)) |

---

## 🏆 Recently Completed Pre-Launch Milestones

| # | Task | Area | Spec | Highlights | Status |
|---|---|---|---|---|---|
| **5.9.30** | Context-Aware Dynamic Tool Dispatcher & Feature Pack Filtering | Desktop Ergonomics & Tool Discovery | [Spec 054](specs/054-context-aware-dynamic-tool-dispatcher/spec.md) | Intelligent tool ranking based on uploaded file types (Excel spotlight, Multi-File + Batch operations on $\ge 2$ PDFs) and 1-click Feature Pack enabling. | ✅ **Completed** (PR #193, `9c51606`) |
| **5.9.29** | Adaptive Multi-Layout Workspace & Studio Hub Unification | Desktop UX & Layout | [Spec 053](specs/053-adaptive-multi-layout-workspace/spec.md) | Upgraded Home view into a widescreen Studio Hub with system status pills, document shelf, and responsive operations cards matrix. | ✅ **Completed** (PR #192, `5f67cb2`) |
| **5.9.28** | Penta-Interface E2E Suite Expansion & Unmocked Playwright Verification | End-to-End Test Automation | [Spec 052](specs/052-penta-interface-expansion-and-unmocked-playwright/spec.md) | 100 new Penta-Interface assertions across 5 surfaces + 12/12 real unmocked Desktop Playwright tests passed with zero mocks. | ✅ **Completed** (PR #191, `2d71840`) |
| **5.9.15** | Universal `pdf_oxide` Core Engine Migration & High-Performance Extraction Suite | Core Engine & Markdown/HTML | [Spec 040](specs/040-pdf-oxide-universal-migration/spec.md) | Migrated Markdown, HTML, Plain Text, Search, and RAG chunking to `pdf_oxide` with Desktop UI integration. | ✅ **Completed** (Commits `70e8424`, `fc66840`) |
| **5.9.7** | Multimodal PDF-to-JSON with Image Extraction & Base64 Inlining | Multimodal AI & 5-Interface Parity | [Spec 032](specs/032-pdf-to-json-multimodal-images/spec.md) | `pdf_to_json` with `--image-mode <none|files|base64>`, 5-interface mandate (CLI, MCP, REST, WASM, Edge). | ✅ **Completed** (PR #166, `2604a8e`) |
