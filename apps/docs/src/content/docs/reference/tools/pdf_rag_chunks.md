---
title: "pdf_rag_chunks"
description: "Reference, CLI, MCP, and REST API commands for pdf_rag_chunks."
sidebar:
  order: 35
---

**Status**: ✅ Verified  |  **Format**: PDF $\to$ JSON

### What it does
**Splits a PDF document into section-level semantic text chunks tailored for RAG vector indexing.**  
`pdf_rag_chunks` parses the layout and heading hierarchy of a PDF, partitioning the content into discrete chunks categorized by `page`, `section_title`, `content`, and estimated `token_count`.

### CLI
```bash
# Extract RAG chunks and print JSON output to stdout
paperpilot rag-chunks --input tests/e2e_fixtures/real/single_page.pdf --json

# Save structured RAG chunks directly to a JSON file
paperpilot rag-chunks \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output /tmp/rag_chunks_out.json
```

**CLI Output (`/tmp/rag_chunks_out.json`):**
```json
[
  {
    "page": 1,
    "section_title": "PaperPilot Enterprise Solutions",
    "content": "Global Document Technology Division",
    "token_count": 4
  },
  {
    "page": 1,
    "section_title": "DATE:",
    "content": "November 6, 2026",
    "token_count": 3
  },
  {
    "page": 1,
    "section_title": "TO:",
    "content": "Corporate Engineering & Financial Operations",
    "token_count": 5
  },
  {
    "page": 1,
    "section_title": "FROM:",
    "content": "Executive Office of Document Systems",
    "token_count": 5
  },
  {
    "page": 1,
    "section_title": "Executive Summary & Quarterly Document  Statement",
    "content": "This official communication certifies the deployment and verification of the PaperPilot autonomous document processing cluster...",
    "token_count": 152
  }
]
```

### MCP JSON-RPC
Tool definition: `pdf_rag_chunks` in `paperpilot-mcp/src/server.rs`.
```json
{
  "name": "pdf_rag_chunks",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "/tmp/rag_chunks_out.json"
  }
}
```

**MCP Response:**
```json
{
  "success": true,
  "message": "Generated 5 RAG chunks",
  "output_path": "/tmp/rag_chunks_out.json",
  "data": [
    {
      "page": 1,
      "section_title": "PaperPilot Enterprise Solutions",
      "content": "Global Document Technology Division",
      "token_count": 4
    },
    {
      "page": 1,
      "section_title": "DATE:",
      "content": "November 6, 2026",
      "token_count": 3
    },
    {
      "page": 1,
      "section_title": "Executive Summary & Quarterly Document  Statement",
      "content": "This official communication certifies...",
      "token_count": 152
    }
  ]
}
```

### REST API
Dedicated routes: `POST /api/v1/pdf/pdf_rag_chunks` or `POST /api/v1/pdf/tools/pdf_rag_chunks` (Local Gateway default: port 7823).
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_rag_chunks \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }'
```

**REST API Response:**
```json
{
  "success": true,
  "message": "Generated 5 RAG chunks",
  "output_path": null,
  "data": [
    {
      "page": 1,
      "section_title": "PaperPilot Enterprise Solutions",
      "content": "Global Document Technology Division",
      "token_count": 4
    }
  ]
}
```

### Parameter Reference
| Option | CLI Flag | Type | Required | Description |
|---|---|---|---|---|
| `input` | `--input` | `string` | Yes | Path to the input PDF file. |
| `output` | `--output` | `string` | No | Optional destination path to write the JSON chunks array. |

### Comparison: `pdf_rag_chunks` vs `pdf_chunk`
- **`pdf_rag_chunks`**: Section-level chunking based on natural document headings (`section_title`, `content`, `token_count`). Ideal for documents with short, self-contained sections.
- **`pdf_chunk`**: Structure-aware semantic chunking with configurable `target_tokens`, sliding `overlap_tokens`, boilerplate suppression, Markdown table preservation, and heading hierarchy context. Recommended for feeding vector databases and embedding pipelines.
