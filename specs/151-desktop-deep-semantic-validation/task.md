# Task Tracker: Next-Level Deep Semantic & Self-Healing Playwright E2E Validation (Spec 151)

**Parent Specification:** [`qa/playwright_deep_semantic_validation_spec.md`](qa/playwright_deep_semantic_validation_spec.md)  
**Target:** 90 Desktop Tools with In-Depth Multi-Format Semantics & Zero-Manual-Reset Testing.

---

## Task Breakdown

### Phase 1: Test Infrastructure & Deep Inspection Helpers
- [ ] **Task 151.1: Ephemeral Sandbox Lifecycle Manager** (`apps/desktop/tests/helpers/sandboxManager.ts`)
  - UUID-scoped temporary output directories per test run.
  - Automatic directory provisioning and guaranteed post-test cleanup.
  - Periodic sweeper for abandoned fixture directories older than 1 hour.
- [ ] **Task 151.2: PDF Deep Content & Structure Inspector** (`apps/desktop/tests/helpers/pdfInspector.ts`)
  - Exact page count extraction and verification.
  - MediaBox, CropBox, and page rotation inspection.
  - Text search and absence verification (for true redaction testing).
  - AcroForm field value inspection and annotation count validation.
  - Cryptographic signature dictionary and RFC 3161 TSA presence checks.
- [ ] **Task 151.3: Office & Web Format Inspector** (`apps/desktop/tests/helpers/officeInspector.ts`)
  - Excel sheet name, row count, and cell coordinate extraction via `xlsx`.
  - Word OpenXML document reader (`word/document.xml`) for paragraph text.
  - Clean HTML DOM evaluator for headless browser checks.

---

### Phase 2: Suite 1 Deep Semantic Assertions (Tools 01–30: Page Operations & Imposition)
- [ ] **Task 151.4: Page Count & Arithmetic Invariants**
  - Verify `pdf_merge`: $\text{count}(\text{output}) = \text{count}(\text{docA}) + \text{count}(\text{docB})$.
  - Verify `pdf_split`: parts equal exact range specifications.
  - Verify `pdf_delete_pages`: page count decreases by deleted count.
  - Verify `pdf_burst`: creates exactly $N$ individual 1-page documents.
- [ ] **Task 151.5: Geometry & Imposition Visual Validation**
  - Verify `pdf_rotate`: Page dictionary has `/Rotate 90`.
  - Verify `pdf_crop`: MediaBox bounding box matches crop coordinates.
  - Verify `pdf_nup`, `pdf_booklet`, `pdf_posterize`: Imposed page counts and sheet layout.
  - Verify `pdf_render` and `pdf_invert_colors`: High-DPI PNG byte stream validation.

---

### Phase 3: Suite 2 Deep Semantic Assertions (Tools 31–60: Security & Forms)
- [ ] **Task 151.6: Security & Cryptographic Verifications**
  - Verify `pdf_encrypt`: Opening without password fails with password error; opening with `password123` succeeds.
  - Verify `pdf_decrypt`: Decrypted document opens with zero password prompts.
  - Verify `pdf_sign`: `/Sig` dictionary contains authentic PKCS#7 signature and certificates.
  - Verify `pdf_timestamp`: TSA timestamp dictionary present and cryptographically non-empty.
  - Verify `pdf_redact`: Target keyword is absent from all decoded text streams.
- [ ] **Task 151.7: Form & Metadata Verifications**
  - Verify `pdf_fill_form`: AcroForm fields populated with exact JSON test data.
  - Verify `pdf_flatten`: All `/Widget` annotations removed, text rendered into content streams.
  - Verify `pdf_watermark` and `pdf_bates`: Stamped text strings present in document body.
  - Verify `pdf_metadata`: Title, author, and producer match test inputs.

---

### Phase 4: Suite 3 Deep Semantic Assertions (Tools 61–90: Conversions & AI)
- [ ] **Task 151.8: Office & Archival Format Verifications**
  - Verify `pdf_to_xlsx` & `excel_to_json`: Accurate tabular structure and cell content.
  - Verify `pdf_to_docx`: OpenXML archive contains valid paragraphs matching source PDF text.
  - Verify `pdf_to_pdf_a`: Conformance dictionary contains PDF/A standard tags.
- [ ] **Task 151.9: AI, Search & Workflow Verifications**
  - Verify `pdf_ocr`: Scanned document yields recognized text containing fixture keywords.
  - Verify `pdf_rag_chunks`: Chunks JSON has valid token counts and heading metadata.
  - Verify `document_workflow` & `recipe_run`: Multi-step recipes produce valid final outputs.

---

### Phase 5: Continuous Autonomous CI Integration
- [ ] **Task 151.10: Zero-Flake Execution**
  - Verify 3 suites execute concurrently without file collisions.
  - Ensure zero residual test files left in workspace after full run.
  - All tests passing with 100% deterministic repeatability.

