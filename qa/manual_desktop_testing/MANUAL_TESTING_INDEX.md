# PaperPilot Desktop UI Manual Testing Master Matrix

Welcome to the **PaperPilot Desktop UI Manual Testing Campaign**. This testing suite divides all **52 PaperPilot tools and features** across **4 testers** to guarantee 100% human-verified quality, aesthetic excellence, and rock-solid stability before general release.

---

## 📋 Campaign Overview & Allocation

| Tester Assignment | Document Link | Focus Domain | Tool Count | Status |
| :--- | :--- | :--- | :--- | :--- |
| **Tester 1** | [TESTER_1_PAGE_OPERATIONS.md](file:///home/krushna/Project/PaperPilot/qa/manual_desktop_testing/TESTER_1_PAGE_OPERATIONS.md) | Page Operations, Navigation & Document Assembly | 13 Tools | `[ ] Ready` |
| **Tester 2** | [TESTER_2_IMPOSITION_OPTIMIZATION.md](file:///home/krushna/Project/PaperPilot/qa/manual_desktop_testing/TESTER_2_IMPOSITION_OPTIMIZATION.md) | Imposition, Multi-Sheet, Optimization & Rendering | 13 Tools | `[ ] Ready` |
| **Tester 3** | [TESTER_3_SECURITY_FORMS.md](file:///home/krushna/Project/PaperPilot/qa/manual_desktop_testing/TESTER_3_SECURITY_FORMS.md) | Security, Cryptography, Signatures & AcroForms | 13 Tools | `[ ] Ready` |
| **Tester 4** | [TESTER_4_CONVERSIONS_INTELLIGENCE.md](file:///home/krushna/Project/PaperPilot/qa/manual_desktop_testing/TESTER_4_CONVERSIONS_INTELLIGENCE.md) | Conversions, Office Modernization, AI & Intelligence | 13 Tools | `[ ] Ready` |

---

## 🛠️ Tester Environment Setup

### 1. Launching the Desktop Application
Ensure the Tauri desktop app is running in development or release mode:
```bash
# Terminal 1: Run desktop app
cd apps/desktop
npm run tauri dev
```
*Alternatively, launch the packaged binary `target/release/paperpilot-desktop`.*

### 2. Test Fixtures Directory
All test input files are conveniently located in the repository under:
- `tests/fixtures/simple.pdf` (Clean 1-page sample document)
- `tests/fixtures/multipage.pdf` or `tests/e2e_fixtures/real/multi_page.pdf` (Multi-page document for page ops)
- `tests/fixtures/sample_form.pdf` (Interactive form fields)
- `tests/fixtures/scanned_doc.pdf` (Scanned image document for OCR)
- `tests/fixtures/sample.docx`, `test.xlsx`, `test.html`, `test.md` (Conversion inputs)

If you need a new blank document, use **File > New Document** or drop any test PDF onto the application window.

---

## 🧭 How to Find Tools in the Desktop UI

1. **Operations Dock (Right Drawer)**:
   - Click the **Wrench/Toolbox Icon** (`🛠️`) on the right sidebar rail to expand the Operations Panel.
   - Use the category pills: **All**, **Quick**, **Pages**, **Edit**, **Optimize**, **Security**, **Convert**, **AI**.
   - Use the live search bar at the top of the Operations Panel to filter by name or tag.
2. **Instant Open Palette (`Ctrl+K` or `Cmd+K`)**:
   - Press `Ctrl+K` anywhere in the app to open the floating command search palette. Type the tool name and press `Enter` to activate.
3. **Thumbnail Sidebar Context Menu**:
   - In the left sidebar thumbnail view, right-click on any page thumbnail to access page-level actions (`Rotate`, `Delete`, `Extract`, `Crop`).
4. **Main Native Menu**:
   - Top menu bar items (`File`, `Edit`, `View`, `Tools`, `Help`).

---

## 📊 Scorecard & Progress Sign-Off Matrix

### Tester 1: Page Operations & Assembly
- [ ] `merge` — Merge Documents
- [ ] `split` — Split PDF
- [ ] `rotate` — Rotate Pages
- [ ] `delete_pages` — Delete Pages
- [ ] `reorder_pages` — Reorder Pages
- [ ] `extract_pages` — Extract Pages
- [ ] `burst` — Burst PDF
- [ ] `crop` — Crop Pages
- [ ] `remove_blank` — Remove Blank Pages
- [ ] `pdf_merge_alternate` — Alternating PDF Merge
- [ ] `pdf_split_by_outline` — Split by Outline Bookmarks
- [ ] `pdf_split_scanned_book` — Split Scanned Book Spine
- [ ] `pdf_auto_rotate` — Auto Upright Orientation

### Tester 2: Imposition, Optimization & Rendering
- [ ] `compress` — Compress PDF
- [ ] `repair` — Repair PDF
- [ ] `linearize` — Fast Web View
- [ ] `pdf_dedup_streams` — Lossless Stream Deduplication
- [ ] `pdf_scale_pages` — Scale Page Dimensions
- [ ] `pdf_nup` — N-Up Page Imposition
- [ ] `pdf_booklet` — Booklet Imposition
- [ ] `pdf_posterize` — Multi-Sheet Poster Slicer
- [ ] `pdf_stitch_canvas` — Stitch Continuous Canvas
- [ ] `pdf_invert_colors` — Invert Colors (Dark Mode)
- [ ] `pdf_attachments` — PDF Portfolio Attachments
- [ ] `compare` — Compare Documents (Pixel Diff Slider)
- [ ] `render` — Render Page to Image

### Tester 3: Security, Digital Signatures & Forms
- [ ] `encrypt` — Encrypt with Password
- [ ] `decrypt` — Remove Password
- [ ] `metadata` — Inspect / Sanitize Metadata
- [ ] `pdf_sanitize` — Deep Sanitize Document
- [ ] `redact` — Redact Text
- [ ] `sign` — Sign Document
- [ ] `pdf_validate_signatures` — Validate Signatures
- [ ] `pdf_remove_signatures` — Remove Signatures
- [ ] `pdf_change_permissions` — Modify Permissions
- [ ] `pdf_timestamp` — Cryptographic Timestamp TSA
- [ ] `read_form` — Read Form Data
- [ ] `fill_form` — Fill Form Data
- [ ] `create_form_field` — Add Form Field

### Tester 4: Conversions, Markup, Office & Intelligence
- [ ] `watermark` — Add Watermark
- [ ] `bates` — Bates Numbering
- [ ] `page_numbers` — Add Page Numbers
- [ ] `header_footer` — Header & Footer
- [ ] `annotate` — Add Annotations
- [ ] `flatten` — Flatten Annotations
- [ ] `pdf_stamp` — Stamp Image & Logo
- [ ] `pdf_overlay_underlay` — Overlay & Underlay Stationary
- [ ] `pdf_to_docx` — PDF to Word (DOCX)
- [ ] `pdf_to_xlsx` — PDF to Excel (XLSX)
- [ ] `pdf_to_pptx` — PDF to PowerPoint (PPTX)
- [ ] `pdf_to_html` — PDF to HTML
- [ ] `pdf_to_markdown` — PDF to Markdown

---

## 🛑 Bug Logging Protocol
If a tool fails during manual testing:
1. Capture a screenshot (`Alt + PrintScreen`) or note the exact error toast text.
2. Note the console log (`F12` / Inspect Element in desktop app).
3. Record the input file and parameters used in the corresponding tester markdown file under **Notes / Findings**.
4. Mark the tool as `[x] ❌ FAIL` and notify the team.
