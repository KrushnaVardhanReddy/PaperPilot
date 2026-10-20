# Spec 055 — Full Catalog Penta-Interface & Unmocked Desktop UI Parity Suite (Deep Semantic Validation)

## 1. Overview & Problem Statement
PaperPilot's **Penta-Interface Mandate** ([`wiki/25-Penta-Interface-Mandate-And-E2E-Protocol.md`](../../wiki/25-Penta-Interface-Mandate-And-E2E-Protocol.md)) strictly dictates that every document tool must be exposed and verified across all **5 deployment surfaces**:
1. 💻 **CLI** (`paperpilot-cli`)
2. 🤖 **MCP** (`paperpilot-mcp`)
3. 🌐 **REST API** (`paperpilot-gateway`)
4. ⚡ **WASM** (`paperpilot-wasm`)
5. ☁️ **Cloudflare Edge** (`apps/edge`)
And seamlessly orchestrated through the **Desktop UI** (Tauri + Svelte 5).

### Core Principle: Zero Generic Assertions & Deep Semantic Validation
Automated tests that merely check if an output file was generated or if a PDF header exists (`%PDF-`) provide false confidence. A test must **deeply validate semantic correctness**:
- Did `rotate` actually change the page dictionary `/Rotate 90`?
- Did `merge` actually produce a document whose page count equals $pageCount(A) + pageCount(B)$?
- Did `split` extract the exact subset of requested pages?
- Did `watermark` inject the exact requested text into the content stream?
- Did `encrypt` make the PDF unreadable without the password?
- Did `excel_to_json` parse exact column headers, numerical cells, and expected row counts?
- Did `pdf_to_json` extract the correct string content and bounding boxes?

This specification defines the deep semantic assertions across all **5 Interfaces AND the Desktop UI** for the full 50+ tool catalog.

---

## 2. Technical Architecture & Semantic Assertions

### PART 1: The 5-Interface Rust Engine Suite (`tools/penta-interface-e2e`)
Expand `tools/penta-interface-e2e` for the remaining conversion and office tools across 4 complexity tiers (Simple, Medium, Complex, Negative) on all 5 surfaces:
1. `pdf_to_json`:
   - Validates JSON parseability.
   - Asserts schema structure (`pages[].text`, `pages[].images`, `pages[].page_number`).
   - Asserts specific keywords from `tests/e2e_fixtures/real/single_page.pdf` are present in `pages[0].text`.
   - In negative tier: Asserts invalid binary returns a structured error object.
2. `pdf_to_markdown`:
   - Asserts output contains expected text paragraphs and Markdown formatting syntax (`#`, `*`, or tables).
3. `office_to_markdown`:
   - Ingests test `.xlsx` or `.docx` fixture.
   - Asserts tables are rendered into Markdown table syntax (`| --- |`) with exact header names.
4. `office_extract_text`:
   - Asserts plain text output contains exact cell values or paragraph sentences without XML tags.
5. `office_modernize_legacy`:
   - Asserts output file is a valid ZIP container containing `[Content_Types].xml` conforming to OpenXML standards.

---

### PART 2: Unmocked Desktop UI Suite with Deep Content Validation (`apps/desktop/tests/e2e_all_tools_real_execution.spec.ts`)
Run Playwright against `http://127.0.0.1:1420/` without mocking `invoke_mcp_tool`.

#### Semantic Verification Rules per Category:
1. **Merge (`pdf_merge`)**:
   - Inputs: `merge_a.pdf` (1 page) + `merge_b.pdf` (1 page).
   - Assertion: Parse output with `pdf-lib` or Rust inspect to verify the resulting PDF has **exactly 2 pages**, containing content from both source documents.
2. **Split (`pdf_split`)**:
   - Input: Multi-page document (`multi_page.pdf`).
   - Split parameter: `1-1`.
   - Assertion: Generated PDF has **exactly 1 page** matching page 1.
3. **Rotate (`pdf_rotate`)**:
   - Parameter: Angle `90`.
   - Assertion: Inspect page dictionary in generated file to verify `/Rotate 90` is set.
4. **Watermark (`pdf_watermark`)**:
   - Parameter: Text `"CONFIDENTIAL_TEST"`.
   - Assertion: Extract text from output PDF; verify `"CONFIDENTIAL_TEST"` appears in the text stream or annotation layer.
5. **Delete Pages (`pdf_delete_pages`)**:
   - Input: 2-page document; remove page 1.
   - Assertion: Resulting PDF page count is reduced by 1.
6. **Encryption (`pdf_encrypt`)**:
   - Parameter: Password `"SecretPass123"`.
   - Assertion: Attempting to open without password throws encryption error; opening with `"SecretPass123"` succeeds.
7. **Excel to JSON (`excel_to_json`)**:
   - Input: `test.xlsx`.
   - Assertion: Parse output JSON file; verify it is an Array of Objects whose keys match the spreadsheet headers and cell types (numbers as numbers, strings as strings).
8. **Excel to CSV (`excel_to_csv`)**:
   - Input: `test.xlsx`.
   - Assertion: Read output CSV text; verify header row matches Excel column names with comma delimiters and correct line endings.
9. **JSON to PDF (`json_to_pdf`)**:
   - Input: `sample_doc.json` containing specific text blocks.
   - Assertion: Synthesized PDF is valid `%PDF-` and contains the text blocks declared in the JSON.
10. **Form Read / Fill (`read_form`, `fill_form`)**:
    - Input: `form.pdf`.
    - Fill parameter: Field `"TestText" = "VerifiedValue"`.
    - Assertion: Read back filled form; field value equals `"VerifiedValue"`.

---

## 3. Scorecard Generation & Quality Criteria
- **Penta-Interface Report**: `reports/PENTA_INTERFACE_EXPANSION_ALL_TOOLS_REPORT.md`.
- **Desktop UI Scorecard**: `reports/ALL_50_TOOLS_REAL_DESKTOP_E2E_SCORECARD.md`.
- **Zero Generic Checks**: Every test must assert semantic correctness (page counts, dictionary flags, text contents, or JSON schemas).
- **100% Pass Rate**: Zero flakiness, zero unhandled errors.
