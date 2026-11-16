# Task 051A: Multi-Tier AI Agent Supervisor & Dynamic LLM Router Integration

## 1. Overview
Connect the configured AI provider mode (`OfflineNlp`, `Llamafile`, `Universal`, `Byok`) and our on-device fine-tuned model (`PaperPilot-Qwen-0.5B-Instruct` / `paperpilot-qwen-0.5b-q4_k_m.gguf`) into the Desktop natural language resolution pipeline (`resolve_natural_language`), replacing the hardcoded offline-only resolver with `DynamicRouter`, `LlmNlpResolver`, and automatic offline rule-engine fallback.

---

## 2. Implementation Checklist

### A. Desktop Backend (`apps/desktop/src-tauri/`)
- [ ] In `apps/desktop/src-tauri/src/ai_commands.rs`:
  - Export a helper function `resolve_dynamic_query(query: &str, context: &ResolverContext) -> Result<serde_json::Value, String>` that:
    - Reads active mode from `GLOBAL_AI_MODE`.
    - If `OfflineNlp`: calls `OfflineNlpResolver::new().resolve_with_context(query, context)`.
    - If `Llamafile` (Local SLM / GGUF):
      - Checks if local model exists (e.g. `paperpilot-qwen-0.5b-q4_k_m.gguf` in `~/.local/share/paperpilot/models/`).
      - Spawns / ensures `LlamafileSupervisor` or local llama server is running on port 8080.
      - Dispatches through `DynamicRouter::new(RouterMode::LlmWithFallback(llm_resolver))`.
    - If `Universal` or `Byok`:
      - Resolves endpoint URL and API key from OS keyring.
      - Dispatches through `DynamicRouter` with fallback to `OfflineNlpResolver`.
    - Adds metadata to response JSON: `{ ...plan, resolved_by: "llm" | "offline", active_mode: "..." }`.
  - Expose Tauri command `get_ai_execution_status() -> Result<AiExecutionStatus, String>`.
- [ ] In `apps/desktop/src-tauri/src/lib.rs`:
  - Update `resolve_natural_language` to call `ai_commands::resolve_dynamic_query(query, context)`.
  - Register `get_ai_execution_status` in Tauri invoke handler.
  - Hook `LlamafileSupervisor` cleanup into Tauri exit handler (`on_window_event`).

### B. Desktop Chat UI (`apps/desktop/src/lib/components/`)
- [ ] In `apps/desktop/src/lib/components/PdfChatPanel.svelte`:
  - Query `get_ai_config` / `get_ai_execution_status` on mount to display the active AI status pill in the chat header or above the input box:
    - `🔒 Offline Rules` (green)
    - `⚡ PaperPilot Qwen 0.5B` (purple)
    - `🌐 Universal LLM` (blue)
    - `🔑 Cloud BYOK` (orange)
  - Clicking the pill opens `AiSettingsModal.svelte`.
  - If a query resolution fell back from LLM to offline, display a subtle badge on the assistant response: *"Resolved via Offline Rules"*.

### C. Testing & Verification
- [ ] Add unit test in `apps/desktop/src-tauri/src/lib.rs` verifying that `resolve_natural_language` executes properly across modes.
- [ ] Verify clean compilation: `cargo check -p desktop` & `pnpm --dir apps/desktop check`.
- [ ] Verify test suite: `cargo test -p paperpilot-ai` & `pnpm --dir apps/desktop test`.
