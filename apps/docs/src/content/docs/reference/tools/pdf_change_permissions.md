---
title: "pdf_change_permissions"
description: "Reference, CLI, MCP, and REST API commands for pdf_change_permissions."
sidebar:
  order: 54
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Configures user and owner permissions for printing, copying, and modification.**  
Enables granular control over user rights across PDF readers according to ISO 32000-1 specification encryption dictionaries.

### Parameters

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

### CLI
```bash
paperpilot pdf-change-permissions --input document.pdf --output restricted.pdf --allow-printing false --allow-copying false
```

### MCP JSON-RPC
```json
{
  "name": "pdf_change_permissions",
  "arguments": {
    "input": "document.pdf",
    "output": "restricted.pdf",
    "allow_printing": false,
    "allow_copying": false
  }
}
```

### REST API
```bash
curl -X POST http://localhost:7823/api/v1/pdf/change-permissions -H "Content-Type: application/json" -d '{"input": "document.pdf", "output": "restricted.pdf", "allow_printing": false, "allow_copying": false}'
```
