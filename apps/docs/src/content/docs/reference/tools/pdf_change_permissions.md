---
title: Modify PDF Permissions (pdf_change_permissions)
description: Configure user and owner permissions for printing, copying, and modification.
---

# Modify PDF Permissions

Configure user and owner permissions for printing, copying, and modification according to the ISO 32000-1 specification. This tool enables granular control over user rights across document viewing platforms.

## Overview

The `pdf_change_permissions` tool utilizes `lopdf`'s standard security handler (`EncryptionVersion::V2`) to recalculate keys and set the correct encryption dictionary `/P` bitmask to revoke or grant document permissions.

## Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `input` | `String` | Yes | The path or reference to the input PDF document. |
| `output` | `String` | Yes | The path or reference for the resulting PDF document. |
| `allow_printing` | `Boolean` | No | Grant or revoke printing permissions. |
| `allow_copying` | `Boolean` | No | Grant or revoke copying (text extraction) permissions. |
| `allow_modifying` | `Boolean` | No | Grant or revoke document modifying permissions. |
| `allow_annotations` | `Boolean` | No | Grant or revoke the ability to add or modify annotations. |
| `allow_form_filling` | `Boolean` | No | Grant or revoke the ability to fill forms. |
| `allow_accessibility` | `Boolean` | No | Grant or revoke text extraction for accessibility purposes. |
| `owner_password` | `String` | No | Configure owner password. |
| `user_password` | `String` | No | Configure user password. |

## CLI Invocation

```bash
paperpilot pdf-change-permissions \
    --input document.pdf \
    --output restricted.pdf \
    --allow-printing false \
    --allow-copying false
```

## MCP JSON-RPC Invocation

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "execute_tool",
  "params": {
    "tool": "pdf_change_permissions",
    "arguments": {
      "input": "document.pdf",
      "output": "restricted.pdf",
      "allow_printing": false,
      "allow_copying": false
    }
  }
}
```

## REST API Invocation

```bash
curl -X POST http://localhost:7823/api/v1/pdf/change-permissions \
  -H "Content-Type: application/json" \
  -d '{
    "input": "document.pdf",
    "output": "restricted.pdf",
    "allow_printing": false,
    "allow_copying": false
  }'
```
