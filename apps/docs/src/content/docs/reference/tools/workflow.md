---
title: "Workflow Engine"
description: "Run automated sequences of operations conditionally."
---

The Workflow engine allows you to execute a series of PDF operations defined as a JSON or `.ppflow` (TOML) recipe. You can also specify dynamic runtime conditions to control whether individual steps are executed.

## Usage

### CLI

```bash
# Run a built-in preset
paperpilot workflow run --input document.pdf --output out.pdf --preset preset-clean-scan

# Run a custom recipe file
paperpilot workflow run --input document.pdf --output out.pdf --recipe my-workflow.ppflow

# View available presets
paperpilot workflow list

# Inspect a workflow's steps
paperpilot workflow inspect --preset preset-legal-discovery

# Export a built-in workflow to a file
paperpilot workflow export --preset preset-redact-protect --output redact.ppflow
```

### Condition Examples (.ppflow)

Steps can be skipped if a condition is not met:

```toml
[[steps]]
id = "step-1"
operation_id = "pdf_bates"

  [steps.condition.leaf.page_count]
  gt = 10

  [steps.on_no_match]
  action = "skip"
```
