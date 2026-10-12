# Spec 051 — Multi-Tier AI Agent Supervisor & Dynamic LLM Router Integration

## 1. Overview & Problem Statement
PaperPilot has completed two foundational AI pillars:
1. **Offline Embedded NLP (`paperpilot-nlp`)**: Rule-based regex and `TinyBERT` intent classification and slot-filling running in <0.4ms.
2. **AI Provider Config & Subprocess Infrastructure (`paperpilot-ai` & `ai_commands.rs`)**:
   - `LlamafileSupervisor`: Background subprocess supervisor managing standalone local `.llamafile` LLM binaries on port 8080.
   - `UniversalLlmClient`: Universal OpenAI-compatible HTTP client connecting to local Ollama, LM Studio, vLLM, or BYOK endpoints.
   - `AiSettingsModal.svelte` & `aiConfig.svelte.ts`: 4-way mode selection (`offlinenlp`, `llamafile`, `universal`, `byok`).

### The Missing Link:
Currently in `apps/desktop/src-tauri/src/lib.rs`, `resolve_natural_language` **strictly instantiates `OfflineNlpResolver::new()`**, completely bypassing user settings!
When a user selects "Local Llamafile", "Universal Endpoint", or "BYOK Cloud" in Settings, the natural language prompt in `PdfChatPanel.svelte` still only runs through the hardcoded offline rule engine.
Furthermore, the `DynamicRouter` in `paperpilot-ai` exists but is not wired into the Tauri state or IPC commands.

---

## 2. Technical Requirements

### 2.1 Dynamic AI Router Wiring in Tauri (`apps/desktop/src-tauri`)
- Connect `resolve_natural_language` to active `AiSettingsConfig`:
  - Read active mode from `GLOBAL_AI_MODE`, endpoint from `GLOBAL_UNIVERSAL_ENDPOINT`, and API key from OS Keyring.
  - If mode is `OfflineNlp`: use `OfflineNlpResolver`.
  - If mode is `Llamafile`:
    - Check if `LlamafileSupervisor` is active or if port 8080 is reachable. If not running and a downloaded model exists, automatically boot the supervisor or connect.
    - Dispatch query to `LlmNlpResolver` with fallback to `OfflineNlpResolver`.
  - If mode is `Universal` or `Byok`:
    - Dispatch query to `LlmNlpResolver` with endpoint and credentials.
    - Fallback gracefully to `OfflineNlpResolver` if network or model fails, accompanied by a status toast indicator.

### 2.2 IPC Command Enhancements
1. Add `get_ai_execution_status` Tauri command:
   - Returns current active provider mode, model name, whether the local runner is currently active/streaming, and response latency.
2. Streamed / Step-by-Step Tool Execution:
   - Provide multi-step plan support for queries that contain compound instructions (e.g., "Rotate page 1 by 90 degrees and then compress").

### 2.3 UI Indicators in `PdfChatPanel.svelte`
1. **Mode Pill**: Display an interactive badge in the chat header showing the current resolver:
   - `🔒 Offline TinyBERT` (green)
   - `⚡ Local Llamafile` (purple)
   - `🌐 Universal LLM` (blue)
   - `🔑 Cloud BYOK` (orange)
2. **One-Click Settings Shortcut**: Clicking the mode pill opens `AiSettingsModal.svelte` directly from the chat drawer.
3. **Fallback Banner**: If an LLM call times out or errors, show a non-intrusive badge: *"Fell back to Offline Rule Engine"*.

---

## 3. Architecture & Constraints
- **Concurrency & Resource Limits**: Keep memory under 35MB overhead for PaperPilot itself.
- **Port Conflict & Auto-Kill**: `LlamafileSupervisor` must shut down cleanly when Tauri exits.
- **Testing**:
  - Unit tests in `paperpilot-ai` and `apps/desktop/src-tauri` asserting dynamic mode resolution.
  - Mock server tests for LLM fallback behavior.
