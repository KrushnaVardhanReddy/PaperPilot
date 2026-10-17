# Spec 061: Clean-Room E2E Verification & Audit for Top 12 Launch Tools

## 1. Overview & Objective
Following the real-world user feedback fixes (wiring `excel_to_json` to disk, fixing extension handling in `excel_to_csv`, adding `split --every`, and hardening OCR/watermark/DOCX), this specification defines an automated, clean-room audit of the **Top 12 Core Launch Tools**.

Every test run must execute inside an **isolated, newly generated output sandbox** (`tests/e2e_fixtures/out/top12_audit_<TIMESTAMP>/`) so that stale files cannot produce false passes.

---

## 2. The Top 12 Core Launch Tools Matrix

Every tool execution must record and report:
1. **Tool Name & Subcommand**
2. **Exact Input Fixture(s)**
3. **Exact Output File Generated**
4. **Deep Semantic Verification Performed**

| # | Tool ID | Interface Under Test | Exact Input Fixture | Output Target | Deep Semantic Verification (Validation Criteria) |
|---|---|---|---|---|---|
| 1 | `pdf_merge` | CLI & Desktop MCP | `tests/e2e_fixtures/real/merge_a.pdf`, `merge_b.pdf` (1 page each) | `out/merged.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Exact page count must equal 2 (`PdfAssertions::assert_page_count`). |
| 2 | `pdf_split` | CLI & Desktop MCP | `tests/e2e_fixtures/real/multi_page.pdf` (5 pages) with `--every 2` | `out/split_dir/` | Check 1: Directory contains `split_1.pdf`, `split_2.pdf`, `split_3.pdf`.<br>Check 2: `split_1.pdf` and `split_2.pdf` have 2 pages; `split_3.pdf` has 1 page. |
| 3 | `pdf_rotate` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` (angle: 90) | `out/rotated.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Lopdf inspect dictionary: page 1 `/Rotate` attribute equals `90`. |
| 4 | `pdf_delete_pages` | CLI & Desktop MCP | `tests/e2e_fixtures/real/multi_page.pdf` (5 pages, delete: `2,4`) | `out/deleted.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Exact page count must equal 3 (`5 - 2 = 3`). |
| 5 | `pdf_reorder_pages` | CLI & Desktop MCP | `tests/e2e_fixtures/real/multi_page.pdf` (order: `5,4,3,2,1`) | `out/reordered.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Exact page count 5; page 1 text contains "Page 5". |
| 6 | `pdf_compress` | CLI & Desktop MCP | `tests/e2e_fixtures/real/large_doc.pdf` (quality: medium) | `out/compressed.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Output file size is strictly less than or equal to original input size; opens without syntax corruption. |
| 7 | `pdf_watermark` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` (text: "CONFIDENTIAL / DO NOT COPY") | `out/watermarked.pdf` | Check 1: File $> 0$ bytes.<br>Check 2: Page 1 content stream contains "CONFIDENTIAL" text string; text does not clip page bounds. |
| 8 | `pdf_ocr` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` (format: txt) | `out/ocr.txt` | Check 1: If models present, output text $> 0$ characters.<br>Check 2: If models absent, returns structured error with setup prompt. |
| 9 | `pdf_extract_text` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/extracted.txt` | Check 1: File exists and $> 0$ bytes.<br>Check 2: Contains expected text without binary garbage or missing stream errors. |
| 10 | `pdf_to_docx` | CLI & Desktop MCP | `tests/e2e_fixtures/real/single_page.pdf` | `out/converted.docx` | Check 1: File $> 0$ bytes.<br>Check 2: Valid OpenXML ZIP archive (`PK\x03\x04`) containing `[Content_Types].xml` and `word/document.xml`. |
| 11 | `excel_to_json` | CLI & Desktop MCP | `tests/e2e_fixtures/real/test.xlsx` | `out/converted.json` | Check 1: File exists and $> 0$ bytes.<br>Check 2: Parses as valid JSON containing `sheets` array with columns `ColA, ColB, ColC` and rows count $> 0$. |
| 12 | `excel_to_csv` | CLI & Desktop MCP | `tests/e2e_fixtures/real/test.xlsx` | `out/converted.csv` | Check 1: File exists, extension strictly `.csv`, size $> 0$ bytes.<br>Check 2: Contains comma-delimited headers `ColA,ColB,ColC` and corresponding numerical rows. |

---

## 3. Execution Requirements & Deliverables

1. **Dedicated Test Runner / Script**:
   - Provide an automated test script (`tests/top12_cleanroom_audit.rs` or runnable via `cargo test --test top12_audit`).
2. **Clean-Room Sandbox Isolation**:
   - Every run creates a brand-new directory: `tests/e2e_fixtures/out/top12_audit_<timestamp>/`.
   - Never re-use existing files.
3. **Comprehensive Markdown Audit Scorecard**:
   - Write output report to `reports/TOP_12_CORE_TOOLS_AUDIT_REPORT.md`.
   - The report must list for each of the 12 tools:
     - **Status**: PASS / FAIL
     - **Input**: Exact file path & size
     - **Output**: Exact file path & size
     - **Validation**: Exact checks performed and assertion results
     - **Latency**: Measured execution duration
