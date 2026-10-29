---
title: "cli_ask"
description: "Reference, CLI, MCP, and REST API commands for cli_ask."
sidebar:
  order: 15
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

### What it does
**Executes PDF operations using natural language prompts.**  
Interprets plain English instructions (e.g., "compress aggressively", "extract pages 2 to 5") and routes them directly to local deterministic tool engines.

### CLI
```bash
paperpilot ask "compress this document aggressively" input.pdf --yes -o small.pdf
```

### MCP JSON-RPC
```json
{
  "name": "cli_ask",
  "arguments": {
    "prompt": "compress this document aggressively",
    "input": "input.pdf",
    "output": "small.pdf"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/ai/ask -H 'Content-Type: application/json' -d '{"prompt": "compress this document aggressively", "input": "input.pdf", "output": "small.pdf"}'
```
