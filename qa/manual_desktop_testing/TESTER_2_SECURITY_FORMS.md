# Manual Desktop UI Testing Guide: Tester 2 (30 Tools — Security, Cryptography, AcroForms, Batch, Markup & Synthesis)

**Assigned Tester:** Tester 2  
**Total Tools Assigned:** 30 Tools (Tools 31 to 60)  
**Domain:** Cryptographic Security, Digital Signatures, PII Scanning, AcroForms, CSV Batch Form Engine, Watermarking, Bates Numbering, True Redaction & Document Synthesis  
**Target Branch:** `feature/dev` / `feat/desktop-ui-manual-testing-matrix-4p`

---

## 🚀 Quick Setup & Test Fixtures (Run Once Before Testing)

All 30 tools can be tested using pre-existing real test fixtures from the repository.

### 1. Launch the Tauri Desktop Application
Open a terminal in the project root and start the desktop app:
```bash
cd apps/desktop
npm run tauri dev
```
*(Keep system RAM footprint light; close heavy unused browser tabs).*

### 2. One-Liner Fixture Setup Command
Run this one command in your terminal to create a clean `tester2_fixtures` folder on your Desktop with all required test files:
```bash
mkdir -p ~/Desktop/tester2_fixtures
cp tests/e2e_fixtures/real/multi_page.pdf ~/Desktop/tester2_fixtures/
cp tests/e2e_fixtures/real/single_page.pdf ~/Desktop/tester2_fixtures/
cp tests/e2e_fixtures/real/encrypted.pdf ~/Desktop/tester2_fixtures/
cp tests/e2e_fixtures/real/signed.pdf ~/Desktop/tester2_fixtures/
cp tests/e2e_fixtures/real/form.pdf ~/Desktop/tester2_fixtures/
cp tests/e2e_fixtures/real/form_data.json ~/Desktop/tester2_fixtures/
cp tests/e2e_fixtures/real/img1.png ~/Desktop/tester2_fixtures/
cp tests/e2e_fixtures/real/json_doc.json ~/Desktop/tester2_fixtures/
cp tests/e2e_fixtures/real/test.csv ~/Desktop/tester2_fixtures/
cp tests/e2e_fixtures/real/test.md ~/Desktop/tester2_fixtures/
cp tests/e2e_fixtures/real/test.html ~/Desktop/tester2_fixtures/
```

### 📋 Fixture-to-Tool Mapping Matrix

| Test Fixture File | Format / Purpose | Tools Covered |
| :--- | :--- | :--- |
| `~/Desktop/tester2_fixtures/encrypted.pdf` | Password-Encrypted PDF | **Tool 31** |
| `~/Desktop/tester2_fixtures/multi_page.pdf` | 5-Page Standard PDF | **Tools 32, 33, 34, 35, 36, 39, 40, 41, 42, 43, 50, 51, 52, 53, 54, 55** |
| `~/Desktop/tester2_fixtures/signed.pdf` | Cryptographically Signed PDF | **Tools 37, 38** |
| `~/Desktop/tester2_fixtures/form.pdf` & `form_data.json` | AcroForm Document & Field JSON | **Tools 44, 45, 46, 47, 48, 49** |
| `~/Desktop/tester2_fixtures/test.csv` | Tabular CSV Records for Batch Fill | **Tools 48, 49** |
| `~/Desktop/tester2_fixtures/json_doc.json`, `test.md`, `test.html` | Structured JSON, Markdown & HTML | **Tools 56, 57, 58, 59, 60** |

---

## 🛠️ Step-by-Step Testing Guide (Tools 31 to 60)

---

### Tool 31: `pdf_decrypt` — Remove Password Restrictions
- **UI Access:** Operations Dock > **Security** > **Remove Password**.
- **Test File:** `~/Desktop/tester2_fixtures/encrypted.pdf` (or a file encrypted with password `pass123`).
- **Manual Steps:**
  1. Open encrypted document and unlock it.
  2. Click **Remove Password** and confirm password.
- **Expected Result:** Document re-saves unencrypted; subsequent opens require no password.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 32: `pdf_sanitize` — Deep Enterprise Sanitization
- **UI Access:** Operations Dock > **Security** > **Sanitize Document**.
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Check **Strip JavaScript**, **Remove Private Data**, **Clean Metadata**. Click **Sanitize**.
- **Expected Result:** Potentially malicious actions, scripts, and tracking dictionaries are stripped without breaking layout.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 33: `pdf_metadata` — Inspect & Clean Metadata
- **UI Access:** Operations Dock > **Security** > **Edit / Sanitize Metadata** (or *File > Properties*).
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Change Title to: `Verified Release 2026`. Click **Save Metadata**.
- **Expected Result:** Document metadata dictionary updates cleanly; reopening Properties confirms persisted title.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 34: `pdf_change_permissions` — Configure PDF Permissions
- **UI Access:** Operations Dock > **Security** > **Modify PDF Permissions**.
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Disable **Content Copying**. Enable **Form Filling**. Set owner password. Click **Apply**.
- **Expected Result:** Permissions bitmask updates and enforces restrictions in compliant PDF viewers.
- **Sign-off:** `[ ] PASS` | `[ x ] FAIL` | Notes: `____________________`

---

### Tool 35: `pdf_timestamp` — RFC 3161 TSA Timestamping
- **UI Access:** Operations Dock > **Security** > **Cryptographic Timestamp TSA**.
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Click **Apply Timestamp Token**.
- **Expected Result:** A cryptographically verified TSA timestamp token is embedded into the document signature dictionary.
- **Sign-off:** `[ ] PASS` | `[ x ] FAIL` | Notes: `____________________`

---

### Tool 36: `pdf_sign` — Digital Certificate & Visual Pen Signing
- **UI Access:** Operations Dock > **Security** > **Sign Document** (or toolbar Pen tool).
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Draw visual signature or select certificate signing mode. Place signature on Page 1.
  2. Click **Apply Signature**.
- **Expected Result:** Signature appearance and cryptographic dictionary are embedded on Page 1.
- **Sign-off:** `[ ] PASS` | `[ x ] FAIL` | Notes: `____________________`

---

### Tool 37: `pdf_validate_signatures` — Verify Signatures & Cert Chains
- **UI Access:** Operations Dock > **Security** > **Validate Digital Signatures**.
- **Test File:** `~/Desktop/tester2_fixtures/signed.pdf`.
- **Manual Steps:**
  1. Open `signed.pdf`. Click **Validate Signatures**.
- **Expected Result:** Drawer shows verification report with signature status, signer identity, and integrity check.
- **Sign-off:** `[ ] PASS` | `[ x ] FAIL` | Notes: `____________________`

---

### Tool 38: `pdf_remove_signatures` — Strip Digital Signatures
- **UI Access:** Operations Dock > **Security** > **Remove Digital Signatures**.
- **Test File:** `~/Desktop/tester2_fixtures/signed.pdf`.
- **Manual Steps:**
  1. Open `signed.pdf`. Click **Strip Digital Signatures**.
- **Expected Result:** Signature fields and cryptographic locks are removed, unlocking the document for editing.
- **Sign-off:** `[ ] PASS` | `[ x ] FAIL` | Notes: `____________________`

---

### Tool 39: `pdf_hash` — Cryptographic Checksums
- **UI Access:** Operations Dock > **Security** > **Generate Hash**.
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Select `SHA-256` (or `SHA-512` / `MD5`). Click **Generate Hash**.
- **Expected Result:** Drawer displays cryptographic digest string with a one-click copy button.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 40: `pdf_validate` — PDF Conformance & Syntax Validation
- **UI Access:** Operations Dock > **Security** > **Validate PDF**.
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Click **Run Conformance Validation**.
- **Expected Result:** Reports validation status (e.g. *Structure Valid, PDF 1.7 compliant, 0 fatal errors*).
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 41: `document_pii_scan` — Detect SSNs, Credit Cards & PII
- **UI Access:** Operations Dock > **Security** > **PII Scanner**.
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Click **Scan Document for PII**.
- **Expected Result:** Drawer completes scan and lists any detected sensitive entities (or confirms 0 PII entities detected) with page numbers and confidence scores.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 42: `pdf_overlay_underlay` — Overlay & Underlay Stationary
- **UI Access:** Operations Dock > **Edit** > **Overlay & Underlay Stationary**.
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf` + `single_page.pdf` as stationary template.
- **Manual Steps:**
  1. Select `single_page.pdf` as template. Select **Underlay (Background)**. Click **Apply**.
- **Expected Result:** Template artwork renders behind the document text across pages.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 43: `pdf_stamp` — Stamp Coordinate-Exact Images, Logos & Badges
- **UI Access:** Operations Dock > **Edit** > **Stamp Image & Logo**.
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf` + `img1.png`.
- **Manual Steps:**
  1. Choose `img1.png`. Set coordinates `X: 450, Y: 700`. Click **Place Stamp**.
- **Expected Result:** Image stamp renders at the exact designated position on the page.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 44: `pdf_flatten` — Bake Annotations & Forms Permanently
- **UI Access:** Operations Dock > **Edit** > **Flatten Annotations**.
- **Test File:** `~/Desktop/tester2_fixtures/form.pdf`.
- **Manual Steps:**
  1. Open `form.pdf`. Click **Flatten Document**.
- **Expected Result:** Interactive AcroForm inputs and annotations are baked into non-interactive page content streams.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 45: `pdf_read_form` — Extract AcroForm Field Values to JSON
- **UI Access:** Operations Dock > **Edit** > **Read Form Data**.
- **Test File:** `~/Desktop/tester2_fixtures/form.pdf`.
- **Manual Steps:**
  1. Open `form.pdf` in viewer. Click **Read Form Fields**.
- **Expected Result:** JSON drawer displays all form field names, field types, and current values.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 46: `pdf_fill_form` — Populate AcroForms from JSON or Direct Typing
- **UI Access:** Direct on-canvas clicking, or Operations Dock > **Edit** > **Fill Form Data**.
- **Test File:** `~/Desktop/tester2_fixtures/form.pdf` + `form_data.json`.
- **Manual Steps:**
  1. Click into field on canvas and type `Alice Smith`, or import `form_data.json`.
  2. Click **Apply / Save Filled Form**.
- **Expected Result:** Values populate into the form widgets and persist when saving and reopening.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 47: `pdf_create_form_field` — Interactive Form Field Builder
- **UI Access:** Operations Dock > **Edit** > **Add Form Field**.
- **Test File:** `~/Desktop/tester2_fixtures/single_page.pdf`.
- **Manual Steps:**
  1. Select **Text Field**, field name: `account_number`. Set bounding box coordinates or drag on canvas. Click **Create Field**.
- **Expected Result:** A new interactive AcroForm text field is added to the document and appears in `pdf_read_form`.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 48: `batch_fill` — Multi-Record CSV Batch Form Generator
- **UI Access:** Operations Dock > **Edit** > **Batch Form Fill**.
- **Test File:** `~/Desktop/tester2_fixtures/form.pdf` + `test.csv`.
- **Manual Steps:**
  1. Load template `form.pdf` and `test.csv`.
  2. Click **Generate Batch PDFs**.
- **Expected Result:** Generates filled PDFs from CSV rows along with a batch completion summary.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 49: `cli_fill_batch` — Automated Batch Engine & Receipt Generation
- **UI Access:** Command Palette (`Ctrl+K`) > `cli_fill_batch` (or Batch Automation runner).
- **Test File:** `~/Desktop/tester2_fixtures/form.pdf` + `test.csv`.
- **Manual Steps:**
  1. Trigger batch run with output directory specified.
- **Expected Result:** Rows are filled with detailed execution audit receipts logged.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 50: `pdf_watermark` — Semi-Transparent Text & Diagonal Branding Watermark
- **UI Access:** Operations Dock > **Edit** > **Add Watermark**.
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Text: `CONFIDENTIAL DRAFT`. Opacity: `20%`. Angle: `45° Diagonal`. Click **Apply**.
- **Expected Result:** Semi-transparent watermark text appears across all pages without obscuring underlying text.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 51: `pdf_bates` — Legal Bates Numbering Sequence Stamping
- **UI Access:** Operations Dock > **Edit** > **Bates Numbering**.
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Prefix: `EXHIBIT-A-`. Start Number: `1`. Digits: `6`. Position: `Bottom Right`. Click **Apply**.
- **Expected Result:** Pages are sequentially stamped: `EXHIBIT-A-000001`, `EXHIBIT-A-000002`, etc.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 52: `pdf_page_numbers` — Dynamic Header/Footer Page Numbering
- **UI Access:** Operations Dock > **Pages** > **Add Page Numbers**.
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Format: `Page {n} of {total}`. Position: `Bottom Center`. Click **Apply**.
- **Expected Result:** Clean sequential page numbering appears at the bottom center of all 5 pages.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 53: `pdf_header_footer` — Header & Footer Custom Text Margins
- **UI Access:** Operations Dock > **Edit** > **Header & Footer**.
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Header: `Project Alpha Documentation`. Footer: `Proprietary & Confidential`. Click **Apply**.
- **Expected Result:** Consistent header and footer text are placed with clean top/bottom margins.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 54: `pdf_annotate` — Annotations (Highlighter, Sticky Notes, Callouts)
- **UI Access:** Top Annotation Toolbar or Operations Dock > **Edit** > **Annotate**.
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Highlight a sentence in yellow. Place a sticky note with comment `"Verify date"`. Save document.
- **Expected Result:** Annotations render cleanly on canvas and persist on save.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 55: `pdf_redact` — Irreversible True Text Blackout
- **UI Access:** Operations Dock > **Security** > **Redact Text** (or toolbar Redaction tool).
- **Test File:** `~/Desktop/tester2_fixtures/multi_page.pdf`.
- **Manual Steps:**
  1. Draw redaction box or specify text pattern to redact. Style: `Blackout`. Click **Apply Redactions**.
- **Expected Result:** Target text is permanently eradicated from underlying content streams; copy-paste yields no redacted text.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 56: `json_to_pdf` — Synthesize PDF from Structured JSON Data
- **UI Access:** Operations Dock > **Convert** > **JSON to PDF**.
- **Test File:** `~/Desktop/tester2_fixtures/json_doc.json`.
- **Manual Steps:**
  1. Load `json_doc.json`. Click **Synthesize PDF**.
- **Expected Result:** Generates a formatted PDF document matching the JSON structure.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 57: `md_to_pdf` — Render Markdown Document into Styled PDF
- **UI Access:** Operations Dock > **Convert** > **Markdown to PDF**.
- **Test File:** `~/Desktop/tester2_fixtures/test.md`.
- **Manual Steps:**
  1. Load `test.md` or type Markdown into editor drawer. Click **Generate PDF**.
- **Expected Result:** Produces a styled PDF with typography matching Markdown formatting.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 58: `html_to_pdf` — Convert HTML Code/URL into Formatted PDF
- **UI Access:** Operations Dock > **Convert** > **HTML to PDF**.
- **Test File:** `~/Desktop/tester2_fixtures/test.html`.
- **Manual Steps:**
  1. Load `test.html` or input HTML snippet. Click **Convert to PDF**.
- **Expected Result:** Renders a clean PDF page preserving HTML structure and styling.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 59: `pdf_convert_html` — Convert Ingested HTML Strings to PDF
- **UI Access:** Command Palette (`Ctrl+K`) > `pdf_convert_html`.
- **Test File:** `~/Desktop/tester2_fixtures/test.html`.
- **Manual Steps:**
  1. Select `test.html` and execute HTML conversion.
- **Expected Result:** Outputs a valid PDF starting with `%PDF` header.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 60: `pdf_convert_markdown` — Parse & Style Raw Markdown to PDF
- **UI Access:** Command Palette (`Ctrl+K`) > `pdf_convert_markdown`.
- **Test File:** `~/Desktop/tester2_fixtures/test.md`.
- **Manual Steps:**
  1. Select `test.md` and execute conversion.
- **Expected Result:** Outputs a valid styled PDF document.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

## 🏁 Summary Sign-Off Scorecard: Tester 2 (30 Tools)

| Tool # | Tool Identifier | UI Location | Test Input File | Status | Notes |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **31** | `pdf_decrypt` | Operations Dock > Security | `encrypted.pdf` | `[ ] PASS` | |
| **32** | `pdf_sanitize` | Operations Dock > Security | `multi_page.pdf` | `[ ] PASS` | |
| **33** | `pdf_metadata` | Operations Dock > Security | `multi_page.pdf` | `[ ] PASS` | |
| **34** | `pdf_change_permissions` | Operations Dock > Security | `multi_page.pdf` | `[ ] PASS` | |
| **35** | `pdf_timestamp` | Operations Dock > Security | `multi_page.pdf` | `[ ] PASS` | |
| **36** | `pdf_sign` | Operations Dock > Security | `multi_page.pdf` | `[ ] PASS` | |
| **37** | `pdf_validate_signatures` | Operations Dock > Security | `signed.pdf` | `[ ] PASS` | |
| **38** | `pdf_remove_signatures` | Operations Dock > Security | `signed.pdf` | `[ ] PASS` | |
| **39** | `pdf_hash` | Operations Dock > Security | `multi_page.pdf` | `[ ] PASS` | |
| **40** | `pdf_validate` | Operations Dock > Security | `multi_page.pdf` | `[ ] PASS` | |
| **41** | `document_pii_scan` | Operations Dock > Security | `multi_page.pdf` | `[ ] PASS` | |
| **42** | `pdf_overlay_underlay` | Operations Dock > Edit | `multi_page.pdf`, `single_page.pdf` | `[ ] PASS` | |
| **43** | `pdf_stamp` | Operations Dock > Edit | `multi_page.pdf`, `img1.png` | `[ ] PASS` | |
| **44** | `pdf_flatten` | Operations Dock > Edit | `form.pdf` | `[ ] PASS` | |
| **45** | `pdf_read_form` | Operations Dock > Edit | `form.pdf` | `[ ] PASS` | |
| **46** | `pdf_fill_form` | Operations Dock > Edit | `form.pdf`, `form_data.json` | `[ ] PASS` | |
| **47** | `pdf_create_form_field` | Operations Dock > Edit | `single_page.pdf` | `[ ] PASS` | |
| **48** | `batch_fill` | Operations Dock > Edit | `form.pdf`, `test.csv` | `[ ] PASS` | |
| **49** | `cli_fill_batch` | Command Palette (`Ctrl+K`) | `form.pdf`, `test.csv` | `[ ] PASS` | |
| **50** | `pdf_watermark` | Operations Dock > Edit | `multi_page.pdf` | `[ ] PASS` | |
| **51** | `pdf_bates` | Operations Dock > Edit | `multi_page.pdf` | `[ ] PASS` | |
| **52** | `pdf_page_numbers` | Operations Dock > Pages | `multi_page.pdf` | `[ ] PASS` | |
| **53** | `pdf_header_footer` | Operations Dock > Edit | `multi_page.pdf` | `[ ] PASS` | |
| **54** | `pdf_annotate` | Top Annotation Toolbar | `multi_page.pdf` | `[ ] PASS` | |
| **55** | `pdf_redact` | Operations Dock > Security | `multi_page.pdf` | `[ ] PASS` | |
| **56** | `json_to_pdf` | Operations Dock > Convert | `json_doc.json` | `[ ] PASS` | |
| **57** | `md_to_pdf` | Operations Dock > Convert | `test.md` | `[ ] PASS` | |
| **58** | `html_to_pdf` | Operations Dock > Convert | `test.html` | `[ ] PASS` | |
| **59** | `pdf_convert_html` | Command Palette (`Ctrl+K`) | `test.html` | `[ ] PASS` | |
| **60** | `pdf_convert_markdown` | Command Palette (`Ctrl+K`) | `test.md` | `[ ] PASS` | |

**Tester Signature:** ___________________________  
**Date of Testing:** ___________________________  
**Overall Result:** `[ ] FULL PASS (30/30)` | `[ ] ISSUES DETECTED`

