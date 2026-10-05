# ADR 0002: On-Device AI Engine Selection & Embedded Local RAG

## Status
Accepted

## Context
PaperPilot requires an offline, privacy-first natural language interface for desktop and CLI workflows. Users must be able to issue single-step and chained operations in conversational English with immediate execution.
Constraints:
1. Total installer size must remain under 100MB (target ~50MB).
2. Latency on CPU must be sub-5ms for basic commands.
3. Zero Python or cloud runtime dependencies.
4. Users must be able to query documentation accurately in chat.

## Considered Options

1. **Option 1: Full-Size Local LLMs (Llama 3 8B, Mistral 7B)**:
   - *Pros*: Excellent reasoning.
   - *Cons*: Requires 4GB–8GB of RAM/disk and a GPU; unacceptable for bundled free desktop utility.
2. **Option 2: Pure Regex / Keyword Dictionaries**:
   - *Pros*: Zero megabytes.
   - *Cons*: Brittle; breaks on phrasing variations, synonyms, typos, and non-standard syntax.
3. **Option 3: Embedded ONNX TinyBERT (INT8) + SQLite Local RAG**:
   - *Pros*: ~14MB disk footprint (compresses to ~7MB with zstd), <2ms CPU execution, embeds directly into binary with `include_bytes!`. 
   - Uses `sqlite-vec` for document question-answering with zero hallucinations.

## Decision
We adopt **Option 3**:
* **Free Tier**: `TinyBERT-4L-312D` (INT8 ONNX, ~14MB, ~2ms CPU latency) embedded directly in the binary via `ort`.
* **Pro Tier**: `SmolLM-135M-Instruct` (~75MB Q4 GGUF) for multi-step agent plans, alongside BYOK APIs (OpenAI/Gemini/Claude) and Ollama.
* **Documentation Search**: SQLite with `sqlite-vec` vector embeddings.

## Consequences
- Fast, deterministic intent and parameter extraction without network access.
- Total app size is kept around ~50MB.
- Seamless upgrade path for users wanting deep multi-step conversational agent plans.
