---
title: "cli_ask"
description: "Reference, CLI, MCP, and REST API commands for cli_ask natural language PDF intent execution."
sidebar:
  order: 15
---

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data $\to$ Executable Action / Plan / File

### What it does
**Executes PDF operations using natural language prompts.**  
Interprets plain English instructions (such as `"compress aggressively"`, `"rotate 90 degrees"`, or `"extract text"`) and resolves them into deterministic execution plans and operations using a fast, privacy-first offline NLP resolver.

- **Offline Intent Resolution**: Operates entirely locally with zero cloud dependencies or external API calls.
- **Deterministic Action Routing**: Resolves ambiguous conversational prompts into precise structured tool invocations (e.g., `pdf_compress`, `pdf_rotate`, `pdf_extract_text`).
- **Interactive or Direct Automation**: Supports interactive confirmation, `--yes` / `-y` flag for automated headless pipelines, and `--dry-run` to output execution plans without modifying documents.

---

### CLI

#### Non-Interactive Execution (Direct Run)
```bash
# Execute natural language compression directly
paperpilot ask "compress this document aggressively" input.pdf -y -o output.pdf --json
```

**Output**:
```json
{"success":true,"operation":"Ask","error":null}
```

#### Dry-Run Execution Plan
Inspect the parsed intent and planned execution steps without mutating or generating output files:
```bash
paperpilot ask "rotate 90 degrees" input.pdf --dry-run --json
```

**Output**:
```json
{
  "success": true,
  "operation": "Ask",
  "error": null,
  "plan": {
    "intent": "rotate",
    "parameters": {
      "angle": 90
    },
    "input_files": ["input.pdf"],
    "output_file": null
  }
}
```

#### CLI Parameters
| Flag | Short | Description |
| --- | --- | --- |
| `<PROMPT>` | — | Natural language instruction (e.g. `"compress document"`, `"rotate 90 degrees"`) |
| `[INPUT]` | — | Target input PDF document |
| `--yes` | `-y` | Skip interactive user confirmation and execute immediately |
| `--output` | `-o` | Explicit path for the resulting output PDF or artifact |
| `--dry-run` | — | Parse and output execution plan without executing operations |
| `--offline` | — | Force offline NLP resolution only |
| `--json` | — | Output structured JSON result |

---

### MCP JSON-RPC

Call tool `cli_ask` or its alias `ask`:

```json
{
  "name": "cli_ask",
  "arguments": {
    "prompt": "compress this document aggressively",
    "input": "input.pdf",
    "output": "compressed.pdf"
  }
}
```

**MCP Response**:
```json
{
  "success": true,
  "message": "Interpreted Plan for intent: Compress",
  "output_path": "compressed.pdf",
  "diff_detected": null,
  "data": {
    "intent": "Compress",
    "confidence": 1.0,
    "input_files": ["input.pdf"],
    "output_file": "compressed.pdf",
    "parameters": {
      "level": "aggressive"
    }
  }
}
```

---

### REST API

#### Endpoint
`POST /api/v1/pdf/tools/cli_ask`

#### Request
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/cli_ask \
  -H 'Content-Type: application/json' \
  -d '{
    "prompt": "rotate 90 degrees",
    "input": "document.pdf",
    "output": "rotated_plan.txt",
    "dry_run": true
  }'
```

#### Response
```json
{
  "success": true,
  "message": "Interpreted Plan for intent: Rotate",
  "output_path": "rotated_plan.txt",
  "data": {
    "intent": "Rotate",
    "confidence": 1.0,
    "input_files": ["document.pdf"],
    "output_file": "rotated_plan.txt",
    "parameters": {
      "angle": "90"
    }
  }
}
```

---

### Known Limitations & Troubleshooting

- **Complex Multi-Step Intent**: While single-step actions (rotate, compress, extract text, split) resolve with high confidence, compound multi-stage intents (e.g., "watermark then compress then encrypt") should be modeled as declarative recipes using [`document_workflow`](/reference/tools/document_workflow/) or [`recipe_run`](/reference/tools/recipe_run/).
- **Unambiguous Target Inputs**: When running via CLI without positional `[INPUT]`, the prompt must clearly identify the input file, or the tool will return an invalid parameter error.
