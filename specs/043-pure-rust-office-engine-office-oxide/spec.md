# Spec 043 — High-Speed Office Document Engine (DOCX, XLSX, PPTX & Legacy Binary) via `office_oxide`

## 1. Overview & Problem Statement
Currently, PaperPilot has dedicated operations for PDF conversions, but handling incoming **Office formats** (`.docx`, `.xlsx`, `.pptx`, and legacy `.doc`, `.xls`, `.ppt`) presents challenges:
1. Converting spreadsheets (`.xlsx`) to structured tables or Markdown relies on basic parsers like `calamine` which do not support full write/edit pipelines.
2. Converting Word (`.docx`) and PowerPoint (`.pptx`) often requires external headless Office suites or complex multi-crate plumbing.
3. Legacy binary Office files (`.doc`, `.xls`, `.ppt`) from older legal/enterprise archives cannot be ingested directly without third-party converters.

`office_oxide` is a pure-Rust, permissive (MIT/Apache), ultra-high-speed (up to 100× faster than Python tools) library supporting all 6 Microsoft Office formats:
- **OOXML**: `.docx`, `.xlsx`, `.pptx` (Read, Write, and Edit)
- **Legacy Binary**: `.doc`, `.xls`, `.ppt` (Read & Convert to OOXML)
- Native conversion to clean Markdown, document text extraction, and Intermediate Representation (IR) JSON dump.
- Fully compatible with pure-Rust and WebAssembly (WASM).

**Objective:**
Integrate `office_oxide` into PaperPilot's conversion and ingestion pipeline to provide **bidirectional Office processing**, **instant legacy binary document ingestion**, and **high-speed Office $\to$ Markdown / PDF workflows**.

---

## 2. Target Features & Operations

### 2.1 Office-to-Markdown & Text Extraction (`office_to_markdown`, `office_to_text`)
- Supports `.docx`, `.xlsx`, `.pptx`, `.doc`, `.xls`, `.ppt`.
- Extracts structured Markdown (preserving tables from Excel and headings from Word).
- Sub-5ms text extraction for AI agent ingestion (`paperpilot-ai` / RAG).

### 2.2 Legacy Binary Modernizer (`office_convert_legacy`)
- Automatically modernizes legacy binary `.doc` $\to$ `.docx`, `.xls` $\to$ `.xlsx`, `.ppt` $\to$ `.pptx` in-memory.
- Solves enterprise legal discovery use-cases where old case archives contain `.doc` files from 1997–2003.

### 2.3 Office-to-PDF Modern Pipeline (`office_to_pdf`)
- In conjunction with PaperPilot's HTML/PDF renderers, converts Office files directly to PDFs without LibreOffice or Microsoft Office installed.

---

## 3. Penta-Interface Support
- **CLI**:
  - `paperpilot office extract --input doc.docx`
  - `paperpilot office to-markdown --input spreadsheet.xlsx`
  - `paperpilot office modernize --input legacy.doc --output modern.docx`
- **MCP**:
  - `office_extract_text`, `office_to_markdown`, `office_modernize_legacy`
- **REST Gateway**:
  - `POST /api/v1/office/convert`
- **WASM**:
  - In-browser local Office document inspection and text extraction without uploading files to any server.

---

## 4. Acceptance Criteria
1. Successfully parse and convert sample `.docx`, `.xlsx`, and `.pptx` documents into structured Markdown.
2. Modernize sample legacy `.doc` into `.docx`.
3. 100% in-memory execution with zero filesystem leaks.
4. All test runs bounded with `-j 6`.
5. Author `reports/OFFICE_OXIDE_INTEGRATION_REPORT.md`.
