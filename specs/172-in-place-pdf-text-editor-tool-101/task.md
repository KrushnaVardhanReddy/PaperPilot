# Task 172: Implement Tool 101 `pdf_edit_text` Across Penta-Interfaces

## Overview
Implement Spec 172 by providing an in-place PDF text search and replacement engine across all 5 PaperPilot interfaces (Rust Core, CLI, MCP Server, REST Gateway, and WebAssembly / Desktop UI).

---

## Subtasks & Checklist

- [x] **Subtask 1: Core Engine Implementation (`paperpilot-pdf`)**
  - [x] Create `paperpilot-pdf/src/operations/pdf_edit_text.rs` with token stream parsing and encoding resolution.
  - [x] Support `Tj`, `TJ`, and `'` string operator mutation.
  - [x] Support selective `page`, `occurrence`, and `match_case` options.
  - [x] Export `pdf_edit_text` in `paperpilot-pdf/src/operations/mod.rs`.

- [x] **Subtask 2: CLI Subcommand (`paperpilot-cli`)**
  - [x] Add `EditText` variant to `paperpilot-cli/src/cli.rs`.
  - [x] Implement CLI command execution in `paperpilot-cli/src/commands/pdf_edit_text.rs`.
  - [x] Wire `--find`, `--replace`, `--page`, `--occurrence`, `--match-case`.

- [x] **Subtask 3: REST Gateway & OpenAPI (`paperpilot-gateway`)**
  - [x] Register `/api/v1/pdf/edit-text` in `paperpilot-gateway/src/server.rs`.
  - [x] Add OpenAPI route definitions and request payload schemas in `paperpilot-gateway/docs/api/openapi.json`.
  - [x] Add integration tests in `paperpilot-gateway/tests/server_test.rs`.

- [x] **Subtask 4: MCP Protocol Parity (`paperpilot-mcp`)**
  - [x] Add `pdf_edit_text` to `tools/list` schema in `paperpilot-mcp/src/server.rs`.
  - [x] Implement execution handler in `tools/call`.

- [x] **Subtask 5: WebAssembly Engine & Web/Embed/Edge Integration**
  - [x] Expose `pdf_edit_text` in `paperpilot-wasm/src/operations.rs`.
  - [x] Integrate into `apps/web/src/views/OperationsView.svelte`.
  - [x] Integrate into `apps/embed/src/lib/Widget.svelte` and `EmbedGenerator.svelte`.
  - [x] Integrate into `apps/edge/src/index.ts`.

- [x] **Subtask 6: Verification & Test Suite**
  - [x] Add WebAssembly tests in `paperpilot-wasm/tests/wasm_tests.rs`.
  - [x] Add penta-interface E2E case in `tools/penta-interface-e2e/src/cases/pdf_edit_text.rs`.
  - [x] Add desktop automated tests in `apps/desktop/test/specs/pdf_edit_text.e2e.ts`.
