---
title: "cli_fill_batch"
description: "Reference, CLI, MCP, and REST API specifications for cli_fill_batch."
sidebar:
  order: 3
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) + CSV/JSON -> Output Directory + `execution_receipt.json`

### What it does
**Command-line batch mail-merge engine for generating populated, flattened AcroForm PDFs with audit receipts.**  
Parses the input PDF template once, generates populated PDF documents for every record in a CSV or JSON dataset, and saves an `execution_receipt.json` audit manifest containing row outcomes, execution durations, and SHA-256 digests. Powered by the same underlying core engine as [`batch_fill`](/reference/tools/batch_fill/).

---

### CLI
Run batch generation via `paperpilot fill-batch` (or `paperpilot cli_fill_batch`):

#### 1. Standard CSV Merge with Dynamic Filename Formatting
```bash
paperpilot fill-batch \
  --template tests/e2e_fixtures/real/form.pdf \
  --data tests/e2e_fixtures/real/batch_data.csv \
  --output-dir tests/e2e_fixtures/out/batch9_test/cli_fill_batch_out \
  --name-template "{row.FullName}.pdf" \
  --json
```

Output:
```json
{"success":true,"operation":"FillBatch","error":null,"batch":{"data":"tests/e2e_fixtures/real/batch_data.csv","data_sha256":"f54db1a3d0d0ddd838ce6cacdfece9c4f5f5f03dc290d8e429adafc30f569a14","data_source":"file","date":"2026-11-16","execution_time_ms":44,"failed":0,"job_id":"fill-batch-1794843820820392-93","operations_applied":["pdf_fill_form","pdf_flatten"],"receipt_path":"tests/e2e_fixtures/out/batch9_test/cli_fill_batch_out/execution_receipt.json","results":[{"index":1,"output_path":"tests/e2e_fixtures/out/batch9_test/cli_fill_batch_out/Jane Doe.pdf","output_sha256":"a7c3074932506c87d17eadaa03290cc3df84da243a7b4c9f5426a03a0498402c","success":true}],"started_at":"2026-11-16T15:43:40.820392996+00:00","succeeded":4,"success":true,"template":"tests/e2e_fixtures/real/form.pdf","template_sha256":"25316f962c77870acaeb5b3317b2e467fa32ebe29e5574bf8c872df542a86cf1","total":4}}
```

#### 2. Numbered Outputs with Editable Fields Preserved
```bash
paperpilot fill-batch \
  --template tests/e2e_fixtures/real/form.pdf \
  --data tests/e2e_fixtures/real/batch_data.csv \
  --output-dir tests/e2e_fixtures/out/batch9_test/cli_editable_out \
  --name-template "{index:04}.pdf" \
  --keep-editable \
  --json
```

#### CLI Parameters
- `--template` *(string, required)*: Local AcroForm PDF template.
- `--data` *(string, required)*: UTF-8 CSV with header row, or a JSON array of record objects.
- `--output-dir` *(string, required)*: Output directory where filled PDFs and `execution_receipt.json` will be written.
- `--name-template` *(string, optional, default: `"{index:04}.pdf"`)*: Filename pattern supporting `{row.column}`, `{date}`, `{index}`, and `{index:04}`.
- `--field-map` *(string, optional)*: Path to JSON mapping object translating PDF form field names to data column names.
- `--keep-editable` *(flag, optional)*: Retain interactive PDF fields instead of baking form appearances into page content.
- `--date` *(string, optional)*: Fixed `YYYY-MM-DD` date override for deterministic, repeatable batch runs.
- `--json` *(flag, optional)*: Print JSON batch summary and execution receipt to stdout.

---

### Execution Receipts
Every batch run generates a tamper-evident `execution_receipt.json` in the target directory:

```json
{
  "success": true,
  "job_id": "fill-batch-1794843820820392-93",
  "started_at": "2026-11-16T15:43:40.820392996+00:00",
  "date": "2026-11-16",
  "template": "tests/e2e_fixtures/real/form.pdf",
  "data": "tests/e2e_fixtures/real/batch_data.csv",
  "data_source": "file",
  "template_sha256": "25316f962c77870acaeb5b3317b2e467fa32ebe29e5574bf8c872df542a86cf1",
  "data_sha256": "f54db1a3d0d0ddd838ce6cacdfece9c4f5f5f03dc290d8e429adafc30f569a14",
  "receipt_path": "tests/e2e_fixtures/out/batch9_test/cli_fill_batch_out/execution_receipt.json",
  "operations_applied": [
    "pdf_fill_form",
    "pdf_flatten"
  ],
  "total": 4,
  "succeeded": 4,
  "failed": 0,
  "execution_time_ms": 44,
  "results": [
    {
      "index": 1,
      "success": true,
      "output_path": "tests/e2e_fixtures/out/batch9_test/cli_fill_batch_out/Jane Doe.pdf",
      "output_sha256": "a7c3074932506c87d17eadaa03290cc3df84da243a7b4c9f5426a03a0498402c"
    }
  ]
}
```

---

### MCP JSON-RPC
`cli_fill_batch` is exposed in Model Context Protocol clients as an identical tool alias to `batch_fill`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "cli_fill_batch",
    "arguments": {
      "template": "tests/e2e_fixtures/real/form.pdf",
      "data": "tests/e2e_fixtures/real/batch_data.csv",
      "output_dir": "tests/e2e_fixtures/out/batch9_test/mcp_cli_batch",
      "name_template": "{row.FullName}.pdf"
    }
  },
  "id": 1
}
```

---

### REST API
Execute via REST API through the gateway automation endpoints:

```bash
curl -s -X POST http://127.0.0.1:7823/v1/automation/fill-batch \
  -H "Content-Type: application/json" \
  -d '{
    "template": "tests/e2e_fixtures/real/form.pdf",
    "data": "tests/e2e_fixtures/real/batch_data.csv",
    "output_dir": "tests/e2e_fixtures/out/batch9_test/api_cli_batch_out",
    "name_template": "{row.FullName}.pdf"
  }'
```

---

### WASM Support
Not applicable in browser WASM. Batch file creation, collision checks, receipt generation, and directory isolation require native filesystem access.

---

### Known Limitations & Troubleshooting
- **Existing Receipts**: Re-running a batch into a directory that already contains `execution_receipt.json` causes an error to prevent overwriting prior run audits.
- **Filename Sanitization**: Tokens containing path separators (`/`, `\`) or reserved characters (`:`, `*`, `?`, `"`, `<`, `>`, `|`) are converted to underscores `_` to guarantee filesystem safety.
- **Header Matching**: Column headers must correspond with PDF field names. If names differ, provide a `--field-map` mapping JSON file.
