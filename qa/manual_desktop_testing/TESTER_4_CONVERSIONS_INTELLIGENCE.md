# Manual Desktop UI Testing Guide: Tester 4 (23 Office Conversions, AI Intelligence & Workflow Tools)

**Assigned Tester:** Tester 4  
**Total Tools Assigned:** 23 Tools (Tools 67 to 89)  
**Domain:** Office OpenXML Conversions, Multimodal Layout Extraction, Neural OCR, Workspace Search & Workflow Recipes  
**Target Branch:** `feature/dev` / `feat/desktop-ui-manual-testing-matrix-4p`  

---

## 🚀 Quick Setup & Test Fixtures (Run Once Before Testing)

All 23 tools can be tested using **6 pre-existing real test fixtures** from the repository, plus dedicated input/output watcher directories.

### 1. Launch the Tauri Desktop Application
Open a terminal in the project root and start the desktop app:
```bash
cd apps/desktop
npm run tauri dev
```
*(Keep system RAM footprint light; close heavy unused browser tabs).*

### 2. One-Liner Fixture Setup Command
Run this one command in your terminal to create a clean `tester4_fixtures` folder on your Desktop with all required test files:
```bash
mkdir -p ~/Desktop/tester4_fixtures/watch_in ~/Desktop/tester4_fixtures/watch_out
cp tests/e2e_fixtures/real/multi_page.pdf ~/Desktop/tester4_fixtures/
cp tests/e2e_fixtures/real/search_test.pdf ~/Desktop/tester4_fixtures/
cp tests/e2e_fixtures/real/ocr.pdf ~/Desktop/tester4_fixtures/
cp tests/e2e_fixtures/real/converted.docx ~/Desktop/tester4_fixtures/
cp tests/e2e_fixtures/real/test.xls ~/Desktop/tester4_fixtures/
cp tests/e2e_fixtures/real/test.xlsx ~/Desktop/tester4_fixtures/
```

### 📋 Fixture-to-Tool Mapping Matrix

| Test Fixture File | Format / Purpose | Tools Covered |
| :--- | :--- | :--- |
| `~/Desktop/tester4_fixtures/multi_page.pdf` | 5-Page Standard PDF with Headings & Layout | **Tools 67, 68, 69, 70, 71, 72, 74, 83, 84, 85, 86, 87, 88, 89** |
| `~/Desktop/tester4_fixtures/search_test.pdf` | Search-Optimized Text Document | **Tools 73, 81** |
| `~/Desktop/tester4_fixtures/ocr.pdf` | Scanned Bitmap Document Image | **Tool 75** |
| `~/Desktop/tester4_fixtures/converted.docx` | Microsoft Word OpenXML Document | **Tools 76, 77, 82** |
| `~/Desktop/tester4_fixtures/test.xls` | Legacy Microsoft Excel 97–2003 Binary | **Tool 78** |
| `~/Desktop/tester4_fixtures/test.xlsx` | Modern Microsoft Excel Spreadsheet | **Tools 79, 80, 82** |
| `~/Desktop/tester4_fixtures/watch_in/` & `watch_out/` | Hot-Folder Daemon Test Directories | **Tool 89** |

---

## 🛠️ Step-by-Step Testing Guide (All 23 Tools)

---

### Tool 67: `pdf_to_docx` — Convert PDF into Microsoft Word (.docx)
- **Input File:** `~/Desktop/tester4_fixtures/multi_page.pdf` (or `tests/e2e_fixtures/real/multi_page.pdf`)
- **UI Access:** 
  - Operations Dock (`🛠️` on right rail) > click **Convert** category tab > click **PDF to Word (DOCX)**
  - *Shortcut:* Press `Ctrl+K` (or `Cmd+K`), type `docx`, and select **PDF to Word (DOCX)**.
- **Manual Steps:**
  1. Drag and drop `multi_page.pdf` into the main viewer canvas (or click **File > Open Document**).
  2. In the right Operations Dock, click the **Convert** category pill.
  3. Select **PDF to Word (DOCX)**.
  4. Note the pre-populated output file name: `multi_page.docx`.
  5. Click the primary **Convert to DOCX** button.
- **What to Verify:**
  - Toast alert appears: `Operation completed successfully`.
  - A valid `multi_page.docx` file is saved to disk.
  - Open `multi_page.docx` in LibreOffice Writer, Microsoft Word, or Google Docs: verify body paragraphs, headings, and formatting remain structured.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 68: `pdf_to_xlsx` — Extract Tabular Layout into Microsoft Excel (.xlsx)
- **Input File:** `~/Desktop/tester4_fixtures/multi_page.pdf` (or `tests/e2e_fixtures/real/multi_page.pdf`)
- **UI Access:** 
  - Operations Dock > **Convert** > **PDF to Excel (XLSX)**
  - *Shortcut:* Press `Ctrl+K`, type `xlsx`, and select **PDF to Excel (XLSX)**.
- **Manual Steps:**
  1. Ensure `multi_page.pdf` is open in the viewer.
  2. Open the Operations Dock and select the **Convert** category.
  3. Click **PDF to Excel (XLSX)**.
  4. Leave output path as `multi_page.xlsx` (or choose custom folder).
  5. Click **Extract to Excel (.xlsx)**.
- **What to Verify:**
  - Green success toast notification triggers.
  - Generates valid `multi_page.xlsx` workbook on disk.
  - Open `multi_page.xlsx` in LibreOffice Calc, Excel, or PaperPilot's built-in spreadsheet viewer: verify rows and columns are populated cleanly.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 69: `pdf_to_pptx` — Export PDF Pages into Presentation Slides (.pptx)
- **Input File:** `~/Desktop/tester4_fixtures/multi_page.pdf` (or `tests/e2e_fixtures/real/multi_page.pdf`)
- **UI Access:** 
  - Operations Dock > **Convert** > **PDF to PowerPoint (PPTX)**
  - *Shortcut:* Press `Ctrl+K`, type `pptx`, and select **PDF to PowerPoint**.
- **Manual Steps:**
  1. Open `multi_page.pdf` in the viewer.
  2. In Operations Dock > **Convert**, click **PDF to PowerPoint (PPTX)**.
  3. Destination defaults to `multi_page.pptx`.
  4. Click **Export as PPTX Slides**.
- **What to Verify:**
  - Toast confirmation appears without errors.
  - Generates a valid `.pptx` presentation.
  - Open `multi_page.pptx` in LibreOffice Impress or PowerPoint: verify each PDF page is mapped to a presentation slide.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 70: `pdf_to_html` — Export PDF as Responsive Styled HTML5 Web Page
- **Input File:** `~/Desktop/tester4_fixtures/multi_page.pdf` (or `tests/e2e_fixtures/real/multi_page.pdf`)
- **UI Access:** 
  - Operations Dock > **Convert** > **PDF to HTML**
  - *Shortcut:* Press `Ctrl+K`, type `html`, and select **PDF to HTML**.
- **Manual Steps:**
  1. Open `multi_page.pdf` in the viewer.
  2. In Operations Dock > **Convert**, select **PDF to HTML**.
  3. Keep output path as `multi_page.html`.
  4. Click **Convert to HTML5**.
- **What to Verify:**
  - Success toast appears upon export.
  - Generates `multi_page.html`.
  - Double-click `multi_page.html` to open in Chrome or Firefox: verify clean HTML5 markup with embedded CSS styling matching page text.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 71: `pdf_to_markdown` — Clean LLM-Ready Markdown Extraction
- **Input File:** `~/Desktop/tester4_fixtures/multi_page.pdf` (or `tests/e2e_fixtures/real/multi_page.pdf`)
- **UI Access:** 
  - Operations Dock > **Convert** > **PDF to Markdown**
  - *Shortcut:* Press `Ctrl+K`, type `markdown`, and select **PDF to Markdown**.
- **Manual Steps:**
  1. Open `multi_page.pdf` in the viewer.
  2. In Operations Dock > **Convert**, select **PDF to Markdown**.
  3. Output defaults to `multi_page.md`.
  4. Click **Extract Markdown**.
- **What to Verify:**
  - Generates `multi_page.md` on disk.
  - Open `multi_page.md` in any text editor (e.g. VS Code, Kate, Gedit): verify clean markdown structure (`# Heading`, `- Bullet`, paragraphs) ready for LLM prompt ingestion.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 72: `pdf_to_json` — Multimodal JSON Layout with Bounding Boxes
- **Input File:** `~/Desktop/tester4_fixtures/multi_page.pdf` (or `tests/e2e_fixtures/real/multi_page.pdf`)
- **UI Access:** 
  - Operations Dock > **Convert** > **PDF to JSON**
  - *Shortcut:* Press `Ctrl+K`, type `json`, and select **PDF to JSON**.
- **Manual Steps:**
  1. Open `multi_page.pdf` in the viewer.
  2. In Operations Dock > **Convert**, click **PDF to JSON**.
  3. Click **Extract Multimodal JSON**.
- **What to Verify:**
  - Generates `multi_page.json`.
  - Open `multi_page.json`: verify valid JSON syntax containing hierarchical page objects, font metadata, text spans, and bounding coordinates `[x, y, w, h]`.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 73: `pdf_extract_text` — Fast Plain Text Dumper (.txt)
- **Input File:** `~/Desktop/tester4_fixtures/search_test.pdf` (or `tests/e2e_fixtures/real/search_test.pdf`)
- **UI Access:** 
  - Operations Dock > **Convert** > **PDF to Plain Text**
  - *Shortcut:* Press `Ctrl+K`, type `plain text`, and select **PDF to Plain Text**.
- **Manual Steps:**
  1. Open `search_test.pdf` in the viewer.
  2. In Operations Dock > **Convert**, select **PDF to Plain Text**.
  3. Output defaults to `search_test_text.txt`.
  4. Click **Extract Plain Text**.
- **What to Verify:**
  - Fast extraction completes instantly (< 50ms).
  - Open `search_test_text.txt`: verify clean raw text dump without corrupt binary characters or escape garbage.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 74: `pdf_extract_tables` — High-Precision Table Matrix Extractor
- **Input File:** `~/Desktop/tester4_fixtures/multi_page.pdf` (or `tests/e2e_fixtures/real/multi_page.pdf`)
- **UI Access:** 
  - Operations Dock > **AI** > **Extract Tables**
  - *Shortcut:* Press `Ctrl+K`, type `tables`, and select **Extract Tables**.
- **Manual Steps:**
  1. Open `multi_page.pdf`.
  2. In Operations Dock, switch to the **AI** category pill.
  3. Select **Extract Tables**.
  4. Select format: **CSV** or **JSON Matrix**.
  5. Click **Extract Tables**.
- **What to Verify:**
  - Generates table data export (`.csv` or `.json`).
  - Open output file: verify table cells and tabular rows are extracted with correct matrix alignment.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 75: `pdf_ocr` — Neural Optical Character Recognition on Scanned Pages
- **Input File:** `~/Desktop/tester4_fixtures/ocr.pdf` (or `tests/e2e_fixtures/real/ocr.pdf`)
- **UI Access:** 
  - Operations Dock > **AI** > **OCR Text Recognition**
  - *Shortcut:* Press `Ctrl+K`, type `ocr`, and select **OCR Text Recognition**.
- **Manual Steps:**
  1. Drag and drop `ocr.pdf` (scanned bitmap document) into the viewer.
  2. In Operations Dock > **AI**, select **OCR Text Recognition**.
  3. Choose OCR Language: **English** (default).
  4. Select Output Mode: **Searchable PDF** or **Text (.txt)**.
  5. Click **Run OCR**.
- **What to Verify:**
  - Scanned bitmap is processed; text becomes selectable on canvas or exports to `.txt`.
  - If optional offline OCR neural weights are not yet installed locally, verify UI displays a clear informative banner rather than freezing or crashing.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 76: `office_to_markdown` — Convert Word/Excel/PowerPoint to Markdown
- **Input File:** `~/Desktop/tester4_fixtures/converted.docx` (or `tests/e2e_fixtures/real/converted.docx`)
- **UI Access:** 
  - Operations Dock > **Convert** > **Convert Office to Markdown**
  - *Shortcut:* Press `Ctrl+K`, type `office markdown`, and select **Convert to Markdown**.
- **Manual Steps:**
  1. Drag and drop `converted.docx` directly into PaperPilot (or click **File > Open**).
  2. In Operations Dock > **Convert**, click **Convert to Markdown**.
  3. Output filename defaults to `converted.md`.
  4. Click **Convert Office to Markdown**.
- **What to Verify:**
  - Produces `converted.md` on disk.
  - Open `converted.md`: verify Word document body, bolding, lists, and headers are cleanly converted into GitHub-flavored Markdown.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 77: `office_extract_text` — Extract Raw Text from Office Documents
- **Input File:** `~/Desktop/tester4_fixtures/converted.docx` (or `test.xlsx`)
- **UI Access:** 
  - Operations Dock > **Convert** > **Office to Plain Text**
  - *Shortcut:* Press `Ctrl+K`, type `office text`, and select **Office to Plain Text**.
- **Manual Steps:**
  1. Ensure `converted.docx` is loaded in PaperPilot.
  2. In Operations Dock > **Convert**, select **Office to Plain Text**.
  3. Output filename defaults to `converted.txt`.
  4. Click **Extract Text**.
- **What to Verify:**
  - Produces `converted.txt`.
  - Check file content: verify all text paragraphs extracted from the OpenXML package without XML markup tags (`<w:p>`, `<w:t>`).
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 78: `office_modernize_legacy` — Upgrade Legacy .doc/.xls/.ppt
- **Input File:** `~/Desktop/tester4_fixtures/test.xls` (or `tests/e2e_fixtures/real/test.xls` — Legacy Excel 97–2003 binary format)
- **UI Access:** 
  - Operations Dock > **Convert** > **Modernize Legacy Office**
  - *Shortcut:* Press `Ctrl+K`, type `modernize`, and select **Modernize Legacy Office**.
- **Manual Steps:**
  1. Drop `test.xls` into PaperPilot.
  2. In Operations Dock > **Convert**, click **Modernize Legacy Office**.
  3. Click **Upgrade to Modern Office Format**.
- **What to Verify:**
  - Toast confirms successful format conversion.
  - Generates a modern `.xlsx` OpenXML spreadsheet.
  - Open the upgraded `.xlsx` in LibreOffice Calc / Excel: verify it opens without "Legacy binary format" warnings.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 79: `excel_to_json` — Convert Spreadsheet Sheets into JSON Objects
- **Input File:** `~/Desktop/tester4_fixtures/test.xlsx` (or `tests/e2e_fixtures/real/test.xlsx`)
- **UI Access:** 
  - Operations Dock > **Convert** > **Excel to JSON**
  - *Shortcut:* Press `Ctrl+K`, type `excel to json`, and select **Excel to JSON**.
- **Manual Steps:**
  1. Drop `test.xlsx` into PaperPilot (opens in the built-in Spreadsheet Viewer).
  2. In Operations Dock > **Convert**, select **Excel to JSON**.
  3. Output filename defaults to `test.json`.
  4. Click **Convert to JSON**.
- **What to Verify:**
  - Produces `test.json`.
  - Open `test.json`: verify an array of JSON objects where each object represents a spreadsheet row keyed by column headers.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 80: `excel_to_csv` — Export Spreadsheet Sheets to RFC 4180 CSV
- **Input File:** `~/Desktop/tester4_fixtures/test.xlsx` (or `tests/e2e_fixtures/real/test.xlsx`)
- **UI Access:** 
  - Operations Dock > **Convert** > **Excel to CSV**
  - *Shortcut:* Press `Ctrl+K`, type `excel to csv`, and select **Excel to CSV**.
- **Manual Steps:**
  1. Open `test.xlsx` in PaperPilot.
  2. In Operations Dock > **Convert**, select **Excel to CSV**.
  3. Output filename defaults to `test.csv`.
  4. Click **Export CSV**.
- **What to Verify:**
  - Produces `test.csv`.
  - Open `test.csv`: verify comma-delimited columns formatted according to standard RFC 4180 (with quote escaping for special characters).
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 81: `pdf_search` — Real-Time Text Search and Highlighting in Viewer
- **Input File:** `~/Desktop/tester4_fixtures/search_test.pdf` (or `tests/e2e_fixtures/real/search_test.pdf`)
- **UI Access:** 
  - Keyboard Shortcut: `Ctrl+F` (or `Cmd+F`)
  - *Alternative:* Click the `🔍 Search` icon on the top viewer toolbar.
- **Manual Steps:**
  1. Open `search_test.pdf` in the main canvas viewer.
  2. Press `Ctrl+F` to open the search bar overlay at the top of the canvas.
  3. Type query: `test` (or `Page`).
  4. Press `Enter` to cycle forward; use the Next (`↓`) and Prev (`↑`) buttons to navigate through matches.
- **What to Verify:**
  - Search count badge displays (e.g. `1 of 5 matches`).
  - Search occurrences are highlighted on the canvas in yellow.
  - Active match is highlighted in orange; canvas viewer smoothly auto-scrolls to the active match.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 82: `workspace_search` — Recursive Multi-Format Workspace Search
- **Input Files:** Open 3 tabs simultaneously:
  - Tab 1: `multi_page.pdf`
  - Tab 2: `converted.docx`
  - Tab 3: `test.xlsx`
- **UI Access:** 
  - Keyboard Shortcut: `Ctrl+Shift+F` (or `Cmd+Shift+F`) to open the **Workspace Search Modal**.
- **Manual Steps:**
  1. With all 3 documents open in PaperPilot tabs, press `Ctrl+Shift+F`.
  2. Ensure search mode is set to **Exact**.
  3. Type search query: `test` (or `content`).
  4. Click file filter pills at the top: toggle between **All Files**, **PDFs**, **Word**, and **Excel**.
  5. Click on any search snippet in the results list.
- **What to Verify:**
  - Modal instantly displays total matches grouped by document (e.g. `X matches across 3 files`).
  - Clicking a result immediately focuses that document tab and jumps to the matched content.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 83: `semantic_search` — Vector Semantic Retrieval Across Chunks
- **Input File:** `~/Desktop/tester4_fixtures/multi_page.pdf` (open in viewer)
- **UI Access:** 
  - Press `Ctrl+Shift+F` > click **🧠 Semantic** toggle button in the search modal header.
  - *Alternative:* Command Palette (`Ctrl+K`) > search `semantic_search`.
- **Manual Steps:**
  1. Open `multi_page.pdf`.
  2. Press `Ctrl+Shift+F` to open Workspace Search.
  3. Click the **🧠 Semantic** mode button.
  4. Enter a conceptual query, e.g.: `financial obligations and performance metrics`.
  5. Press `Enter`.
- **What to Verify:**
  - Returns semantically relevant document excerpts ranked with similarity percentage scores, even when exact keywords differ.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 84: `pdf_classify_type` — Document Type & Category Classifier
- **Input File:** `~/Desktop/tester4_fixtures/multi_page.pdf` (or `tests/e2e_fixtures/real/multi_page.pdf`)
- **UI Access:** 
  - Operations Dock > **AI** > **Classify Document**
  - *Shortcut:* Press `Ctrl+K`, type `classify`, and select **Classify Document**.
- **Manual Steps:**
  1. Open `multi_page.pdf` in the viewer.
  2. In Operations Dock > **AI**, select **Classify Document**.
  3. Click **Classify Document Type**.
- **What to Verify:**
  - Drawer displays classification card with category archetype (e.g. `Category: Report / Document`, `Confidence: 0.92+`).
  - Breakdown lists detected structural traits (e.g. page count, text density, image presence).
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 85: `pdf_rag_chunks` — Semantic Chunk Generator for RAG LLM Pipelines
- **Input File:** `~/Desktop/tester4_fixtures/multi_page.pdf` (or `tests/e2e_fixtures/real/multi_page.pdf`)
- **UI Access:** 
  - Command Palette (`Ctrl+K`) > type `pdf_rag_chunks` > press `Enter`.
- **Manual Steps:**
  1. Open `multi_page.pdf`.
  2. Press `Ctrl+K`, type `pdf_rag_chunks`, and select it.
  3. Enter parameters:
     - `Chunk Size`: `500 tokens`
     - `Chunk Overlap`: `50 tokens`
  4. Click **Generate Chunks**.
- **What to Verify:**
  - Produces structured JSON output containing chunks array with `chunk_id`, `page_number`, `token_count`, and text content.
  - Chunks respect sentence/paragraph boundaries without truncation.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 86: `document_workflow` — Declarative Recipe Workflow Runner (.ppflow)
- **Input File:** `~/Desktop/tester4_fixtures/multi_page.pdf` (or `tests/e2e_fixtures/real/multi_page.pdf`)
- **UI Access:** 
  - Top Navigation Bar > click **⚡ Pipeline** tab to open the Visual Pipeline Canvas.
- **Manual Steps:**
  1. Click **⚡ Pipeline** in the top navigation bar.
  2. From the left **Tools Palette**:
     - Click **Rotate** (from *Transform* category) to add a Rotate node.
     - Click **Compress** (from *Optimize* category) to add a Compress node.
  3. Drag a connector line from the output port of the Rotate node to the input port of the Compress node.
  4. Drag `multi_page.pdf` into the canvas document dropzone (or select from open tabs).
  5. Click the primary **Run Pipeline** button in the top runner bar.
- **What to Verify:**
  - Pipeline executes sequentially: active nodes pulse, then display green checkmarks with execution timings.
  - Transformed output PDF is produced with both rotation and compression applied.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 87: `recipe_run` — Instant Pipeline Execution of Multi-Step Tools
- **Input File:** `~/Desktop/tester4_fixtures/multi_page.pdf` (or `tests/e2e_fixtures/real/multi_page.pdf`)
- **UI Access:** 
  - Top Navigation Bar > **⚡ Pipeline** > **Flow Selector** dropdown.
- **Manual Steps:**
  1. Navigate to the **⚡ Pipeline** view.
  2. In the top bar, click the **Flow Selector** dropdown.
  3. Choose any pre-saved workflow (or click **Open Recipe...** to load a `.ppflow` / `.json` file).
  4. Notice the pre-configured workflow nodes appear on the canvas.
  5. Click **Run Pipeline**.
- **What to Verify:**
  - Execution completes with a full execution audit log showing step-by-step status.
  - Success badge displays total execution time in milliseconds.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 88: `cli_ask` — Natural Language AI Assistant for Document Tasks
- **Input File:** `~/Desktop/tester4_fixtures/multi_page.pdf` (or any open document)
- **UI Access:** 
  - Click the **🤖 AI Chat** icon on the right sidebar or bottom navigation.
- **Manual Steps:**
  1. Open `multi_page.pdf` in the canvas viewer.
  2. Open the AI Chat panel (`🤖`).
  3. Type in the chat input: `"Compress this PDF and rotate 90 degrees"`.
  4. Press `Enter` (or click Send).
  5. Review the generated plan card: the offline NLP engine should resolve the prompt into `Rotate` and `Compress` steps.
  6. Click **Execute Plan** on the action card.
- **What to Verify:**
  - Assistant responds with resolved plan card without external cloud API dependencies.
  - Clicking **Execute Plan** executes the operations and reloads the viewer with the rotated, compressed document.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 89: `watcher_start` / `watcher_stop` — Hot-Folder Watcher Daemon Automation
- **Input Test Directories:** 
  - Watch Folder: `~/Desktop/tester4_fixtures/watch_in`
  - Output Folder: `~/Desktop/tester4_fixtures/watch_out`
- **UI Access:** 
  - Top Navigation / Sidebar > click **⚙️ Settings** > scroll down to **Hot-Folder Watchers** section.
- **Manual Steps:**
  1. In PaperPilot, click **⚙️ Settings** tab.
  2. In the **Hot-Folder Watchers** card:
     - Set Watch Directory: enter or browse to `~/Desktop/tester4_fixtures/watch_in` (e.g. `/home/krushna/Desktop/tester4_fixtures/watch_in`).
     - Set Output Directory: enter or browse to `~/Desktop/tester4_fixtures/watch_out`.
     - Workflow: Select a preset (e.g. `Compress`).
  3. Click **Start watching**.
  4. Verify the folder status badge switches to `watching` (active).
  5. In your file manager or terminal, copy `multi_page.pdf` into `watch_in/`:
     ```bash
     cp ~/Desktop/tester4_fixtures/multi_page.pdf ~/Desktop/tester4_fixtures/watch_in/
     ```
  6. Wait 2 seconds: check `watch_out/` directory. Verify the processed PDF appears.
  7. In PaperPilot Settings, click **Stop watching**.
- **What to Verify:**
  - Status updates to `watching` when started and returns to idle when stopped.
  - Dropped files are automatically picked up by the daemon, processed, and deposited into `watch_out/`.
  - Daemon gracefully shuts down when stopped without leaving orphan file locks.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

## 🏁 Summary Sign-Off Scorecard: Tester 4

| Tool # | Tool Identifier | UI Location | Test Input File | Status | Notes |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **67** | `pdf_to_docx` | Operations Dock > Convert | `multi_page.pdf` | `[ ] PASS` | |
| **68** | `pdf_to_xlsx` | Operations Dock > Convert | `multi_page.pdf` | `[ ] PASS` | |
| **69** | `pdf_to_pptx` | Operations Dock > Convert | `multi_page.pdf` | `[ ] PASS` | |
| **70** | `pdf_to_html` | Operations Dock > Convert | `multi_page.pdf` | `[ ] PASS` | |
| **71** | `pdf_to_markdown` | Operations Dock > Convert | `multi_page.pdf` | `[ ] PASS` | |
| **72** | `pdf_to_json` | Operations Dock > Convert | `multi_page.pdf` | `[ ] PASS` | |
| **73** | `pdf_extract_text` | Operations Dock > Convert | `search_test.pdf` | `[ ] PASS` | |
| **74** | `pdf_extract_tables` | Operations Dock > AI | `multi_page.pdf` | `[ ] PASS` | |
| **75** | `pdf_ocr` | Operations Dock > AI | `ocr.pdf` | `[ ] PASS` | |
| **76** | `office_to_markdown` | Operations Dock > Convert | `converted.docx` | `[ ] PASS` | |
| **77** | `office_extract_text` | Operations Dock > Convert | `converted.docx` | `[ ] PASS` | |
| **78** | `office_modernize_legacy` | Operations Dock > Convert | `test.xls` | `[ ] PASS` | |
| **79** | `excel_to_json` | Operations Dock > Convert | `test.xlsx` | `[ ] PASS` | |
| **80** | `excel_to_csv` | Operations Dock > Convert | `test.xlsx` | `[ ] PASS` | |
| **81** | `pdf_search` | Viewer Toolbar (`Ctrl+F`) | `search_test.pdf` | `[ ] PASS` | |
| **82** | `workspace_search` | Global Modal (`Ctrl+Shift+F`) | Multi-Tab (PDF, DOCX, XLSX) | `[ ] PASS` | |
| **83** | `semantic_search` | Global Modal (`Ctrl+Shift+F` > Semantic) | `multi_page.pdf` | `[ ] PASS` | |
| **84** | `pdf_classify_type` | Operations Dock > AI | `multi_page.pdf` | `[ ] PASS` | |
| **85** | `pdf_rag_chunks` | Command Palette (`Ctrl+K`) | `multi_page.pdf` | `[ ] PASS` | |
| **86** | `document_workflow` | Top Nav `⚡ Pipeline` Canvas | `multi_page.pdf` | `[ ] PASS` | |
| **87** | `recipe_run` | Top Nav `⚡ Pipeline` Runner | `multi_page.pdf` | `[ ] PASS` | |
| **88** | `cli_ask` | Right Sidebar `🤖 AI Chat` | `multi_page.pdf` | `[ ] PASS` | |
| **89** | `watcher_start` / `watcher_stop` | Settings `⚙️` > Watched Folders | `watch_in/` & `watch_out/` | `[ ] PASS` | |

**Tester Signature:** ___________________________  
**Date of Testing:** ___________________________  
**Overall Result:** `[ ] FULL PASS (23/23)` | `[ ] ISSUES DETECTED`
