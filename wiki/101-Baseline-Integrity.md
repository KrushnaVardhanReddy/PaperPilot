# Spec 101: Pre-Sign Baseline Tamper Detection

## Overview
Standard digital signature validation proves a document was not modified *after* signing. However, if a malicious party alters contract clauses before applying their cryptographically valid signature, standard validation will pass (identifying a valid signature on altered content).

Spec 101 addresses this by comparing the signed document against the **original, authoritative baseline** document. This provides a dual-layer audit:
1. **Cryptographic Validation:** Validates PKCS#7 / CMS signatures, byte ranges, certificates, and post-signing alterations.
2. **Baseline Structural & Semantic Integrity:** Compares page counts and `/Contents` streams against the trusted baseline document to detect pre-signature alterations.

## Data Structures
- `PdfValidateSignaturesOptions` includes:
  - `baseline_path`: Optional path to baseline document.
  - `baseline_bytes`: Optional raw bytes of baseline document (for API).
  - `baseline`: Boolean flag used by MCP to trigger forensics mapping.

- `BaselineComparisonReport`: Provides the verdict `UNTOUCHED_JUST_SIGNED` or `CONTENT_ALTERED_PRIOR_TO_SIGNATURE`, `baseline_match` status, a list of `altered_pages`, and a `diff_summary` array describing discrepancies in structural streams.

## Implementation Details
The core logic resides in `paperpilot-pdf/src/operations/pdf_validate_signatures.rs`. The CLI adds a `--baseline` flag for supplying a reference PDF, which triggers the check. The MCP server processes the `baseline` property and the Gateway provides the equivalent JSON mapping parameter.
