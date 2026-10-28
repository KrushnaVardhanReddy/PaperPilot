---
title: "Conditional Workflow Pipelines"
description: "Build and run autonomous document processing pipelines with conditional logic."
---

# Workflow Pipeline Engine

The `workflow` command allows you to define and execute automated pipelines with conditional logic. You can use this to route PDFs dynamically, such as automatically running OCR only on scanned documents, or stopping execution if a document is password protected.

## Available Actions

### Run a workflow
Runs a saved workflow preset or local recipe file on a PDF document.

**CLI**
```bash
paperpilot workflow run --input document.pdf --output processed.pdf --preset preset-clean-scan
paperpilot workflow run --input in.pdf --output out.pdf --recipe my-pipeline.ppflow
```

### List available presets
```bash
paperpilot workflow list
```

### Export a workflow recipe
Exports a preset to a file, which you can modify and import.

**CLI**
```bash
paperpilot workflow export preset-clean-scan --output clean-scan.toml --format toml
```

### Import a workflow recipe
Imports a `.ppflow`, `.json`, or `.toml` pipeline file into your local environment (`~/.paperpilot/workflows/`).

**CLI**
```bash
paperpilot workflow import custom-pipeline.ppflow
```

## Conditional Logic Recipes

Pipeline stages can contain conditional `condition` expressions and failure routing `on_no_match` instructions.

Here is an example `.toml` pipeline:

```toml
id = "my-smart-pipeline"
name = "Smart Ingestion"
updated_at = "2025-10-28T00:00:00Z"

[[steps]]
id = "ocr-scans"
operation_id = "pdf_ocr"
on_no_match = { type = "skip" }

  [steps.condition]
  is_scanned = true

[[steps]]
id = "compress-large-files"
operation_id = "pdf_compress"
on_no_match = { type = "skip" }

  [steps.condition.file_size_bytes]
  gt = 5000000

  [steps.params]
  quality = 75
```
