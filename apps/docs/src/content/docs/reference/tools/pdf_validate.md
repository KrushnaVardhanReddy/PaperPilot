---
title: "pdf_validate"
description: "Reference, CLI, MCP, and REST API commands for pdf_validate PDF document structural verification."
sidebar:
  order: 51
---

**Status**: ✅ Verified  |  **Format**: PDF $\to$ Validation Report (Text / JSON)

### What it does
**Runs deep diagnostic checks on PDF structure and compliance.**  
Validates cross-reference tables, font descriptors, catalog integrity, indirect objects, and identifies syntax errors or corruption warnings.

- **Structural Catalog Audit**: Confirms the presence and validity of the PDF document catalog (`/Root`), page tree nodes (`/Pages`), and object streams.
- **Cross-Reference Table Verification**: Ensures xref offsets and subsection indices point to valid byte offsets in the file.
- **Page Tree Integrity**: Recursively counts and validates every page object dictionary and verifies correct page hierarchy resolution.
- **Dual Output Modes**: Emits concise human-readable verification status or structured machine-readable JSON reports.

---

### CLI
`pdf_validate` is invoked via the CLI command `paperpilot validate`:

```bash
# Verify document structure with human-readable console output
paperpilot validate --input tests/e2e_fixtures/real/single_page.pdf

# Verify document structure with structured JSON response
paperpilot validate --input tests/e2e_fixtures/real/single_page.pdf --json

# Save structured validation report directly to disk
paperpilot validate \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/val_report.json \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input PDF file to validate.
- `--output` *(string, optional)*: Destination file path to write the JSON report.
- `--json` *(flag, optional)*: Output operation status as JSON to stdout.

---

### MCP JSON-RPC
Invoke `pdf_validate` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_validate",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf"
    }
  },
  "id": 1
}
```

#### With Report Output File
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_validate",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/val_report.json"
    }
  },
  "id": 2
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: The input PDF file path.
- `output` *(string, optional)*: Path to save the JSON validation report.

---

### REST API
Execute structural validation via the PaperPilot Gateway:

```bash
# Dedicated Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_validate \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }'
```

```bash
# Universal Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_validate \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/val_report.json"
  }'
```

---

### Return Structure & Output Schema
When evaluated, `pdf_validate` produces:

```json
{
  "file": "tests/e2e_fixtures/real/single_page.pdf",
  "is_valid": true,
  "page_count": 1,
  "status": "VALID_PDF_STRUCTURE"
}
```

When returned via MCP or Gateway REST wrapper:
```json
{
  "success": true,
  "message": "PDF is valid (1 pages)",
  "output_path": "true",
  "data": {
    "file": "tests/e2e_fixtures/real/single_page.pdf",
    "is_valid": true,
    "page_count": 1,
    "status": "VALID_PDF_STRUCTURE"
  }
}
```

---

### Known Limitations & Troubleshooting
- **Digital Signatures vs Document Structure**: `pdf_validate` performs structural validation of the PDF syntax tree and catalog. To verify cryptographic signature integrity, PKCS#7 byte ranges, or certificate chains, use [`pdf_validate_signatures`](/reference/tools/pdf_validate_signatures/).
- **Encrypted PDFs**: Documents protected with an owner or user password must be decrypted with `paperpilot decrypt` before structural inspection, or the parser will fail with a decryption error.
- **WASM Support**: Structural validation is supported across CLI, MCP, and REST API. For web browser client runtimes, use the REST gateway or verify signatures via `@paperpilot/wasm`.
