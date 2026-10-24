# Task 5.9.90: Implement Baseline Comparison in `pdf_validate_signatures` (Spec 101)

## Metadata
- **Spec**: [Spec 101: Baseline Integrity & Pre-Sign Tamper Detection](file:///home/krushna/Project/PaperPilot/specs/101-pdf-validate-signatures-baseline/spec.md)
- **Tool ID**: `pdf_validate_signatures`
- **Primary Crate**: `paperpilot-pdf`
- **Interfaces**: CLI (`--baseline`), MCP (`baseline`), REST Gateway (`baseline`), WASM
- **Target Fixtures**: 
  - `tests/e2e_fixtures/real/credit_agreement_original.pdf`
  - `tests/e2e_fixtures/real/credit_agreement_signed_clean.pdf`
  - `tests/e2e_fixtures/real/credit_agreement_signed_tampered.pdf`

---

## Acceptance Criteria
1. `PdfValidateSignaturesOptions` accepts `baseline_path: Option<String>` and `baseline_bytes: Option<Vec<u8>>`.
2. When baseline is provided, the output JSON contains a `baseline_report` with:
   - `baseline_checked: true`
   - `baseline_match: bool`
   - `verdict`: `"UNTOUCHED_JUST_SIGNED"` or `"CONTENT_ALTERED_PRIOR_TO_SIGNATURE"`
   - `diff_summary`: Detailed human-readable differences.
3. Unit tests in `paperpilot-pdf` verify:
   - Untouched signed document reports `baseline_match = true`.
   - Tampered signed document reports `baseline_match = false` with altered page numbers and modified text.
4. CLI supports `--baseline <PATH>`.
5. REST Gateway and MCP arguments support `baseline`.
6. Tool reference documentation in `apps/docs/src/content/docs/reference/tools/pdf_validate_signatures.md` is updated.
7. Execution latency remains sub-35ms.
