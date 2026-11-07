---
title: "recipe_run"
description: "Reference, CLI, MCP, and REST API commands for recipe_run."
sidebar:
  order: 1
---

**Status**: ✅ Verified  |  **Format**: JSON Recipe / Various

### What it does
**Execute a predefined sequence of operations.**
This tool runs a chain of PDF, Office, or automation operations as specified in a JSON recipe file.

### CLI
```bash
paperpilot recipe run --config my_recipe.json --input in/ --output out/
```

### MCP JSON-RPC
```json
{
  "name": "recipe_run",
  "arguments": {
    "config": "my_recipe.json",
    "input": "in/",
    "output": "out/"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/automation/recipes/run \
  -H 'Content-Type: application/json' \
  -d '{"config": "my_recipe.json", "input": "in/", "output": "out/"}'
```
