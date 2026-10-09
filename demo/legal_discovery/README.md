# Local AI Agent Legal Due Diligence & Discovery Demo

## Overview

This standalone demo showcases PaperPilot's power as the foundational **PDF Operating System for AI Agents**. It demonstrates an autonomous, chained processing pipeline handling a realistic **Legal Discovery & Due Diligence** mandate entirely offline and 100% locally on-device.

### The Problem

Modern LLM-based autonomous agents (e.g., using frameworks like LangChain, AutoGen, or Anthropic MCP) struggle immensely with raw PDF manipulations. Traditional AI agents hallucinate boundaries, fail to parse complex visual structures, and rely on slow, expensive, unsecure cloud APIs to modify documents.

In a high-stakes Legal Discovery environment, sending highly sensitive PII (Social Security Numbers, banking details, unredacted financials) to a third-party cloud API for processing is a severe compliance violation. Agents need local, fast, and deterministic tools.

### The Solution: PaperPilot

PaperPilot exposes determinist, zero-cloud PDF tools directly to LLMs via the Model Context Protocol (MCP). The AI agent orchestrates the logic, while PaperPilot executes the heavy-lifting manipulations locally on the host machine in sub-seconds.

## Execution Pipeline

In this demo, the AI Agent must execute the following mandate on a set of 3 heterogeneous documents (totaling 9 pages):
1. Correct the orientation of a skewed scanned page (`pdf_rotate`).
2. Redact sensitive PII text and underlying streams from a contract (`pdf_redact`).
3. Apply standard sequential courtroom Bates stamping identifiers (`pdf_bates`).
4. Assemble all processed exhibits into a single master courtroom bundle (`pdf_merge`).
5. Stamp a legal privilege warning / protective order watermark (`pdf_watermark`).
6. Web-optimize the master dossier for fast, sequential browser streaming preview (`pdf_linearize`).
7. Generate a tamper-evident SHA-256 audit digest (`pdf_hash`).

## How AI Agents Use PaperPilot MCP

This demo uses the CLI to emulate the exact sequence of commands an AI agent would issue through PaperPilot's Model Context Protocol (MCP) server.

When an MCP client (like Claude Desktop) connects to the `paperpilot-mcp` server, it receives the schemas for these tools.

For instance, an agent reasoning about step 2 (redaction) emits a JSON-RPC request to the MCP server:
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_redact",
    "arguments": {
      "input_path": "demo/legal_discovery/input/01_asset_purchase_agreement.pdf",
      "output_path": "demo/legal_discovery/tmp/01_redacted.pdf",
      "page": 1,
      "rect": "100,500,300,520"
    }
  },
  "id": 1
}
```

The `paperpilot-mcp` server (powered by `paperpilot-core`) executes the redaction instantly using pure Rust code and returns a success response to the agent, which then proceeds to the next step.

## How to Run the Demo

### Mode A: 1-Click Automated Script (Terminal / CI / Benchmark)
Run the demo using the Makefile at the root of the PaperPilot workspace:

```bash
make demo-legal-discovery
```

This will:
1. Compile the demo fixture generator (`demo-legal-fixtures`) and PaperPilot CLI (`paperpilot`).
2. Programmatically generate the 3 mock input legal PDFs in `demo/legal_discovery/input/`.
3. Execute the 7-step autonomous toolchain sequentially in **~94 milliseconds**.
4. Output the finalized court bundle to `demo/legal_discovery/output/final_court_bundle.pdf`.
5. Display the performance scorecard and tamper-evident SHA-256 audit digest.

---

### Mode B: Zero-Install Local AI Agent via Mozilla `llamafile` (Recommended 100% Offline)
Run an open-weights model locally with zero installation, zero Docker, and zero cloud API keys:

1. Download any tool-calling `llamafile` (e.g. `Llama-3.2-3B-Instruct.llamafile` or `Qwen2.5-Coder-7B.llamafile` from [Mozilla AI](https://github.com/Mozilla-Ocho/llamafile)).
2. Make it executable and start its local OpenAI-compatible server:
   ```bash
   chmod +x ./model.llamafile
   ./model.llamafile --server --jinja --port 8080
   ```
3. Run the PaperPilot agent against the local llamafile endpoint:
   ```bash
   paperpilot agent --endpoint http://127.0.0.1:8080/v1 --prompt "Prepare discovery files in demo/legal_discovery/input for court production"
   ```
   *The entire LLM inference and all PDF mutations execute in local memory with zero external internet access.*

---

### Mode C: Local Developer Runtime via `ollama`
If you already use Ollama:
```bash
ollama run llama3.2:3b
paperpilot agent --endpoint http://127.0.0.1:11434/v1 --prompt "Prepare discovery files in demo/legal_discovery/input"
```

---

### Mode D: Interactive Frontier Agent via Claude Desktop (MCP)
To run with Claude 3.5 Sonnet:
1. Add `paperpilot-mcp` to your `claude_desktop_config.json`:
   ```json
   {
     "mcpServers": {
       "paperpilot": {
         "command": "paperpilot-mcp"
       }
     }
   }
   ```
2. Prompt Claude:
   > *"Inspect the discovery PDFs in `./demo/legal_discovery/input/`. Correct any tilted page rotation, permanently redact sensitive SSNs, sequentially Bates-stamp the documents starting at 1, merge them into a master dossier, apply a CONFIDENTIAL protective order watermark, and compute the SHA-256 audit digest using PaperPilot tools."*
3. Claude coordinates the reasoning, while PaperPilot executes all heavy byte-level operations 100% locally on your machine.
