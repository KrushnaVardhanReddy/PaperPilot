# Task 129A: Automated Security Regression & Fuzzing Harness for Malformed PDFs

## Metadata
- **Task ID**: `Task 129A`
- **Spec**: `[Spec 129]` (`specs/129-pdf-fuzzing-and-malformed-harness/spec.md`)
- **Phase**: `Phase 5.9.98`
- **Target Branch**: `feature/dev`
- **Target Crates**: `paperpilot-pdf` (`paperpilot-pdf/tests/malformed_regression.rs`)

---

## 1. Goal
Implement an automated penetration and fuzzing regression test suite in `paperpilot-pdf` that subjects the core document parser and operations to corrupted, adversarial, and malformed PDF inputs, asserting zero crashes, zero memory exhaustion, and clean `Result::Err` propagation.

---

## 2. Implementation Steps

### Step 1: Create `paperpilot-pdf/tests/malformed_regression.rs`
Implement comprehensive regression tests:
1. `test_fuzz_corrupted_header_and_random_bytes`:
   - Empty slice `b""`
   - Incomplete headers `b"%PDF-1.7\n"`
   - Missing `startxref` or `%%EOF`
   - Non-printable binary garbage
2. `test_fuzz_circular_page_tree_loop`:
   - Craft a synthetic PDF with circular references (`/Pages 2 0 R` referencing itself as `/Parent` and `/Kids`).
   - Execute operations like `SplitOperation`, `PageCountOperation`, and `ExtractTextOperation` to verify they terminate cleanly without stack overflow.
3. `test_fuzz_stream_length_mismatch`:
   - Craft a synthetic PDF with declared `/Length 1073741824` but only 5 actual stream bytes.
   - Assert parser rejects or bounds memory allocation cleanly without OOM panic.
4. `test_fuzz_deeply_nested_dictionaries`:
   - Craft deeply nested dictionary/array structures (100+ depth) to verify the parser recurses with depth limits or exits with an error without crashing.
5. `test_fuzz_corrupted_xref_offsets`:
   - Corrupted byte offsets in the cross-reference table.
   - Assert engine returns `Err(PdfError::ParseError)` gracefully.

### Step 2: Verification
Run the targeted verification commands:
```bash
cargo check -p paperpilot-pdf -j 2
cargo test -p paperpilot-pdf --test malformed_regression -j 2
cargo clippy -p paperpilot-pdf --test malformed_regression -- -D warnings
semgrep scan --config auto paperpilot-pdf/tests/malformed_regression.rs
```

---

## 3. Exit Criteria
- `paperpilot-pdf/tests/malformed_regression.rs` compiles cleanly and all tests pass with zero panics.
- Runtime under 1 second.
- Semgrep scan reports 0 findings.
- Pull request submitted against `feature/dev`.
