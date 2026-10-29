---
title: "semantic_search"
description: "Reference, CLI, MCP, and REST API commands for semantic_search."
sidebar:
  order: 58
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Searches document chunks using semantic cosine vector similarity.**  
Queries the embedded `paperpilot-storage` vector store database to retrieve relevant passage chunks based on conceptual meaning rather than exact keyword matches.

### CLI
```bash
paperpilot search --query "quarterly revenue trends" --dir ./docs --json
```

### MCP JSON-RPC
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "quarterly revenue trends",
    "limit": 5
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/search/semantic -H "Content-Type: application/json" -d '{"query": "quarterly revenue trends", "limit": 5}'
```
