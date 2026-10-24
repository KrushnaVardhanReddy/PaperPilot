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

## 3. Core Engine Algorithm (3-Tier Hierarchical Detection)

When `--baseline` is supplied, PaperPilot executes a 3-tier verification pipeline designed for sub-25ms throughput:

### Tier 1: Page-by-Page Content Stream Hashing (`SHA-256` Fast Path)
- For every page `i` in `1..=page_count`:
  - Extract the raw byte stream(s) of `/Contents` from both `baseline_doc` and `signed_doc`.
  - Compute `sha256(baseline_page_contents)` vs `sha256(signed_page_contents)`.
- If hashes across all pages match identically:
  - **Result**: Immediate Early Exit $\to$ `baseline_match = true`, `verdict = "UNTOUCHED_JUST_SIGNED"` (Latency: `< 5ms`).

### Tier 2: Signature Object & Annotation Whitelist Verification
- If any page content or object hash differs:
  - Check whether the diff is caused solely by signature addition.
  - Whitelist:
    - `/Type /Sig` dictionary entries.
    - `/Subtype /Widget` annotations on the signature target page where `/FT == /Sig`.
    - Updates to the page's `/Annots` array and `/Root /AcroForm` (`/Fields`, `/SigFlags`).
- If all differences belong strictly to the signature whitelist:
  - **Result**: `baseline_match = true`, `verdict = "UNTOUCHED_JUST_SIGNED"`.

### Tier 3: Semantic Text Token Diffing (Exact Clause & Number Pinpointing)
- If non-signature differences exist on page `k`:
  - Extract text tokens and text operators (`Tj`, `TJ`, `'`, `"`) from both baseline and signed content streams.
  - Normalize whitespace and identify exact token differences (e.g. `"24.99%"` $\to$ `"4.99%"`).
  - Record modified page numbers in `altered_pages`.
  - Append descriptive diff lines to `diff_summary` (e.g. `"Page 1: modified text detected ('24.99%' vs '4.99%')"`).
  - **Result**: `baseline_match = false`, `verdict = "CONTENT_ALTERED_PRIOR_TO_SIGNATURE"`.

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
