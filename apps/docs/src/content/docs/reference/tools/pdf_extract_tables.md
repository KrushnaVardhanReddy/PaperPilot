---
title: "pdf_extract_tables"
description: "Reference, CLI, MCP, and REST API commands for pdf_extract_tables."
sidebar:
  order: 48
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ JSON (`.json`) / CSV (`.csv`)

### What it does
**Extracts structured tabular data from PDF documents into JSON or CSV.**  
Automatically parses text baseline layout, column alignments, and grid boundaries to reconstruct clean, multi-row tabular records without manual copy-pasting.

---

### CLI
Extract tables via the PaperPilot CLI using `extract-tables`:

#### Export to JSON
```bash
paperpilot extract-tables \
  --input tests/e2e_fixtures/real/financial_tables.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/tables.json \
  --format json \
  --pages all \
  --json
```

#### Export to CSV
```bash
paperpilot extract-tables \
  --input tests/e2e_fixtures/real/financial_tables.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/tables.csv \
  --format csv \
  --pages all \
  --json
```

#### CLI Parameters
- `--input <INPUT>` *(string, required)*: Path to the source PDF document.
- `--output <OUTPUT>` *(string, optional)*: Destination file path for JSON or CSV output.
- `--format <FORMAT>` *(string, optional, default: `json`)*: Target table format (`json` or `csv`).
- `--pages <PAGES>` *(string, optional, default: `all`)*: 1-based page indices or ranges (e.g. `1`, `1-3`, `all`).
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_extract_tables`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_tables",
    "arguments": {
      "input": "tests/e2e_fixtures/real/financial_tables.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/tables.json",
      "format": "json",
      "pages": "all"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `output` *(string, optional)*: Path where extracted tables are saved. If omitted, structured data is returned directly in the response payload.
- `format` *(string, optional)*: Output serialization format (`json` or `csv`).
- `pages` *(string, optional)*: Page selection (`all` or page range like `'1,2'`).

---

### REST API
Extract tables via PaperPilot Gateway:

#### 1. Direct Route (`/api/v1/pdf/pdf_extract_tables`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_extract_tables \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/financial_tables.pdf",
    "format": "json",
    "pages": "all"
  }'
```

#### 2. Universal Tool Route (`/api/v1/pdf/tools/pdf_extract_tables`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_extract_tables \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/financial_tables.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/tables.csv",
    "format": "csv"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "6d52ef98-5b72-40f2-b206-6b4d9a7a7656",
  "message": "Success",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Extracted 4 tables successfully.\",\"output_path\":null,\"data\":{\"total_tables\":4,\"tables\":[{\"page\":1,\"table_index\":1,\"rows\":12,\"cols\":5,\"headers\":[\"FinancialMetric\",\"FY2022\",\"FY2023\",\"FY2024\",\"GrowthRate\"],\"data\":[[\"GrossRevenue\",\"12450000\",\"15800000\",\"21300000\",\"+34.8%\"]]}]}}"
      }
    ]
  }
}
```

---

### Known Limitations & Troubleshooting
- **Borderless / Implicit Tables**: Tables relying entirely on whitespace without consistent vertical column baselines may be detected as flowing paragraph text rather than discrete table cells.
- **Multi-Table CSV Concatenation**: When exporting documents containing multiple distinct tables to CSV, each table matrix is separated by double newlines (`\n\n`) within the single CSV output file.
- **Embedded Scans**: Pure image scans must undergo OCR preprocessing via [`pdf_ocr`](/reference/tools/pdf_ocr) before table detection can parse textual coordinates.
