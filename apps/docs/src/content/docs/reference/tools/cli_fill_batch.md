---
title: "cli_fill_batch"
description: "Generate one filled AcroForm PDF per CSV or JSON record, with an execution receipt."
sidebar:
  order: 3
---

### What it does

`paperpilot fill-batch` performs local AcroForm mail merge. It parses the template
once, fills an independent copy for each record, and writes a local
`execution_receipt.json` with row outcomes, paths, SHA-256 hashes and timings.
PDF processing uses the same in-memory form engine as the single-document tools.

Text fields (including multiline fields) and checkboxes are supported. Values
are stored using PDF text-string encoding, including UTF-16 for Chinese text.
Appearances use Helvetica for ASCII and the standard STSong CJK fallback for
non-ASCII BMP text. Viewers need standard CJK font support; fonts are not
downloaded or embedded by this command. Non-BMP values, unsupported field types,
read-only templates and XFA forms are rejected explicitly. Values too long to
fit at a readable size fail that row instead of being silently clipped.

### CLI

```bash
paperpilot fill-batch --template form.pdf --data records.csv --output-dir ./out \
  --name-template '{row.name}_{date}_{index:04}.pdf' --json
```

The default filename is `{index:04}.pdf`. Supported tokens are `{row.column}`,
`{date}`, `{index}`, and `{index:04}` (padding up to 12 digits). `{date}` uses the
UTC batch date; `--date YYYY-MM-DD` provides an explicit, reproducible override.
Column substitutions replace path separators and forbidden filename characters
with underscores. Reserved Windows names, oversized names and paths are rejected.

Example CSV:

```csv
name,email,subscribe
Alice,alice@example.test,true
李雷,,false
```

Or use a JSON array with `--data records.json`:

```json
[
  {"name": "张三", "email": null, "subscribe": true},
  {"name": "Alice", "email": "alice@example.test", "subscribe": false}
]
```

CSV headers must be unique and nonempty. UTF-8 BOMs, quoted commas and quoted
multiline CSV values are accepted. JSON record values must be strings, numbers,
booleans or null; null becomes an empty string. Missing keys fail that row. Extra
columns can be used for naming without being sent to the form engine.

Field names match column names exactly, including fully qualified hierarchical
names such as `customer.name`. Override individual bindings with
`--field-map mapping.json`; keys are PDF field names and values are source columns:

```json
{"customer.name": "name", "customer.email": "email"}
```

Checkboxes accept true/false, 1/0, yes/no, on/off or the template's checked export
state. Empty values clear the checkbox. Every supported form field needs a bound
column; an empty string is different from a missing column.

### Output and failures

By default, form appearances are painted into page content and the widgets are
removed. Existing page content and unrelated annotations remain. Hidden widgets
are removed without revealing their values. `--keep-editable` preserves filled
interactive fields with regenerated appearances and checkbox states.

Output files are published without overwriting existing PDFs. Duplicate names
fail the later row. Other rows continue. An existing `execution_receipt.json`
prevents reusing an old batch directory; choose a fresh output directory for a
new run. No partial temporary PDFs remain after handled row failures.

- Exit 0: all rows succeeded.
- Exit 1: one or more rows failed; successful outputs and the full receipt remain.
- Invalid template, data format or configuration: fail before starting the batch.
- `--json`: one JSON object with `success`, `operation`, `error`, and a `batch`
  receipt after batch processing. CLI argument syntax errors use Clap's usual
  diagnostics. The receipt includes `results[]` with a 1-based row index,
  success/error, output path and output hash, plus template/data hashes and UTC
  timestamps. Input record values are not copied into the receipt.

### MCP JSON-RPC

This is a CLI automation command. A dedicated MCP batch endpoint is planned
separately in Task 4.7.9. Existing single-document `pdf_fill_form` and
`pdf_flatten` tools share its core form implementation.

### REST API

Use the CLI for batch mail merge. Existing single-document form endpoints remain
available; this task does not add a batch REST endpoint.
