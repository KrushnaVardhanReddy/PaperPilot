---
title: Alternating PDF Merge
description: Interleave pages from two PDF documents (ideal for duplex scanner collation).
---

The `pdf_merge_alternate` tool allows you to interleave pages from two different PDF documents. This is a critical operation for manual duplex scanning, where you might have one PDF containing all the front pages and another PDF containing the back pages.

## Usage Examples

### CLI

To merge two documents, alternating one page from each:

```bash
paperpilot pdf-merge-alternate \
  --input-a front_pages.pdf \
  --input-b back_pages.pdf \
  --output merged.pdf
```

To reverse the order of the back pages (common in some batch scanner feeds) and take 1 page at a time:

```bash
paperpilot pdf-merge-alternate \
  --input-a front_pages.pdf \
  --input-b back_pages.pdf \
  --output merged.pdf \
  --reverse-b \
  --step 1 \
  --step-b 1
```

### MCP (JSON-RPC)

You can invoke this tool over MCP by providing `input_a`, `input_b`, and `output`.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "call_tool",
  "params": {
    "name": "pdf_merge_alternate",
    "arguments": {
      "input_a": "/path/to/front.pdf",
      "input_b": "/path/to/back.pdf",
      "output": "/path/to/merged.pdf",
      "reverse_b": true
    }
  }
}
```

### REST API

Send a `POST` request to the gateway endpoint:

**Endpoint:** `POST /api/v1/pdf/merge-alternate`

**Payload:**

```json
{
  "input_a": "/path/to/front.pdf",
  "input_b": "/path/to/back.pdf",
  "output": "/path/to/merged.pdf",
  "step": 1,
  "step_b": 1,
  "reverse_a": false,
  "reverse_b": true
}
```
