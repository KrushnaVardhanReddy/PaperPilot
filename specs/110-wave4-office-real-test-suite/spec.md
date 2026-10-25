# Spec 110: Wave 4 (Office Modernization & Spreadsheets) Unmocked E2E & Real Testing Suite

## 1. Executive Summary & Objective
Wave 4 (`v0.8.0`) represents PaperPilot's native Office Modernization and Spreadsheet conversion engine:
1. `pdf_to_docx`: Converts PDF documents to editable Microsoft Word OpenXML (`.docx`).
2. `pdf_to_xlsx`: Extracts tabular grids into multi-sheet Excel workbooks (`.xlsx`).
3. `pdf_to_pptx`: Converts PDF presentations into editable PowerPoint slides (`.pptx`).
4. `office_to_markdown`: Converts Office documents to clean Markdown tables and text (`.md`).
5. `office_extract_text`: Extracts raw text streams from Office documents without office suite dependencies.
6. `office_modernize_legacy`: Transcodes legacy binary formats (`.doc`, `.xls`, `.ppt`) to modern OpenXML (`.docx`, `.xlsx`, `.pptx`).

While these tools are integrated into `paperpilot-pdf` and `paperpilot-cli`, they need a **dedicated unmocked test suite** mirroring Wave 2 (`paperpilot-mcp/tests/wave2_mcp_tests.rs`) and explicit wiki documentation to provide testers with copy-pasteable verification commands.

---

## 2. Deliverables & Scope

### 2.1 Dedicated MCP Integration Test (`paperpilot-mcp/tests/wave4_mcp_tests.rs`)
Direct in-memory MCP execution test mirroring `wave2_mcp_tests.rs`:
- Tests all 6 Wave 4 operations:
  - `pdf_to_docx`: Asserts `.docx` output exists, size > 0, and valid ZIP magic header (`PK\x03\x04`).
  - `pdf_to_xlsx`: Asserts `.xlsx` output exists, size > 0, and valid ZIP magic header (`PK\x03\x04`).
  - `pdf_to_pptx`: Asserts `.pptx` output exists, size > 0, and valid ZIP magic header (`PK\x03\x04`).
  - `office_to_markdown`: Asserts `.md` output exists and contains markdown table formatting (`|` and `---`).
  - `office_extract_text`: Asserts extracted text contains expected fixture tokens.
  - `office_modernize_legacy`: Asserts modernized `.xlsx` output exists and is a valid ZIP package.

### 2.2 Living Tester Documentation (`wiki/33-Wave-4-Office-Penta-Testing.md`)
A complete tester guide mirroring `wiki/09-Wave-2-Penta-Testing.md` detailing:
- Exact CLI commands with sample fixtures.
- Exact MCP JSON-RPC payloads.
- REST API `curl` commands.
- Expected assertion checks (magic bytes, valid OpenXML structure, exit codes).
