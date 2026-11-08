# Task 158A: Implement `pdf_extract_patterns` Across Penta-Interface

## Checklist
- [ ] Scaffold tool structures across all 5 layers using `penta-scaffold`
- [ ] Implement core Rust pattern & multi-keyword matching in `paperpilot-pdf/src/operations/pdf_extract_patterns.rs`
- [ ] Wire CLI subcommand `pdf-extract-patterns` in `paperpilot-cli`
- [ ] Wire MCP tool `pdf_extract_patterns` in `paperpilot-mcp`
- [ ] Wire REST Gateway route in `paperpilot-gateway`
- [ ] Register tool in Desktop UI `apps/desktop/src/lib/types/toolList.ts` and `apps/web`
- [ ] Add UI controls in `DynamicActionControls.svelte` for keyword chips and regex list
- [ ] Run test suites across `paperpilot-pdf`, `paperpilot-cli`, `paperpilot-mcp`, and desktop
