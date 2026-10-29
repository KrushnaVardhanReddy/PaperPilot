# Spec 137: Advanced Multi-Condition Engine

The `paperpilot-core` unified workflow engine supports declarative and recursive boolean expressions allowing sophisticated workflow configurations and evaluation over Document states.

## Conditional Predicates and Combinators

A `WorkflowStage` may include a `condition` object. A condition object is evaluated across the input bytes and the current document state. The evaluation leverages short-circuiting natively, efficiently halting evaluation the moment a condition's outcome is definitive.

### Combinators

- `all`: A list of condition expressions. Evaluates to `true` if and only if **all** internal conditions evaluate to true. If the list is empty, this operates under the logical "vacuous truth" standard and resolves to `true`.
- `any`: A list of condition expressions. Evaluates to `true` if and only if **at least one** internal condition evaluates to true. If the list is empty, it resolves to `false`.
- `not`: A wrapper inverting a child condition (a logical NOT).

### Condition Leaves (State Inspections)

The basic inspection criteria include:
- `text_contains`: Performs a case-insensitive string comparison against up to 16 MiB of extractable document text.
- `has_form_fields`: Detects AcroForm integration and interactive PDF fields.
- `is_signed`: Specifically inspects standard `/SigFlags` and signature annotations (`/FT /Sig`) to declare presence of Digital Signatures.
- `is_encrypted`: Indicates if the pipeline document payload is structurally encrypted.
- `file_size_bytes`: Performs robust comparison (`eq`, `gt`, `gte`, `lt`, `lte`, `between: [min, max]`) directly on byte length representations.

> *Backward Compatibility*: Flat models from prior Specs (`if_scanned`, `if_page_count_gt`, `comparator` and `value`) implicitly fallback to the `Leaf` representation seamlessly.

### Example PPFlow syntax

```toml
name = "Invoice Parser"
version = "1"

[[stage]]
id = "rotate_if_scanned"
op = "rotate"
condition = { if_scanned = true }
[stage.params]
angle = 90

[[stage]]
id = "text_pipeline"
op = "pdf_compress"
depends_on = ["rotate_if_scanned"]
on_no_match = "skip"
[stage.condition]
all = [
    { text_contains = "invoice" },
    { not = { leaf = { is_signed = true } } },
    { leaf = { file_size_bytes = { between = [500000, 10000000] } } }
]
```