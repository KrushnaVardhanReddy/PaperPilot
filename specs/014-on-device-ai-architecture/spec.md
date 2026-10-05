# 014: On-Device AI Architecture (TinyBERT & Local RAG)

## Status
Approved

## Context
PaperPilot requires a responsive, 100% offline natural language interface that parses user instructions (e.g., "squish this pdf", "take first 5 pages of contract", "convert to docx") into structured `OperationPlan` executions. We required an architecture that avoids Python dependencies, keeps the total installer size under 50MB, executes in <2ms on CPU, and enables users to query living documentation directly in the chat.

## Decisions

### 1. Two-Tier AI Strategy
* **Free Personal Tier (Bundled)**: `TinyBERT-4L-312D` (INT8 ONNX).
  - Size on disk: ~14MB (compressed to ~7MB via `zstd`).
  - Active memory: ~25MB RAM.
  - Latency: ~1.5ms to 2ms on standard CPU.
  - Capabilities: Single-intent classification and token slot-filling for arguments across arbitrary phrasing.
  - Deployment: Embedded directly in the Rust binary via `include_bytes!` and executed in-memory via `ort` (ONNX Runtime).
* **Pro / Teams Tier**: `SmolLM-135M-Instruct` (Q4 GGUF, ~75MB) + Bring-Your-Own-Key (BYOK) cloud APIs (OpenAI, Gemini, Claude) + Local Ollama.
  - Capabilities: Multi-step conversational agent planning and chained execution recipes.

### 2. Embedded Documentation RAG (`sqlite-vec`)
Rather than fine-tuning models on rapidly changing documentation (which causes hallucinations), PaperPilot embeds the living documentation (`TRI_INTERFACE_E2E_AND_DOCS.md`, user guides) into an embedded SQLite database using `sqlite-vec`.
* Questions in the desktop chat query the embedded vector index in <0.5ms.
* Exact, copy-pasteable CLI and REST API snippets are returned directly with 100% factual accuracy.

### 3. Anti-Brute-Force & Offline Memory-Burn
For encrypted PDFs, brute-force protection is enforced without server phone-home dependencies using **Argon2id Memory-Hard Key Derivation** (1–2GB RAM cost per attempt). Optional enterprise KMS key escrow allows remote burn-switches when connected.

## Consequences
* Total application size remains ~50MB, well under the 100MB constraint.
* Zero external Python, server, or cloud dependencies for AI operation.
* Instant sub-2ms response times on any desktop hardware.
