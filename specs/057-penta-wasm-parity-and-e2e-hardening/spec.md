# Spec 057 — Penta-Interface In-Memory WASM Parity & Sandboxed E2E Hardening

## 1. Overview & Problem Statement
PaperPilot's **Penta-Interface Mandate** strictly requires authentic evaluation across all 5 deployment surfaces:
1. 💻 **CLI** (`paperpilot-cli`)
2. 🤖 **MCP** (`paperpilot-mcp`)
3. 🌐 **REST API** (`paperpilot-gateway`)
4. ⚡ **WASM** (`paperpilot-wasm`)
5. ☁️ **Cloudflare Edge** (`apps/edge`)

While the master E2E suite (`tools/penta-interface-e2e`) executes 100% real binaries and endpoints for CLI, MCP, and REST (1,041 tests passing), several WASM and Cloudflare Edge assertions rely on simulated in-memory strings (`[Simulated] WasmEngine call`). Additionally, repeated local test runs write directly to a shared output folder (`tests/e2e_fixtures/out/penta_e2e/`), creating risk of stale-cache false passes.

This specification defines **Spec 057** to harden test integrity, sandboxing, and genuine in-memory WASM execution.

---

## 2. Technical Requirements

### A. Sandboxed Test Run Isolation & Dual-Check File Verification
1. **Clean Run Guarantee**:
   - In `tools/penta-interface-e2e/src/main.rs`, before executing test suites, cleanly reset or wipe temporary test output directories (`tests/e2e_fixtures/out/penta_e2e_*`).
   - Alternatively, isolate each test execution run using a unique run identifier/subfolder so that tests can NEVER pass against residual files from previous runs.
2. **Mandatory Dual-Check File Verification on Disk**:
   Every test generating a file must perform a two-step validation:
   - **Step 1 (Creation & Timestamp Check)**: Verify the file exists on disk, was created/modified during the current test execution window, and has a size $> 0$ bytes.
   - **Step 2 (Deep Content & Semantic Inspection)**:
     - For PDFs (`merge`, `split`, `rotate`, `watermark`, `crop`): Use `PdfAssertions` to verify exact page counts, text strings on specific pages, rotation dictionary tags (`/Rotate 90`), and encryption locks.
     - For Office to Markdown (`office_to_markdown`): Verify file contains table syntax (`|` and `---`) and expected table headers.
     - For Office Text Extraction (`office_extract_text`): Verify plain text output contains expected text lines without XML tags.
     - For Office Modernization (`office_modernize_legacy`): Inspect the output file header and verify it opens as a valid OpenXML ZIP archive (`[Content_Types].xml`).
     - For Spreadsheets to JSON/CSV (`excel_to_json`, `excel_to_csv`): Parse JSON to verify expected column keys and array length; parse CSV to verify delimiter formatting.

### B. True In-Memory WASM Parity Execution
1. **Direct In-Memory Buffer Calls**:
   - Replace simulated strings in `tools/penta-interface-e2e/src/cases/conversions_office_tools.rs`, `conversions_pdf_tools.rs`, and `analysis.rs` with real in-memory engine processing.
   - For office tools (`office_to_markdown`, `office_extract_text`, `office_modernize_legacy`), pass raw input byte slices directly to `office_oxide` and assert the returned byte streams (e.g. valid Markdown string, plain text without XML tags, or OpenXML ZIP buffer starting with `PK\x03\x04`).
   - For PDF operations (`pdf_rag_chunks`, `workspace_search`), pass in-memory PDF byte buffers and assert structured JSON/chunk arrays directly.
2. **Zero Mocking Enforcement**:
   - Disallow hardcoded `passed: true` without an underlying assertion evaluation on returned data.

### C. Deep Semantic Playwright Desktop Expansion (`Spec 055` Alignment)
1. **Full 50+ Tool Coverage**:
   - In `apps/desktop/tests/e2e_all_tools_real_execution.spec.ts`, expand the test matrix from 35 tools to all 50+ tools declared in `toolList.ts`.
2. **Deep Content Assertions**:
   - Verify output artifacts using `pdf-lib` (page count, dimensions, rotation angles, metadata dictionary) or `fs.readFileSync` (CSV delimiters, JSON schemas) rather than only checking that files exist.

---

## 3. Quality Gates & Success Metrics
1. `cargo fmt --all -- --check` must pass cleanly (0 diffs).
2. `cargo clippy -p penta-interface-e2e -p paperpilot-wasm -j 4 -- -D warnings` must pass with 0 warnings.
3. `cargo run -p penta-interface-e2e -- --group all -j 4` must achieve **100% authentic passes across all 1,041 tests**.
4. Desktop Playwright test suite passes cleanly against `http://localhost:1420`.
