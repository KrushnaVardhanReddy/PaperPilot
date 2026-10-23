# Spec 095: Cryptographic Digital Signature Verification Engine (`pdf_validate_signatures`)

## 1. Background & Executive Summary
In Phase 1, PaperPilot implemented `pdf_sign`, which marks AcroForm dictionaries and embeds `/SigFlags` and placeholder signature fields. In legal, financial, and government workflows, documents received from counterparties must be verified for:
1. Presence of digital signatures (`/Type /Sig`, `/FT /Sig`).
2. Integrity of the `/ByteRange` cryptographic byte offsets.
3. Cryptographic signature validity (PKCS#7 / CMS / SHA-256 / RSA / ECDSA signatures).
4. Signer identity (Common Name, Organization, Issuer, Signing Time).
5. Post-signing document alteration/tampering detection.

**Spec 095** implements `pdf_validate_signatures` as a pure-Rust operation across all 5 deployment surfaces (Core Rust Engine, CLI, MCP Server, REST Gateway, WASM, and Cloudflare Edge).

---

## 2. Technical Architecture & Data Model

### 2.1 Structured Output (`SignatureValidationReport`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalSignatureInfo {
    pub field_name: String,
    pub signer_name: Option<String>,
    pub organization: Option<String>,
    pub signing_time: Option<String>,
    pub sub_filter: String,         // e.g. "adbe.pkcs7.detached" or "ETSI.CAdES.detached"
    pub byte_range: Vec<i64>,       // [offset1, len1, offset2, len2]
    pub is_valid: bool,
    pub is_tampered: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureValidationReport {
    pub total_signatures: usize,
    pub valid_signatures: usize,
    pub all_valid: bool,
    pub signatures: Vec<DigitalSignatureInfo>,
}
```

### 2.2 Core Extraction & Verification Strategy
1. **Traverse Document Tree via `lopdf::Document`**:
   * Inspect Catalog (`/Root`) $\to$ `/AcroForm` $\to$ `/Fields`.
   * Find fields where `/FT == /Sig` (Signature Form Fields) or dictionary with `/Type == /Sig`.
   * Extract signature dictionary properties:
     * `/Name` or `/M` (Signer Name / Signing Date).
     * `/SubFilter` (Signature format: `adbe.pkcs7.detached`, `adbe.pkcs7.sha1`).
     * `/ByteRange` (Array of 4 integers defining the hashed byte boundaries).
     * `/Contents` (Hexadecimal or binary string containing the PKCS#7 / DER signature container).
2. **ByteRange Integrity Check**:
   * Verify that the `/ByteRange` spans cover `0..offset1` and `offset2..(offset2+len2)` up to EOF.
   * If bytes exist after `offset2 + len2` that modify structural objects, mark `is_tampered = true`.
3. **Graceful Handling of Unsigned Documents**:
   * If a PDF contains 0 signatures, return `total_signatures = 0, valid_signatures = 0, all_valid = true, signatures = []`.
   * Never panic or return a fatal error on valid PDFs lacking signatures.

---

## 3. Surface Integration (Scaffolded by `penta-scaffold`)
The boilerplate across all 5 surfaces is already registered via `tools/penta-scaffold`:
* `paperpilot-pdf/src/operations/pdf_validate_signatures.rs`
* `paperpilot-cli/src/commands/pdf_validate_signatures.rs` (`paperpilot validate-signatures input.pdf --output report.json`)
* `paperpilot-mcp/src/server.rs` (`pdf_validate_signatures` tool)
* `paperpilot-gateway/src/handlers.rs` (`POST /api/v1/pdf/validate-signatures`)
* `paperpilot-wasm/src/operations.rs` & `paperpilot-wasm/src/lib.rs` (`WasmPdfEngine.pdf_validate_signatures(...)`)
* `apps/desktop/src/lib/types/toolList.ts` (Tool card registered in Studio)
* `tools/penta-interface-e2e/src/cases/pdf_validate_signatures.rs` (10 unmocked E2E assertions)

---

## 4. Verification & Testing
1. Test with an unsigned PDF: returns `total_signatures: 0`.
2. Test with a signed fixture (`tests/fixtures/signed.pdf` or synthetic signed document): validates `/ByteRange` and signature fields.
3. Test corrupted byte ranges: returns `is_tampered: true`.
4. Run `cargo test -p paperpilot-pdf --lib -j 2`.
5. Run `cargo run -p penta-interface-e2e -j 2`.
