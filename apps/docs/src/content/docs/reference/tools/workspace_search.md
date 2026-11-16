---
title: "workspace_search"
description: "Reference, CLI, MCP, and REST API commands for workspace_search."
sidebar:
  order: 53
---

**Status**: ✅ Verified  |  **Format**: Multi-Format (`.pdf`, `.docx`, `.xlsx`, `.txt`, `.md`, `.json`, `.csv`)

### What it does
**Performs recursive, multi-format text discovery and snippet extraction across a workspace directory.**  
Recursively traverses folder trees to find matching search terms across PDF documents, Microsoft Word files (`.docx`), Excel spreadsheets (`.xlsx`), Markdown notes (`.md`), plain text (`.txt`), JSON data, and CSV sheets without requiring third-party indexing engines.

---

### CLI
Search a local workspace or specific folder:

```bash
paperpilot workspace-search \
  --dir tests/e2e_fixtures/real \
  --query "invoice" \
  --formats "pdf,json" \
  --output /tmp/paperpilot_test/ws_search.json
```

#### CLI Parameters
- `--dir` or `--input` *(string, optional, default: `"."`)*: Root folder directory or single document file path to inspect.
- `--query` *(string, required)*: Text keyword to search for across files.
- `--formats` *(string, optional, default: `"all"`)*: Comma-separated list of extensions to include (`pdf`, `docx`, `xlsx`, `txt`, `md`, `csv`, `json`).
- `--output` *(string, optional)*: Destination path to write structured JSON search results.
- `--json` *(flag, optional)*: Emit status as JSON.

#### Example Output (`/tmp/paperpilot_test/ws_search.json`)
```json
{
  "formats_searched": ["pdf", "json"],
  "query": "invoice",
  "total_matches": 3,
  "pages": [1, 1, 1],
  "matches": [
    {
      "file": "single_page.pdf",
      "type": "pdf",
      "location": "page 1",
      "snippet": "SUBJECT:   Single Page Corporate Memo & Invoice Verification  Executive Summary & Quart"
    },
    {
      "file": "stationery.pdf",
      "type": "pdf",
      "location": "page 1",
      "snippet": "SUBJECT:   Single Page Corporate Memo & Invoice Verification  Executive Summary & Quart"
    }
  ]
}
```

---

### MCP JSON-RPC
Invoke `workspace_search` through Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "workspace_search",
    "arguments": {
      "dir": "tests/e2e_fixtures/real",
      "query": "invoice",
      "formats": "pdf,docx,xlsx,md"
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `query` *(string, required)*: Target search text.
- `dir` or `input` *(string, optional, default: `"."`)*: Workspace directory path or single file.
- `formats` *(string, optional, default: `"all"`)*: File extensions filter (`pdf,txt,md,docx,xlsx,csv,json` or `all`).
- `output` *(string, optional)*: Output JSON path.

---

### REST API
Execute directory scans via PaperPilot Gateway:

```bash
# Dedicated Search Route
curl -s -X POST http://127.0.0.1:7823/api/v1/search/workspace_search \
  -H 'Content-Type: application/json' \
  -d '{
    "dir": "tests/e2e_fixtures/real",
    "query": "invoice",
    "formats": "pdf,json"
  }'
```

```bash
# Universal MCP Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/workspace_search \
  -H 'Content-Type: application/json' \
  -d '{
    "dir": "tests/e2e_fixtures/real",
    "query": "invoice",
    "formats": "all"
  }'
```

---

### Known Limitations & Troubleshooting
- **Binary & Encrypted PDFs**: Password-protected PDFs are reported in system logs as encrypted and will be bypassed unless decrypted beforehand.
- **Large Excel Workbooks**: Searches across `.xlsx` read every worksheet row sequentially. Extremely large multi-megabyte spreadsheets may increase scan latency.
- **WASM Support**: Full local filesystem directory recursion is unavailable in browser WASM sandboxes due to browser security constraints. Use CLI or REST API for workspace traversal.
