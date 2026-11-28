# PaperPilot Python Examples

> *"Let the LLM be the brain. Let PaperPilot handle the documents."*

## Installation

```bash
pip install usepaperpilot
```

## Real-World LLM + PaperPilot Use Cases

| Use Case | PaperPilot Tools | Why It Matters |
| --- | --- | --- |
| **Zero-Token RAG Ingestion** | `to_markdown_bytes`, `chunk_rag_bytes` | Parse PDFs into clean Markdown and semantic chunks locally in Rust at 0 token cost before embedding or prompting. |
| **Privacy-Safe LLM Intake** | `redact` | Strip PII (names, emails, SSNs) locally before any document text leaves your machine for OpenAI, Claude, or external APIs. |
| **Agent Pre-Flight & Cost Control** | `info_bytes`, `split`, `compress_bytes` | Inspect page counts, slice relevant page ranges, and compress oversized PDFs before an AI agent touches them. |
| **Self-Healing Pipeline** | `repair` | Auto-fix malformed or corrupted user-uploaded PDFs before extraction so your ingestion pipeline never crashes. |

## Examples

- [`llm_rag_pipeline.py`](./llm_rag_pipeline.py) — Standalone end-to-end script demonstrating Zero-Token RAG Ingestion, Privacy-Safe LLM Intake, Agent Pre-Flight & Cost Control, and LLM client integration (OpenAI-compatible, Ollama, Anthropic).

## Running

```bash
python3 paperpilot-py/examples/llm_rag_pipeline.py
```
