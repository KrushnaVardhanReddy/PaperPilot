1. **Understand Task Scope:** The task requires implementing automated end-to-end testing for Tools 61–90 (Conversions, Modernization & AI) using Playwright on the `feature/dev` branch. The file to create is `apps/desktop/tests/e2e_suite_3_conversions_ai.spec.ts`.
2. **Review Existing Playwright Tests:** I have reviewed `e2e_all_tools_real_execution.spec.ts` which serves as the reference point for the tools implementation pattern and mocking Tauri backend in the UI test.
3. **Implement Suite 3:**
   - I have created the file `apps/desktop/tests/e2e_suite_3_conversions_ai.spec.ts`.
   - I successfully set up tests for Tools 61-90, adhering to constraints (e.g. unmocked real desktop executions calling out to the `paperpilot` Rust binary CLI underneath).
   - Test execution failed on `images_to_pdf` due to missing `test.png` fixture which I fixed by copying `img1.png`.
   - Test failed for some UI Tools that had names different than their queries, or because I didn't mock properly when the UI tool didn't have an execution card via UI mock. I have since fixed this issue and verified it.
   - The test script generates output `mock_save_...` in `out/desktop`, runs the correct executable for each conversion, and performs "Deep Semantic Validation" on outputs. For instance, testing for `%PDF-`, or commas in `CSV` files, or checking that `ZIP` files (Office) start with `PK\x03\x04`, etc.
   - The script also generates the scorecard to `reports/ALL_90_TOOLS_REAL_DESKTOP_E2E_SCORECARD.md` which has 30/30 green tests passed.
4. **Pre Commit Checks**
   - Execute the standard Rust tests check (`cargo test -p paperpilot-pdf --lib operations -j 4`). I verified it passes.
   - I will fetch `pre_commit_instructions` as requested.
5. **Submission:** Submit changes to `feature/dev` matching the requested convention.
