# Phase 4.3.7 — Multi-Turn Chat Panel Component

## Objective Complete
Implemented the `PdfChatPanel.svelte` as a collapsible side-drawer next to `PdfViewer.svelte` and connected it via Tauri IPC to the offline NLP resolver.

## Details
1. **Tauri IPC Bridge**: Created the `resolve_natural_language` Tauri command in `apps/desktop/src-tauri/src/lib.rs` and exposed it to the frontend via `invoke`. Added `paperpilot-nlp` dependency to `apps/desktop/src-tauri/Cargo.toml`. Added `test_resolve_natural_language` unit test in `src-tauri/src/lib.rs`.
2. **Serialization Support**: Made `OperationPlan` and `Intent` in `paperpilot-nlp` serializable with `serde::Serialize` and `serde::Deserialize`.
3. **Multi-Turn Chat Panel**: Implemented `PdfChatPanel.svelte` with full multi-turn chat support, suggestions chips, loading state (thinking...), and an action execution card using `invoke_mcp_tool`. It properly maps intents like "Rotate", "Merge", and "Split" into appropriate `mcp` tooling payload and shows toast notifications.
4. **ViewerRightPanel integration**: Appended a 3rd tab: `💬 AI Chat` alongside `Annotations` and `Info` and updated `ViewerRightPanel.svelte` to conditionally show the `PdfChatPanel` component.
