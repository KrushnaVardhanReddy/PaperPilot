# PaperPilot v1.0 Launch Plan — Community Tier & Core Launch Scope

> **Focus:** The **100% Free / Community Tier** ($0 Forever, 100% Local Processing, Zero Cloud Dependency, Zero-Chrome pure Rust runtime).

---

## 🎯 Frozen "Top 12 Core Launch Suite" (v1.0 Priority Scope)

To ensure zero launch defects and maximum end-to-end reliability, the following 12 operations are frozen as the **v1.0 Core Suite**. These tools undergo **unmocked real-disk Playwright execution** and full **Penta-Interface coverage**:

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

## 🚧 Active In-Flight Verification & Release Gates

| # | Task | Area | Spec | Target Outcome / Deliverables | Status |
|---|---|---|---|---|---|
| **5.9.28** | Penta-Interface E2E Suite Expansion (5 Tools) & Unmocked Playwright Desktop Verification | Comprehensive End-to-End Testing | [Spec 052](specs/052-penta-interface-expansion-and-unmocked-playwright/spec.md) | 100 new Penta assertions across CLI, MCP, REST, WASM, Edge for `excel_to_json`, `excel_to_csv`, `pdf_rag_chunks`, `pdf_to_html`, `workspace_search` + unmocked Desktop Playwright tests for Top 12. | ⏳ **In Progress** (Jules Session [`9530387047429935978`](https://jules.google.com/session/9530387047429935978)) |

---

## 🏆 Recently Completed Pre-Launch Milestones

| # | Task | Area | Spec | Highlights | Status |
|---|---|---|---|---|---|
| **5.9.15** | Universal `pdf_oxide` Core Engine Migration & High-Performance Extraction Suite | Core Engine & Markdown/HTML | [Spec 040](specs/040-pdf-oxide-universal-migration/spec.md) | Migrated Markdown, HTML, Plain Text, Search, and RAG chunking to `pdf_oxide` with Desktop UI integration. | ✅ **Completed** (Commits `70e8424`, `fc66840`) |
| **5.9.7** | Multimodal PDF-to-JSON with Image Extraction & Base64 Inlining | Multimodal AI & 5-Interface Parity | [Spec 032](specs/032-pdf-to-json-multimodal-images/spec.md) | `pdf_to_json` with `--image-mode <none|files|base64>`, 5-interface mandate (CLI, MCP, REST, WASM, Edge). | ✅ **Completed** (PR #166, `2604a8e`) |

---

## 📋 Post-Launch v1.1+ Roadmap (Deferred)

The following items are deliberately deferred to post-v1.0 release cycles to maintain laser focus on quality:
- Bi-weekly release rhythm: Test, fix, and promote 2–3 additional tools from the 44-tool catalog every two weeks.
- Mobile compilation target (iOS / iPadOS via Tauri 2.0 mobile bindings).
- Templating CLI generator (`paperpilot create --template report`).
