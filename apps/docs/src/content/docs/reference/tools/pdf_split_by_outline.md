---
title: Split PDF by Outline
description: Split multi-chapter PDF documents based on their /Outlines bookmark hierarchy
---

# Split PDF by Outline (`pdf_split_by_outline`)

The `pdf_split_by_outline` tool automatically extracts sections of a PDF document based on its internal Bookmark (`/Outlines`) structure. This is particularly useful for textbooks or technical manuals where you want a single file split into multiple chapter PDFs.

## Options

- **`max_depth`**: The maximum depth to traverse the bookmark tree. `1` limits the extraction to top-level sections (e.g., Chapters). `2` includes Sub-sections, etc. Defaults to `1`.
- **`sanitize_names`**: Whether to sanitize bookmark titles when generating output filenames, replacing illegal filename characters with underscores. Defaults to `true`.

## Usage Examples

### CLI

```bash
paperpilot pdf-split-by-outline --input my-book.pdf --output output.zip --max-depth 1
```

### REST API

```bash
curl -X POST http://localhost:7823/api/v1/pdf/split-by-outline \
  -H "Content-Type: application/json" \
  -d '{
    "input": "my-book.pdf",
    "output": "output.zip",
    "max_depth": 1,
    "sanitize_names": true
  }'
```

### MCP (JSON-RPC)

```json
{
  "jsonrpc": "2.0",
  "method": "call_tool",
  "params": {
    "name": "pdf_split_by_outline",
    "arguments": {
      "input": "my-book.pdf",
      "output": "output.zip",
      "max_depth": 1,
      "sanitize_names": true
    }
  },
  "id": 1
}
```
