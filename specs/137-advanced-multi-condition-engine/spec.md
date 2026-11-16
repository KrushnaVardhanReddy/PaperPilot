# Spec 137: Advanced Multi-Condition Predicates & Combinators (`.ppflow` / `SavedWorkflow`)

## Metadata
- **Spec Number**: 137
- **Phase**: Phase 4.7.10 / 8.2.10
- **Status**: 📋 In Development
- **Target Branch**: `feat/multi-condition-pipeline-engine-task-4-7-10`
- **Impacted Crates**: `paperpilot-core`, `paperpilot-pdf`, `paperpilot-cli`, `paperpilot-wasm`, `paperpilot-gateway`, `paperpilot-mcp`
- **Associated Phase Docs**: `phases/phase-04-ai-layer.md` (Task 4.7.10), `phases/phase-08-growth-platform.md` (Task 8.2.10)

---

## 1. Overview & Objective

Task 4.7.6 / 8.2.3 introduced the foundational conditional execution engine with:
- Topological dependency resolution and single-dependency fan-out.
- Atomic execution receipts and CLI output preservation.
- Initial single/flat predicates: `if_scanned`, `if_page_count_gt`, and `comparator`.

**Task 4.7.10 / 8.2.10 completes the full declarative condition specification (Spec 137)** by extending the shared runtime engine across all 5 surfaces (CLI, Desktop, WASM, MCP, REST Gateway) with:
1. **Recursive Boolean Combinators**: `all`, `any`, and `not` expression trees supporting arbitrary nesting and short-circuit evaluation.
2. **Extended Document Inspection Predicates**:
   - `text_contains`: In-flight case-insensitive keyword or substring matching over page text layers.
   - `has_form_fields`: Detection of interactive AcroForm / XFA form fields.
   - `is_signed`: Detection of digital signatures / `/Sig` dictionaries.
   - `is_encrypted`: Detection of encryption dictionaries (for workflows operating on pre-authenticated streams or terminal pipelines).
   - `file_size_bytes`: Numeric comparator over document byte size (`eq`, `gt`, `gte`, `lt`, `lte`, `between: [min, max]`).

---

## 2. Specification & Schema Contract

### 2.1 Core Condition Types (`paperpilot-core::workflow`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WorkflowCondition {
    /// Combinator: all of the child conditions must evaluate to true (vacuous truth if empty: true)
    All { all: Vec<WorkflowCondition> },
    /// Combinator: at least one child condition must evaluate to true (empty: false)
    Any { any: Vec<WorkflowCondition> },
    /// Inverter: negates the inner condition
    Not { not: Box<WorkflowCondition> },
    /// Flat / Leaf predicate inspection
    Leaf(Box<LeafCondition>),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LeafCondition {
    pub if_scanned: Option<bool>,
    pub if_page_count_gt: Option<u32>,
    pub comparator: Option<PageCountComparator>,
    pub value: Option<u32>,
    pub text_contains: Option<String>,
    pub has_form_fields: Option<bool>,
    pub is_signed: Option<bool>,
    pub is_encrypted: Option<bool>,
    pub file_size_bytes: Option<NumericComparator>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum NumericComparator {
    Between { between: [u64; 2] },
    Comparison {
        #[serde(default)]
        eq: Option<u64>,
        #[serde(default)]
        gt: Option<u64>,
        #[serde(default)]
        gte: Option<u64>,
        #[serde(default)]
        lt: Option<u64>,
        #[serde(default)]
        lte: Option<u64>,
    },
}
```

### 2.2 Backward Compatibility
- Simple flat conditions from Spec 136 (`condition = { if_scanned = true }` or `condition = { if_page_count_gt = 2 }`) deserialize seamlessly into `WorkflowCondition::Leaf`.
- Omitted conditions continue to evaluate to `true` (execute unconditionally).
- Vacuous truth: `all: []` evaluates to `true`; `any: []` evaluates to `false`.

### 2.3 Example `.ppflow` TOML Recipe

```toml
name = "Smart Legal Discovery Pipeline"
version = "1"

[[stage]]
id = "ocr_if_scanned"
op = "ocr"
condition = { if_scanned = true }
on_no_match = "skip"

[[stage]]
id = "redact_or_watermark"
op = "watermark"
depends_on = ["ocr_if_scanned"]
on_no_match = "skip"
[stage.params]
text = "CONFIDENTIAL DISCOVERY"
[stage.condition]
all = [
    { any = [
        { text_contains = "DEFENDANT" },
        { text_contains = "PLAINTIFF" }
    ]},
    { not = { leaf = { is_signed = true } } }
]

[[stage]]
id = "flatten_forms"
op = "flatten"
condition = { has_form_fields = true }
on_no_match = "skip"
```

---

## 3. Inspection Engine Requirements (`paperpilot-pdf`)

1. **`text_contains`**:
   - Case-insensitive substring match against extractable text across pages up to 16 MiB inspection limit.
   - If document is scanned (no text layer), returns `false` (does not error).
2. **`has_form_fields`**:
   - Inspects `doc.catalog` for `/AcroForm` dictionary. Returns `true` if `/Fields` array contains at least one object reference.
3. **`is_signed`**:
   - Inspects page annotations and `/AcroForm` for `/FT /Sig` or `/ByteRange` signature dictionaries.
4. **`file_size_bytes`**:
   - Evaluates `bytes.len() as u64` against the specified `NumericComparator`. Validates `between: [min, max]` where `min <= max`.
5. **Short-Circuit Evaluation**:
   - `any` terminates evaluation on the first `true` child.
   - `all` terminates evaluation on the first `false` child.
   - Evaluation reports reasons cleanly in the stage receipt (e.g., `"any[0] matched: text_contains 'DEFENDANT'"`).

---

## 4. Acceptance Criteria Checklist

- [ ] `paperpilot-core`:
  - Deserializes nested `all`, `any`, `not` and leaf conditions from both TOML and JSON.
  - Full backward compatibility with existing Spec 136 recipes without modification.
  - Strict validation: rejects empty leaf conditions, invalid `between` intervals (`min > max`), and unknown fields.
- [ ] `paperpilot-pdf`:
  - Implements `has_form_fields`, `is_signed`, `text_contains`, and `file_size_bytes` in `operations/workflow/conditions.rs`.
  - Re-uses single-parsed `lopdf::Document` to avoid redundant deserializations.
  - Real synthetic tests covering all combinators and new inspectors without mocks.
- [ ] Penta-Interface Parity:
  - Verified across CLI (`paperpilot run`), WASM, MCP, and REST Gateway.
