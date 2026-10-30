# Manual Desktop UI Testing Guide: Tester 3 (22 Forms, Batch Automation, Markup & Synthesis Tools)

**Assigned Tester:** Tester 3  
**Total Tools Assigned:** 22 Tools  
**Domain:** AcroForms, Batch Form Engine, Watermarking, Legal Bates Numbering, True Redaction & Document Synthesis

---

### Tool 45: `pdf_read_form` — Extract AcroForm Field Values to JSON
- **UI Access:** Operations Dock > **Edit** > **Read Form Data**.
- **Test File:** `tests/fixtures/sample_form.pdf`.
- **Manual Steps:**
  1. Open form in viewer. Click **Read Form Fields**.
- **Expected Result:** JSON drawer displays all fields, types (text, checkbox, radio), and current values.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 46: `pdf_fill_form` — Populate AcroForms from JSON or Direct Typing
- **UI Access:** Direct on-canvas clicking, or Operations Dock > **Edit** > **Fill Form Data**.
- **Test File:** `tests/fixtures/sample_form.pdf`.
- **Manual Steps:**
  1. Click into field on canvas, type `Alice Smith`, check the checkbox.
  2. Or click **Import JSON** and supply a key-value mapping.
- **Expected Result:** Values populate into the form widgets and persist when switching pages or saving.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 47: `pdf_create_form_field` — Interactive Form Field Builder
- **UI Access:** Operations Dock > **Edit** > **Add Form Field** (or form design toolbar).
- **Test File:** Blank or static PDF document.
- **Manual Steps:**
  1. Select **Text Field**, field name: `account_number`. Drag a box on canvas. Click **Create Field**.
- **Expected Result:** A new interactive blue-bordered text field appears on canvas ready for typing.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 48: `batch_fill` — Multi-Record CSV Batch Form Generator
- **UI Access:** Operations Dock > **Edit** > **Batch Form Fill** (or *Tools > Automation > Batch Fill*).
- **Test File:** `tests/fixtures/sample_form.pdf` + a 3-row CSV file (`name,email,amount`).
- **Manual Steps:**
  1. Load template form PDF. Load CSV file. Map CSV column headers to PDF form field names.
  2. Click **Generate Batch PDFs**.
- **Expected Result:** Generates 3 separate filled PDFs named dynamically from CSV rows, plus an execution receipt.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 49: `cli_fill_batch` — Automated Batch Engine & Receipt Generation
- **UI Access:** Command Palette (`Ctrl+K`) > `batch_fill` or Automation tab.
- **Test File:** Sample AcroForm + CSV data file.
- **Manual Steps:**
  1. Trigger batch run with output directory specified.
- **Expected Result:** All rows are filled in headless mode with detailed execution audit receipts logged.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 50: `pdf_watermark` — Semi-Transparent Text & Diagonal Branding Watermark
- **UI Access:** Operations Dock > **Edit** > **Add Watermark**.
- **Test File:** Any PDF.
- **Manual Steps:**
  1. Text: `CONFIDENTIAL DRAFT`. Opacity: `20%`. Angle: `45° Diagonal`. Color: `Gray`. Click **Apply**.
- **Expected Result:** Semi-transparent watermark text appears centered across all pages without obscuring text.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 51: `pdf_bates` — Legal Bates Numbering Sequence Stamping
- **UI Access:** Operations Dock > **Edit** > **Bates Numbering**.
- **Test File:** Multi-page PDF.
- **Manual Steps:**
  1. Prefix: `EXHIBIT-A-`. Start Number: `1`. Padding: `6`. Position: `Bottom Right`. Click **Apply**.
- **Expected Result:** Pages are sequentially stamped: `EXHIBIT-A-000001`, `EXHIBIT-A-000002`, etc.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 52: `pdf_page_numbers` — Dynamic Header/Footer Page Numbering
- **UI Access:** Operations Dock > **Pages** > **Add Page Numbers**.
- **Test File:** Multi-page PDF.
- **Manual Steps:**
  1. Format: `Page {n} of {total}`. Position: `Bottom Center`. Click **Apply**.
- **Expected Result:** Clean sequential page numbering appears at the bottom of all pages.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 53: `pdf_header_footer` — Header & Footer Custom Text Margins
- **UI Access:** Operations Dock > **Edit** > **Header & Footer**.
- **Test File:** Any PDF.
- **Manual Steps:**
  1. Header: `Project Alpha Documentation`. Footer: `Proprietary & Confidential`. Click **Apply**.
- **Expected Result:** Consistent header and footer text are placed with correct top/bottom margins.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 54: `pdf_annotate` — Annotations (Highlighter, Sticky Notes, Callouts)
- **UI Access:** Top Annotation Toolbar.
- **Test File:** Any PDF.
- **Manual Steps:**
  1. Highlight a sentence in yellow. Place a sticky note with comment `"Verify date"`. Save document.
- **Expected Result:** Annotations render cleanly, note opens on click, and annotations persist on save.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 55: `pdf_redact` — Irreversible True Text Blackout
- **UI Access:** Operations Dock > **Security** > **Redact Text** (or toolbar Redaction tool).
- **Test File:** Any document with sensitive text.
- **Manual Steps:**
  1. Draw redaction box over sensitive line. Style: `Blackout`. Click **Apply Redactions**.
- **Expected Result:** Text is permanently eradicated from underlying streams; copy-paste yields no text.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 56: `json_to_pdf` — Synthesize PDF from Structured JSON Data
- **UI Access:** Operations Dock > **Convert** > **JSON to PDF**.
- **Test File:** Sample JSON file with document structure, paragraphs, and styles.
- **Manual Steps:**
  1. Load JSON file. Click **Synthesize PDF**.
- **Expected Result:** Generates a beautifully formatted PDF document matching JSON specification.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 57: `md_to_pdf` — Render Markdown Document into Styled PDF
- **UI Access:** Operations Dock > **Convert** > **Markdown to PDF**.
- **Test File:** `test.md` (or any markdown file with headers, lists, code blocks).
- **Manual Steps:**
  1. Load markdown file or type into editor drawer. Click **Generate PDF**.
- **Expected Result:** Produces a styled PDF with typography matching Markdown formatting.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 58: `html_to_pdf` — Convert HTML Code/URL into Formatted PDF
- **UI Access:** Operations Dock > **Convert** > **HTML to PDF**.
- **Test File:** `test.html` (or URL string).
- **Manual Steps:**
  1. Input HTML snippet or web URL. Click **Convert to PDF**.
- **Expected Result:** Renders a clean PDF page preserving CSS styling.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 59: `pdf_convert_html` — Convert Ingested HTML Strings to PDF
- **UI Access:** Command Palette (`Ctrl+K`) > `pdf_convert_html`.
- **Test File:** Raw HTML file.
- **Manual Steps:**
  1. Execute HTML string conversion.
- **Expected Result:** Converts string to a standardized PDF starting with `%PDF` header.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 60: `pdf_convert_markdown` — Parse & Style Raw Markdown to PDF
- **UI Access:** Command Palette (`Ctrl+K`) > `pdf_convert_markdown`.
- **Test File:** Raw `.md` text file.
- **Manual Steps:**
  1. Execute conversion.
- **Expected Result:** Outputs styled PDF document.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 61: `pdf_convert_excel` — Render Raw Spreadsheets to Paginated PDF
- **UI Access:** Operations Dock > **Convert** > **CSV to PDF** / `pdf_convert_excel`.
- **Test File:** `.xlsx` or `.csv` spreadsheet file.
- **Manual Steps:**
  1. Load spreadsheet file. Set grid lines: **On**. Click **Convert to PDF**.
- **Expected Result:** Tabular rows and columns are paginated cleanly onto PDF pages.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 62: `pdf_comic_archive` — Package PDF Pages into Comic Archive (.cbz)
- **UI Access:** Operations Dock > **Convert** > **Comic Archive Conversion**.
- **Test File:** Graphic novel or multi-page image PDF.
- **Manual Steps:**
  1. Set DPI: `150 DPI`. Click **Convert to Comic Archive (.cbz)**.
- **Expected Result:** Generates `.cbz` zip archive containing `page_001.png`, `page_002.png` with valid PK zip header.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 63: `pdf_to_epub` — Convert Fixed Layout PDF into Reflowable EPUB 3.0
- **UI Access:** Operations Dock > **Convert** > **PDF to EPUB 3.0**.
- **Test File:** Text-heavy eBook or whitepaper PDF.
- **Manual Steps:**
  1. Title: `My eBook`. Language: `en`. Click **Convert to EPUB**.
- **Expected Result:** Generates valid `.epub` eBook package with chapters and TOC.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 64: `pdf_to_pdf_a` — Archive-Ready PDF/A ISO Conformance
- **UI Access:** Operations Dock > **Convert** > **Convert to PDF/A**.
- **Test File:** Standard PDF.
- **Manual Steps:**
  1. Conformance level: `PDF/A-1b` or `PDF/A-2b`. Click **Convert**.
- **Expected Result:** Generates archive-compliant PDF with embedded fonts and device-independent color profiles.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 65: `extract_images` — Extract and Export All Embedded Raster Images
- **UI Access:** Operations Dock > **AI** > **Extract Embedded Images**.
- **Test File:** Document with photos, charts, or figures.
- **Manual Steps:**
  1. Click **Extract All Images** and select destination folder.
- **Expected Result:** All raster images are saved as individual PNG/JPEG files in output folder.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 66: `pdf_extract_images` — Filtered Image Extractor with DPI Scaling
- **UI Access:** Command Palette (`Ctrl+K`) > `pdf_extract_images`.
- **Test File:** Document with images.
- **Manual Steps:**
  1. Set minimum width/height filter: `100px`. Click **Extract**.
- **Expected Result:** Extracts all qualifying graphics while ignoring tiny 1x1 spacer images.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`
