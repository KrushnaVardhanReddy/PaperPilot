---
title: "pdf_edit_text"
description: "Reference, CLI, MCP, and REST API commands for pdf_edit_text (Edit PDF Text)."
sidebar:
  order: 90
---

**Status**: ✅ Verified  |  **Added in**: `v0.8.0+`  |  **Format**: PDF $\to$ .pdf

### What it does
**Find and replace text typos in-place inside PDF content streams**

---

### CLI
`pdf_edit_text` is invoked via `paperpilot edit-text` (alias `pdf-edit-text`):

```bash
paperpilot edit-text \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/pdf_edit_text_out.pdf \
  --find "Helo" \
  --replace "Hello" \
  --page 1 \
  --occurrence 1 \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input file.
- `--output` *(string, required)*: Path to the destination output file.
- `--find` *(string, required)*: Literal text string to find in PDF content streams.
- `--replace` *(string, required)*: Replacement text string.
- `--page` *(integer, optional)*: 1-indexed page number to edit (omit for all pages).
- `--occurrence` *(integer, optional)*: 1-indexed match occurrence to replace (e.g. `1` replaces only the 1st match; omit to replace all matches).
- `--match-case` *(boolean, optional)*: Case-sensitive matching flag (defaults to `true`).
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
Invoke `pdf_edit_text` via Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_edit_text",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/pdf_edit_text_out.pdf",
      "find": "Helo",
      "replace": "Hello",
      "page": 1,
      "occurrence": 1,
      "match_case": true
    }
  },
  "id": 1
}
```

---

### REST API
Invoke the REST API endpoint `/api/v1/pdf/edit-text` (or `/api/v1/pdf/pdf_edit_text`):

```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/edit-text \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/pdf_edit_text_out.pdf",
    "find": "Helo",
    "replace": "Hello",
    "page": 1,
    "occurrence": 1,
    "match_case": true
  }'
```
