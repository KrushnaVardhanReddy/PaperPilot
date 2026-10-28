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
    /// Complex recursive boolean tree: all, any, not
    Complex(ComplexCondition),
    /// Direct leaf predicates (flat map)
    Leaf(StagePredicate),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct ComplexCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<ConditionExpr>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub any: Option<Vec<ConditionExpr>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<ConditionExpr>>,
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

### 3.1 Strict Validation & Edge Case Rules

To prevent ambiguous configurations and silent failures, the deserializer and semantic validator enforce the following rules:

1. **Vacuous Truth & Identity Elements**:
   - `all: []` (empty conjunction) evaluates to **`true`** (standard mathematical vacuous truth).
   - `any: []` (empty disjunction) evaluates to **`false`** (identity element for OR).
2. **Ambiguous Combinators**:
   - A single `ComplexCondition` node MUST NOT specify more than one combinator (e.g. specifying both `all` and `any` at the same level is disallowed). Semantic validation returns `ConditionValidationError::AmbiguousCombinator`.
3. **Numeric Range Boundaries**:
   - For `NumberComparator::Between(min, max)`, validator enforces `min <= max`. If `min > max` (e.g. `between: [20, 5]`), validation fails with `ConditionValidationError::InvalidRange { min, max }`.
4. **Unknown Fields**:
   - All condition structures use `#[serde(deny_unknown_fields)]` to immediately reject typos like `text_contain:` or `page_counts:` at workflow load time.

### 3.2 WorkflowStep Integration & Backward Compatibility

Existing workflows without conditional fields deserialize seamlessly with zero changes:

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

Inspection operates directly on in-memory buffers via `lopdf::Document::load_mem` without writing temporary files or invoking external subshells:

1. **`page_count`**: Read via `doc.get_pages().len()`.
2. **`file_size_bytes`**: Read via `bytes.len()`.
3. **`is_scanned` (Scanned PDF Detection)**:
   - **Inspection Depth**: First 3 pages (or all pages if total document page count <= 5).
   - **Classification Rule**: A document is classified as `is_scanned: true` IF AND ONLY IF:
     1. Total extractable non-whitespace characters across inspected pages is `< 10` (permits stray scan margin noise), AND
     2. At least one page in the inspected set contains a raster image XObject (`/Subtype /Image`).
   - **Mixed Documents**: If a document has pages with substantial text (> 10 characters), `is_scanned` evaluates to `false`.
   - **Blank Documents**: A document with 0 text characters and 0 image XObjects evaluates `is_scanned: false` and emits an audit notice (`is_blank`).
4. **`text_contains`**:
   - Case-insensitive substring match against extracted text across inspected pages (1..3).
   - Whitespace in the query and target text is normalized (collapsed single spaces).
5. **`has_form_fields`**:
   - Traverses Document Catalog -> `/AcroForm` -> `/Fields`. Evaluates to `true` if `/Fields` array contains at least 1 object reference.
6. **`is_encrypted`**:
   - Checks trailer dictionary for `/Encrypt` object. Evaluates to `true` if present.
7. **`is_signed`**:
   - Inspects `/AcroForm` fields for `/Sig` subtype or document-level `/ByteRange` signatures. Evaluates to `true` if a cryptographic signature dictionary is present.

### 4.2 Missing Information & Inspection Error Handling

A condition evaluator must **never silently convert an inspection failure into a `false` result**:

| Inspection Target | Normal Absence (Returns `false`) | Corrupt / Broken Data (Returns `Err(PdfError::InspectionFailure)`) |
|---|---|---|
| **Text Extraction (`text_contains`, `is_scanned`)** | Page stream is valid, but contains no text drawing operators (`Tj`, `TJ`). | FlateDecode decompress error, corrupted stream dictionary, unparseable CMap / font encoding table. |
| **Form Fields (`has_form_fields`)** | Document Catalog has no `/AcroForm` or `/Fields` is empty array. | `/AcroForm` points to non-dictionary object or indirect reference fails to resolve in xref table. |
| **Signatures (`is_signed`)** | Document has no signature dictionary or `/Sig` field. | `/ByteRange` array is truncated, malformed, or points past EOF. |
| **Encryption (`is_encrypted`)** | Document trailer contains no `/Encrypt` dictionary. | `/Encrypt` dictionary present but standard security handler fails to initialize. |

When an inspection error occurs, the pipeline halts immediately with:
```rust
PdfError::InspectionFailure {
    stage_id: String,
    predicate: String,
    reason: String,
}
```

### 4.3 Pipeline Step Execution Loop

In `run_workflow_bytes`:

```rust
for (index, step) in workflow.steps.iter().enumerate() {
    if let Some(condition) = &step.condition {
        let matched = evaluate_condition_expr(condition, &current_bytes, &inspector)
            .map_err(|e| PdfError::InspectionFailure {
                stage_id: step.id.clone(),
                predicate: format!("{:?}", condition),
                reason: e.to_string(),
            })?;

        if !matched {
            match &step.on_no_match {
                OnNoMatchAction::Skip => {
                    report.stages.push(StageExecutionRecord {
                        id: step.id.clone(),
                        op: step.operation_id.clone(),
                        status: StageStatus::Skipped,
                        condition_matched: Some(false),
                        reason: Some("Condition evaluated to false".to_string()),
                        duration_ms: 0,
                    });
                    report.steps_skipped += 1;
                    continue;
                }
                OnNoMatchAction::Stop => {
                    report.stages.push(StageExecutionRecord {
                        id: step.id.clone(),
                        op: step.operation_id.clone(),
                        status: StageStatus::Stopped,
                        condition_matched: Some(false),
                        reason: Some("Condition evaluated to false; pipeline stopped cleanly".to_string()),
                        duration_ms: 0,
                    });
                    report.steps_stopped += 1;
                    
                    // Mark all remaining unreached stages
                    for unreached_step in &workflow.steps[index + 1..] {
                        report.stages.push(StageExecutionRecord {
                            id: unreached_step.id.clone(),
                            op: unreached_step.operation_id.clone(),
                            status: StageStatus::Unreached,
                            condition_matched: None,
                            reason: Some("Previous stage invoked on_no_match: stop".to_string()),
                            duration_ms: 0,
                        });
                        report.steps_unreached += 1;
                    }
                    report.status = WorkflowExecutionStatus::StoppedEarly;
                    break;
                }
                OnNoMatchAction::Error => {
                    report.status = WorkflowExecutionStatus::Failed;
                    return Err(PdfError::ConditionFailed(format!(
                        "Stage '{}' condition failed with on_no_match: error",
                        step.id
                    )));
                }
                OnNoMatchAction::CustomError { error } => {
                    report.status = WorkflowExecutionStatus::Failed;
                    return Err(PdfError::ConditionFailed(error.clone()));
                }
            }
        }
    }

    let start = std::time::Instant::now();
    current_bytes = execute_step_in_memory(&current_bytes, step)?;
    report.stages.push(StageExecutionRecord {
        id: step.id.clone(),
        op: step.operation_id.clone(),
        status: StageStatus::Executed,
        condition_matched: step.condition.as_ref().map(|_| true),
        reason: None,
        duration_ms: start.elapsed().as_millis() as u64,
    });
    report.steps_completed += 1;
}
```

---

## 5. Execution Receipt & Audit Report Contract

Every workflow run produces a structured execution receipt matching this full contract:

### 5.1 Rust Schema (`paperpilot-core::workflow`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StageStatus {
    Executed,
    Skipped,
    Stopped,
    Failed,
    Unreached,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowExecutionStatus {
    Success,
    StoppedEarly,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StageExecutionRecord {
    pub id: String,
    pub op: String,
    pub status: StageStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_matched: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowExecutionReport {
    pub workflow_id: String,
    pub workflow_name: String,
    pub status: WorkflowExecutionStatus,
    pub steps_total: usize,
    pub steps_completed: usize,
    pub steps_skipped: usize,
    pub steps_stopped: usize,
    pub steps_unreached: usize,
    pub stages: Vec<StageExecutionRecord>,
    pub duration_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
```

### 5.2 JSON Output Example

```json
{
  "workflow_id": "smart-invoice-pipeline",
  "workflow_name": "Smart Invoice Ingestion",
  "status": "stopped_early",
  "steps_total": 4,
  "steps_completed": 1,
  "steps_skipped": 1,
  "steps_stopped": 1,
  "steps_unreached": 1,
  "stages": [
    {
      "id": "step_ocr",
      "op": "ocr",
      "status": "skipped",
      "condition_matched": false,
      "reason": "is_scanned: expected true, got false (found 1,420 chars)",
      "duration_ms": 0
    },
    {
      "id": "step_compress",
      "op": "compress",
      "status": "executed",
      "condition_matched": true,
      "duration_ms": 42
    },
    {
      "id": "step_enforce_invoice",
      "op": "noop",
      "status": "stopped",
      "condition_matched": false,
      "reason": "text_contains 'TAX INVOICE' not found; on_no_match: stop",
      "duration_ms": 0
    },
    {
      "id": "step_bates_number",
      "op": "bates_number",
      "status": "unreached",
      "reason": "Previous stage invoked on_no_match: stop",
      "duration_ms": 0
    }
  ],
  "duration_ms": 45
}
```

---

## 6. Comprehensive Test Matrix

Implementation is verified across this complete matrix:

### 6.1 Unit Tests (`paperpilot-core::workflow`)

| Test Name | Condition Input | Expected Behavior |
|---|---|---|
| `test_comparator_eq_match` | `page_count: { eq: 5 }`, doc: 5 pages | `true` |
| `test_comparator_eq_mismatch` | `page_count: { eq: 5 }`, doc: 6 pages | `false` |
| `test_comparator_gt_boundary` | `page_count: { gt: 10 }`, doc: 10 pages | `false` (boundary is strict) |
| `test_comparator_gte_boundary` | `page_count: { gte: 10 }`, doc: 10 pages | `true` |
| `test_comparator_between_valid` | `page_count: { between: [5, 10] }`, doc: 5 & 10 | `true` (inclusive) |
| `test_comparator_between_invalid_range` | `page_count: { between: [20, 5] }` | Validation error: `min > max` |
| `test_boolean_empty_all` | `condition: { all: [] }` | `true` (vacuous truth) |
| `test_boolean_empty_any` | `condition: { any: [] }` | `false` (empty disjunction) |
| `test_boolean_ambiguous_combinators` | `condition: { all: [...], any: [...] }` | Validation error: `AmbiguousCombinator` |
| `test_serde_deny_unknown_fields` | `condition: { text_contain: "foo" }` | Deserialization error: unknown field |
| `test_legacy_workflow_backward_compat` | Step without `condition` / `on_no_match` | Deserializes with `condition: None`, `on_no_match: Skip` |
| `test_toml_json_serde_parity` | Identical pipeline in TOML and JSON | Yields identical Rust struct representations |

### 6.2 Integration Tests (`paperpilot-pdf::operations::workflow`)

| Test Name | PDF Fixture | Workflow Configuration | Expected Outcome |
|---|---|---|---|
| `test_eval_scanned_pdf_true` | `fixtures/scanned_page.pdf` (pure image) | `condition: { is_scanned: true }` | Stage executes |
| `test_eval_scanned_pdf_false_on_digital` | `fixtures/digital_invoice.pdf` (selectable text) | `condition: { is_scanned: true }`, `on_no_match: skip` | Stage skipped, receipt records skip |
| `test_eval_mixed_pdf_classification` | `fixtures/mixed_scan_and_text.pdf` | `condition: { is_scanned: true }` | Evaluates `false` (>10 text characters present) |
| `test_eval_text_contains_case_insensitive` | `fixtures/digital_invoice.pdf` | `condition: { text_contains: "INVOICE" }` (target has "Invoice") | `true`, stage executes |
| `test_eval_has_form_fields` | `fixtures/acroform_sample.pdf` | `condition: { has_form_fields: true }` | `true` |
| `test_eval_is_signed` | `fixtures/digitally_signed.pdf` | `condition: { is_signed: true }` | `true` |
| `test_eval_is_encrypted` | `fixtures/password_protected.pdf` | `condition: { is_encrypted: true }` | `true` |
| `test_eval_corrupt_pdf_fails_explicitly` | `fixtures/truncated_corrupt.pdf` | Any condition | Returns `Err(PdfError::InspectionFailure)`, does NOT return `false` |
| `test_routing_stop_pipeline` | `fixtures/sample.pdf` | Step 1 matches, Step 2 fails with `stop`, Step 3 present | Step 1 executed, Step 2 stopped, Step 3 unreached, report status: `stopped_early` |
| `test_routing_custom_error` | `fixtures/sample.pdf` | Step fails with `{ error: "Custom limit" }` | Pipeline aborts, error string matches |
| `test_existing_preset_legal_discovery` | `fixtures/contract.pdf` | Built-in `preset-legal-discovery` | Runs identically without regressions |

---

## 7. Acceptance Criteria Checklist

- [ ] **Contract & Parsing**:
  - [ ] `paperpilot-core::workflow` deserializes simple flat conditions and nested `all`/`any`/`not` trees from both JSON and TOML.
  - [ ] `#[serde(deny_unknown_fields)]` rejects unknown/mistyped fields.
  - [ ] Empty `all: []` evaluates to `true`; empty `any: []` evaluates to `false`.
  - [ ] `between: [min, max]` enforces `min <= max`.
- [ ] **Backward Compatibility**:
  - [ ] Existing workflows without conditions deserialize cleanly with default `None` and run without behavioral regressions.
  - [ ] Built-in presets (`preset-legal-discovery`, `preset-clean-scan`, `preset-web-optimize`) function without changes.
- [ ] **Document Inspection**:
  - [ ] `is_scanned` accurately differentiates between pure raster/scanned PDFs and digital PDFs with text layers (using image presence and character thresholds).
  - [ ] Corrupt or unparseable PDFs return `PdfError::InspectionFailure` instead of evaluating to `false`.
  - [ ] `page_count` correctly evaluates `eq`, `gt`, `gte`, `lt`, `lte`, and `between: [min, max]`.
- [ ] **Execution & Routing**:
  - [ ] `on_no_match` respects `"skip"`, `"stop"`, `"error"`, and `{ error: "<msg>" }`.
  - [ ] Early `stop` marks subsequent steps as `unreached` and emits `status: stopped_early`.
- [ ] **Audit Receipt**:
  - [ ] Output receipt includes comprehensive counts (`steps_total`, `steps_completed`, `steps_skipped`, `steps_stopped`, `steps_unreached`), stage statuses, durations, and skip reasons.
- [ ] **CLI & Testing**:
  - [ ] Comprehensive unit test suite in `paperpilot-core` and integration test suite in `paperpilot-pdf`.
  - [ ] CLI command `paperpilot workflow run` correctly handles conditional recipes and displays execution receipts.

