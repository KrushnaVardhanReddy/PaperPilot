# Prompt for Jules: Automate Desktop E2E Across All 90 PaperPilot Tools

Copy and paste the exact prompt below into Jules's task prompt window:

---

```markdown
Hello Jules,

Please automate the end-to-end testing for all 90 PaperPilot Desktop Tools using Playwright on `feature/dev`.

### Specifications & Documentation
1. Technical Specification: `qa/playwright_e2e_specification.md`
2. Implementation Task List: `qa/playwright_e2e_tasks.md`
3. Reference Playwright Implementation: `apps/desktop/tests/e2e_all_tools_real_execution.spec.ts`
4. Configuration: `apps/desktop/playwright.config.ts`
5. QA Guides with exact steps and form arguments:
   - `qa/manual_desktop_testing/TESTER_1_PAGE_OPERATIONS.md` (Tools 01–30)
   - `qa/manual_desktop_testing/TESTER_2_SECURITY_FORMS.md` (Tools 31–60)
   - `qa/manual_desktop_testing/TESTER_3_CONVERSIONS_INTELLIGENCE.md` (Tools 61–90)
   - `qa/manual_desktop_testing/MANUAL_TESTING_INDEX.md`
6. Test Fixtures: `tests/e2e_fixtures/real/` (`single_page.pdf`, `multi_page.pdf`, `merge_a.pdf`, `merge_b.pdf`, `form.pdf`, `dummy.p12` [password: `password123`], `test.xlsx`, `test.png`)

---

### Core Constraints & Invariants
- Target Branch: `feature/dev` (never push to `main`).
- Build limit: If compiling Rust, use `cargo build --bin paperpilot -j 4`.
- Zero-Mock Execution: The UI must trigger real child-process engine commands (`target/debug/paperpilot`) and produce real output files in `tests/e2e_fixtures/out/desktop/`.
- Deep Semantic Validation: Every test must verify byte-level correctness (e.g. `%PDF-` header, page geometry, JSON schemas, Office `PK\x03\x04` signatures).
- Environment settings for test runs:
  ```bash
  export PAPERPILOT_OCR_MODELS_DIR="$(pwd)/../../models/ocr/ppocrv4"
  export NO_PROXY="localhost,127.0.0.1"
  export no_proxy="localhost,127.0.0.1"
  unset HTTP_PROXY HTTPS_PROXY http_proxy https_proxy
  ```

---

### Implementation Instructions

1. **Split into 3 Modular Test Suites**:
   - `apps/desktop/tests/e2e_suite_1_page_operations.spec.ts` (Tools 01–30)
   - `apps/desktop/tests/e2e_suite_2_security_forms.spec.ts` (Tools 31–60)
   - `apps/desktop/tests/e2e_suite_3_conversions_ai.spec.ts` (Tools 61–90)

2. **Per-Tool Playwright Standard Flow**:
   Follow the proven loop from `e2e_all_tools_real_execution.spec.ts`:
   - Reset: `await page.evaluate(() => (window as any).appState?.clearAllDocuments())` and `await page.click('#tab-btn-home')`.
   - Stage fixture: `await fileInput.setInputFiles([path.resolve('../../tests/e2e_fixtures/real/<fixture>')])`.
   - Select context button: `#context-btn-pdf`, `#context-btn-office`, or `#context-btn-image`.
   - Search: Type the exact tool title into `#tool-search-input` (e.g., `'Sign Document'`, `'Validate PDF'`) and click the resulting `.tool-card`.
   - Fill inputs: Populate any required parameters (e.g., `#angle`, `#password`, `#splitPoints`).
   - Run & wait: Click `.run-btn`, wait for `.toast`.
   - Semantic check: Inspect the generated output in `tests/e2e_fixtures/out/desktop/`.
   - Reset view: Click `.back-btn` or `#tab-btn-home` to return to `#tool-search-input`.

3. **Deliverables**:
   - All 3 suites passing cleanly with:
     ```bash
     cd apps/desktop
     npx playwright test tests/e2e_suite_1_page_operations.spec.ts
     npx playwright test tests/e2e_suite_2_security_forms.spec.ts
     npx playwright test tests/e2e_suite_3_conversions_ai.spec.ts
     ```
   - Scorecard report generated at `reports/ALL_90_TOOLS_REAL_DESKTOP_E2E_SCORECARD.md` with 90/90 passing.
   - Vitest component tests intact: `pnpm test` (153/153 passing).
   - Commit cleanly to `feature/dev`.
```

