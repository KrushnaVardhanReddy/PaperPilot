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
paperpilot pdf-chunk --input document.pdf --target-tokens 500 --overlap 50 --strip-boilerplate --json

# Custom chunking saving output directly to a JSON file
paperpilot pdf-chunk \
  --input large_report.pdf \
  --target-tokens 1000 \
  --overlap 100 \
  --strip-boilerplate \
  --break-on-headings \
  --output report_chunks.json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_chunk",
  "arguments": {
    "input": "document.pdf",
    "target_tokens": 500,
    "overlap_tokens": 50,
    "strip_boilerplate": true,
    "break_on_headings": true
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/chunk \
  -H 'Content-Type: application/json' \
  -d '{"input": "document.pdf", "target_tokens": 500, "overlap_tokens": 50, "strip_boilerplate": true, "break_on_headings": true}'
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
| Option | Type | Default | Description |
|---|---|---|---|
| `input` | `string` | *(required)* | Path to the input PDF file. |
| `target_tokens` | `integer` | `500` | Target token size per chunk (min: 50, max: 4000). |
| `overlap_tokens` | `integer` | `50` | Number of overlap tokens between adjacent chunks (min: 0, max: 500). |
| `strip_boilerplate` | `boolean` | `true` | Suppresses recurring running headers and footers across pages. |
| `break_on_headings` | `boolean` | `true` | Enforces hard chunk boundaries at Markdown headings (`#`, `##`). |
| `output` | `string` | `null` | Optional file path to write output JSON to disk. |
