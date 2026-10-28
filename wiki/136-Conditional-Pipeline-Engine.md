# 136 - Conditional Pipeline Engine

## Overview
PaperPilot supports conditional execution in saved pipelines (`.ppflow`). This allows stages to automatically trigger or skip depending on document characteristics, such as whether it is a scanned document, has form fields, contains specific text, or exceeds a given file size or page count.

## Supported Predicates
The engine inspects documents using pure-Rust analysis to evaluate conditions accurately without external dependencies:

* `is_scanned`: Differentiates between text PDFs and pure-image/scanned PDFs based on character density and the presence of image XObjects.
* `text_contains`: Case-insensitive text search.
* `page_count`: Number threshold checking (e.g., `eq`, `gt`, `lt`, `between`).
* `file_size_bytes`: File size checking in bytes.
* `has_form_fields`: Detects `AcroForm` data.
* `is_encrypted`: Checks if the document is password-protected.
* `is_signed`: Looks for digital signature fields.

## Logical Operators
Complex conditions can be created using boolean logic combinators:
* `all`: True if all nested conditions are true.
* `any`: True if at least one nested condition is true.
* `not`: True if the nested condition is false.

## Routing Strategies (`on_no_match`)
When a condition evaluates to false, the step specifies how the engine should route the pipeline execution:
* `skip` (default): Skips the current step and continues the pipeline.
* `stop`: Cleanly halts the pipeline execution, returning the current PDF state and marking the remaining steps as `Unreached`.
* `error`: Fails the entire pipeline run immediately.
* `{ "error": "Custom Message" }`: Fails the pipeline run with a custom error message.

## Audit Receipts
All workflow runs produce a comprehensive `WorkflowExecutionReport` summarizing the total elapsed time, the status of every stage (Executed, Skipped, Stopped, Unreached), input/output sizes, and reasoning for any skipped stages.

## File Format
Workflows can be managed in `.json`, `.toml`, or `.ppflow` formats. The CLI provides `paperpilot workflow export` and `paperpilot workflow import` commands to help construct and share robust document pipelines locally.
