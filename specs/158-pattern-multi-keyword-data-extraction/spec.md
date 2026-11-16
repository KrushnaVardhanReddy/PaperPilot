# Spec 158: Pattern & Multi-Keyword Data Extraction (`pdf_extract_patterns`)

## 1. Executive Summary & Problem Statement
Currently, PaperPilot supports `document_pii_scan` for scanning hardcoded PII patterns (SSN, credit cards, emails, phone numbers, IBANs), and `pdf_search` for searching a single text query string. However, users, legal discovery specialists, and compliance auditors frequently need to:
1. Scan documents for an arbitrary array of domain-specific keywords (e.g. `["Confidential", "NDA", "Acquisition", "Gross Revenue"]`).
2. Scan documents for custom regular expressions with assigned category labels (e.g. `{"po_number": "PO-\\d{6}", "currency": "\\$[0-9,]+(\\.[0-9]{2})?", "case_id": "CASE-\\d{4}-[A-Z]+"}`).
3. Extract matching occurrences grouped by keyword/pattern with exact page numbers, hit frequencies, and surrounding text context snippets into a structured JSON report.

Spec 158 implements **`pdf_extract_patterns`** across all five PaperPilot interfaces (Rust Core, CLI, MCP Server, REST Gateway, Desktop UI / Web).

---

## 2. Technical Requirements & Architecture

### 2.1 Core Rust Operation (`paperpilot-pdf`)
- Implement in `paperpilot-pdf/src/operations/pdf_extract_patterns.rs`.
- Extract text per page using `lopdf::Document::extract_text`.
- For each supplied keyword:
  - Perform case-insensitive or case-sensitive match.
  - Record `label`, `match_text`, `page`, and surrounding `context_snippet` (default 30 characters before and after).
- For each supplied regex pattern:
  - Compile pattern safely using `regex::RegexBuilder` (respecting `case_sensitive`).
  - Stream matches, recording `label`, `match_text`, `page`, and `context_snippet`.
- Generate summary statistics:
  - `total_matches`: total count of all matched instances.
  - `matches_by_label`: HashMap mapping each keyword / regex key to its match count.
  - `results`: vector of `PatternFinding`.
  - `execution_time_ms`: duration in milliseconds.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternFinding {
    pub label: String,
    pub pattern_type: String, // "keyword" | "regex"
    pub value: String,
    pub page: u32,
    pub context_snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternExtractionReport {
    pub total_matches: usize,
    pub matches_by_label: HashMap<String, usize>,
    pub findings: Vec<PatternFinding>,
    pub execution_time_ms: f64,
}
```

### 2.2 Penta-Interface Coverage
1. **Rust Library (`paperpilot-pdf`)**:
   - `extract_patterns(input_bytes: &[u8], options: &PdfExtractPatternsOptions) -> Result<PatternExtractionReport, PdfError>`
2. **CLI (`paperpilot-cli`)**:
   - `paperpilot pdf-extract-patterns <INPUT> [OUTPUT] [--keywords <KEYWORD>...] [--patterns <KEY=REGEX>...] [--case-sensitive] [--context-len <CHARS>]`
3. **MCP Server (`paperpilot-mcp`)**:
   - Expose `pdf_extract_patterns` tool with complete JSON Schema describing `keywords`, `patterns`, `case_sensitive`, `context_len`, `input`, and optional `output`.
4. **REST API Gateway (`paperpilot-gateway`)**:
   - POST `/api/v1/pdf-extract-patterns` multipart file + JSON payload returning the extraction report.
5. **Desktop / Web UI (`apps/desktop` & `apps/web`)**:
   - Register in `toolList.ts` under category `intelligence` with icon `🔍`.
   - Add parameter fields in `DynamicActionControls.svelte` (tags input for keywords, key-value table for regex patterns, context length slider).
   - Show preview report card displaying match counts and snippets.

---

## 3. Verification & Acceptance Criteria
- Unit tests in `paperpilot-pdf` verifying keyword matches, regex matches, empty documents, invalid regexes, and case-sensitivity.
- CLI execution test verifying stdout JSON report and file write.
- MCP test verifying tool invocation and JSON-RPC response.
- Desktop UI tests in `apps/desktop` verifying `toolList.ts` and `buildOperationArgs`.
