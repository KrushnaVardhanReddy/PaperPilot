# Tool Reference: `pdf_extract_patterns` (Spec 158)

### Summary & Status
- **Canonical ID:** `pdf_extract_patterns`
- **Category:** `intelligence`
- **Input Format:** PDF (`.pdf`)
- **Output Format:** JSON (`.json`)
- **Interfaces Supported:** Rust Engine (`paperpilot-pdf`), CLI (`paperpilot-cli`), MCP Server (`paperpilot-mcp`), REST Gateway (`paperpilot-gateway`), WebAssembly (`paperpilot-wasm`), Desktop / Web UI.

---

### Description
Scan document for custom keywords and regex patterns with match contexts. Extracts **all occurrences** (not just the first match) across all pages, recording:
- Target match label
- Pattern type (`"keyword"` or `"regex"`)
- Matched value
- Exact page number
- Surrounding contextual snippet ($\pm 30$ characters)

---

### Penta-Interface Invocation Examples

#### 1. Command Line Interface (CLI)
```bash
# Keyword scan across all pages
paperpilot pdf-extract-patterns --input sample.pdf --keywords "Confidential,NDA,Financial"

# Multi-regex pattern extraction outputting JSON report
paperpilot pdf-extract-patterns \
  --input sample.pdf \
  --output extraction_report.json \
  --pattern "ssn=\d{3}-\d{2}-\d{4}" \
  --pattern "email=[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}" \
  --context-len 35
```

#### 2. MCP Server (JSON-RPC)
```json
{
  "name": "pdf_extract_patterns",
  "arguments": {
    "input": "/path/to/sample.pdf",
    "output": "/path/to/report.json",
    "keywords": ["Confidential", "Acquisition"],
    "patterns": {
      "ssn": "\\d{3}-\\d{2}-\\d{4}",
      "email": "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"
    },
    "case_sensitive": false,
    "context_len": 30
  }
}
```

#### 3. REST API Gateway
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/extract-patterns \
  -H "Content-Type: application/json" \
  -d '{
    "input": "/path/to/sample.pdf",
    "output": "/path/to/report.json",
    "keywords": ["Contract"],
    "patterns": {
      "ssn": "\\d{3}-\\d{2}-\\d{4}"
    }
  }'
```

#### 4. WebAssembly (WASM) / Browser
```javascript
import init, { pdf_extract_patterns } from 'paperpilot-wasm';

await init();
const reportBytes = pdf_extract_patterns(pdfUint8Array, JSON.stringify({
  keywords: ["Confidential"],
  patterns: { ssn: "\\d{3}-\\d{2}-\\d{4}" }
}));
const report = JSON.parse(new TextDecoder().decode(reportBytes));
console.log(report.total_matches, report.findings);
```

#### 5. Desktop & Web UI
- Available in the **Intelligence / Search** panel with icon `🔍`.
- Enter comma-separated keywords and custom regex definitions.
- Set context length via the interactive slider and run the local zero-cloud extraction.
