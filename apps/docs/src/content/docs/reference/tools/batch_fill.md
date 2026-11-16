---
title: "batch_fill"
description: "Reference, CLI, MCP, and REST API specifications for batch_fill."
sidebar:
  order: 4
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) + CSV/JSON -> Directory of PDFs + `execution_receipt.json`

### What it does
**High-throughput AcroForm mail-merge engine for automated document generation with audit receipts.**  
Fills a local template PDF once per CSV/JSON record or inline JSON record array. Generates independent PDF files and writes a cryptographic `execution_receipt.json` containing SHA-256 hashes, timestamps, and row-level success/error reports. Flattens form appearances by default to prevent downstream tampering.

---

### CLI
`batch_fill` is invoked from the command line using the primary command `paperpilot fill-batch` (or aliases `batch-fill`, `cli_fill_batch`, `cli-fill-batch`):

```bash
paperpilot fill-batch \
  --template tests/e2e_fixtures/real/form.pdf \
  --data tests/e2e_fixtures/real/batch_data.csv \
  --output-dir tests/e2e_fixtures/out/batch9_test/batch_out \
  --name-template "{index:04}.pdf" \
  --json
```

Output:
```json
{"success":true,"operation":"FillBatch","error":null,"batch":{"data":"tests/e2e_fixtures/real/batch_data.csv","data_sha256":"f54db1a3d0d0ddd838ce6cacdfece9c4f5f5f03dc290d8e429adafc30f569a14","data_source":"file","date":"2026-11-16","execution_time_ms":9,"failed":0,"job_id":"fill-batch-1794843815207495-95","operations_applied":["pdf_fill_form","pdf_flatten"],"receipt_path":"tests/e2e_fixtures/out/batch9_test/batch_out/execution_receipt.json","results":[{"index":1,"output_path":"tests/e2e_fixtures/out/batch9_test/batch_out/0001.pdf","output_sha256":"a7c3074932506c87d17eadaa03290cc3df84da243a7b4c9f5426a03a0498402c","success":true}],"started_at":"2026-11-16T15:43:35.207495714+00:00","succeeded":4,"success":true,"template":"tests/e2e_fixtures/real/form.pdf","template_sha256":"25316f962c77870acaeb5b3317b2e467fa32ebe29e5574bf8c872df542a86cf1","total":4}}
```

#### CLI Parameters
- `--template` *(string, required)*: Path to base AcroForm PDF template.
- `--data` *(string, required)*: Path to UTF-8 CSV with headers or a JSON array file.
- `--output-dir` *(string, required)*: Directory to create generated PDF files and `execution_receipt.json`.
- `--name-template` *(string, optional, default: `"{index:04}.pdf"`)*: Dynamic filename pattern supporting `{row.column}`, `{date}`, `{index}`, and `{index:04}`.
- `--field-map` *(string, optional)*: Path to JSON mapping PDF field names to dataset column names.
- `--keep-editable` *(flag, optional)*: Retain interactive form fields instead of baking appearances into page streams.
- `--date` *(string, optional)*: Explicit UTC batch date (`YYYY-MM-DD`) for deterministic naming.
- `--json` *(flag, optional)*: Emit structured JSON receipt to stdout.

---

### MCP JSON-RPC
Invoke `batch_fill` via Model Context Protocol:

#### 1. Using Local Data File Path
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "batch_fill",
    "arguments": {
      "template": "tests/e2e_fixtures/real/form.pdf",
      "data": "tests/e2e_fixtures/real/batch_data.csv",
      "output_dir": "tests/e2e_fixtures/out/batch9_test/mcp_batch_out",
      "name_template": "{index:04}.pdf"
    }
  },
  "id": 1
}
```

#### 2. Using Inline JSON Records
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "batch_fill",
    "arguments": {
      "template": "tests/e2e_fixtures/real/form.pdf",
      "output_dir": "tests/e2e_fixtures/out/batch9_test/mcp_inline_out",
      "records": [
        {"FullName": "Ada Lovelace", "Email": "ada@example.org", "AccountType": "Enterprise", "AgreedToTerms": "Yes"},
        {"FullName": "Grace Hopper", "Email": "grace@example.org", "AccountType": "Enterprise", "AgreedToTerms": "Yes"}
      ],
      "keep_editable": true
    }
  },
  "id": 2
}
```

#### MCP Output Response
Returns the full serialized execution receipt:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"data\":\"tests/e2e_fixtures/real/batch_data.csv\",\"data_sha256\":\"f54db1a3d0d0ddd838ce6cacdfece9c4f5f5f03dc290d8e429adafc30f569a14\",\"data_source\":\"file\",\"date\":\"2026-11-16\",\"execution_time_ms\":9,\"failed\":0,\"job_id\":\"fill-batch-1794843815207495-95\",\"operations_applied\":[\"pdf_fill_form\",\"pdf_flatten\"],\"receipt_path\":\"/tests/e2e_fixtures/out/batch9_test/mcp_batch_out/execution_receipt.json\",\"results\":[...],\"started_at\":\"2026-11-16T15:43:35.207495714+00:00\",\"succeeded\":4,\"success\":true,\"template\":\"tests/e2e_fixtures/real/form.pdf\",\"template_sha256\":\"25316f96...\",\"total\":4}"
      }
    ],
    "isError": false
  }
}
```

---

### REST API
Execute batch operations via PaperPilot Gateway:

#### Endpoints
- Dedicated Batch Route: `POST /v1/automation/fill-batch`
- Generic Tool Route: `POST /api/v1/pdf/tools/batch_fill`

#### Request Payload
```bash
curl -s -X POST http://127.0.0.1:7823/v1/automation/fill-batch \
  -H "Content-Type: application/json" \
  -d '{
    "template": "tests/e2e_fixtures/real/form.pdf",
    "data": "tests/e2e_fixtures/real/batch_data.csv",
    "output_dir": "tests/e2e_fixtures/out/batch9_test/api_batch_out",
    "name_template": "{row.FullName}.pdf"
  }'
```

#### Status Codes & Responses
- **`200 OK`**: All batch records succeeded. Returns full JSON receipt.
- **`207 Multi-Status`**: Partial batch completion (one or more records failed, valid outputs preserved). Returns receipt detailing failed row indices and error causes.
- **`400 Bad Request`**: Configuration error, ambiguous data sources (both `data` and `records` provided), or existing receipt in destination directory.

---

### WASM Support
Batch operations rely on filesystem batch directory writing, multi-threaded record dispatch, and SHA-256 verification. Browser WASM environments operate on single-buffer streams; use individual [`pdf_fill_form`](/reference/tools/pdf_fill_form/) and [`pdf_flatten`](/reference/tools/pdf_flatten/) calls for client-side batching.

---

### Known Limitations & Troubleshooting
- **No Overwriting**: `batch_fill` strictly refuses to overwrite existing files. If an output directory already contains `execution_receipt.json` or generated PDFs that collide with target names, the batch fails.
- **Ambiguous Sources**: Exactly one of `data` (file path) or `records` (inline array) must be supplied; providing neither or both causes an immediate validation failure.
- **Record Validation**: Missing expected fields or nested objects/arrays in JSON records fail that individual row while permitting other rows to proceed.
