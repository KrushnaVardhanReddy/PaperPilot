# Spec 086: Automated PII & Sensitive Information Scanner (`document_pii_scan`)

## 1. Executive Summary
Organizations, legal teams, healthcare systems, and financial institutions deal with massive volumes of documents containing Personally Identifiable Information (PII) and compliance-regulated data (GDPR, HIPAA, PCI-DSS).

While `pdf_redact` performs physical coordinate blackout and `pdf_sanitize` strips metadata and active payloads, users currently have no automated mechanism to detect *where* sensitive data resides within documents.

**Spec 086** defines `document_pii_scan`, a high-throughput, offline, pure-Rust pattern recognition and entity scanning engine built following the **Single Core Runner + 5 Thin Wrappers** architecture. It scans document text, identifies PII categories, computes confidence scores, and returns exact page and character bounding box coordinates ready for automated or one-click batch redaction.

---

## 2. PII Detection Capabilities

The engine implements regex-based and algorithmic checksum detectors (e.g., Luhn algorithm for credit cards, ISO 7064 for IBANs) across the following standard categories:

1. **Government & Identification Numbers**:
   - Social Security Numbers (US SSN: `\b\d{3}-\d{2}-\d{4}\b`)
   - Individual Taxpayer Identification Numbers (ITIN)
   - Driver's License & Passport patterns
2. **Financial Data**:
   - Credit Card Numbers (Visa, Mastercard, Amex, Discover with Luhn validation)
   - International Bank Account Numbers (IBAN with MOD 97-10 check)
   - Routing & Bank Account Numbers
3. **Personal & Contact Information**:
   - Email Addresses (`[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}`)
   - Phone Numbers (E.164, US formatted `(XXX) XXX-XXXX` or `XXX-XXX-XXXX`, International)
   - Street Addresses & ZIP/Postal Codes
4. **Health & Biometric Identifiers (HIPAA)**:
   - Medical Record Numbers (MRN)
   - Health Insurance Claim Numbers / NPI numbers
   - Dates of Birth paired with patient identifiers

---

## 3. Architecture Pattern: Single Core Runner + 5 Thin Wrappers

```
                     ┌────────────────────────────────────────────────────────┐
                     │   Single Core Runner (paperpilot-pdf / nlp)             │
                     │   fn scan_pii(bytes, &options) -> Result<PiiScanReport> │
                     └──────────────────────────┬─────────────────────────────┘
                                                │
         ┌──────────────────┬───────────────────┼───────────────────┬──────────────────┐
         │                  │                   │                   │                  │
         ▼                  ▼                   ▼                   ▼                  ▼
    1. Native CLI      2. Agent MCP       3. REST Gateway       4. WASM Engine     5. Edge Worker
  paperpilot pii-scan  document_pii_scan  POST /api/v1/pdf/     WasmPdfEngine.     POST /api/v1/
                         tool call            pii-scan             pii_scan           pii-scan
```

### 3.1 Data Structures (`paperpilot-core` or `paperpilot-pdf`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiScanOptions {
    #[serde(default = "default_true")]
    pub scan_ssn: bool,
    #[serde(default = "default_true")]
    pub scan_credit_cards: bool,
    #[serde(default = "default_true")]
    pub scan_emails: bool,
    #[serde(default = "default_true")]
    pub scan_phones: bool,
    #[serde(default = "default_true")]
    pub scan_iban: bool,
    #[serde(default)]
    pub custom_regex_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiFinding {
    pub category: String,       // "ssn", "credit_card", "email", "phone", "iban"
    pub value_masked: String,   // "XXX-XX-1234"
    pub page: u32,              // 1-indexed page number
    pub confidence: f32,        // 0.0 to 1.0
    pub context_snippet: String,// Surrounding text snippet
    pub bbox: Option<BoundingBox>, // [x, y, width, height] for direct visual redaction
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiScanReport {
    pub total_findings: usize,
    pub findings_by_category: HashMap<String, usize>,
    pub findings: Vec<PiiFinding>,
    pub execution_time_ms: f64,
}
```

---

## 4. Interface Specifications

### 4.1 💻 Native CLI
```bash
paperpilot pii-scan --input confidential.pdf --json
paperpilot pii-scan --input confidential.pdf --output pii_report.json
```

### 4.2 🤖 MCP Server
Tool: `document_pii_scan`
- `input`: File path or document identifier
- `categories`: Optional array of categories to filter
- Returns structured JSON-RPC report with findings and suggested redaction bounding boxes.

### 4.3 🌐 REST Gateway
- `POST /api/v1/pdf/pii-scan`
- Accepts multipart file upload or JSON payload `{ "input": "..." }`.
- Returns `200 OK` with `PiiScanReport`.

### 4.4 ⚡ WASM Engine (Browser Embed)
- `WasmPdfEngine.pii_scan(pdf_bytes: &[u8], options_json: &str) -> Result<String, JsValue>`
- Pure client-side regex & checksum evaluation without sending data to any external server.

### 4.5 ☁️ Cloudflare Edge
- `POST /api/v1/pii-scan` on the Edge worker.
- Executes via the WASM module in sub-10ms.

---

## 5. Verification Checklist & Success Criteria
1. Single Core Runner implemented and unit-tested in `paperpilot-pdf/src/operations/pii.rs` (or `scan_pii.rs`).
2. 5 Thin Wrappers cleanly implemented and wired.
3. 10/10 Penta-Interface real semantic assertions passing in `reports/PENTA_INTERFACE_E2E_PII_SCAN_REPORT.md`.
4. Zero unhandled panics, zero external network dependencies.
