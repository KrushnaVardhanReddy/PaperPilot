# Spec 064: 50-Tool Complete Clean-Room Isolated Sandbox Verification & Audit

## 1. Overview & Objective
Following the clean-room verification of the Top 12 tools (Spec 061), this specification expands the automated clean-room audit to **all 50 PaperPilot tools** across CLI, MCP, and Core Engine traits.

Every test run must execute inside an **isolated, newly generated output sandbox** (`tests/e2e_fixtures/out/all50_audit_<TIMESTAMP>/`) so that stale files cannot produce false passes. Every tool execution records exact input files, generated output files on disk, execution latency in milliseconds, and deep semantic checks (page counts, magic bytes, JSON/CSV parsing, or dictionary checks).

---

## 2. Complete 50-Tool Audit & Validation Matrix

| # | Tool ID | Interface Under Test | Exact Input Fixture | Output Target | Deep Semantic Verification (Validation Criteria) |
|---|---|---|---|---|---|
| 1 | `pdf_merge` | CLI & Desktop MCP | `tests/e2e_fixtures/real/merge_a.pdf`, `merge_b.pdf` | `out/merged.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Exact page count == 2 (`PdfAssertions::assert_page_count`). |
| 2 | `pdf_split` | CLI & Desktop MCP | `tests/e2e_fixtures/real/multi_page.pdf` (5 pages) with `--every 2` | `out/split_dir/` | Check 1: Directory contains `split_1.pdf`, `split_2.pdf`, `split_3.pdf`.<br>Check 2: `split_1.pdf` (2p), `split_2.pdf` (2p), `split_3.pdf` (1p). |
| 3 | `pdf_extract_pages` | CLI & Desktop MCP | `tests/e2e_fixtures/real/multi_page.pdf` (pages `1,3`) | `out/extracted_pages.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Exact page count == 2. |
| 4 | `pdf_delete_pages` | CLI & Desktop MCP | `tests/e2e_fixtures/real/multi_page.pdf` (pages `2,4`) | `out/deleted.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Exact page count == 3 (`5 - 2 = 3`). |
| 5 | `pdf_reorder_pages` | CLI & Desktop MCP | `tests/e2e_fixtures/real/multi_page.pdf` (order `5,4,3,2,1`) | `out/reordered.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Page count 5; page 1 contains text 'Page 5'. |
| 6 | `pdf_rotate` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` (angle 90) | `out/rotated.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Lopdf inspect dictionary: page 1 `/Rotate` == 90. |
| 7 | `pdf_crop` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` (margins `10,10,200,200`) | `out/cropped.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: `/CropBox` or `/MediaBox` dictionary coordinates match input crop. |
| 8 | `pdf_burst` | CLI & Desktop MCP | `tests/e2e_fixtures/real/multi_page.pdf` (5 pages) | `out/burst_dir/` | Check 1: Contains 5 separate PDF files.<br>Check 2: Each burst file contains exactly 1 page. |
| 9 | `pdf_remove_blank` | CLI & Desktop MCP | `tests/e2e_fixtures/real/multi_page.pdf` | `out/no_blanks.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Opens cleanly without syntax corruption. |
| 10 | `pdf_compress` | CLI & Desktop MCP | `tests/e2e_fixtures/real/large_doc.pdf` (quality: medium) | `out/compressed.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Output file size $\le$ original input size; valid PDF syntax. |
| 11 | `pdf_repair` | CLI & Desktop MCP | `tests/e2e_fixtures/real/multi_page.pdf` | `out/repaired.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Valid cross-reference table and `%PDF` EOF marker. |
| 12 | `pdf_linearize` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/linearized.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Primary dictionary contains `/Linearized` hint stream or version tag. |
| 13 | `pdf_encrypt` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` (pass `user123`) | `out/encrypted.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Contains `/Encrypt` dictionary and cannot be read without password. |
| 14 | `pdf_decrypt` | CLI & Desktop MCP | `tests/e2e_fixtures/real/encrypted.pdf` (pass `user123`) | `out/decrypted.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: `/Encrypt` dictionary stripped; readable as standard unencrypted PDF. |
| 15 | `pdf_watermark` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` (text `CONFIDENTIAL`) | `out/watermarked.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Page content stream contains "CONFIDENTIAL" string. |
| 16 | `pdf_redact` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` (box `50,50,100,100`) | `out/redacted.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Blackout fill rectangle drawn in content stream. |
| 17 | `pdf_metadata` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` (title `AuditTitle`) | `out/metadata.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Lopdf `/Info` dictionary `/Title` equals "AuditTitle". |
| 18 | `pdf_sign` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` (signer `Audit Signer`) | `out/signed.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Digital visual signature seal or `/ByteRange` entry placed. |
| 19 | `pdf_flatten` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/flattened.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Interactive `/AcroForm` or form fields flattened into static canvas stream. |
| 20 | `pdf_to_pdf_a` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/pdf_a.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Contains PDF/A XMP metadata stream (`pdfaExtension`). |
| 21 | `pdf_header_footer` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` (header `Test Header`) | `out/header_footer.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Content stream contains "Test Header" string. |
| 22 | `pdf_bates` | CLI & Desktop MCP | `tests/e2e_fixtures/real/multi_page.pdf` (prefix `CONF-`) | `out/bates.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Page 1 contains "CONF-000001", page 2 contains "CONF-000002". |
| 23 | `pdf_page_numbers` | CLI & Desktop MCP | `tests/e2e_fixtures/real/multi_page.pdf` (format `{n}/{total}`) | `out/page_numbers.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Page 1 content stream contains "1/5". |
| 24 | `pdf_extract_text` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/extracted.txt` | Check 1: File exists and $> 0$ bytes.<br>Check 2: Plaintext contains expected words without binary garbage. |
| 25 | `pdf_extract_images` | CLI & Desktop MCP | `tests/e2e_fixtures/real/multimodal.pdf` | `out/extracted_imgs/` | Check 1: Directory created with extracted PNG/JPEG files.<br>Check 2: Each image file starts with valid image magic bytes (`\x89PNG` or `\xFF\xD8`). |
| 26 | `pdf_search` | CLI & Desktop MCP | `tests/e2e_fixtures/real/multi_page.pdf` (query `Page`) | `out/search_results.json` | Check 1: File exists and $> 0$ bytes.<br>Check 2: Valid JSON array containing match snippets and page indices. |
| 27 | `pdf_render` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` (page 1) | `out/rendered.png` | Check 1: File $> 0$ bytes.<br>Check 2: Starts with standard PNG magic bytes (`\x89PNG\r\n\x1a\n`). |
| 28 | `pdf_compare` | CLI & Desktop MCP | `tests/e2e_fixtures/real/merge_a.pdf`, `merge_b.pdf` | `out/diff.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Diff overlay highlighting generated with valid `%PDF` structure. |
| 29 | `pdf_ocr` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/ocr.txt` | Check 1: If models present, output text $> 0$ characters.<br>Check 2: If models absent, returns structured error with setup prompt. |
| 30 | `pdf_bookmarks` | CLI & Desktop MCP | `tests/e2e_fixtures/real/multi_page.pdf` | `out/bookmarks.json` | Check 1: File exists and $> 0$ bytes.<br>Check 2: Valid JSON array of outline/bookmark tree items. |
| 31 | `pdf_images_to_pdf` | CLI & Desktop MCP | `tests/e2e_fixtures/real/test.png` | `out/img_converted.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Starts with `%PDF`; contains 1 page with embedded image XObject. |
| 32 | `pdf_annotate` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` (note `Review Note`) | `out/annotated.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Contains `/Annots` dictionary array with text note. |
| 33 | `pdf_classify_type` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/classify.json` | Check 1: Parses as valid JSON.<br>Check 2: Contains classification object (`type`, `confidence`, `is_scanned`). |
| 34 | `pdf_validate` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/validation.json` | Check 1: Parses as valid JSON.<br>Check 2: Returns `{ "valid": true, "version": "..." }`. |
| 35 | `pdf_hash` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/hash.json` | Check 1: Parses as valid JSON.<br>Check 2: Contains valid 64-character SHA-256 hex string. |
| 36 | `pdf_read_form` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/form_data.json` | Check 1: Parses as valid JSON.<br>Check 2: Contains fields map or empty array if unformattable. |
| 37 | `pdf_fill_form` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/form_filled.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Valid `%PDF` output without syntax corruption. |
| 38 | `pdf_create_form_field` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/form_created.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Contains `/AcroForm` entry in catalog. |
| 39 | `pdf_to_docx` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/converted.docx` | Check 1: File $> 0$ bytes.<br>Check 2: Valid OpenXML ZIP (`PK\x03\x04`) containing `word/document.xml`. |
| 40 | `pdf_to_xlsx` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/converted.xlsx` | Check 1: File $> 0$ bytes.<br>Check 2: Valid OpenXML ZIP (`PK\x03\x04`) containing `xl/workbook.xml`. |
| 41 | `pdf_to_pptx` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/converted.pptx` | Check 1: File $> 0$ bytes.<br>Check 2: Valid OpenXML ZIP (`PK\x03\x04`) containing `ppt/presentation.xml`. |
| 42 | `pdf_convert_html` | CLI & Desktop MCP | `tests/e2e_fixtures/real/test.html` | `out/html_converted.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Starts with `%PDF`; contains rendered DOM text. |
| 43 | `pdf_convert_markdown` | CLI & Desktop MCP | `tests/e2e_fixtures/real/test.md` | `out/md_converted.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Starts with `%PDF`; contains styled headings and paragraph text. |
| 44 | `pdf_convert_excel` | CLI & Desktop MCP | `tests/e2e_fixtures/real/test.xlsx` | `out/xlsx_converted.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Starts with `%PDF`; contains formatted spreadsheet grid table. |
| 45 | `pdf_to_html` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/converted.html` | Check 1: File $> 0$ bytes.<br>Check 2: Contains valid HTML tags (`<!DOCTYPE html>` or `<html>`). |
| 46 | `pdf_to_markdown` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/converted.md` | Check 1: File $> 0$ bytes.<br>Check 2: Contains text formatted with markdown headers or paragraphs. |
| 47 | `pdf_to_json` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/pdf_tree.json` | Check 1: File $> 0$ bytes.<br>Check 2: Valid JSON containing page count, dimensions, and text objects. |
| 48 | `json_to_pdf` | CLI & Desktop MCP | `tests/e2e_fixtures/json_doc.json` | `out/json_converted.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Starts with `%PDF`; synthesized from JSON specification. |
| 49 | `excel_to_json` | CLI & Desktop MCP | `tests/e2e_fixtures/real/test.xlsx` | `out/converted.json` | Check 1: File exists and $> 0$ bytes.<br>Check 2: Parses as valid JSON containing `sheets` array with `ColA, ColB, ColC` rows. |
| 50 | `excel_to_csv` | CLI & Desktop MCP | `tests/e2e_fixtures/real/test.xlsx` | `out/converted.csv` | Check 1: File exists, extension strictly `.csv`, size $> 0$ bytes.<br>Check 2: Contains comma-delimited headers `ColA,ColB,ColC` and corresponding rows. |

---

## 3. Execution Requirements & Deliverables

1. **Dedicated Test Runner**:
   - Provide `paperpilot-cli/tests/all50_cleanroom_audit.rs` (runnable via `cargo test --test all50_cleanroom_audit -j 2`).
2. **Clean-Room Sandbox Isolation**:
   - Every run creates a brand-new directory: `tests/e2e_fixtures/out/all50_audit_<timestamp>/`.
   - Stale outputs from prior runs must never be touched or checked.
3. **Makefile Integration**:
   - Add `.PHONY: test-cleanroom-top12 test-cleanroom-all50`.
   - `test-cleanroom-top12`: Runs `cargo test --test top12_cleanroom_audit -j 2`.
   - `test-cleanroom-all50`: Runs `cargo test --test all50_cleanroom_audit -j 2`.
4. **Comprehensive Markdown Audit Scorecard**:
   - Write output report to `reports/ALL_50_TOOLS_CLEANROOM_AUDIT_REPORT.md`.
   - The report must list for every tool:
     - Tool ID & Name
     - Status: PASS / FAIL
     - Exact Input path and size
     - Exact Output path and size
     - Exact Semantic verification checks performed
     - Latency in milliseconds

---

## 4. Success Criteria
- [ ] `cargo test --test all50_cleanroom_audit -j 2` completes with 50/50 tests passing (100%).
- [ ] `Makefile` targets `test-cleanroom-top12` and `test-cleanroom-all50` are added and succeed.
- [ ] Every tool writes to an isolated clean-room timestamped directory.
- [ ] Zero overrides or regressions on engine fixes (`split --every`, `excel_to_json` output paths, `excel_to_csv` extension, etc.).
- [ ] Scorecard written to `reports/ALL_50_TOOLS_CLEANROOM_AUDIT_REPORT.md`.
