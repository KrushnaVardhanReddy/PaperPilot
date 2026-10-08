# Spec 031: Real Independent Semantic Assertions & Fixtures Suite (Tri-Interface E2E)

## Status: APPROVED / READY FOR IMPLEMENTATION

## 1. Problem Statement
The current Tri-Interface E2E test script (`scripts/test_tri_interface_e2e.py`) relies primarily on shallow verification:
- Checks if the process exits with code 0.
- Checks if the output file is non-zero bytes.
- Uses naive regex `len(re.findall(rb'/Type\s*/Page\b', data))` to estimate page count.

As documented in `RealAssertion.md`, shallow verification does NOT guarantee semantic correctness:
- A redacted PDF could still leak target text in raw streams.
- A rotated PDF might not set `/Rotate 90` or may scramble text.
- An encrypted PDF might remain readable if encryption fails silently.
- A converted DOCX or XLSX could produce empty/dummy containers.

## 2. Objective & Scope (TEST ONLY — NO BUG FIXES)
Enhance the test framework in `scripts/test_tri_interface_e2e.py` and provide realistic test fixtures to enforce **independent 3rd-party semantic assertions** across all 44 tools and all 3 developer interfaces (CLI, MCP, REST API = 132 tests), strictly following the specifications in `RealAssertion.md`.

**CRITICAL RULE**: This is a test verification task. **DO NOT MODIFY APPLICATION CODE** (`paperpilot-core/`, `paperpilot-pdf/`, `paperpilot-cli/`, `paperpilot-mcp/`, `paperpilot-gateway/`, `apps/`). If an existing tool produces output that fails an assertion, the test harness must record the exact failure in the report without modifying the underlying engine.

## 3. Architecture & Requirements

### A. Realistic Fixtures Generation (`tests/e2e_fixtures/real/`)
Create or generate the realistic test corpus required by `RealAssertion.md`:
1. `multi_page.pdf`: 5 pages, each with distinct text marker ("P1", "P2", "P3", "P4", "P5").
2. `invoice_text.pdf`: 3 pages with structured text, tables, and known search strings.
3. `with_blank.pdf`: Multi-page document containing text pages, blank pages, and near-blank pages.
4. `redact_test.pdf`: Document containing exact target strings (e.g., "SECRET 12345") at known coordinates.
5. `form.pdf`: AcroForm with text input, checkbox, radio button, and dropdown.
6. `encrypted.pdf`: Password-protected document with known user/owner passwords.
7. `image_sample.png`: 800x600 test image for `pdf_images_to_pdf`.

### B. Independent Python Verification Library
The test runner must use standard Python parsing libraries to inspect files independently:
- `pypdf` (or `pypdf`/`pdfplumber` for text extraction, `/Rotate` validation, encryption status, form fields, AcroForm verification).
- `openpyxl` for Excel conversions (`pdf_to_xlsx`).
- `python-docx` for Word conversions (`pdf_to_docx`).
- `python-pptx` for PowerPoint conversions (`pdf_to_pptx`).
- Python `hashlib` for SHA-256 validation.

### C. Complete Semantic Assertions Matrix for All 44 Tools
1. **Page Operations**:
   - `pdf_merge`: Page count = sum of inputs; page 1 contains first file text, page 2 contains second file text; merging 3+ files works; order preserved.
   - `pdf_split`: Page count matches requested split range; text matches expected page markers ("P1".."P5"); invalid range returns error.
   - `pdf_extract_pages`: Output contains exactly extracted pages; original file untouched; out-of-range errors.
   - `pdf_delete_pages`: Target page text is absent; remaining pages preserved; deleting all pages returns error.
   - `pdf_reorder_pages`: Pages follow the requested permutation (e.g. P2, P1, P3, P4, P5); duplicate/missing indices error.
   - `pdf_rotate`: Target page has `/Rotate` = 90; text remains extractable; rotating twice gives 180; invalid angle errors.
   - `pdf_crop`: MediaBox/CropBox dimensions equal requested crop rect; page count unchanged.
   - `pdf_burst`: Produces 1-page individual PDFs sorted in page order matching page count.
   - `pdf_remove_blank`: Blank page is omitted; near-blank handled; text pages retained in order; CLI and MCP give identical results.

2. **Optimization & Repair**:
   - `pdf_compress`: File size reduced or bounded; text and page count preserved; low < medium < high quality size progression.
   - `pdf_repair`: Output parses with `pypdf` without syntax error even when input has damaged/truncated xref.
   - `pdf_linearize`: Output contains `/Linearized` object dictionary; passes linearization check.

3. **Security**:
   - `pdf_encrypt`: Opening without password fails; opening with wrong password fails; opening with correct password succeeds; `/Encrypt` dictionary present.
   - `pdf_decrypt`: Decrypted file opens without password; `/Encrypt` absent; page count and text match original; wrong password returns error.
   - `pdf_redact`: Target string (`SECRET 12345`) is **completely absent** from extracted text and raw content streams; black box present.
   - `pdf_sign`: Output contains `/Sig` with `/ByteRange`; signer name matches certificate; wrong password errors.
   - `pdf_hash`: Consistent SHA-256 hash output; changing 1 byte produces different hash; matches `sha256sum`.

4. **Stamping & Numbering**:
   - `pdf_watermark`: Extracted text on every target page contains watermark string; original text still present.
   - `pdf_header_footer`: Header text appears at top region, footer at bottom region; page count unchanged.
   - `pdf_bates`: Sequential Bates numbers present (e.g., `CONF-000001` to `CONF-000005`); output differs from header_footer.
   - `pdf_page_numbers`: Formatted page numbers appear on each page in requested position; format `{page}/{total}` respected.
   - `pdf_annotate`: Output contains `/Annots` with `/Highlight` at given coordinates; color and comment text present.

5. **Forms**:
   - `pdf_read_form`: Reads all field names, types, and values correctly; PDF with no form returns empty list.
   - `pdf_fill_form`: Re-reading with `pypdf` confirms newly filled values; unicode text preserved; CLI and MCP match.
   - `pdf_flatten`: AcroForm field count becomes 0; text remains visible on page; page count unchanged.
   - `pdf_create_form_field`: New field appears in `read_form` with correct name, type, and rect; field is fillable.

6. **Extraction & Analysis**:
   - `pdf_extract_text`: Extracted text matches known strings in reading order; table layout not scrambled; unicode preserved.
   - `pdf_extract_images`: Exactly extracted image count; each decodes as valid PNG/JPEG; dimensions match originals.
   - `pdf_search`: Returns correct hit counts, coordinates inside page, and page indices for target words.
   - `pdf_render`: Produces valid non-solid PNG matching scaled dimensions; file size > 5KB for text page.
   - `pdf_compare`: Same file returns "no differences"; different files list differing pages and text.
   - `pdf_metadata`: Returns accurate Title, Author, and creation metadata; reads metadata and never writes.
   - `pdf_bookmarks`: Returns correct outline hierarchy, titles, and target pages; empty doc returns empty list.
   - `pdf_classify_type`: Correctly classifies "text", "scanned/image", or "form".
   - `pdf_validate`: Valid file returns valid; corrupt file returns invalid with specific error message.
   - `pdf_ocr`: Output has a text layer containing scanned strings (e.g., invoice total); visual appearance preserved.

7. **Conversions**:
   - `pdf_images_to_pdf`: Produces valid PDF with page count matching image count; image aspect ratio preserved.
   - `pdf_to_pdf_a`: Output contains PDF/A identification in XMP metadata; no `/Encrypt`; text preserved.
   - `pdf_to_docx`: Output opens with `docx` package and contains target heading and table text.
   - `pdf_to_xlsx`: Output opens with `openpyxl` and cell values match numbers accurately.
   - `pdf_to_pptx`: Slide count = page count; each slide contains text/image; opens in `python-pptx`.
   - `pdf_convert_html`: Output PDF contains HTML headings and text; CSS styling respected.
   - `pdf_convert_markdown`: Output PDF contains Markdown headings, lists, code, and table text.
   - `pdf_convert_excel`: Output PDF contains table rows; columns not cut off.

## 4. Deliverables
1. `scripts/generate_real_fixtures.py` (Script to generate the required realistic fixtures).
2. `scripts/test_tri_interface_e2e.py` (Enhanced with independent semantic assertions and detailed reporting).
3. `reports/REAL_ASSERTIONS_TRI_INTERFACE_REPORT.md` (The comprehensive execution report with assertion-level breakdowns and failure analysis).
4. `wiki/25-Real-Assertions-Verification.md` (Knowledge wiki documenting the real assertions architecture).
