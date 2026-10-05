# Context-Aware NLP Resolution Report

## Overview
This report documents the enhancement of the `paperpilot-nlp` resolution bridge. The goal of this task was to ensure that user queries without explicitly provided input files implicitly bind to the currently active document open in the viewer, preventing failure messages like `"requires at least one PDF file path"`.

This was accomplished by introducing a `ResolverContext` structure into the Natural Language API boundary and making the resolver state-aware without breaking headless compatibility.

## Implementation Details

### 1. NLP Traits and Context (`paperpilot-nlp/src/traits.rs`)
- Added a new structured data type `ResolverContext` deriving `Serialize` and `Deserialize` to bridge the frontend state to the Rust backend:
  ```rust
  pub struct ResolverContext {
      pub active_document: Option<String>,
      pub open_documents: Vec<String>,
  }
  ```
- Enhanced the core trait `NlpResolver` by adding the `resolve_with_context` function, leaving `resolve` to act as a backwards-compatible wrapper initializing a `Default` context.

### 2. Offline Resolver (`paperpilot-nlp/src/resolver.rs`)
- Updated the resolution logic (both RuleEngine Layer 1 and the ONNX model Layer 2) to dynamically check if the extracted entity files list (`entities.files`) is empty.
- If it is empty, the resolver attempts to bind the `active_document` if provided, or defaults to the single open document if only one exists in the `open_documents` array.
- This ensures operations like `"split pages 1 to 2"` properly append the local viewer's currently focused file to the generated operation plan.

### 3. Tauri IPC Command (`apps/desktop/src-tauri/src/lib.rs`)
- Modified the Tauri command `resolve_natural_language` to parse an optional `context` parameter from the UI thread.
- If `context` is provided via Svelte bindings, it calls `resolver.resolve_with_context(&query, &ctx)`.

### 4. Frontend Component (`apps/desktop/src/lib/components/PdfChatPanel.svelte`)
- Bound `appState.svelte.ts` into `PdfChatPanel.svelte` using `$lib/state/app.svelte`.
- Refactored `handleSend` to evaluate `appState.selectedDocumentIndex` mapping the internal active document reference (accounting for transient object keys like `_localPath` and `name`) and dynamically creating the `context` JSON payload to append to the Tauri invocation.

## Verification
- Unit and integration tests were extended within `traits.rs`, `resolver.rs`, and the `src-tauri/src/lib.rs` environments.
- Tests validated explicit instantiation of `ResolverContext`.
- Context binding was verified with tests evaluating situations where an `active_document` was present as well as fallback behavior handling single-document queues.
- `cargo test --workspace` validated the integrity of the updated API interface.
- Frontend builds and type-checking verified safe JSON property extraction from standard File/Document definitions across the Svelte application state.
