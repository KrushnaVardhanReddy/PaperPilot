# Task 158A: Implement `pdf_extract_patterns` Across Penta-Interface

## Checklist
- [x] Scaffold tool structures across all 5 layers using `penta-scaffold`
- [x] Implement core Rust pattern & multi-keyword matching in `paperpilot-pdf/src/operations/pdf_extract_patterns.rs` (guaranteed extraction of ALL matches across all pages)
- [x] Wire CLI subcommand `pdf-extract-patterns` in `paperpilot-cli`
- [x] Wire MCP tool `pdf_extract_patterns` in `paperpilot-mcp`
- [x] Wire REST Gateway route in `paperpilot-gateway`
- [x] Register tool in Desktop UI `apps/desktop/src/lib/types/toolList.ts` and `apps/web`
- [x] Add UI parameter controls in `OperationsPanel.svelte` (keywords input, custom regex mapping, case sensitivity, context length slider)
- [x] Run test suites across `paperpilot-pdf`, `paperpilot-cli`, `paperpilot-mcp`, `paperpilot-gateway`, and `paperpilot-wasm` with `-j 4`

