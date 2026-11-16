---
title: "pdf_extract_patterns"
description: "Reference, CLI, MCP, REST API, and WASM commands for pdf_extract_patterns."
sidebar:
  order: 95
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) $\to$ JSON (`.json`)

### What it does
**Scans PDF documents for custom keywords and arbitrary regular expression patterns, capturing match contexts and spatial coordinates across all pages.**  
Allows compliance officers, legal auditors, and AI pipelines to search arbitrary keyword lists (e.g. `["Confidential", "NDA", "Acquisition"]`) and regex patterns (e.g. `{"ssn": "\\d{3}-\\d{2}-\\d{4}", "po_number": "PO-\\d{6}"}`).

Every match captures its assigned label, value, page number, bounding box rectangle (`rect`), and surrounding context snippet ($\pm 30$ characters).

---

### CLI
Scan patterns and keywords via the PaperPilot CLI using `pdf-extract-patterns`:

```bash
paperpilot pdf-extract-patterns \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --keywords "PaperPilot,Enterprise" \
  --pattern "year=\d{4}" \
  --context-len 25 \
  --output tests/e2e_fixtures/out/penta_e2e/patterns.json \
  --json
```

#### CLI Parameters
- `--input <INPUT>` *(string, required)*: Path to the source PDF document.
- `--output <OUTPUT>` *(string, optional)*: Destination file path for the JSON report.
- `--keywords <KEYWORDS>` *(string, optional)*: Comma-separated list of keywords to match.
- `--pattern <LABEL=REGEX>` *(string, optional, repeatable)*: Regular expressions formatted as `label=regex`.
- `--case-sensitive` *(flag, optional, default: `false`)*: Perform case-sensitive matching.
- `--context-len <CONTEXT_LEN>` *(integer, optional, default: `30`)*: Context length in characters before and after match.
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `pdf_extract_patterns`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_patterns",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/patterns.json",
      "keywords": ["PaperPilot", "Enterprise"],
      "patterns": {
        "year": "\\d{4}"
      },
      "case_sensitive": false,
      "context_len": 25
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `input` *(string, required)*: Path to the input PDF file.
- `output` *(string, optional)*: Path for the output report JSON.
- `keywords` *(string[], optional)*: Array of keywords to search for across all pages.
- `patterns` *(object, optional)*: Key-value map of pattern label to regex string (e.g. `{"ssn": "\\d{3}-\\d{2}-\\d{4}"}`).
- `case_sensitive` *(boolean, optional, default: `false`)*: Whether matching should be case-sensitive.
- `context_len` *(integer, optional, default: `30`)*: Context characters surrounding the match.

---

### REST API
Execute pattern extraction via PaperPilot Gateway:

#### 1. Direct Route (`/api/v1/pdf/extract-patterns`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/extract-patterns \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "keywords": ["PaperPilot"],
    "patterns": {
      "year": "\\d{4}"
    },
    "context_len": 20
  }'
```

#### 2. Canonical Tool Route (`/api/v1/pdf/tools/pdf_extract_patterns`)
```bash
curl -s --noproxy '*' -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_extract_patterns \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "keywords": ["PaperPilot"],
    "output": "tests/e2e_fixtures/out/penta_e2e/patterns.json"
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "f45f8ad2-e7d3-42b7-86ed-9ea08de0b10e",
  "message": "Success",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Extract Keywords & Patterns executed successfully\",\"output_path\":null,\"data\":{\"execution_time_ms\":19.8,\"findings\":[{\"context_snippet\":\"PaperPilot Enterprise Solution\",\"label\":\"PaperPilot\",\"page\":1,\"pattern_type\":\"keyword\",\"rect\":[56.69,762.69,98.91,20.0],\"value\":\"PaperPilot\"}],\"matches_by_label\":{\"PaperPilot\":4,\"year\":6},\"total_matches\":10}}"
      }
    ]
  }
}
```

---

### WebAssembly (WASM)
In browser or worker contexts using `@paperpilot/wasm`:

```javascript
import { WasmPdfEngine } from 'paperpilot-wasm';

const options = JSON.stringify({
  keywords: ["Confidential", "NDA"],
  patterns: { ssn: "\\d{3}-\\d{2}-\\d{4}" },
  case_sensitive: false,
  context_len: 30
});

// Returns Uint8Array encoded JSON report bytes
const reportBytes = WasmPdfEngine.pdf_extract_patterns(pdfBytes, options);
const report = JSON.parse(new TextDecoder().decode(reportBytes));
console.log(`Found ${report.total_matches} matches`);
```

---

### Known Limitations & Troubleshooting
- **Regex Dialect**: Regular expressions follow the Rust `regex` crate specification (PCRE-compatible, without lookaround assertions like `(?=...)` or `(?!...)`).
- **CLI Subcommand Naming**: Note that the CLI subcommand uses hyphens (`pdf-extract-patterns`), while MCP and REST API use underscores (`pdf_extract_patterns`).
- **Memory Footprint on Large Books**: Scanning large documents ($>1,000$ pages) with dozens of complex regex patterns compiles each pattern once and scans per page. For ultra-fast queries on static texts, consider combining with pre-indexed search.
