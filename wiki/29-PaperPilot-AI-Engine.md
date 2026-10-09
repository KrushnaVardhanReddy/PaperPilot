# PaperPilot AI Engine (`paperpilot-ai`)

The `paperpilot-ai` crate serves as the bridge between the PaperPilot PDF operating system and external LLMs, enabling autonomous, multi-step document operations driven by natural language.

## Architecture

PaperPilot provides an implementation of `paperpilot_nlp::traits::NlpResolver` that calls an OpenAI-compatible endpoint.

### `UniversalLlmClient`
A standalone, async HTTP client built on `reqwest` that connects to any endpoint implementing the OpenAI `/v1/chat/completions` protocol.
- Supports `endpoint_url` customization (e.g. `http://localhost:11434/v1` for Ollama, `http://127.0.0.1:8080/v1` for Llamafile, or `https://api.openai.com/v1`).
- Supports tool-calling (`tools` array, JSON schema mapping).

### `LlmNlpResolver`
Translates user queries into tool calls using the `UniversalLlmClient`.
- Injects available PaperPilot tools as `ToolDefinition` schemas.
- Translates the LLM's `ToolCall` response directly into a strictly-typed `OperationPlan` for the underlying PDF engine.
- Bypasses traditional heuristic intent-matching for more complex, deterministic LLM-driven execution.

### `DynamicRouter`
A runtime switcher to allow users to toggle between `OfflineNlpResolver` (100% offline embedded model using ONNX/RTEN) and `LlmNlpResolver` (BYOK or Local LLM).
- Graceful Fallbacks: Automatically defaults to the offline resolver if the user's Local LLM endpoint is unreachable.

## Usage

```rust
use paperpilot_ai::client::{LlmConfig, UniversalLlmClient};
use paperpilot_ai::resolver::LlmNlpResolver;
use paperpilot_nlp::traits::{NlpResolver, ResolverContext};

let config = LlmConfig {
    endpoint_url: "http://localhost:11434/v1".to_string(),
    model: "llama-3.2-3b-instruct".to_string(),
    api_key: None,
    temperature: Some(0.1),
};

let client = UniversalLlmClient::new(config);
let resolver = LlmNlpResolver::new(client);

let plan = resolver.resolve_with_context(
    "merge file1.pdf and file2.pdf into result.pdf",
    &ResolverContext::default()
).unwrap();

// plan.intent == Intent::Merge
// plan.input_files == ["file1.pdf", "file2.pdf"]
// plan.output_file == "result.pdf"
```
