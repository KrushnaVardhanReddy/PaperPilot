---
title: "pdf_chunk"
description: "Reference, CLI, MCP, and REST API commands for pdf_chunk intelligent semantic chunking."
sidebar:
  order: 36
---

**Status**: ✅ Verified  |  **Format**: PDF $\to$ JSON

### What it does
**Splits PDF documents into intelligent, structure-aware semantic text chunks tailored for LLMs and AI agent context windows.**

Unlike naive character or fixed-window splitters, `pdf_chunk` respects the intrinsic hierarchy and structural flow of documents:
- **Heading Hierarchy Preservation**: Recognizes Markdown headings (`#`, `##`, `###`) and enforces clean hard breaks (when `break_on_headings: true`), ensuring topics are not awkwardly split across chunk boundaries.
- **Heading Context Tracking**: Every generated chunk includes `headings_context`, an array of active ancestor headings that preserves conversational context even for chunks deep within sub-sections.
- **Markdown Table Integrity**: Detects Markdown tables (`| ... |`) and keeps rows together in a single chunk whenever possible, avoiding fragmented data representations.
- **Boilerplate Suppression**: When `strip_boilerplate: true`, automatically detects and removes repetitive running headers and footers that repeat across consecutive pages (e.g., page numbers, confidential notices, company letterheads).
- **Dynamic Overlap**: When beginning subsequent chunks, rewinds by up to `overlap_tokens` sentences/words from previous content to maintain semantic continuity, without carrying over major heading boundaries.
- **Memory-Mapped Streaming**: Seamlessly integrates with `DocumentSource` zero-copy memory mapping on native platforms for fast, low-memory chunking of large multi-hundred-page files.

### CLI
```bash
# Basic semantic chunking with default 500 target tokens and 50 token overlap
paperpilot pdf-chunk --input tests/e2e_fixtures/real/single_page.pdf --json

# Custom chunking saving output directly to a JSON file
paperpilot pdf-chunk \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --target-tokens 500 \
  --overlap 50 \
  --strip-boilerplate \
  --break-on-headings \
  --output /tmp/pdf_chunk_out.json
```

### MCP JSON-RPC
Tool definition: `pdf_chunk` in `paperpilot-mcp/src/server.rs`.
```json
{
  "name": "pdf_chunk",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "target_tokens": 500,
    "overlap_tokens": 50,
    "strip_boilerplate": true,
    "break_on_headings": true
  }
}
```

**MCP Response:**
```json
{
  "success": true,
  "message": "Generated 2 semantic chunks (375 total tokens)",
  "output_path": null,
  "data": {
    "total_chunks": 2,
    "total_tokens": 375,
    "chunks": [
      {
        "chunk_index": 0,
        "page_start": 1,
        "page_end": 1,
        "estimated_tokens": 68,
        "text": "# PaperPilot Enterprise Solutions\n\nGlobal Document Technology Division\n\n#### DATE:\n\nNovember 6, 2026\n\n#### TO:\n\nCorporate Engineering & Financial Operations\n\n#### FROM:\n\nExecutive Office of Document Systems\n\n#### SUBJECT: Single Page Corporate Memo & Invoice Verification",
        "headings_context": [
          "PaperPilot Enterprise Solutions"
        ]
      },
      {
        "chunk_index": 1,
        "page_start": 1,
        "page_end": 1,
        "estimated_tokens": 307,
        "text": "# Executive Summary & Quarterly Document  Statement\n\nThis official communication certifies the deployment and verification of the PaperPilot autonomous\n\ndocument processing cluster. Single page text verification demonstrates deterministic layout fidelity,\n\nhigh-precision typography rendering, and compliance with enterprise archiving standards.\n\n**Itemized Invoice Statement #INV-2026-8819**\n\n| Service Description | Units | Unit Rate | Total (USD) |\n|---|---|---|---|\n| Autonomous Pipeline Execution Engine | 1 Core Node | $1,250.00 | $1,250.00 |\n| Lossless Document Transformation & Imposition | 50,000 Ops | $0.015 | $750.00 |\n| Cryptographic Integrity & Signature Engine | 1 Dedicated Seat | $500.00 | $500.00 |\n|  |  |  | **$2,500.00** |\n\n**Authorized Representative:** J. Sterling, Director of\n\nOperations\n\n**Date Approved:** November 6, 2026\n\nPaperPilot Enterprise Solutions • Confidential • Single Page Document Invariant Verification\n\n100 Technology Plaza, Suite 400\n\nSan Francisco, CA 94105\n\n[contact@paperpilot.local](mailto:contact@paperpilot.local) | +1 (555) 019-2834\n\n**Total Amount Due:**\n\n**System Verification:**\n\nCORP-01\n\n**Document Reference:**\n\nverified\n\nSHA-256 Engine Verified #PP-\n\nSingle page text memo",
        "headings_context": [
          "Executive Summary & Quarterly Document  Statement"
        ]
      }
    ],
    "execution_time_ms": 0.17
  }
}
```

### REST API
Dedicated routes: `POST /api/v1/pdf/chunk` or `POST /api/v1/pdf/pdf_chunk` (Local Gateway default: port 7823).
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/chunk \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "target_tokens": 500,
    "overlap_tokens": 50,
    "strip_boilerplate": true,
    "break_on_headings": true
  }'
```

### Return Structure & Output Schema
```json
{
  "total_chunks": 2,
  "total_tokens": 375,
  "chunks": [
    {
      "chunk_index": 0,
      "page_start": 1,
      "page_end": 1,
      "estimated_tokens": 68,
      "text": "# Executive Summary\n\nThis quarterly report highlights key operational milestones...",
      "headings_context": [
        "Executive Summary"
      ]
    },
    {
      "chunk_index": 1,
      "page_start": 1,
      "page_end": 2,
      "estimated_tokens": 307,
      "text": "## Financial Performance\n\n| Department | Budget | Spend |\n|---|---|---|\n| Engineering | $1.2M | $1.1M |\n...",
      "headings_context": [
        "Executive Summary",
        "Financial Performance"
      ]
    }
  ],
  "execution_time_ms": 12.4
}
```

### Parameter Reference
| Option | CLI Flag | Type | Default | Description |
|---|---|---|---|---|
| `input` | `--input` | `string` | *(required)* | Path to the input PDF file. |
| `target_tokens` | `--target-tokens` | `integer` | `500` | Target token size per chunk (min: 50, max: 4000). |
| `overlap_tokens` | `--overlap` | `integer` | `50` | Number of overlap tokens between adjacent chunks (min: 0, max: 500). |
| `strip_boilerplate` | `--strip-boilerplate` / `--no-strip-boilerplate` | `boolean` | `true` | Suppresses recurring running headers and footers across pages. |
| `break_on_headings` | `--break-on-headings` / `--no-break-on-headings` | `boolean` | `true` | Enforces hard chunk boundaries at Markdown headings (`#`, `##`). |
| `output` | `--output` | `string` | `null` | Optional file path to write output JSON to disk. |

### Comparison: `pdf_chunk` vs `pdf_rag_chunks`
- **`pdf_chunk`** (Spec 159): Advanced structure-aware chunker with target token sizing, dynamic overlap windowing, boilerplate suppression, heading context tracking, and table protection. Recommended for RAG vector databases and LLM agents.
- **`pdf_rag_chunks`**: Section-level chunker splitting directly on detected section headings (`section_title`, `content`, `token_count`).
