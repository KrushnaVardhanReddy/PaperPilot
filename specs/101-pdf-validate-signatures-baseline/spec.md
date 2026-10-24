# Spec 101: Baseline Integrity & Pre-Sign Tamper Detection (`pdf_validate_signatures --baseline`)

## 1. Background & Executive Summary
In legal, banking, commercial insurance, and government contracting, digital signatures (PKCS#7 / CMS) guarantee cryptographic authenticity and verify that a document has not been altered **after** the signature was placed.

However, standard signature validation cannot determine whether a signer altered clauses, rates, or terms **prior** to signing the document. If an counterparty takes an issued contract, changes an interest rate or liability limit, and then digitally signs the file, the digital signature itself is 100% cryptographically valid.

**Spec 101** extends PaperPilot's existing `pdf_validate_signatures` operation across all 5 interfaces with an optional `--baseline <baseline_pdf>` parameter. When provided, PaperPilot performs a dual-layer audit in a single high-performance execution:
1. **Cryptographic Validation**: Verifies certificate chains, digest hashes, and byte-range locks.
2. **Baseline Structural & Semantic Integrity**: Compares document objects, structural trees, and page text streams against the authoritative baseline document.

---

## 2. Technical Architecture & Data Model

### 2.1 Options & Report Data Structures (`paperpilot-pdf`)

In `paperpilot-pdf/src/operations/pdf_validate_signatures.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PdfValidateSignaturesOptions {
    /// Optional path to the baseline reference PDF (e.g. original unsigned contract)
    #[serde(default)]
    pub baseline_path: Option<String>,
    /// Optional raw bytes of baseline reference PDF (for WASM / API memory workflows)
    #[serde(default)]
    pub baseline_bytes: Option<Vec<u8>>,
    /// If true, only check byte range coverage without full certificate parsing
    #[serde(default)]
    pub verify_byte_range_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineComparisonReport {
    /// True if baseline was provided and compared
    pub baseline_checked: bool,
    /// True if all content streams, page text, and structure match baseline exactly
    pub baseline_match: bool,
    /// High-level verdict: "UNTOUCHED_JUST_SIGNED" | "CONTENT_ALTERED_PRIOR_TO_SIGNATURE" | "STRUCTURE_MISMATCH"
    pub verdict: String,
    /// Page numbers that contain text or layout differences
    pub altered_pages: Vec<u32>,
    /// Human-readable list of detected changes
    pub diff_summary: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureValidationReport {
    pub total_signatures: usize,
    pub valid_signatures: usize,
    pub all_valid: bool,
    pub signatures: Vec<DigitalSignatureInfo>,
    /// Baseline comparison result (populated if baseline was supplied)
    pub baseline_report: Option<BaselineComparisonReport>,
}
```

---

## 3. Core Engine Algorithm

1. **Digital Signature Verification**:
   - Extract `/AcroForm` and `/Page` `/Annots` signature references.
   - Parse `/ByteRange`, `/Contents`, and `/SubFilter`.
   - Compute digest over specified byte ranges and verify against PKCS#7 container.
   - Check if incremental updates exist past `/ByteRange` (`is_tampered`).

2. **Baseline Comparison Pipeline (when baseline is supplied)**:
   - Load baseline bytes from `options.baseline_path` or `options.baseline_bytes`.
   - Parse both `baseline_doc` and `signed_doc` via `lopdf::Document`.
   - **Page Count & Geometry Check**: Assert identical page count and `/MediaBox` dimensions.
   - **Content Stream Diffing**:
     - For each page, extract text operators and `/Contents` streams.
     - Normalize whitespace and compare text tokens.
     - Identify any modified numbers, clauses, or text lines (e.g., `"24.99%"` vs `"4.99%"`).
   - **Object Whitelist Verification**:
     - A legitimate "Sign-Only" operation should *only* introduce:
       - `/Sig` dictionary.
       - Signature `/Widget` annotation.
       - Modified `/Annots` array on the target page.
       - Modified `/AcroForm` `/Fields` array and `/SigFlags`.
     - Any modified `/Contents`, `/Resources`, or non-signature form fields flag the document as `CONTENT_ALTERED_PRIOR_TO_SIGNATURE`.

---

## 4. Penta-Interface Surface Specifications

### 4.1 CLI (`paperpilot-cli`)
```bash
# Basic signature validation
paperpilot pdf-validate-signatures --input agreement_signed.pdf --json

# Baseline integrity validation
paperpilot pdf-validate-signatures --input agreement_signed.pdf --baseline agreement_original.pdf --json
```

### 4.2 MCP Server (`paperpilot-mcp`)
Tool `pdf_validate_signatures`:
```json
{
  "name": "pdf_validate_signatures",
  "arguments": {
    "input": "agreement_signed.pdf",
    "baseline": "agreement_original.pdf"
  }
}
```

### 4.3 REST Gateway (`paperpilot-gateway`)
`POST /api/v1/pdf/validate-signatures`:
```json
{
  "input": "agreement_signed.pdf",
  "baseline": "agreement_original.pdf"
}
```

### 4.4 Documentation & Playbook
Update:
- `apps/docs/src/content/docs/reference/tools/pdf_validate_signatures.md`
- `wiki/095-Pdf-Validate-Signatures.md`
