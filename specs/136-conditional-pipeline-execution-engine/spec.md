# Spec 136: Conditional Pipeline Execution Engine (`.ppflow` / `SavedWorkflow`)

## Metadata
- **Spec Number**: 136
- **Phase**: Phase 4.7.6 / 8.2.3
- **Status**: 📋 In Development
- **Target Branch**: `feat/conditional-pipeline-engine-task-4-7-6`
- **Impacted Crates**: `paperpilot-core`, `paperpilot-pdf`, `paperpilot-cli`
- **Associated Phase Docs**: `phases/phase-04-ai-layer.md` (Task 4.7.6), `phases/phase-08-growth-platform.md` (Task 8.2.3)

---

## 1. Overview & Objective

PaperPilot currently executes workflow pipelines (`SavedWorkflow`, `.ppflow`) as static linear sequences of operations. If a pipeline defines 5 steps, all 5 steps execute sequentially regardless of the document's content, structure, or size.

**Task 4.7.6 / 8.2.3 introduces runtime conditional evaluation** into the pipeline execution engine. Each stage in a workflow can declare:
1. **`condition`**: A predicate or boolean expression tree (`all`, `any`, `not`) evaluated against the in-flight PDF.
2. **`on_no_match`**: The routing action taken if the condition evaluates to `false` (`skip`, `stop`, `error`).

This transforms static pipelines into intelligent document automation workflows (e.g., *"Only run OCR if the document is scanned; if it's over 20 pages, compress aggressively; if it contains 'CONFIDENTIAL', watermark and stop"*).

---

## 2. Complete Contract Specification (YAML / TOML)

The pipeline specification format supports both **TOML** (`.ppflow`) and **YAML/JSON** (`.ppflow.json` / `SavedWorkflow`) transparently.

### 2.1 Simple Conditions Contract (Flat / Implicit AND)

```yaml
# Simple Contract: Flat key-value map
# Semantic Rule: All leaf keys specified at the root of `condition` are implicitly ANDed together.

# Case 1: Single boolean check (Skip OCR if text layer already exists)
stage_simple_ocr:
  id: "conditional_ocr"
  op: "ocr"
  condition:
    is_scanned: true
  on_no_match: "skip"            # Bypasses this step, passes input PDF to next step untouched
  params:
    languages: ["eng"]

# Case 2: Numeric comparator
stage_simple_page_filter:
  id: "compress_large_docs"
  op: "compress"
  condition:
    page_count:
      gt: 10                     # Supported: eq, gt, gte, lt, lte, between: [min, max]
  on_no_match: "skip"
  params:
    level: "aggressive"

# Case 3: Implicit AND of multiple leaf conditions
stage_simple_implicit_and:
  id: "flatten_large_scanned_forms"
  op: "flatten"
  condition:
    is_scanned: true             # AND
    has_form_fields: true        # AND
    file_size_bytes:             # AND
      lte: 52428800              # 50 MB
  on_no_match: "skip"
```

### 2.2 Complex Conditions Contract (Explicit `all`, `any`, `not`)

```yaml
# Complex Contract: Recursive boolean expression tree
# Semantic Rule: Supports arbitrary nesting with short-circuit evaluation.

# Case 1: Logical OR (`any`)
stage_complex_or:
  id: "quarantine_or_watermark"
  op: "watermark"
  condition:
    any:
      - text_contains: "CONFIDENTIAL"
      - text_contains: "RESTRICTED"
      - page_count:
          gt: 100
  on_no_match: "skip"
  params:
    text: "AUDIT REQUIRED"

# Case 2: Logical NOT (`not`)
stage_complex_not:
  id: "encrypt_unsigned_docs"
  op: "encrypt"
  condition:
    not:
      is_signed: true            # Run encryption ONLY IF document is NOT digitally signed
  on_no_match: "stop"            # Stop pipeline cleanly without error if already signed
  params:
    password: "${env:DOC_PASSWORD}"

# Case 3: Nested Tree ((A OR B) AND (NOT C))
stage_complex_nested_tree:
  id: "legal_discovery_branch"
  op: "bates_number"
  condition:
    all:
      - any:
          - text_contains: "DEFENDANT"
          - text_contains: "PLAINTIFF"
      - not:
          is_encrypted: true
      - page_count:
          between: [1, 500]
  on_no_match: "skip"
  params:
    prefix: "CASE-2026-"
    start: 1
```

### 2.3 Edge Cases & Routing Contracts

```yaml
# Edge Case 1: Empty or Omitted Condition
# Semantic Rule: Evaluates to TRUE immediately. Step always executes.
stage_edge_empty_condition:
  id: "always_run"
  op: "linearize"
  condition: {}                  # Omitting the key entirely has identical behavior
  on_no_match: "skip"

# Edge Case 2: `on_no_match` Routing Behaviors
# Semantic Rule: Controls pipeline continuation when condition is FALSE.
# Supported actions:
#   - "skip"                 -> Skip stage, continue to next stage with current PDF
#   - "stop"                 -> Gracefully exit pipeline, write out current PDF as final output
#   - "error"                -> Terminate pipeline immediately with exit code 1
#   - { error: "<message>" } -> Terminate pipeline immediately with custom error message

stage_edge_route_stop:
  id: "enforce_invoice_type"
  op: "noop"
  condition:
    text_contains: "TAX INVOICE"
  on_no_match: "stop"            # Graceful stop: non-invoices are preserved as-is

stage_edge_route_custom_error:
  id: "assert_court_filing_limit"
  op: "noop"
  condition:
    page_count:
      lte: 50
  on_no_match:
    error: "Document rejected: Exceeds court filing limit of 50 pages."

# Edge Case 3: Numeric Boundary / Between Operator
# Semantic Rule: `between: [min, max]` is inclusive: (val >= min && val <= max).
stage_edge_between:
  id: "medium_document_handler"
  op: "compress"
  condition:
    page_count:
      between: [5, 20]
  on_no_match: "skip"

# Edge Case 4: Text Normalization & Search Bounds
# Semantic Rule:
#   - `text_contains`: Case-insensitive substring match.
#   - Strips leading and trailing whitespace.
#   - Scans first 3 pages by default for high throughput (avoids full 1000-page extractions).
stage_edge_text_normalization:
  id: "detect_tax_form"
  op: "tag"
  condition:
    text_contains: "form w-9"    # Matches "Form W-9", "FORM W-9", "form   w-9"
  on_no_match: "skip"

# Edge Case 5: Empty / Corrupt PDF Document
# Semantic Rule:
# If input PDF cannot be parsed, condition evaluation does NOT silently return false.
# It immediately bubbles up `PdfError::ParseError` or `PdfError::CorruptPdf`.
```

---

## 3. Data Architecture (`paperpilot-core`)

The data types reside in `paperpilot_core::workflow`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NumberComparator {
    Eq(usize),
    Gt(usize),
    Gte(usize),
    Lt(usize),
    Lte(usize),
    Between(usize, usize),
}

impl NumberComparator {
    pub fn matches(&self, value: usize) -> bool {
        match self {
            Self::Eq(expected) => value == *expected,
            Self::Gt(threshold) => value > *threshold,
            Self::Gte(threshold) => value >= *threshold,
            Self::Lt(threshold) => value < *threshold,
            Self::Lte(threshold) => value <= *threshold,
            Self::Between(min, max) => value >= *min && value <= *max,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct StagePredicate {
    pub is_scanned: Option<bool>,
    pub page_count: Option<NumberComparator>,
    pub file_size_bytes: Option<NumberComparator>,
    pub text_contains: Option<String>,
    pub has_form_fields: Option<bool>,
    pub is_encrypted: Option<bool>,
    pub is_signed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConditionExpr {
    /// Boolean combinator tree: all, any, not
    Complex {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        all: Option<Vec<ConditionExpr>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        any: Option<Vec<ConditionExpr>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        not: Option<Box<ConditionExpr>>,
    },
    /// Direct leaf predicates (flat map)
    Leaf(StagePredicate),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum OnNoMatchAction {
    #[default]
    Skip,
    Stop,
    Error,
    CustomError { error: String },
}
```

And `WorkflowStep` in `paperpilot-core/src/workflow.rs` is updated with backward compatibility:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowStep {
    pub id: String,
    #[serde(rename = "operationId", alias = "operation_id")]
    pub operation_id: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub condition: Option<ConditionExpr>,
    #[serde(default, rename = "onNoMatch", alias = "on_no_match")]
    pub on_no_match: OnNoMatchAction,
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}
```

---

## 4. Execution Engine Integration (`paperpilot-pdf`)

### 4.1 In-Memory Document Inspection (`DocumentInspector`)
Inspection must be ultra-fast and pure-Rust, leveraging `lopdf::Document::load_mem`:

1. **`page_count`**: Read via `doc.get_pages().len()`.
2. **`is_scanned`**: Extract text from pages 1..3. If total non-whitespace characters == 0, `is_scanned = true`.
3. **`file_size_bytes`**: Read via `bytes.len()`.
4. **`text_contains`**: Case-insensitive substring search on extracted text from pages 1..3.
5. **`has_form_fields`**: Check if document catalog contains `/AcroForm` with `/Fields` array.
6. **`is_encrypted`**: Check if trailer contains `/Encrypt`.
7. **`is_signed`**: Check `/AcroForm` fields for `/Sig` type or `/ByteRange` dictionary.

### 4.2 Pipeline Step Execution Loop
In `run_workflow_bytes`:

```rust
for step in &workflow.steps {
    if let Some(condition) = &step.condition {
        let matched = evaluate_condition_expr(condition, &current_bytes, &inspector)?;
        if !matched {
            match &step.on_no_match {
                OnNoMatchAction::Skip => {
                    report.stage_records.push(StageExecutionRecord {
                        stage_id: step.id.clone(),
                        status: StageStatus::Skipped,
                        reason: Some("Condition evaluated to false".to_string()),
                    });
                    continue;
                }
                OnNoMatchAction::Stop => {
                    report.stage_records.push(StageExecutionRecord {
                        stage_id: step.id.clone(),
                        status: StageStatus::Stopped,
                        reason: Some("Condition evaluated to false; pipeline stopped cleanly".to_string()),
                    });
                    break;
                }
                OnNoMatchAction::Error => {
                    return Err(PdfError::ConditionFailed(format!(
                        "Stage '{}' condition failed with on_no_match: error",
                        step.id
                    )));
                }
                OnNoMatchAction::CustomError { error } => {
                    return Err(PdfError::ConditionFailed(error.clone()));
                }
            }
        }
    }

    current_bytes = execute_step_in_memory(&current_bytes, step)?;
    completed += 1;
}
```

---

## 5. Execution Receipt & Audit Trail

Every workflow run outputs an expanded `WorkflowExecutionReport` and optional `execution_receipt.json`:

```json
{
  "workflow_id": "smart-invoice-pipeline",
  "workflow_name": "Smart Invoice Ingestion",
  "steps_total": 4,
  "steps_completed": 2,
  "steps_skipped": 2,
  "stages": [
    {
      "id": "step_ocr",
      "op": "ocr",
      "status": "skipped",
      "condition_matched": false,
      "reason": "is_scanned: expected true, got false (found 1,420 chars)",
      "duration_ms": 1
    },
    {
      "id": "step_compress",
      "op": "compress",
      "status": "executed",
      "condition_matched": true,
      "duration_ms": 42
    }
  ],
  "duration_ms": 45
}
```

---

## 6. Acceptance Criteria

- [ ] `paperpilot-core::workflow` deserializes simple flat conditions and nested `all`/`any`/`not` trees from both JSON and TOML.
- [ ] Existing workflows without conditions deserialize cleanly with default `None` and run without behavioral regressions.
- [ ] `is_scanned` accurately differentiates between pure raster/scanned PDFs and digital PDFs with text layers.
- [ ] `page_count` correctly evaluates `eq`, `gt`, `gte`, `lt`, `lte`, and `between: [min, max]`.
- [ ] `on_no_match` respects `"skip"`, `"stop"`, and `"error"` (including custom error strings).
- [ ] Comprehensive unit test suite in `paperpilot-core` and integration test suite in `paperpilot-pdf` using genuine real-world test PDFs.
- [ ] CLI command `paperpilot workflow run` correctly handles conditional recipes and logs skipped/executed stages.
