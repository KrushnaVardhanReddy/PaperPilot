# Spec 153: Enterprise Document Test Corpus & Deep Invariant Verification

## 1. Background & Rationale
PaperPilot includes 125 tools and a 90-tool zero-mock deep semantic E2E test suite split into:
- **Suite 1:** Tools 01–30 (Page Operations, Merging, Splitting, Imposition, Optimization & Rendering)
- **Suite 2:** Tools 31–60 (Security, Cryptography, Annotations & Forms)
- **Suite 3:** Tools 61–90 (Conversions, Intelligence, Office & Workflows)

Historically, fixtures located in `tests/e2e_fixtures/real/` were minimal synthetic stubs (500–1,400 bytes) without realistic text streams, multi-page flows, AcroForms, or complex formatting. While passing superficial exit code checks, they prevented meaningful semantic validation of tools like OCR, PII scan, form-filling, table extraction, and EPUB/Office conversion.

Spec 153 replaces these minimal fixtures with **high-fidelity, tool-specific, deterministically generated documents** created directly by dogfooding the `paperpilot` CLI (`html_to_pdf`, `md_to_pdf`, `encrypt`, `signature`, `watermark`, `fill-batch`), and upgrades the 3 Playwright test suites to run against this enterprise test corpus with deep structural invariants.

---

## 2. Architecture & Execution Strategy

### 2.1 The Dogfooding Generator Engine
Rather than relying on third-party binary generators or unvetted external downloads, all corpus PDF fixtures are rendered deterministically using PaperPilot's native pure-Rust `html_to_pdf` and `md_to_pdf` conversion engines. 

```
HTML/MD Templates (Semantic, Tool-Specific)
                    │
                    ▼  paperpilot convert --format pdf
       Base Enterprise PDFs (15 KB – 200 KB)
                    │
                    ├─► paperpilot encrypt       ──► encrypted.pdf
                    ├─► paperpilot signature     ──► signed.pdf
                    ├─► paperpilot watermark     ──► watermarked.pdf
                    ├─► paperpilot render        ──► test.png
                    └─► Embedded AcroForm/XLSX   ──► form.pdf, test.xlsx
```

### 2.2 Tri-Prompt Jules / Gemini Delegation Structure
To maintain focused context windows and guarantee rigorous execution, implementation is divided into three separate prompts matching the three test suites:

1. **Phase 153.1 (Suite 1 — Tools 01–30):**
   - HTML templates for multi-page legal agreements, alternating page layouts, single-page headers, and image-rich documents.
   - Script `scripts/gen_corpus_suite1.sh` generating `multi_page.pdf` (8-10 pages), `single_page.pdf`, `merge_a.pdf`, `merge_b.pdf`, `test.png`.
   - Update `apps/desktop/tests/e2e_suite_1_page_operations.spec.ts` with deep invariant assertions (page count arithmetic, raster size, split count).

2. **Phase 153.2 (Suite 2 — Tools 31–60):**
   - HTML templates for PII-dense contracts (names, emails, SSNs, credit cards) and programmatic AcroForm generation with interactive field dictionaries.
   - Script `scripts/gen_corpus_suite2.sh` generating `text_pii.pdf`, `encrypted.pdf`, `signed.pdf`, `form.pdf`, `batch_data.csv`, `form_data.json`.
   - Update `apps/desktop/tests/e2e_suite_2_security_forms.spec.ts` asserting PII entity detection, form field values, password protection, and signature verification.

3. **Phase 153.3 (Suite 3 — Tools 61–90):**
   - HTML/MD/Office templates for academic two-column papers with tables, financial models, Markdown documentation, and tabular CSV/XLSX data.
   - Script `scripts/gen_corpus_suite3.sh` generating `research_paper.pdf`, `financial_tables.pdf`, `test.html`, `test.md`, `test.csv`, `test.xlsx`, `test.docx`.
   - Update `apps/desktop/tests/e2e_suite_3_conversions_ai.spec.ts` asserting table extraction rows/cols, EPUB structure, Office conversion validity, and OCR confidence.

---

## 3. Makefile Targets & Workflow

```makefile
# Generate the full enterprise test corpus
gen-corpus: build-cli gen-corpus-suite-1 gen-corpus-suite-2 gen-corpus-suite-3

gen-corpus-suite-1: build-cli
	@bash scripts/gen_corpus_suite1.sh

gen-corpus-suite-2: build-cli
	@bash scripts/gen_corpus_suite2.sh

gen-corpus-suite-3: build-cli
	@bash scripts/gen_corpus_suite3.sh

# Run all 90 tools against the high-fidelity corpus
test-e2e-all-90-corpus: gen-corpus test-e2e-all-90
```

---

## 4. Success Criteria & Verification
1. `make gen-corpus` executes deterministically without network calls and produces valid fixtures in `tests/e2e_fixtures/real/`.
2. All 90 tools in `make test-e2e-all-90` pass 100% cleanly against the new corpus without timeouts or memory bloat.
3. Assertions verify real semantic transformations (non-empty PII scans, non-empty OCR extractions, mathematically exact page count combinations).
