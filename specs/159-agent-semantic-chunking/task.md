# Task 159A: Implement Semantic Chunking (`pdf_chunk`) Across Penta-Interface

## Checklist
- [ ] Scaffold tool definition using `penta-scaffold` or manual penta interfaces
- [ ] Implement token estimation and structural chunking algorithm in `crates/paperpilot-pdf/src/operations/pdf_chunk.rs`
- [ ] Integrate boilerplate suppression for headers/footers in chunk text extraction
- [ ] Wire CLI subcommand `pdf-chunk` in `crates/paperpilot-cli/src/main.rs` & command registry
- [ ] Wire MCP server tool handler `pdf_chunk` in `crates/paperpilot-mcp/src/tools.rs`
- [ ] Wire REST Gateway endpoint in `crates/paperpilot-gateway/src/routes/`
- [ ] Register tool in `apps/desktop/src/lib/types/toolList.ts` with icon 🧩, input `PDF`, output `.JSON`
- [ ] Add visual sliders (Target Tokens, Overlap) in `OperationsPanel.svelte`
- [ ] Add comprehensive unit tests in `crates/paperpilot-pdf/tests/test_pdf_chunk.rs`
- [ ] Verify zero regressions with `cargo check --workspace -j 4`
