# Manual Desktop UI Testing Guide: Tester 4 (23 Office Conversions, AI Intelligence & Workflow Tools)

**Assigned Tester:** Tester 4  
**Total Tools Assigned:** 23 Tools  
**Domain:** Office OpenXML Conversions, Multimodal Layout Extraction, Neural OCR, Workspace Search & Workflow Recipes

---

### Tool 67: `pdf_to_docx` — Convert PDF into Microsoft Word (.docx)
- **UI Access:** Operations Dock > **Quick** / **Convert** > **PDF to Word (DOCX)**.
- **Test File:** Multi-page PDF with headings and body text.
- **Manual Steps:**
  1. Click **Export as Word (DOCX)**. Choose output location.
- **Expected Result:** Generates valid `.docx` OpenXML file with structured paragraphs and headings.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 68: `pdf_to_xlsx` — Extract Tabular Layout into Microsoft Excel (.xlsx)
- **UI Access:** Operations Dock > **Convert** > **PDF to Excel (XLSX)**.
- **Test File:** PDF containing tables or numerical figures.
- **Manual Steps:**
  1. Click **Extract to Excel (.xlsx)**.
- **Expected Result:** Generates valid `.xlsx` workbook; tables are converted into clean spreadsheet rows and columns.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 69: `pdf_to_pptx` — Export PDF Pages into Presentation Slides (.pptx)
- **UI Access:** Operations Dock > **Convert** > **PDF to PowerPoint (PPTX)**.
- **Test File:** Landscape presentation slide PDF.
- **Manual Steps:**
  1. Click **Export as PPTX Slides**.
- **Expected Result:** Generates `.pptx` presentation file where each page becomes a presentation slide.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 70: `pdf_to_html` — Export PDF as Responsive Styled HTML5 Web Page
- **UI Access:** Operations Dock > **Convert** > **PDF to HTML**.
- **Test File:** Any PDF document.
- **Manual Steps:**
  1. Click **Convert to HTML5**.
- **Expected Result:** Generates clean HTML5 markup with embedded CSS styling matching page fonts and colors.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 71: `pdf_to_markdown` — Clean LLM-Ready Markdown Extraction
- **UI Access:** Operations Dock > **Convert** > **PDF to Markdown** (or LLM Export icon).
- **Test File:** Multi-page document with headings, bullet lists, and tables.
- **Manual Steps:**
  1. Click **Extract Markdown**. Review text in editor drawer.
- **Expected Result:** Produces formatted Markdown (`# Heading`, `- Bullet`, `| Table |`) suitable for LLM prompts.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 72: `pdf_to_json` — Multimodal JSON Layout with Bounding Boxes
- **UI Access:** Operations Dock > **Convert** > **PDF to JSON**.
- **Test File:** Any PDF.
- **Manual Steps:**
  1. Click **Extract Multimodal JSON**.
- **Expected Result:** Output JSON document contains hierarchical page objects, font metadata, text spans, and bounding coordinates `[x, y, w, h]`.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 73: `pdf_extract_text` — Fast Plain Text Dumper (.txt)
- **UI Access:** Operations Dock > **Convert** > **PDF to Plain Text**.
- **Test File:** Any PDF.
- **Manual Steps:**
  1. Click **Extract Plain Text**.
- **Expected Result:** Fast text extraction outputs raw document text without binary garbage.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 74: `pdf_extract_tables` — High-Precision Table Matrix Extractor
- **UI Access:** Operations Dock > **AI** > **Extract Tables**.
- **Test File:** Financial statement or invoice PDF.
- **Manual Steps:**
  1. Format: **CSV** or **JSON Matrix**. Click **Extract Tables**.
- **Expected Result:** Table borders and cell coordinates are detected; exported as structured CSV/JSON.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 75: `pdf_ocr` — Neural Optical Character Recognition on Scanned Pages
- **UI Access:** Operations Dock > **AI** > **OCR Text Recognition**.
- **Test File:** Scanned bitmap document (e.g. `tests/fixtures/scanned_doc.pdf`).
- **Manual Steps:**
  1. Select language: **English** (default). Click **Run OCR**.
- **Expected Result:** Scanned page images are processed; text becomes selectable or exports to `.txt`. Gracefully handles missing weights with informative notice.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 76: `office_to_markdown` — Convert Word/Excel/PowerPoint to Markdown
- **UI Access:** Operations Dock > **Convert** > **Convert Office to Markdown**.
- **Test File:** Sample `.docx`, `.xlsx`, or `.pptx` file.
- **Manual Steps:**
  1. Drop Office file into converter. Click **Convert to Markdown**.
- **Expected Result:** Extracts document body into structured Markdown syntax.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 77: `office_extract_text` — Extract Raw Text from Office Documents
- **UI Access:** Operations Dock > **Convert** > **Office to Plain Text**.
- **Test File:** Sample `.docx` or `.xlsx`.
- **Manual Steps:**
  1. Drop file. Click **Extract Text**.
- **Expected Result:** Fast raw text extraction from OpenXML document packages.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 78: `office_modernize_legacy` — Upgrade Legacy .doc/.xls/.ppt
- **UI Access:** Operations Dock > **Convert** > **Modernize Legacy Office**.
- **Test File:** Older binary Office file (`.doc`, `.xls`, or `.ppt`).
- **Manual Steps:**
  1. Drop binary file. Click **Upgrade to Modern Office Format**.
- **Expected Result:** Converts older binary format to modern standard `.docx`, `.xlsx`, or `.pptx`.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 79: `excel_to_json` — Convert Spreadsheet Sheets into JSON Objects
- **UI Access:** Operations Dock > **Convert** > **Excel to JSON**.
- **Test File:** `test.xlsx` (or any spreadsheet).
- **Manual Steps:**
  1. Load spreadsheet. Click **Convert to JSON**.
- **Expected Result:** Outputs JSON array where each object represents a table row with column headers as keys.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 80: `excel_to_csv` — Export Spreadsheet Sheets to RFC 4180 CSV
- **UI Access:** Operations Dock > **Convert** > **Excel to CSV**.
- **Test File:** `test.xlsx`.
- **Manual Steps:**
  1. Load spreadsheet. Click **Export CSV**.
- **Expected Result:** Generates standard RFC 4180 comma-delimited text file.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 81: `pdf_search` — Real-Time Text Search and Highlighting in Viewer
- **UI Access:** Canvas Search Bar (`Ctrl+F` or `Cmd+F`).
- **Test File:** Multi-page document.
- **Manual Steps:**
  1. Press `Ctrl+F`. Type: `Page`.
  2. Press `Enter` to cycle through matches using Next (`↓`) and Prev (`↑`).
- **Expected Result:** Search occurrences highlight on canvas in yellow; viewer scrolls automatically to active match.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 82: `workspace_search` — Recursive Multi-Format Workspace Search
- **UI Access:** Operations Dock > **AI** > **Workspace Search**.
- **Test File:** A folder containing multiple `.pdf`, `.docx`, `.xlsx`, and `.md` files.
- **Manual Steps:**
  1. Select target directory. Query: `report`. Click **Search Workspace**.
- **Expected Result:** Recursively scans directory and returns matched documents with page numbers without directory errors.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 83: `semantic_search` — Vector Semantic Retrieval Across Chunks
- **UI Access:** Command Palette (`Ctrl+K`) > `semantic_search`.
- **Test File:** Any multi-paragraph PDF document.
- **Manual Steps:**
  1. Query: `financial obligations and liabilities`. Limit: `5`. Click **Execute Search**.
- **Expected Result:** Returns semantically related text excerpts ranked by similarity score.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 84: `pdf_classify_type` — Document Type & Category Classifier
- **UI Access:** Operations Dock > **AI** > **Classify Document**.
- **Test File:** Invoice, contract, or academic paper PDF.
- **Manual Steps:**
  1. Click **Classify Document Type**.
- **Expected Result:** Drawer shows classification result (e.g. `Category: Invoice/Financial`, `Confidence: 0.94`).
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 85: `pdf_rag_chunks` — Semantic Chunk Generator for RAG LLM Pipelines
- **UI Access:** Command Palette (`Ctrl+K`) > `pdf_rag_chunks`.
- **Test File:** Any multi-page document.
- **Manual Steps:**
  1. Set chunk size: `500 tokens`, overlap: `50 tokens`. Click **Generate Chunks**.
- **Expected Result:** Generates JSON list of text chunks with page indices and section headers.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 86: `document_workflow` — Declarative Recipe Workflow Runner (.ppflow)
- **UI Access:** Visual Pipeline Builder tab (`⚡ Pipeline` top nav button).
- **Test File:** Any input document.
- **Manual Steps:**
  1. Open Visual Pipeline Canvas. Connect `Rotate(90)` → `Watermark("Confidential")` → `Compress`.
  2. Click **Run Pipeline**.
- **Expected Result:** Multi-step pipeline executes sequentially with real-time step progress animations and produces final output.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 87: `recipe_run` — Instant Pipeline Execution of Multi-Step Tools
- **UI Access:** Visual Pipeline Builder > **Execute Recipe**.
- **Test File:** Any PDF.
- **Manual Steps:**
  1. Load a pre-saved recipe (`.json` or `.ppflow`). Click **Execute Recipe**.
- **Expected Result:** Recipe runs and displays execution audit log with success checkmarks on each node.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 88: `cli_ask` — Natural Language AI Assistant for Document Tasks
- **UI Access:** Operations Dock AI Chat Panel (`🤖`).
- **Test File:** Any open document.
- **Manual Steps:**
  1. Type in chat: `"Rotate this document 90 degrees and add confidential watermark"`.
  2. Review resolved plan. Click **Execute Plan**.
- **Expected Result:** Offline NLP resolver maps query to `Rotate` and `Watermark` operations and applies them to the document.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 89: `watcher_start` / `watcher_stop` — Hot-Folder Watcher Daemon Automation
- **UI Access:** Operations Dock > **Automation** > **Folder Watchers**.
- **Test File:** A temporary folder (e.g. `tests/watch_in`).
- **Manual Steps:**
  1. Set Watch Directory: `tests/watch_in`. Set Output Directory: `tests/watch_out`. Assign Recipe: `Compress`.
  2. Click **Start Watcher**.
  3. Drop a test PDF into `tests/watch_in`.
  4. Verify the processed file appears in `tests/watch_out`. Click **Stop Watcher**.
- **Expected Result:** Background daemon automatically processes arriving files and stops when requested.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`
