# Tasks: Spec 153 — Enterprise Document Test Corpus & Deep Invariant Verification

- [x] **153.1 — Phase 1: Suite 1 Fixtures & Test Hardening (Tools 01–30: Page Operations & Imposition)**
  - [x] 153.1.1 Create HTML generation templates in `tests/fixtures_templates/suite1/` (`multi_page_contract.html`, `merge_doc_a.html`, `merge_doc_b.html`, `single_page_doc.html`).
  - [x] 153.1.2 Create `scripts/gen_corpus_suite1.sh` invoking `paperpilot convert --format pdf` to generate `tests/e2e_fixtures/real/multi_page.pdf` (multi-page with headers/footers), `single_page.pdf`, `merge_a.pdf`, `merge_b.pdf`, and `test.png`.
  - [x] 153.1.3 Add `gen-corpus-suite-1` target to `Makefile`.
  - [x] 153.1.4 Update `apps/desktop/tests/e2e_suite_1_page_operations.spec.ts` with deep domain invariant assertions (exact page count arithmetic on merge/split/burst, dimension checks on crop/scale, pixel assertions on render).
  - [x] 153.1.5 Verify `make gen-corpus-suite-1 && make test-e2e-suite-1` passes 30/30 tools.

- [x] **153.2 — Phase 2: Suite 2 Fixtures & Test Hardening (Tools 31–60: Security, Forms & Metadata)**
  - [x] 153.2.1 Create PII document HTML template with real names, emails, SSNs, credit card numbers, and legal confidentiality notices (`tests/fixtures_templates/suite2/pii_agreement.html`).
  - [x] 153.2.2 Create programmatic AcroForm generator (via base64 or Rust lopdf script) producing a valid 8-field AcroForm PDF (`form.pdf`) containing text inputs, checkboxes, and radio buttons.
  - [x] 153.2.3 Create `scripts/gen_corpus_suite2.sh` generating `text_pii.pdf`, `encrypted.pdf` (password-protected), `signed.pdf` (digitally signed), `form.pdf`, `form_data.json`, and `batch_data.csv`.
  - [x] 153.2.4 Add `gen-corpus-suite-2` target to `Makefile`.
  - [x] 153.2.5 Update `apps/desktop/tests/e2e_suite_2_security_forms.spec.ts` with deep semantic assertions (PII scan detects entities, form fill modifies target fields, signature validation inspects byte ranges, decrypt verifies password removal).
  - [x] 153.2.6 Verify `make gen-corpus-suite-2 && make test-e2e-suite-2` passes 30/30 tools.

- [ ] **153.3 — Phase 3: Suite 3 Fixtures & Test Hardening (Tools 61–90: Conversions, AI & Workflows)**
  - [ ] 153.3.1 Create templates for structured two-column academic paper, financial statements with multi-row tables, rich Markdown manual with code blocks, and tabular CSV/XLSX data.
  - [ ] 153.3.2 Create `scripts/gen_corpus_suite3.sh` generating `research_paper.pdf`, `financial_tables.pdf`, `test.html`, `test.md`, `test.csv`, `test.xlsx`, and `test.docx`.
  - [ ] 153.3.3 Add `gen-corpus-suite-3` and `gen-corpus` aggregate targets to `Makefile`.
  - [ ] 153.3.4 Update `apps/desktop/tests/e2e_suite_3_conversions_ai.spec.ts` with deep semantic assertions (table extraction detects columns/rows, EPUB container contains spine/manifest, OCR outputs recognized characters).
  - [ ] 153.3.5 Verify `make gen-corpus-suite-3 && make test-e2e-suite-3` passes 30/30 tools.
  - [ ] 153.3.6 Run full verification: `make test-e2e-all-90` against the 100% refreshed corpus.
