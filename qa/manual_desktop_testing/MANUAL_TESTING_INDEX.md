# PaperPilot Desktop UI Manual Testing Master Matrix (All 89 Tools)

Welcome to the **Complete PaperPilot Desktop UI Manual Testing Campaign**. This testing suite covers the complete platform catalog of **89 tools and features** (across core PDF operations, layout imposition, cryptography, AcroForms, multi-format synthesis, office modernization, AI intelligence, and document workflow automation), divided across **4 testers** (~22 tools each).

---

## 📋 Campaign Allocation Overview

| Tester | Test Guide File | Primary Domain | Tool Count | Status |
| :--- | :--- | :--- | :--- | :--- |
| **Tester 1** | [TESTER_1_PAGE_OPERATIONS.md](file:///home/krushna/Project/PaperPilot/qa/manual_desktop_testing/TESTER_1_PAGE_OPERATIONS.md) | Page Operations, Merging, Splitting & Imposition | 22 Tools | `[ ] In Progress` |
| **Tester 2** | [TESTER_2_IMPOSITION_OPTIMIZATION.md](file:///home/krushna/Project/PaperPilot/qa/manual_desktop_testing/TESTER_2_IMPOSITION_OPTIMIZATION.md) | Optimization, Rendering, Security & Cryptography | 22 Tools | `[ ] In Progress` |
| **Tester 3** | [TESTER_3_SECURITY_FORMS.md](file:///home/krushna/Project/PaperPilot/qa/manual_desktop_testing/TESTER_3_SECURITY_FORMS.md) | Forms, Batch Automation, Markup & Document Synthesis | 22 Tools | `[ ] In Progress` |
| **Tester 4** | [TESTER_4_CONVERSIONS_INTELLIGENCE.md](file:///home/krushna/Project/PaperPilot/qa/manual_desktop_testing/TESTER_4_CONVERSIONS_INTELLIGENCE.md) | Office Conversions, AI Intelligence & Workflows | 23 Tools | `[ ] In Progress` |
| **TOTAL** | **4 Testers** | **Full System Coverage** | **89 Tools** | `[ ] 0 / 89 Verified` |

---

## 🛠️ Testing Environment & Quick Start

### 1. Launch the Desktop Application
```bash
# Terminal: Run Desktop UI
cd apps/desktop
npm run tauri dev
```
*Note: Ensure memory footprint remains normal. Close heavy background tabs if necessary.*

### 2. Test Fixtures Directory
Input documents are available in the workspace:
- `tests/fixtures/simple.pdf` (Clean 1-page sample document)
- `tests/fixtures/multipage.pdf` or `tests/e2e_fixtures/real/multi_page.pdf` (5-page document)
- `tests/fixtures/sample_form.pdf` (Interactive AcroForm with text/checkbox fields)
- `tests/fixtures/sample.docx`, `test.xlsx`, `test.html`, `test.md` (Multi-format inputs)
- `tests/fixtures/test_bookmarks.pdf` (TOC Outline Bookmarks)

### 3. How to Find Tools in Desktop UI
- **Operations Dock (Right Drawer)**: Click `🛠️` on the right sidebar rail. Browse category pills: **All**, **Pages**, **Edit**, **Optimize**, **Security**, **Convert**, **AI**.
- **Command Palette (`Ctrl+K` / `Cmd+K`)**: Type any tool name or keyword to jump directly to its modal.
- **Thumbnail Rail Context Menu**: Right-click page thumbnails for page-level actions.
- **Top Application Menus**: Access `File`, `Edit`, `View`, and `Tools` menus.

---

## 📊 Complete 89-Tool Sign-Off Scorecard

### Tester 1: Page Operations, Merging & Imposition (22 Tools)
- [ ] 01. `pdf_merge` — Merge Documents (Combine Tabs)
- [ ] 02. `pdf_merge_alternate` — Alternating PDF Merge (Interleave Pages)
- [ ] 03. `pdf_split` — Split PDF by Range or Count
- [ ] 04. `pdf_split_by_outline` — Split by Outline Bookmarks / TOC
- [ ] 05. `pdf_split_scanned_book` — Split Scanned Book Spine (2-up to 1-up)
- [ ] 06. `pdf_burst` — Burst Document into Single-Page PDFs
- [ ] 07. `pdf_crop` — Crop Page Bounding Boxes
- [ ] 08. `pdf_delete_pages` — Delete Specific Pages
- [ ] 09. `pdf_extract_pages` — Extract Page Range to New PDF
- [ ] 10. `pdf_reorder_pages` — Drag-and-Drop Page Reordering
- [ ] 11. `pdf_rotate` — Rotate Pages (90°, 180°, 270°)
- [ ] 12. `pdf_auto_rotate` — Auto Upright Orientation Baseline Detection
- [ ] 13. `pdf_remove_blank` — Detect and Remove Blank Pages
- [ ] 14. `pdf_scale_pages` — Scale Page Dimensions (Letter/A4/A3/%)
- [ ] 15. `pdf_nup` — N-Up Page Imposition (2/4/9/16-up grid)
- [ ] 16. `pdf_booklet` — Saddle-Stitch Folding Booklet Imposition
- [ ] 17. `pdf_posterize` — Multi-Sheet Poster Tile Slicer with Cut Guides
- [ ] 18. `pdf_stitch_canvas` — Stitch Continuous Canvas (Vertical/Horizontal)
- [ ] 19. `pdf_bookmarks` — Extract and Inspect Document Bookmarks
- [ ] 20. `pdf_attachments` — PDF Portfolio Attachments Inspector/Extractor
- [ ] 21. `pdf_images_to_pdf` — Combine Images into PDF
- [ ] 22. `img_to_pdf` — Image to PDF Single/Multi Synthesis

### Tester 2: Optimization, Security & Cryptography (22 Tools)
- [ ] 23. `pdf_compress` — Lossy & Lossless File Size Reduction
- [ ] 24. `pdf_dedup_streams` — Lossless Hash-Based Indirect Stream Deduplication
- [ ] 25. `pdf_linearize` — Fast Web View Stream Optimization
- [ ] 26. `pdf_repair` — Rebuild Damaged XREF Tables & Corrupt Streams
- [ ] 27. `pdf_invert_colors` — High-Contrast Dark Mode & Color Inversion
- [ ] 28. `pdf_render` — High-DPI Page Rasterization to PNG/JPEG
- [ ] 29. `pdf_compare` — Visual Pixel Diff Slider & Change Highlighting
- [ ] 30. `pdf_encrypt` — Protect Document with AES-256 Password
- [ ] 31. `pdf_decrypt` — Remove Password Restrictions
- [ ] 32. `pdf_sanitize` — Deep Enterprise Sanitization (Strip JS & Private Data)
- [ ] 33. `pdf_metadata` — Inspect & Clean Author/Title/Producer Metadata
- [ ] 34. `pdf_change_permissions` — Configure Printing, Copying & Editing Bits
- [ ] 35. `pdf_timestamp` — RFC 3161 Cryptographic TSA Timestamping
- [ ] 36. `pdf_sign` — Digital Certificate (PKCS#12) & Visual Pen Signing
- [ ] 37. `pdf_validate_signatures` — Verify Signatures, Cert Chains & Integrity
- [ ] 38. `pdf_remove_signatures` — Strip Digital Signatures & Cryptographic Locks
- [ ] 39. `pdf_hash` — Cryptographic Checksums (SHA-256, SHA-512, MD5)
- [ ] 40. `pdf_validate` — PDF Conformance & Syntax Structure Validation
- [ ] 41. `document_pii_scan` — Detect SSNs, Credit Cards, Emails & Sensitive PII
- [ ] 42. `pdf_overlay_underlay` — Apply Letterheads & Stationary Behind/Over Pages
- [ ] 43. `pdf_stamp` — Stamp Coordinate-Exact Images, Logos & Badges
- [ ] 44. `pdf_flatten` — Bake Annotations & Forms Permanently into Graphics

### Tester 3: Forms, Batch Automation, Markup & Document Synthesis (22 Tools)
- [ ] 45. `pdf_read_form` — Extract AcroForm Field Values to JSON
- [ ] 46. `pdf_fill_form` — Populate AcroForms from JSON or Direct Typing
- [ ] 47. `pdf_create_form_field` — Interactive Form Field Builder (Text, Checkbox)
- [ ] 48. `batch_fill` — Multi-Record CSV Batch Form Generator
- [ ] 49. `cli_fill_batch` — Automated Batch Engine & Receipt Generation
- [ ] 50. `pdf_watermark` — Semi-Transparent Text & Diagonal Branding Watermark
- [ ] 51. `pdf_bates` — Legal Bates Numbering Sequence Stamping
- [ ] 52. `pdf_page_numbers` — Dynamic Header/Footer Page Numbering
- [ ] 53. `pdf_header_footer` — Header & Footer Custom Text Margins
- [ ] 54. `pdf_annotate` — Annotations (Highlighter, Sticky Notes, Callouts)
- [ ] 55. `pdf_redact` — Irreversible True Text Blackout & Stream Removal
- [ ] 56. `json_to_pdf` — Synthesize PDF from Structured JSON Data
- [ ] 57. `md_to_pdf` — Render Markdown Document into Styled PDF
- [ ] 58. `html_to_pdf` — Convert HTML Code/URL into Formatted PDF
- [ ] 59. `pdf_convert_html` — Convert Ingested HTML Strings to PDF
- [ ] 60. `pdf_convert_markdown` — Parse and Style Raw Markdown to PDF
- [ ] 61. `pdf_convert_excel` — Render Raw Spreadsheets to Paginated PDF
- [ ] 62. `pdf_comic_archive` — Package PDF Pages into Comic Archive (.cbz/.cbr)
- [ ] 63. `pdf_to_epub` — Convert Fixed Layout PDF into Reflowable EPUB 3.0
- [ ] 64. `pdf_to_pdf_a` — Archive-Ready PDF/A ISO 19005 Conformance
- [ ] 65. `extract_images` — Extract and Export All Embedded Raster Images
- [ ] 66. `pdf_extract_images` — Filtered Image Extractor with DPI Scaling

### Tester 4: Office Conversions, AI Intelligence & Workflows (23 Tools)
- [ ] 67. `pdf_to_docx` — Convert PDF into Microsoft Word (.docx)
- [ ] 68. `pdf_to_xlsx` — Extract Tabular Layout into Microsoft Excel (.xlsx)
- [ ] 69. `pdf_to_pptx` — Export PDF Pages into Presentation Slides (.pptx)
- [ ] 70. `pdf_to_html` — Export PDF as Responsive Styled HTML5 Web Page
- [ ] 71. `pdf_to_markdown` — Clean LLM-Ready Markdown Extraction
- [ ] 72. `pdf_to_json` — Multimodal JSON Layout with Bounding Boxes
- [ ] 73. `pdf_extract_text` — Fast Plain Text Dumper (.txt)
- [ ] 74. `pdf_extract_tables` — High-Precision Table Matrix Extractor (CSV/JSON)
- [ ] 75. `pdf_ocr` — Neural Optical Character Recognition on Scanned Pages
- [ ] 76. `office_to_markdown` — Convert Word/Excel/PowerPoint to Markdown
- [ ] 77. `office_extract_text` — Extract Raw Text from Office OpenXML Documents
- [ ] 78. `office_modernize_legacy` — Upgrade Legacy .doc/.xls/.ppt to Modern Format
- [ ] 79. `excel_to_json` — Convert Spreadsheet Sheets into JSON Objects
- [ ] 80. `excel_to_csv` — Export Spreadsheet Sheets to RFC 4180 CSV
- [ ] 81. `pdf_search` — Real-Time Text Search and Highlighting in Viewer
- [ ] 82. `workspace_search` — Recursive Multi-Format Workspace Directory Search
- [ ] 83. `semantic_search` — Vector Semantic Retrieval Across Document Chunks
- [ ] 84. `pdf_classify_type` — Document Type & Legal/Financial Category Classifier
- [ ] 85. `pdf_rag_chunks` — Semantic Chunk Generator for RAG LLM Pipelines
- [ ] 86. `document_workflow` — Declarative Recipe Workflow Runner (.ppflow)
- [ ] 87. `recipe_run` — Instant Pipeline Execution of Multi-Step Tools
- [ ] 88. `cli_ask` — Natural Language AI Assistant for Document Tasks
- [ ] 89. `watcher_start` / `watcher_stop` — Hot-Folder Watcher Daemon Automation
