---
title: "pdf_compare"
description: "Reference, CLI, MCP, and REST API commands for pdf_compare."
sidebar:
  order: 12
---

**Status**: ✅ Verified  |  **Format**: 2 PDFs $\to$ JSON / Diff PDF

### What it does
**Compares two PDF documents page-by-page and generates a structured comparison report.**  
`pdf_compare` inspects text streams, font structures, and visual elements across both documents. It returns whether differences were detected (`diff_detected: boolean`) alongside the list of divergent page numbers (`pages: number[]`).

### CLI
```bash
# Compare two PDFs and output JSON execution summary
paperpilot compare \
  --input tests/e2e_fixtures/real/merge_a.pdf \
  --input-b tests/e2e_fixtures/real/merge_b.pdf \
  --json

# Write structured diff report directly to JSON file
paperpilot compare \
  --input tests/e2e_fixtures/real/merge_a.pdf \
  --input-b tests/e2e_fixtures/real/merge_b.pdf \
  --output /tmp/diff_report.json
```

**CLI Output (`/tmp/diff_report.json`):**
```json
{
  "diff_detected": true,
  "pages": [
    1
  ]
}
```

### MCP JSON-RPC
Tool definition: `pdf_compare` in `paperpilot-mcp/src/server.rs`.
```json
{
  "name": "pdf_compare",
  "arguments": {
    "input_a": "tests/e2e_fixtures/real/merge_a.pdf",
    "input_b": "tests/e2e_fixtures/real/merge_b.pdf",
    "output": "/tmp/diff_report.json"
  }
}
```

**MCP Response:**
```json
{
  "success": true,
  "diff_detected": true,
  "message": "Differences detected.",
  "output_path": "/tmp/diff_report.json",
  "data": null
}
```

### REST API
Dedicated routes: `POST /api/v1/pdf/pdf_compare` or `POST /api/v1/pdf/tools/pdf_compare` (Local Gateway default: port 7823). Accepts JSON payload or multipart form upload.
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_compare \
  -H 'Content-Type: application/json' \
  -d '{
    "file1": "tests/e2e_fixtures/real/merge_a.pdf",
    "file2": "tests/e2e_fixtures/real/merge_b.pdf",
    "output": "/tmp/diff_report.json"
  }'
```

**REST API Response:**
```json
{
  "success": true,
  "diff_detected": true,
  "message": "Differences detected.",
  "output_path": "/tmp/diff_report.json",
  "data": null
}
```

### Parameter Reference
| Option | CLI Flag | Type | Required | Description |
|---|---|---|---|---|
| `input_a` / `file1` | `--input` | `string` | Yes | Path to the baseline PDF document. |
| `input_b` / `file2` | `--input-b` | `string` | Yes | Path to the candidate PDF document to compare against. |
| `output` | `--output` | `string` | No | Path to write the JSON difference report (default in MCP: `diff_report.json`). |

### Known Limitations & Troubleshooting
- **Input Parameter Aliases**: The CLI uses `--input` and `--input-b`. MCP accepts `input_a`/`input_b`, `file_a`/`file_b`, or `file1`/`file2`. REST API supports both `file1`/`file2` and `input_a`/`input_b`.
- **Page Counts**: If document page counts differ, all trailing or missing pages are marked as changed pages in the diff report.
- **WASM Support**: Native document comparison relies on structural stream inspection; browser-side comparisons via WASM can inspect individual page text or compute SHA-256 hashes using `PdfInspector.sha256_hex()`.
