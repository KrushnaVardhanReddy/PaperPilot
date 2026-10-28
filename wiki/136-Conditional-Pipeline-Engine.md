# 136 - Conditional Pipeline Engine

This document outlines the conditional pipeline engine implemented for PaperPilot (`.ppflow` workflows). The engine allows individual stages within a workflow to run, skip, or stop execution based on dynamic conditions about the PDF file being processed.

## Data Models

The engine utilizes several key data models to represent conditions:

-   `StagePredicate`: Represents a single condition (e.g., `is_scanned`, `page_count`, `file_size_bytes`, `text_contains`, `has_form_fields`, `is_encrypted`, `is_signed`).
-   `NumberComparator`: Used for numerical comparisons (`Eq`, `Gt`, `Gte`, `Lt`, `Lte`, `Between`).
-   `ConditionExpr`: Can be a simple `Leaf(StagePredicate)` or a `Complex(ComplexCondition)`.
-   `ComplexCondition`: Allows combining expressions using logical operators (`all`, `any`, `not`).

## Evaluation

The `DocumentInspector` provides methods to inspect a document's properties (page count, size, form fields, text content, encryption, and signatures). The `evaluate_condition_expr` function recursively evaluates a `ConditionExpr` against the `DocumentInspector`.

## Workflow Execution

During workflow execution (`run_workflow_bytes`), each step's condition (if present) is evaluated before running the step's operation. If a condition evaluates to `false`, the step's `on_no_match` action is triggered:

-   `Skip`: The step is skipped, and execution continues.
-   `Stop`: The workflow execution stops cleanly without error.
-   `Error` / `CustomError`: The workflow fails with an error.

The `WorkflowExecutionReport` contains detailed information about each stage's execution status (Executed, Skipped, Stopped, Failed, Unreached).

## CLI Integration

The CLI supports managing workflows:

-   `workflow run --input <IN> --output <OUT> [--preset <PRESET> | --recipe <RECIPE>]`: Runs a workflow preset or recipe on a PDF file.
-   `workflow list`: Lists available built-in workflow presets.
-   `workflow inspect --preset <PRESET>`: Views the steps in a workflow.
-   `workflow export --preset <PRESET> --output <OUT>`: Exports a preset to a `.json` or `.ppflow` (.toml) file.
-   `workflow import --recipe <RECIPE>`: Validates a local recipe.
