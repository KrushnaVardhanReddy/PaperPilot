---
title: "pdf_extract_patterns"
description: "Reference, CLI, MCP, REST API, and Desktop commands for pdf_extract_patterns."
sidebar:
  order: 95
---

**Status**: ✅ Verified  |  **Format**: PDF $\to$ JSON

### What it does
**Scans PDF documents for custom keywords and arbitrary regular expression patterns, extracting all occurrences across all pages.**  
Unlike basic single-query search or hardcoded PII detection, `pdf_extract_patterns` allows compliance officers, legal auditors, and developers to supply arbitrary keyword lists (e.g. `["Confidential", "NDA", "Acquisition"]`) and named regex patterns (e.g. `{"ssn": "\\d{3}-\\d{2}-\\d{4}", "po_number": "PO-\\d{6}"}`).

Every single match across every page is captured with its label, value, page number, and surrounding context snippet ($\pm 30$ characters).

### CLI
```bash
# Basic keyword extraction
paperpilot pdf-extract-patterns --input document.pdf --keywords "Confidential,Acquisition,NDA"

# Regex extraction saved to JSON report
paperpilot pdf-extract-patterns \
  --input document.pdf \
  --output report.json \
  --pattern "ssn=\d{3}-\d{2}-\d{4}" \
  --pattern "email=[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}" \
  --context-len 40
```

### MCP JSON-RPC
```json
{
  "name": "pdf_extract_patterns",
  "arguments": {
    "input": "/path/to/document.pdf",
    "output": "/path/to/report.json",
    "keywords": ["Confidential", "Urgent", "Privileged"],
    "patterns": {
      "ssn": "\\d{3}-\\d{2}-\\d{4}",
      "email": "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"
    },
    "case_sensitive": false,
    "context_len": 30
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/extract-patterns \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "document.pdf",
    "output": "report.json",
    "keywords": ["Confidential"],
    "patterns": { "ssn": "\\d{3}-\\d{2}-\\d{4}" }
  }'
```

### Report JSON Schema
```json
{
  "total_matches": 12,
  "matches_by_label": {
    "ssn": 4,
    "email": 8
  },
  "findings": [
    {
      "label": "ssn",
      "pattern_type": "regex",
      "value": "123-45-6789",
      "page": 1,
      "context_snippet": "...taxpayer identification number 123-45-6789 listed on file..."
    }
  ],
  "execution_time_ms": 14.2
}
```
