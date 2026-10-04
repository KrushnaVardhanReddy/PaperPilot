# Spec: Phase 4.E2E-R3 — Tri-Interface E2E Verification & Interactive Living Documentation (CLI, MCP, REST API)

## 1. Overview & Objective
Now that PaperPilot has complete implementations of CLI, MCP, and REST API across all 44 tools, we require a comprehensive, tri-interface end-to-end verification harness and interactive living documentation suite.

### Primary Goals:
1. **Living Reference Documentation**:
   - The test report serves as the definitive reference handbook for PaperPilot developers and users.
   - For every tool, it provides copy-pasteable, verified working examples for:
     - **CLI**: Exact shell command with flags and expected JSON output.
     - **MCP**: Exact JSON-RPC `tools/call` frame piped over `stdio`.
     - **REST API**: Exact `curl` command with endpoint URL, headers, and request body.
2. **Tri-Interface Parity Validation**:
   - Executes and verifies all 44 PDF operations across all three interfaces:
     - **CLI**: Executed via `target/debug/paperpilot-cli` (or release) using `--json`.
     - **MCP**: Executed via `target/debug/paperpilot-mcp` using JSON-RPC standard stdio frames.
     - **REST API**: Executed via HTTP against `paperpilot-gateway` (`http://127.0.0.1:7823`).
3. **Deep State & File Inspection**:
   - Takes a **Pre-Execution Snapshot** of the input fixture (file path, file size in bytes, page count).
   - Executes the operation across all 3 interfaces.
4. **Check in Test PDFs (Input & Output Artifacts)**:
   - All input fixture PDFs and resulting output test PDFs/artifacts generated during verification must be committed and checked into git (under `tests/e2e_fixtures/` and `tests/e2e_fixtures/out/tri_e2e/`) so that the documentation has actual, real downloadable/inspectable test PDFs associated with each run.
5. **Strict "Report, Don't Fix" Rule**:
   - If a tool fails on any interface, the test harness must **NOT** mask, ignore, or modify underlying tool logic. It must accurately record the failure status, error message, exit code, and root cause in the generated report.

---

## 2. Architecture & File Structure

### 2.1 Test Harness
- **Script**: `scripts/test_tri_interface_e2e.py`
  - Automated Python 3 script.
  - Automatically spins up `paperpilot-gateway` locally on port 7823 in the background (and cleanly stops it on exit).
  - Automatically verifies `paperpilot-mcp` and `paperpilot-cli` binaries exist (or triggers build).
  - Iterates through all 44 tools across CLI, MCP, and REST API.
  - Performs input snapshot, runs tool, inspects output, checks assertions.
  - Generates the markdown documentation & report at `reports/TRI_INTERFACE_E2E_AND_DOCS.md`.

### 2.2 Report & Documentation File
- **Path**: `reports/TRI_INTERFACE_E2E_AND_DOCS.md`
- Contains:
  1. **Executive Scorecard**: Total tools (44), CLI pass rate, MCP pass rate, REST API pass rate, average latency per interface.
  2. **Quick Reference Matrix**: A summary table of all 44 tools with their working status on CLI, MCP, and API.
  3. **Detailed Tool-by-Tool Documentation**:
     - Description of what the tool does.
     - Input PDF details (size, pages).
     - **Working Examples**:
       - 💻 CLI Example (full command)
       - 🤖 MCP Example (JSON-RPC payload)
       - 🌐 REST API Example (`curl` command)
     - Output validation results (output path, size before/after, page change, status PASS/FAIL).

---

## 3. The 44 Operations Catalog

### Group 1: Structural & Page Operations (10 Tools)
1. `pdf_merge`: Merge multiple PDFs into one.
2. `pdf_split`: Split PDF by page ranges or extract pages.
3. `pdf_rotate`: Rotate pages by 90, 180, or 270 degrees.
4. `pdf_extract_pages`: Extract subset of pages into a new PDF.
5. `pdf_delete_pages`: Delete specific pages from a PDF.
6. `pdf_reorder_pages`: Reorder pages based on index list.
7. `pdf_burst`: Split every page into an individual PDF file.
8. `pdf_crop`: Crop page margins by coordinates.
9. `pdf_remove_blank`: Detect and remove blank pages automatically.
10. `pdf_page_numbers`: Stamp dynamic page numbers.

### Group 2: Security & Integrity Operations (7 Tools)
11. `pdf_encrypt`: Encrypt PDF with user/owner password.
12. `pdf_decrypt`: Decrypt password-protected PDF.
13. `pdf_redact`: Black out and redact text/coordinates.
14. `pdf_sign`: Digital signature / stamp certification.
15. `pdf_validate`: Structural validation & corruption check.
16. `pdf_hash`: Compute cryptographic integrity hash.
17. `pdf_repair`: Repair corrupted xref tables and trailers.

### Group 3: Content Transformation & OCR (9 Tools)
18. `pdf_extract_text`: Extract plain text from PDF pages.
19. `pdf_extract_images`: Extract embedded raster images.
20. `pdf_images_to_pdf`: Convert image files into a single PDF.
21. `pdf_render`: Render PDF page to PNG image.
22. `pdf_ocr`: OCR scanned PDF pages into searchable text.
23. `pdf_search`: Search query string across document pages.
24. `pdf_bates`: Bates numbering / legal indexing stamp.
25. `pdf_watermark`: Text or diagonal watermark overlay.
26. `pdf_header_footer`: Header and footer text stamps.

### Group 4: Forms, Metadata & Optimization (8 Tools)
27. `pdf_read_form`: Read AcroForm fields and values as JSON.
28. `pdf_fill_form`: Fill AcroForm fields with provided JSON data.
29. `pdf_create_form_field`: Inject new text field or checkbox.
30. `pdf_metadata`: Read and update PDF document metadata.
31. `pdf_bookmarks`: Extract document outline and bookmarks.
32. `pdf_compress`: Optimize and compress PDF streams.
33. `pdf_linearize`: Linearize PDF for fast web viewing.
34. `pdf_flatten`: Flatten interactive annotations into page content.

### Group 5: Advanced Conversion & Intelligence (10 Tools)
35. `pdf_to_docx`: Convert PDF to editable Word `.docx`.
36. `pdf_to_xlsx`: Convert PDF tables to Excel `.xlsx`.
37. `pdf_to_pptx`: Convert PDF to PowerPoint `.pptx`.
38. `pdf_to_pdf_a`: Convert PDF to archival standard PDF/A.
39. `pdf_classify_type`: Classify PDF document category (Invoice, Contract, Form, etc.).
40. `pdf_compare`: Compare two PDFs and flag visual/text diffs.
41. `pdf_annotate`: Add sticky notes, highlights, and markup.
42. `pdf_convert_html`: Convert HTML to styled PDF with CSS presets.
43. `pdf_convert_markdown`: Convert Markdown to styled PDF with CSS presets.
44. `pdf_convert_excel`: Convert CSV/Excel to semantic HTML styled PDF.

---

## 4. Verification & Reporting Protocol
- **Input Fixture Snapshot**: Compute size in bytes and page count before running each test.
- **Run on CLI**: Execute command, record latency, exit code, stdout.
- **Run on MCP**: Execute JSON-RPC over stdio, record latency, response.
- **Run on REST API**: Execute curl / HTTP POST to gateway, record latency, response.
- **Output Verification**:
  - File exists at target output path.
  - Output file size > 0.
  - Page count verified against expected logic.
  - Format validation (magic bytes or format-specific verification).
- **Rule of Engagement**: If any test fails, record failure reason in `reports/TRI_INTERFACE_E2E_AND_DOCS.md` — do NOT mask failures or skip tools.
