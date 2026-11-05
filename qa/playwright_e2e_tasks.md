# Tasks: Playwright 90-Tool Automation Implementation

**Target Delivery**: 3 modular test suites executing 90 real desktop tools with 100% deep semantic pass rate.

---

## Task 1: Environment & Fixtures Verification
- [ ] Confirm `target/debug/paperpilot` exists; if missing, build with `cargo build --bin paperpilot -j 4`.
- [ ] Verify test fixtures in `tests/e2e_fixtures/real/`:
  - `single_page.pdf`
  - `multi_page.pdf`
  - `merge_a.pdf`, `merge_b.pdf`
  - `form.pdf`, `form_data.json`
  - `dummy.p12`
  - `test.xlsx`, `test.xls`, `test.csv`
  - `test.png`, `sample_photo.png`
- [ ] Verify `PAPERPILOT_OCR_MODELS_DIR` points to `models/ocr/ppocrv4`.

---

## Task 2: Implement Suite 1 — Page Operations (Tools 01–30)
- [ ] Create `apps/desktop/tests/e2e_suite_1_page_operations.spec.ts`.
- [ ] Follow reference pattern from `apps/desktop/tests/e2e_all_tools_real_execution.spec.ts`.
- [ ] Implement operations:
  - Tools 01–06: `pdf_merge`, `pdf_merge_alternate`, `pdf_split`, `pdf_split_by_size`, `pdf_split_by_outline`, `pdf_split_half`
  - Tools 07–12: `pdf_extract_pages`, `pdf_delete_pages`, `pdf_reorder_pages`, `pdf_rotate`, `pdf_crop`, `pdf_auto_crop`
  - Tools 13–18: `pdf_nup`, `pdf_booklet`, `pdf_poster`, `pdf_remove_blank`, `pdf_add_blank_page`, `pdf_duplicate_pages`
  - Tools 19–24: `pdf_reverse_order`, `pdf_interleave`, `pdf_deinterleave`, `pdf_adjust_page_box`, `pdf_scale_pages`, `pdf_burst`
  - Tools 25–30: `pdf_compress`, `pdf_compress_lossless`, `pdf_grayscale`, `pdf_render`, `pdf_render_page_selection`, `pdf_extract_embedded_images`
- [ ] Add semantic assertions: check `%PDF-` header, page geometry, and raster images.
- [ ] Run Suite 1 in isolation: `npx playwright test tests/e2e_suite_1_page_operations.spec.ts`.
- [ ] Verify 30/30 pass.

---

## Task 3: Implement Suite 2 — Security & AcroForms (Tools 31–60)
- [ ] Create `apps/desktop/tests/e2e_suite_2_security_forms.spec.ts`.
- [ ] Implement operations:
  - Tools 31–36: `pdf_encrypt`, `pdf_decrypt`, `pdf_change_password`, `pdf_set_permissions`, `pdf_redact`, `pdf_sanitize`
  - Tools 37–42: `pdf_repair`, `pdf_linearize`, `pdf_sign`, `pdf_validate_signatures`, `pdf_remove_signatures`, `pdf_extract_certs`
  - Tools 43–46: `pdf_fill_form`, `pdf_read_form_fields`, `pdf_flatten_form`, `pdf_remove_form_fields`
  - Tools 47–54: `pdf_watermark`, `pdf_watermark_image`, `pdf_header_footer`, `pdf_bates`, `pdf_page_numbers`, `pdf_add_border`, `pdf_add_margin`, `pdf_flatten`
  - Tools 55–60: `pdf_metadata`, `pdf_set_metadata`, `pdf_strip_metadata`, `pdf_bookmarks`, `pdf_set_bookmarks`, `pdf_hash`
- [ ] Add cryptographic and form field assertions (signature dictionary, encryption flags, metadata streams).
- [ ] Run Suite 2 in isolation: `npx playwright test tests/e2e_suite_2_security_forms.spec.ts`.
- [ ] Verify 30/30 pass.

---

## Task 4: Implement Suite 3 — Conversions & AI (Tools 61–90)
- [ ] Create `apps/desktop/tests/e2e_suite_3_conversions_ai.spec.ts`.
- [ ] Implement operations:
  - Tools 61–69 (PDF to Office/Web/Archival): `pdf_to_docx`, `pdf_to_pptx`, `pdf_to_xlsx`, `pdf_to_html`, `pdf_to_markdown`, `pdf_to_txt`, `pdf_to_json`, `pdf_to_csv`, `pdf_to_pdf_a`
  - Tools 70–76 (Office/Web/Images to PDF): `docx_to_pdf`, `pptx_to_pdf`, `xlsx_to_pdf`, `html_to_pdf`, `markdown_to_pdf`, `txt_to_pdf`, `images_to_pdf`
  - Tools 77–81 (Office Utilities & Modernize): `excel_to_json`, `excel_to_csv`, `office_to_markdown`, `office_extract_text`, `office_modernize_legacy`
  - Tools 82–88 (AI, Extraction, Search, Validation): `pdf_ocr`, `pdf_extract_text`, `pdf_search`, `pdf_classify_type`, `pdf_rag_chunks`, `pdf_compare`, `pdf_validate`
  - Tools 89–90 (Pipeline & Automation): `pdf_pipeline_builder`, `pdf_watch_folder`
- [ ] Add format validation: ZIP magic headers for Office files, schema validation for JSON/RAG chunks, and string matching for OCR text.
- [ ] Run Suite 3 in isolation: `npx playwright test tests/e2e_suite_3_conversions_ai.spec.ts`.
- [ ] Verify 30/30 pass.

---

## Task 5: Master Scorecard & Final Validation
- [ ] Write consolidated markdown report to `reports/ALL_90_TOOLS_REAL_DESKTOP_E2E_SCORECARD.md`.
- [ ] Run Vitest suite: `pnpm test` (verify 153/153 tests pass).
- [ ] Ensure all code changes are staged and committed cleanly on `feature/dev`.
