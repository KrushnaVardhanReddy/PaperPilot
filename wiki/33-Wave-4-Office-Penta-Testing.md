# Phase 5.9.80 — Wave 4 (6 Tools) Office Modernization & Spreadsheets Penta-Interface E2E Testing

## Objective

This document outlines the testing architecture and verification for Wave 4 tools across all 5 supported interfaces (Penta-Interface Spec). The goal is to assert output correctness (size, structure, valid ZIP OpenXML header (`PK\x03\x04`), and Markdown table structures).

## The 6 Wave 4 Tools Covered

1. **`pdf_to_docx`**: Converts a PDF document to DOCX format by extracting its text.
2. **`pdf_to_xlsx`**: Converts a PDF document to XLSX format using best effort table extraction.
3. **`pdf_to_pptx`**: Converts a PDF document to PPTX format via best effort slide generation.
4. **`office_to_markdown`**: Convert Office documents (.docx, .xlsx, .pptx) to Markdown (.md).
5. **`office_extract_text`**: Extract text content from Microsoft Office documents (.docx, .xlsx, .pptx).
6. **`office_modernize_legacy`**: Modernize legacy Office documents (.doc, .xls, .ppt) to Modern Office format (.docx, .xlsx, .pptx).

## Verification Checks

- **Office output formats (`.docx`, `.xlsx`, `.pptx`)**: Asserts that files output magic bytes start with `[0x50, 0x4B]` (`PK` ZIP container) indicating valid OpenXML formatted ZIP files.
- **Markdown output formats (`.md`)**: Asserts structural integrity like the presence of table delimiters (`|` or `---`).
- **Standard Checks**: Outputs must be greater than 0 bytes and commands/responses must return a valid success flag.

## Command References (Penta-Interface Matrix)

### 1. `pdf_to_docx`
- **CLI**: `paperpilot convert --format docx input.pdf output.docx`
- **MCP Payload**:
  ```json
  {
    "name": "pdf_to_docx",
    "arguments": {
      "input": "input.pdf",
      "output": "output.docx"
    }
  }
  ```
- **REST API (`curl`)**:
  ```bash
  curl -X POST http://localhost:3000/api/tools/pdf_to_docx \
       -H "Content-Type: application/json" \
       -d '{"input": "input.pdf", "output": "output.docx"}'
  ```

### 2. `pdf_to_xlsx`
- **CLI**: `paperpilot convert --format xlsx input.pdf output.xlsx`
- **MCP Payload**:
  ```json
  {
    "name": "pdf_to_xlsx",
    "arguments": {
      "input": "input.pdf",
      "output": "output.xlsx"
    }
  }
  ```
- **REST API (`curl`)**:
  ```bash
  curl -X POST http://localhost:3000/api/tools/pdf_to_xlsx \
       -H "Content-Type: application/json" \
       -d '{"input": "input.pdf", "output": "output.xlsx"}'
  ```

### 3. `pdf_to_pptx`
- **CLI**: `paperpilot convert --format pptx input.pdf output.pptx`
- **MCP Payload**:
  ```json
  {
    "name": "pdf_to_pptx",
    "arguments": {
      "input": "input.pdf",
      "output": "output.pptx"
    }
  }
  ```
- **REST API (`curl`)**:
  ```bash
  curl -X POST http://localhost:3000/api/tools/pdf_to_pptx \
       -H "Content-Type: application/json" \
       -d '{"input": "input.pdf", "output": "output.pptx"}'
  ```

### 4. `office_to_markdown`
- **CLI**: `paperpilot office to-markdown input.xlsx output.md`
- **MCP Payload**:
  ```json
  {
    "name": "office_to_markdown",
    "arguments": {
      "input": "input.xlsx",
      "output": "output.md"
    }
  }
  ```
- **REST API (`curl`)**:
  ```bash
  curl -X POST http://localhost:3000/api/tools/office_to_markdown \
       -H "Content-Type: application/json" \
       -d '{"input": "input.xlsx", "output": "output.md"}'
  ```

### 5. `office_extract_text`
- **CLI**: `paperpilot office extract-text input.xlsx output.txt`
- **MCP Payload**:
  ```json
  {
    "name": "office_extract_text",
    "arguments": {
      "input": "input.xlsx",
      "output": "output.txt"
    }
  }
  ```
- **REST API (`curl`)**:
  ```bash
  curl -X POST http://localhost:3000/api/tools/office_extract_text \
       -H "Content-Type: application/json" \
       -d '{"input": "input.xlsx", "output": "output.txt"}'
  ```

### 6. `office_modernize_legacy`
- **CLI**: `paperpilot office modernize input.xls output_modern.xlsx`
- **MCP Payload**:
  ```json
  {
    "name": "office_modernize_legacy",
    "arguments": {
      "input": "input.xls",
      "output": "output_modern.xlsx"
    }
  }
  ```
- **REST API (`curl`)**:
  ```bash
  curl -X POST http://localhost:3000/api/tools/office_modernize_legacy \
       -H "Content-Type: application/json" \
       -d '{"input": "input.xls", "output": "output_modern.xlsx"}'
  ```

## Running the E2E Suite

### Rust (MCP Tests)
```bash
cargo test -p paperpilot-mcp --test wave4_mcp_tests -j 2
```
