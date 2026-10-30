# Manual Desktop UI Testing Guide: Tester 4 (Conversions, Office & AI Intelligence)

**Assigned Tester:** Tester 4  
**Total Core Tools Assigned:** 13 Core Tools (+ Multi-Format Conversion & AI Additions)  
**Domain:** Document Markup, Watermarking, Office Conversions, Comic Packaging & AI Search

---

## Tool 1: Add Watermark (`watermark`)
- **Category in UI:** `Edit & Markup`
- **UI Access:** Operations Dock > **Edit** > **Add Watermark** (or toolbar Watermark icon).
- **Sample File:** Any PDF document.
- **Manual Steps:**
  1. Enter watermark text: `CONFIDENTIAL - DO NOT COPY`.
  2. Adjust parameters:
     - Opacity slider: `25%`
     - Rotation: `Diagonal (45°)`
     - Color: `Red` or `Dark Gray`
     - Layer: `Foreground` or `Background`
  3. Click **Apply Watermark**.
- **Visual Acceptance Criteria:**
  - [ ] Semi-transparent watermark text renders across all pages.
  - [ ] Page text remains clearly legible through the watermark.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 2: Bates Numbering (`bates`)
- **Category in UI:** `Edit & Markup`
- **UI Access:** Operations Dock > **Edit** > **Bates Numbering**.
- **Sample File:** Multi-page legal document.
- **Manual Steps:**
  1. Configure Bates template:
     - Prefix: `CASE-2026-`
     - Start Number: `1`
     - Digits / Zero-padding: `6` (e.g. `000001`)
     - Suffix: `-CONF`
     - Position: **Bottom Right**
  2. Click **Apply Bates Stamps**.
- **Visual Acceptance Criteria:**
  - [ ] Each page features sequential Bates stamping: `CASE-2026-000001-CONF`, `CASE-2026-000002-CONF`.
  - [ ] Stamps do not obscure underlying page content.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 3: Add Page Numbers (`page_numbers`)
- **Category in UI:** `Page Management` / `Edit & Markup`
- **UI Access:** Operations Dock > **Pages** > **Add Page Numbers**.
- **Sample File:** Multi-page document.
- **Manual Steps:**
  1. Select format: **Page {n} of {total}** or **Simple {n}**.
  2. Select position: **Bottom Center**.
  3. Set font size: `10 pt`.
  4. Click **Apply Page Numbers**.
- **Visual Acceptance Criteria:**
  - [ ] Sequential page numbering appears centered at the bottom of every page (e.g., *Page 1 of 5*).
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 4: Header & Footer (`header_footer`)
- **Category in UI:** `Edit & Markup`
- **UI Access:** Operations Dock > **Edit** > **Header & Footer**.
- **Sample File:** Any PDF.
- **Manual Steps:**
  1. Enter Header Left text: `Quarterly Financial Report`.
  2. Enter Header Right text: `Q3 2026`.
  3. Enter Footer Center text: `PaperPilot Enterprise Documentation`.
  4. Set top margin: `36 pt`; bottom margin: `36 pt`.
  5. Click **Apply Header & Footer**.
- **Visual Acceptance Criteria:**
  - [ ] Margins and aligned text appear consistently on all designated pages.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 5: Add Annotations (`annotate`)
- **Category in UI:** `Edit & Markup`
- **UI Access:** Top Annotation Toolbar (Highlighter, Pen, Text Note, Rectangle, Arrow).
- **Sample File:** Any PDF.
- **Manual Steps:**
  1. Select the **Highlighter tool** (`🖍️`) and drag across a sentence.
  2. Select the **Sticky Note tool** (`💬`), click on the page, and type: `"Review this section carefully"`.
  3. Draw a red rectangle box around an image.
  4. Save the document.
- **Visual Acceptance Criteria:**
  - [ ] Highlighting, notes, and shapes render with crisp vector lines.
  - [ ] Notes expand when clicked to display author comments.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 6: Flatten Annotations (`flatten`)
- **Category in UI:** `Edit & Markup`
- **UI Access:** Operations Dock > **Edit** > **Flatten Annotations**.
- **Sample File:** Document with annotations, form fields, or signatures.
- **Manual Steps:**
  1. Click **Flatten Document**.
- **Visual Acceptance Criteria:**
  - [ ] Interactive form inputs, notes, and drawings are baked directly into the static page graphics layer.
  - [ ] Form fields can no longer be edited or selected individually.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 7: Stamp Image & Logo (`pdf_stamp`)
- **Category in UI:** `Edit & Markup`
- **UI Access:** Operations Dock > **Edit** > **Stamp Image & Logo**.
- **Sample File:** Any PDF and a sample PNG logo (`tests/fixtures/` or desktop image).
- **Manual Steps:**
  1. Choose an image file (`.png`, `.jpg`, `.svg`).
  2. Set coordinates: `X: 400`, `Y: 50`, `Width: 120`, `Height: 40`.
  3. Select target page: `Page 1`.
  4. Click **Place Stamp**.
- **Visual Acceptance Criteria:**
  - [ ] Image/logo renders crisply at the designated coordinates.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 8: Overlay & Underlay Stationary (`pdf_overlay_underlay`)
- **Category in UI:** `Edit & Markup`
- **UI Access:** Operations Dock > **Edit** > **Overlay & Underlay Stationary**.
- **Sample File:** A target document and a letterhead template PDF.
- **Manual Steps:**
  1. Select stationary document.
  2. Choose placement mode: **Underlay (Background template)** or **Overlay (Foreground stamp)**.
  3. Click **Apply Stationary**.
- **Visual Acceptance Criteria:**
  - [ ] Background letterhead graphics appear behind text across all pages.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 9: PDF to Word DOCX (`pdf_to_docx`)
- **Category in UI:** `Convert & Export`
- **UI Access:** Operations Dock > **Convert** > **PDF to Word (DOCX)**.
- **Sample File:** Text-based document with headings and paragraphs.
- **Manual Steps:**
  1. Click **Export as Word (DOCX)**.
  2. Select output file name.
- **Visual Acceptance Criteria:**
  - [ ] Creates a valid OpenXML `.docx` file (PK zip header).
  - [ ] Opening the file in LibreOffice/Word verifies paragraphs, headings, and bold styling are preserved.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 10: PDF to Excel XLSX (`pdf_to_xlsx`)
- **Category in UI:** `Convert & Export`
- **UI Access:** Operations Dock > **Convert** > **PDF to Excel (XLSX)**.
- **Sample File:** Document containing data tables or financial figures.
- **Manual Steps:**
  1. Click **Extract Tables to Excel**.
  2. Select output location.
- **Visual Acceptance Criteria:**
  - [ ] Produces a valid `.xlsx` spreadsheet.
  - [ ] Numbers and column headers populate into distinct spreadsheet cells.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 11: PDF to PowerPoint PPTX (`pdf_to_pptx`)
- **Category in UI:** `Convert & Export`
- **UI Access:** Operations Dock > **Convert** > **PDF to PowerPoint (PPTX)**.
- **Sample File:** Landscape presentation slide PDF.
- **Manual Steps:**
  1. Click **Export as PPTX**.
- **Visual Acceptance Criteria:**
  - [ ] Generates `.pptx` presentation file where each PDF page becomes a presentation slide.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 12: PDF to HTML5 (`pdf_to_html`)
- **Category in UI:** `Convert & Export`
- **UI Access:** Operations Dock > **Convert** > **PDF to HTML**.
- **Sample File:** Any multi-paragraph PDF.
- **Manual Steps:**
  1. Click **Convert to HTML**.
- **Visual Acceptance Criteria:**
  - [ ] Generates an HTML5 document with CSS styling.
  - [ ] Opening in a browser displays readable, responsive text matching the document layout.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 13: PDF to Markdown (`pdf_to_markdown`)
- **Category in UI:** `Convert & Export`
- **UI Access:** Operations Dock > **Convert** > **PDF to Markdown** (or LLM Export button).
- **Sample File:** Document with headings, lists, and tables.
- **Manual Steps:**
  1. Click **Convert to Markdown**.
  2. View preview drawer or save to `.md`.
- **Visual Acceptance Criteria:**
  - [ ] Outputs clean GitHub-flavored markdown with `# Headings`, `- Lists`, and `| Tables |`.
  - [ ] Ready for copy-pasting into LLMs or documentation.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## 🌟 Bonus: Packaging & AI Intelligence Tools

### Tool 14: Comic Book Archive (`pdf_comic_archive`)
- **UI Access:** Operations Dock > **Convert** > **Comic Archive Conversion**.
- **Steps:** Click Convert to CBZ. Output file has `.cbz` extension containing sequentially numbered PNG frames (`page_001.png`, `page_002.png`).
- **Status:** `[ ] PASS` | `[ ] FAIL`

### Tool 15: OCR Text Recognition (`ocr`)
- **UI Access:** Operations Dock > **AI** > **OCR Text Recognition**.
- **Steps:** Open scanned PDF. Click Run OCR. Text is extracted from image into selectable text layer or `.txt` file.
- **Status:** `[ ] PASS` | `[ ] FAIL`

### Tool 16: PII Scanner (`document_pii_scan`)
- **UI Access:** Operations Dock > **Security** > **PII Scanner**.
- **Steps:** Click Scan for PII. Inspect list of detected SSNs, credit cards, or email addresses with page locations.
- **Status:** `[ ] PASS` | `[ ] FAIL`

### Tool 17: Workspace Multi-Doc Search (`workspace_search`)
- **UI Access:** Operations Dock > **AI** > **Workspace Search**.
- **Steps:** Enter query keyword and select project directory. Results list matching documents and page numbers without errors.
- **Status:** `[ ] PASS` | `[ ] FAIL`
