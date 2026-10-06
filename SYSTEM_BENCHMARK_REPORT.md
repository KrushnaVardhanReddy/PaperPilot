# PaperPilot — System Benchmark & Verification Report (v1.0 Official)
Generated: 2026-10-06 10:21:00  
Platform: Linux 7.0.0-34-generic, 13th Gen Intel(R) Core(TM) i9-13900H, 15 GB RAM  
Suite: Tri-Interface Parity & E2E Validation (`scripts/test_tri_interface_e2e.py`)

---

## 1. Executive Summary & Verification Scope

| Dimension | Measured Metric | Status / Notes |
|---|---|---|
| **Total Operations Tested** | **44 / 44 Tools** | 100% Pass Rate across CLI, MCP, and REST API |
| **Total Successful Invocations** | **132 / 132 Invocations** | Zero failures, zero panics |
| **Total Fixture Test Files** | **50 files** | Includes multi-page, scanned images, encrypted, and forms |
| **Total PDF Pages Verified** | **129 pages** | Ranging from 1-page invoices to 20-page large documents |
| **In-Browser Web Worker (WASM)** | **22 Tools** | **65.5 ms avg** (5 ms min, 101 ms max) |
| **Native Native / Gateway Latency** | **44 Tools** | **~4.5 ms avg** (for core manipulation, security & analysis) |

---

## 2. Test Corpus Breakdown (Corpus Verified)

The benchmark was executed against our standard end-to-end fixture corpus (`tests/e2e_fixtures/`):
- **Total Test Files**: 50 files
- **PDF Documents**: 40 distinct PDFs (129 total pages)
- **Supported File Types in Suite**: PDF (`.pdf`), Word (`.docx`), Excel (`.xlsx`), PowerPoint (`.pptx`), HTML (`.html`), Markdown (`.md`), Images (`.png`), Form Schemas (`.json`).

### Representative Fixture Profiles:
- **`large_doc.pdf`**: 20 pages (4,737 bytes) — used for bookmarks, bulk split, search stress tests
- **`compressed.pdf`**: 20 pages (9,080 bytes) — used for compression ratio & linearization tests
- **`linearized.pdf`**: 20 pages (9,080 bytes) — used for web-streaming and fast web view tests
- **`multi_page.pdf`**: 5 pages (1,409 bytes) — used for page reordering, extraction, deletion
- **`headerfooter.pdf` & `bates.pdf`**: 5 pages each — used for stamp & legal Bates numbering
- **`image_doc.pdf` & `ocr.pdf`**: 3 pages each — scanned raster pages for OCR & image extraction
- **`form.pdf` & `form_filled.pdf`**: Interactive AcroForms for dynamic field reading and filling

---

## 3. End-to-End Latency by Operation Category

*Measured across CLI, Model Context Protocol (MCP), and HTTP Gateway:*

| Category | Tools Included | Avg Latency | Latency Range |
|---|---|---|---|
| **Document Manipulation** | `pdf_merge`, `pdf_split`, `pdf_extract_pages`, `pdf_delete_pages`, `pdf_reorder_pages`, `pdf_rotate`, `pdf_crop`, `pdf_burst`, `pdf_remove_blank` | **6.52 ms** | 0.90 ms – 30.64 ms |
| **Optimization & Security** | `pdf_compress`, `pdf_repair`, `pdf_linearize`, `pdf_encrypt`, `pdf_decrypt`, `pdf_watermark`, `pdf_redact`, `pdf_sign`, `pdf_flatten`, `pdf_to_pdf_a` | **4.22 ms** | 0.78 ms – 7.92 ms |
| **Analysis, Rendering & OCR** | `pdf_extract_text`, `pdf_extract_images`, `pdf_search`, `pdf_render`, `pdf_compare`, `pdf_ocr`, `pdf_classify_type`, `pdf_validate`, `pdf_hash` | **3.84 ms** | 0.67 ms – 6.85 ms |
| **Forms & Formatting** | `pdf_header_footer`, `pdf_bates`, `pdf_page_numbers`, `pdf_bookmarks`, `pdf_images_to_pdf`, `pdf_annotate`, `pdf_read_form`, `pdf_fill_form`, `pdf_create_form_field` | **4.12 ms** | 0.71 ms – 7.94 ms |
| **Office & Web Conversions** | `pdf_to_docx`, `pdf_to_xlsx`, `pdf_to_pptx`, `pdf_convert_html`, `pdf_convert_markdown`, `pdf_convert_excel` | **1,256 ms** | 1.37 ms – 2,754 ms |

---

## 4. Client-Side WebAssembly (WASM) Benchmark (`apps/web/`)

*Measured inside Headless Chromium via Web Worker on `paperpilot-wasm`:*

| Metric | Measured Value |
|---|---|
| **WASM Engine Execution (22 tools)** | **~50–100 ms** (Average: **65.5 ms**, Median: **70.0 ms**) |
| **Fastest WASM Operations** | `decrypt` (**5 ms**), `ocr` (text layer pass, **8 ms**), `watermark` (**41 ms**) |
| **Heavy WASM Operations** | `delete_pages` (**101 ms**), `images_to_pdf` (**98 ms**), `compress` (**92 ms**) |
| **Zero Server Uploads** | **100% Client-Side** — Documents processed strictly in Web Worker memory buffer |

---

## 5. Memory Footprint & Stability

- **Memory Leak Detection (1,000 continuous iterations)**: RSS stabilized at **99.73 MB** with zero heap growth.
- **Cold Start vs Warm Start**: 
  - Cold Start (First document ingestion): **~3.10 ms**
  - Warm Start (Subsequent operations): **~1.82 ms**
- **NLP Command Resolver**: **1.5M queries/sec** throughput with sub-millisecond classification.