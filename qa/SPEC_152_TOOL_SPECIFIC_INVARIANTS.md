# Specification 152: Tool-Specific Invariant & Deep Domain Validation Catalog (All 90 Tools)

**Parent:** Spec 151 (Deep Semantic Validation Architecture)  
**Status:** In Progress / Authoritative Reference  
**Core Principle:** No generic "file size > 0" or "header starts with %PDF-". Every test must assert the **exact mathematical, structural, content, or cryptographic invariant unique to that specific tool**.

---

## 1. Suite 1: Page Operations, Imposition, Optimization & Rendering (Tools 01–30)

| Tool ID | Canonical Title | Tool-Specific Domain Invariant & Exact Assertion |
|:---|:---|:---|
| `pdf_merge` | Merge Documents | **Page Addition Invariant:** $\text{Pages}(\text{Out}) = \text{Pages}(A) + \text{Pages}(B) = 1 + 1 = 2$. Text on Page 1 must match Doc A, text on Page 2 must match Doc B. |
| `pdf_merge_alternate` | Alternating Merge | **Interleaving Invariant:** Out Page 1 has Doc A content; Out Page 2 has Doc B content; Out Page 3 has Doc A content; Out Page 4 has Doc B content. |
| `pdf_split` | Split Document | **Range Exactness:** Splitting 5-page PDF at page 1 produces a document with $\text{Pages}(\text{Out}) = 1$ containing Page 1 text. |
| `pdf_split_by_outline` | Split by Outline | **Bookmark Hierarchy Slicing:** Extracts separate PDFs per `/Outlines` top-level bookmark title. |
| `pdf_split_scanned_book` | Split Scanned Book | **2-Up Center Spine Slicer:** Splitting a 1-page 2-up scan produces 2 portrait pages, doubling page count ($\text{Pages}(\text{Out}) = 2 \times \text{Pages}(\text{In})$). |
| `pdf_burst` | Burst PDF | **Total Partition:** 5-page PDF explodes into 5 distinct files: `page_1.pdf` through `page_5.pdf`, each with $\text{Pages} = 1$. |
| `pdf_crop` | Crop Pages | **Bounding Box Invariant:** `/MediaBox` or `/CropBox` in PDF dictionary strictly changes to `[0, 0, 100, 100]`. |
| `pdf_delete_pages` | Delete Pages | **Page Subtraction Invariant:** Deleting page 1 from 5-page PDF yields $\text{Pages}(\text{Out}) = 4$. Original Page 1 text must be absent from Page 1 of output. |
| `pdf_extract_pages` | Extract Pages | **Subset Exactness:** Extracting page 1 from 5-page PDF yields $\text{Pages}(\text{Out}) = 1$ matching original Page 1 text. |
| `pdf_reorder_pages` | Reorder Pages | **Permutation Invariant:** Reverse reordering [2, 1] swaps page contents: Out Page 1 text equals In Page 2 text. |
| `pdf_rotate` | Rotate Pages | **Rotation Invariant:** Page dictionary contains `/Rotate 90`. Visual dimensions swap width and height. |
| `pdf_auto_rotate` | Auto Upright | **Orientation Normalization:** Baseline detector corrects upside-down / sideways text to `/Rotate 0`. |
| `pdf_remove_blank` | Remove Blank Pages | **Content Density Threshold:** Drops pages with text length $= 0$ and vector/raster count $= 0$; keeps content pages. |
| `pdf_scale_pages` | Scale Page Dimensions | **MediaBox Dimensional Scaling:** MediaBox points scale by factor (e.g. A4 to US-Letter or 50% scale factor). |
| `pdf_nup` | N-Up Imposition | **Grid Compression:** 4 input pages combined into 1 sheet in a $2 \times 2$ grid: $\text{Pages}(\text{Out}) = \lceil \text{Pages}(\text{In}) / 4 \rceil$. |
| `pdf_booklet` | Booklet Imposition | **Signature Folding Reorder:** Pages reordered into saddle-stitch folding pairs ($[4, 1, 2, 3]$ for 4 pages). |
| `pdf_posterize` | Poster Tile Slicer | **Tile Grid Multiplication:** 1 oversized page sliced across printable grid (e.g. $2 \times 2 = 4$ sheet tiles). |
| `pdf_stitch_canvas` | Stitch Continuous Canvas | **Continuous Single Page:** Multiple pages combined vertically into 1 single page with $\text{Height}(\text{Out}) = \sum \text{Height}(\text{Pages})$. |
| `pdf_bookmarks` | Extract Bookmarks | **Outline JSON Schema:** Returns valid JSON array with `title`, `page`, and nested `children` bookmark nodes. |
| `pdf_attachments` | Portfolio Attachments | **Embedded Files Stream:** `/Names /EmbeddedFiles` dictionary contains attached file bytes with correct MIME type. |
| `pdf_images_to_pdf` | Images to PDF | **Raster Containerization:** Each input PNG/JPEG becomes exactly 1 page in output PDF with `/XObject /Image`. |
| `img_to_pdf` | Image to PDF (Alias) | **Single Image Page:** 1 input PNG produces 1 output PDF page containing that image asset. |
| `pdf_compress` | Compress PDF | **Byte Reduction Invariant:** $\text{Size}(\text{Out}) \le \text{Size}(\text{In})$; text and images remain readable. |
| `pdf_dedup_streams` | Stream Deduplication | **Indirect Object SHA-256 Deduplication:** Duplicate font/image object references point to a single shared object ID. |
| `pdf_linearize` | Fast Web View (Linearize) | **First-Page Optimization:** File starts with `/Linearized 1.0` dictionary and primary hint table stream within first 1024 bytes. |
| `pdf_repair` | Repair Corrupted PDF | **Xref Table Rebuild:** Reconstructs damaged `/XRef` cross-reference table and restores document catalog `/Pages`. |
| `pdf_invert_colors` | Invert Colors / Dark Mode | **Color Inversion Matrix:** Background colors invert from light to dark ($RGB \to 1.0 - RGB$) in page content stream. |
| `pdf_render` | Render Page to Image | **Raster Exactness:** Outputs genuine PNG image starting with `89 50 4E 47`, matching document aspect ratio. |
| `pdf_compare` | Compare PDFs | **Visual & Textual Diff:** JSON diff report identifies added/deleted strings or generates diff PDF with red/green highlights. |
| `pdf_encrypt` | Protect Document | **Cryptographic Lock:** Raw byte stream contains `/Encrypt` dictionary; document fails to open without password. |

---

## 2. Suite 2: Security, Cryptography, AcroForms, Markup & Synthesis (Tools 31–60)

| Tool ID | Canonical Title | Tool-Specific Domain Invariant & Exact Assertion |
|:---|:---|:---|
| `pdf_decrypt` | Remove Password | **Unlocking Invariant:** Output PDF lacks `/Encrypt` dictionary and opens without any password prompt. |
| `pdf_sanitize` | Sanitize Document | **Threat Neutralization:** Completely strips `/JavaScript`, `/JS`, `/Launch`, `/EmbeddedFiles`, and private `/PieceInfo`. |
| `pdf_metadata` | Edit Metadata | **Information Dictionary:** `/Title`, `/Author`, `/Subject`, `/Keywords` in `/Info` dictionary match input strings. |
| `pdf_change_permissions` | Modify PDF Permissions | **Permission Bitmask:** The `/P` integer flag in `/Encrypt` enforces copy/print/edit restrictions. |
| `pdf_timestamp` | RFC 3161 TSA Timestamp | **Cryptographic TSA Token:** Raw document stream contains `%%RFC3161_TSA_TIMESTAMP_VERIFIED` trailer or timestamp token. |
| `pdf_sign` | Sign Document | **Digital Signature:** Document contains `/Sig` dictionary with `/ByteRange`, `/SubFilter /adbe.pkcs7.detached`, and valid DER certificate. |
| `pdf_validate_signatures` | Validate Signatures | **Cryptographic Integrity:** Validates digest against `/ByteRange` and outputs JSON with `valid: true, cert_subject: "..."`. |
| `pdf_remove_signatures` | Remove Signatures | **Signature Stripping:** Removes all `/Sig` and `/ByteRange` structures, leaving clean unsigned PDF. |
| `pdf_hash` | Generate Hash | **Digest Accuracy:** Computes SHA-256 string ($64\text{ hex characters}$); changing 1 byte in PDF changes hash completely. |
| `pdf_validate` | Validate PDF | **ISO 32000 Conformance:** Validates header, object offsets, and xref table; returns JSON `valid: true`. |
| `document_pii_scan` | PII Scanner | **Regex Pattern Finding:** Finds SSNs, credit cards, emails; outputs JSON report with category breakdown and coordinates. |
| `pdf_overlay_underlay` | Overlay / Underlay | **Stationery Composition:** Merges template Form XObject `/TplOverlay` behind or over existing page content. |
| `pdf_stamp` | Stamp Image & Logo | **Coordinate Exactness:** Places `/XObject /Image /ImStamp` at exact $(X, Y, W, H)$ coordinates with `/ExtGState` opacity. |
| `pdf_flatten` | Flatten Annotations | **Widget Baking:** Strips `/AcroForm` and `/Annots` `/Widget`; rasterizes form appearances directly into page `/Contents`. |
| `pdf_read_form` | Read Form Fields | **AcroForm Key-Value Extraction:** Returns JSON map of `{ field_name: field_value }` extracted from `/AcroForm /Fields`. |
| `pdf_fill_form` | Fill Form Data | **Field Population:** Fills text into AcroForm fields; output displays values and updates appearance `/AP` stream. |
| `pdf_create_form_field` | Add Form Field | **Interactive Field Injection:** Appends new `/Widget` annotation with specified field name and type (Text/Checkbox). |
| `batch_fill` | Batch Form Fill | **Multi-Record Generation:** Merges CSV records with PDF form; creates $N$ individual filled PDFs matching row count. |
| `cli_fill_batch` | Automated Batch Engine | **Receipt & Output Execution:** Generates execution receipt `execution_receipt.json` with SHA-256 hashes for all records. |
| `pdf_watermark` | Add Watermark | **Rotated Text Invariant:** Page content stream contains semi-transparent text string rotated at diagonal angle (45°). |
| `pdf_bates` | Bates Numbering | **Sequential Legal Numbering:** Header/footer text stream contains incrementing sequential numbers (e.g. `CONF-000100`). |
| `pdf_page_numbers` | Add Page Numbers | **Dynamic Page Indexing:** Bottom margin contains `1 of 5`, `2 of 5` strings corresponding to each page index. |
| `pdf_header_footer` | Header & Footer | **Margin Positioning:** Fixed header text rendered in top margin; fixed footer text rendered in bottom margin. |
| `pdf_annotate` | Add Annotations | **Annotation Structure:** Appends `/Highlight`, `/Text`, or `/Square` annotation objects to `/Annots` array. |
| `pdf_redact` | Redact Text | **True Content Erasure:** Target word/phrase is **permanently absent** from decoded text stream, covered by solid fill box. |
| `json_to_pdf` | JSON to PDF | **Synthesis from JSON:** Parses structured JSON layout and renders styled headings, body text, and tables to PDF. |
| `md_to_pdf` | Markdown to PDF | **Markdown Compilation:** Renders `# Heading`, `**bold**`, lists, and code blocks into styled PDF typographic elements. |
| `html_to_pdf` | HTML to PDF | **HTML Rendering:** Parses HTML5/CSS DOM and compiles into paginated PDF preserving typography and layout. |
| `pdf_convert_html` | HTML to PDF Direct | **Direct HTML Conversion:** Ingests raw `.html` file and produces authentic `%PDF-` document. |
| `pdf_convert_markdown` | Markdown to PDF Direct | **Direct Markdown Conversion:** Ingests raw `.md` file and produces authentic `%PDF-` document. |

---

## 3. Suite 3: Conversions, Office Modernization, AI & Workflows (Tools 61–90)

| Tool ID | Canonical Title | Tool-Specific Domain Invariant & Exact Assertion |
|:---|:---|:---|
| `pdf_convert_excel` | CSV to PDF | **Tabular Rendering:** Ingests CSV rows and formats them into bordered table grid cells in output PDF. |
| `pdf_comic_archive` | Comic Archive (.cbz) | **ZIP Image Container:** Packs rendered PDF pages as sequential images (`001.png`, `002.png`) inside a valid `.cbz` PKZIP file. |
| `pdf_to_epub` | PDF to EPUB 3.0 | **Reflowable eBook Container:** Valid `.epub` PKZIP container containing `mimetype`, `META-INF/container.xml`, and chapter XHTMLs. |
| `pdf_to_pdf_a` | Convert to PDF/A | **ISO 19005 Archival Tags:** Output dictionary contains `/OutputIntents` with color profile and `/GTS_PDFA1` metadata. |
| `extract_images` | Extract Images | **Raster Asset Extraction:** Unpacks all embedded raster `/XObject /Image` streams into individual PNG files. |
| `pdf_extract_images` | Extract Images (Filtered) | **DPI/Size Filtered Extraction:** Extracts embedded images matching minimum resolution/dimension criteria. |
| `pdf_to_docx` | PDF to Word | **Word OpenXML Container:** Valid `.docx` PKZIP container containing `word/document.xml` with paragraphs matching source text. |
| `pdf_to_xlsx` | PDF to Excel | **Spreadsheet Matrix:** Valid `.xlsx` PKZIP container with worksheet cells (`sheet1.xml`) matching detected PDF tables. |
| `pdf_to_pptx` | PDF to PowerPoint | **Presentation Slides:** Valid `.pptx` PKZIP container with 1 slide per PDF page (`ppt/slides/slide1.xml`). |
| `pdf_to_html` | PDF to HTML | **Responsive Web DOM:** Outputs `.html` containing clean `<div>`, `<span>`, or `<table>` tags reproducing PDF layout. |
| `pdf_to_markdown` | PDF to Markdown | **LLM-Ready Markdown:** Outputs clean Markdown with `#` headings, bullet points, and table syntax matching document. |
| `pdf_to_json` | PDF to JSON | **Bounding Box JSON Schema:** Outputs JSON array of elements with `text`, `page`, `x`, `y`, `width`, `height`, and `font`. |
| `pdf_extract_text` | PDF to Plain Text | **Raw Text Stream:** Plain `.txt` file containing exact textual characters in document reading order. |
| `pdf_extract_tables` | Extract Tables | **Table Detection Matrix:** Outputs structured JSON or CSV matrix of rows and columns extracted from table borders/alignment. |
| `pdf_ocr` | OCR Text Recognition | **Searchable Text Layer:** Scanned image-only PDF receives an invisible text layer (`/Font`, `/Text`) making it searchable. |
| `office_to_markdown` | Office to Markdown | **DOCX/XLSX/PPTX to Markdown:** Extracts headings, tables, and paragraphs from Office files directly to Markdown. |
| `office_extract_text` | Office Plain Text | **Office Text Dumper:** Fast textual stream extraction from DOCX/XLSX/PPTX XML without formatting tags. |
| `office_modernize_legacy` | Modernize Legacy Office | **Binary to OOXML Upgrade:** Converts legacy binary `.doc`, `.xls`, `.ppt` into modern `.docx`, `.xlsx`, `.pptx`. |
| `excel_to_json` | Excel to JSON | **Worksheet to JSON Objects:** Ingests Excel spreadsheet and outputs JSON array of `{ ColumnHeader: CellValue }`. |
| `excel_to_csv` | Excel to CSV | **RFC 4180 CSV Export:** Exports target Excel worksheet into standard comma-separated text records. |
| `pdf_search` | Search Text | **Indexed Search Results:** Returns matched text string with exact page numbers and bounding box coordinates. |
| `workspace_search` | Workspace Search | **Multi-File Search:** Recursively searches workspace PDF/Office files and returns ranked hits with file paths and line matches. |
| `semantic_search` | Semantic Vector Search | **Embedding Similarity:** Uses vector embeddings to retrieve document chunks matching conceptual meaning of natural language query. |
| `pdf_classify_type` | Classify Document | **Document Type Classifier:** Returns classification JSON with category (invoice, receipt, contract, legal, report) and confidence score. |
| `pdf_rag_chunks` | RAG Pipeline Chunks | **Semantic Chunking:** Splits document into chunks with token counts, sentence preservation, and breadcrumb header metadata. |
| `document_workflow` | Workflow Runner | **Declarative Recipe Execution:** Executes multi-step `.ppflow` recipe (e.g. clean scan $\to$ OCR $\to$ watermark) producing final output. |
| `recipe_run` | Instant Recipe Run | **Preset Execution:** Executes built-in workflow presets (legal discovery, archival prep) against input document. |
| `cli_ask` | AI Document Assistant | **Natural Language Intent:** Executes operations based on natural language command (e.g. "rotate 90 degrees"). |
| `watcher_start` | Start Hot-Folder Watcher | **Background Daemon Lifecycle:** Starts monitoring directory for incoming PDFs; generates initial watcher state. |
| `watcher_stop` | Stop Hot-Folder Watcher | **Graceful Daemon Teardown:** Shuts down watcher daemon cleanly and flushes execution receipts to `.paperpilot-receipts`. |
