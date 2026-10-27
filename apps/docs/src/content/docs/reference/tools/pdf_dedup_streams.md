---
title: Lossless Stream Deduplication
description: Lossless hash-based indirect object stream deduplication & CMYK-safe image compression
---

# Lossless Stream Deduplication (`pdf_dedup_streams`)

The `pdf_dedup_streams` tool performs a lossless object stream deduplication pass on the PDF document. It leverages cryptographic hashing (SHA-256) to identify byte-identical streams across the entire document structure, ensuring zero false positives. Duplicate streams are merged into a single canonical indirect object, reducing file size significantly without sacrificing any data.

It also incorporates a CMYK color space guard, preserving original prepress and print quality profiles (`/DeviceCMYK` or `/ICCBased`) during general document compression workflows to prevent irreversible color-space distortion.

### Invocation Examples

#### 1. CLI
Use the CLI to deduplicate streams in a PDF and save to a new file:
```bash
paperpilot dedup-streams input.pdf -o deduped_output.pdf
```

#### 2. MCP JSON-RPC
Invoke the operation via a generic tool execution request in the MCP server:
```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "method": "call_tool",
  "params": {
    "name": "pdf_dedup_streams",
    "arguments": {
      "input": "<base64_encoded_pdf_bytes>"
    }
  }
}
```

#### 3. REST API
Deduplicate streams by making a POST request to the Gateway API:
```bash
curl -X POST http://localhost:3000/api/v1/pdf/dedup-streams \
  -H "Content-Type: application/json" \
  -d '{
    "input": "<base64_encoded_pdf_bytes>"
  }'
```
