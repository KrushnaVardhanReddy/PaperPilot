---
title: "semantic_search"
description: "Reference, CLI, MCP, and REST API commands for semantic_search."
sidebar:
  order: 58
---

**Status**: ✅ Verified  |  **Format**: PDF (`.pdf`) / Document Chunks

### What it does
**Performs structure-aware semantic chunk search and conceptual relevance ranking across PDF documents.**  
Segments documents into semantic context chunks (respecting document headings, paragraph bounds, and token overlap limits), computes token overlap and exact phrase relevance scores (`0.0` - `1.0`), and ranks the most relevant excerpts.

---

### CLI
Search across individual PDF documents or recursive directories:

```bash
paperpilot semantic-search \
  --dir tests/e2e_fixtures/real \
  --query "revenue" \
  --limit 3 \
  --output /tmp/paperpilot_test/sem_search.json
```

#### CLI Parameters
- `--dir` or `--input` *(string, optional, default: `"."`)*: Directory or specific PDF file to index and search.
- `--query` *(string, optional, default: `"test"`)*: Semantic query or topic to rank against document chunks.
- `--limit` *(integer, optional, default: `5`)*: Maximum number of top-scoring chunks to return.
- `--output` *(string, optional)*: File destination to write structured JSON search results.
- `--json` *(flag, optional)*: Emit status as JSON.

#### Example Output (`/tmp/paperpilot_test/sem_search.json`)
```json
{
  "query": "revenue",
  "total_results": 3,
  "matches": [],
  "results": [
    {
      "chunk_id": "financial_tables.pdf_chunk_0",
      "heading": "Document Section",
      "page": 1,
      "score": 1.0,
      "content": "Three-Year Consolidated Financial Model & Operational Forecast (FY2022 - FY2024)\n\n**Gross Revenue (FY24)** **$21,300,000**\n\n+34.8% YoY Expansion\n\n**Consolidated Statement of Operations & Income Performance**\n\n| FinancialMetric | FY2022 | FY2023 | FY2024 | GrowthRate |\n|---|---|---|---|---|\n| GrossRevenue | 12450000 | 15800000 | 21300000 | +34.8% |\n| SubscriptionRevenue | 9850000 | 13200000 | 18400000 | +39.4% |"
    },
    {
      "chunk_id": "financial_tables.pdf_chunk_2",
      "heading": "Document Section",
      "page": 2,
      "score": 1.0,
      "content": "DeferredSubscriptionRevenue | 2450000 | 3800000 | 5600000 | 16.3% |"
    }
  ]
}
```

---

### MCP JSON-RPC
Invoke `semantic_search` through Model Context Protocol:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "semantic_search",
    "arguments": {
      "input": "tests/e2e_fixtures/real",
      "query": "revenue trends",
      "limit": 5
    }
  },
  "id": 1
}
```

#### MCP Schema Parameters
- `query` *(string, required)*: Semantic search query or topic prompt.
- `input` or `dir` *(string, optional, default: `"."`)*: Document file path or directory to chunk and query.
- `limit` *(integer, optional, default: `5`)*: Maximum number of ranked results returned.
- `output` *(string, optional)*: Path to save result JSON file.

---

### REST API
Execute semantic chunk queries via PaperPilot Gateway:

```bash
# Dedicated Semantic Search Route
curl -s -X POST http://127.0.0.1:7823/api/v1/search/semantic_search \
  -H "Content-Type: application/json" \
  -d '{
    "input": "tests/e2e_fixtures/real",
    "query": "revenue trends",
    "limit": 3
  }'
```

```bash
# Universal MCP Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/semantic_search \
  -H "Content-Type: application/json" \
  -d '{
    "input": "tests/e2e_fixtures/real",
    "query": "revenue",
    "limit": 5
  }'
```

---

### Scoring & Chunking Architecture
PaperPilot processes documents using structure-aware intelligent chunking:
1. **Intelligent Chunking**: Target tokens: ~300 tokens with 40-token dynamic overlap, splitting on logical headings and paragraphs.
2. **Scoring Formulation**:
   - Token frequency count with double-weighting for matching heading tokens.
   - Overlap ratio: `matched_tokens / total_query_tokens`.
   - Exact phrase boost (+1.5) for verbatim query matches.
   - Normalized score range: `0.0` to `1.0`.

---

### Known Limitations & Troubleshooting
- **Legacy Encrypted PDFs**: PDFs encrypted with older RC4/Standard Security R≤4 algorithms require compilation with the `legacy-crypto` feature or prior decryption.
- **Scanned Non-OCR PDFs**: Semantic chunking requires extractable text streams. For scanned or image-based PDFs, run `paperpilot ocr` prior to running semantic queries.
- **WASM Support**: Standalone directory scanning is constrained by browser sandboxes. For client-side AI RAG in WASM, use single-document chunks via `pdf_chunk`.
