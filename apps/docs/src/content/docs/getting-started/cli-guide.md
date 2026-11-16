---
title: CLI Guide
description: Headless CLI usage, stdin/stdout piping, and automated scripting.
---

# CLI Guide

The `paperpilot-cli` is a powerful tool for automating PDF workflows.

## Basic Usage

The CLI follows a standard command structure:

```bash
paperpilot-cli <command> [options]
```

## Chaining Operations & Pipelines

You can chain operations in shell scripts or CI/CD pipelines to build end-to-end document automations:

```bash
# 1. Extract specific pages from a document
paperpilot extract --input document.pdf --pages 1 --output page1.pdf

# 2. Compute the cryptographic hash for verification
paperpilot hash --input page1.pdf
# Output: 9debc10df48377bf86e9b0bc5a7d040a66c0b3eb22578f3d5c02a10d8803eca6

# 3. Extract clean text and pipe into standard UNIX text utilities
paperpilot extract-text --input document.pdf --output document.txt
cat document.txt | grep -E "Invoice|Total"
```

## Batch AcroForm Mail Merge

Generate one PDF for each CSV/JSON record:

```bash
paperpilot fill-batch --template form.pdf --data records.csv --output-dir ./out \
  --name-template '{row.name}_{date}_{index:04}.pdf' --json
```

Outputs are flattened by default. Add `--keep-editable` to retain interactive
fields or `--field-map mapping.json` to bind column names to PDF field names.
Existing files are preserved, failed rows do not stop later records, and the
batch writes `execution_receipt.json`. Any failed row produces a nonzero exit
code. See the [full command reference](/reference/tools/cli_fill_batch/).

## Conditional Document Recipes

Run a TOML `.ppflow` or existing JSON workflow locally:

```bash
paperpilot run examples/workflows/conditional-invoice.ppflow invoice.pdf -o processed.pdf --json
```

Recipes can test scan status and page count before each stage, select dependencies,
and skip, return the original document, or fail when conditions do not match.
The JSON result includes per-stage outcomes. See the
[workflow reference](/reference/tools/document_workflow/) for supported operations,
schema, OCR requirements and examples.

## Automated Scripting & Webhooks

Use the `--json` flag to receive structured JSON status output, ideal for parsing in bash scripts, GitHub Actions, or automation pipelines with `jq`:

```bash
paperpilot hash --input document.pdf --json | jq '.operation, .success'
```

You can also fire automated webhooks on completion or failure:

```bash
paperpilot compress --input report.pdf --output compressed.pdf \
  --webhook "https://webhook.site/your-endpoint" \
  --webhook-on-success
```
