# Penta-Scaffold Generator

Adding a new tool to PaperPilot requires updating 8 different layers across the 5 surfaces (Core, CLI, MCP, REST Gateway, WASM, Edge, UI). To avoid boilerplate, `penta-scaffold` automates this process.

## Usage

You can run the generator via the provided Makefile command:
```bash
make new-tool NAME=pdf_validate_signatures TITLE="Validate Digital Signatures" CATEGORY=security ICON="🔏" IN=pdf OUT=json DESC="..."
```

Or run the CLI directly:
```bash
cargo run -p penta-scaffold -- \
  --name pdf_validate_signatures \
  --title "Validate Digital Signatures" \
  --category "security" \
  --icon "🔏" \
  --input-format "pdf" \
  --output-format "json" \
  --description "Cryptographic verification of digital signatures and certificate chains."
```

Append `--dry-run` to preview the code blocks without creating/modifying any files.

## What gets generated?
1. **Core:** `paperpilot-pdf/src/operations/<name>.rs`
2. **CLI:** `paperpilot-cli/src/commands/<name>.rs` & injects into `cli.rs`
3. **MCP:** Injects tool schema and execution into `paperpilot-mcp/src/server.rs`
4. **Gateway:** Adds Axum handler in `paperpilot-gateway/src/handlers.rs` and routes it.
5. **WASM:** Exposes in `paperpilot-wasm/src/operations.rs` & `lib.rs`
6. **Edge:** Appends endpoint in `apps/edge/src/index.ts`
7. **Desktop:** Injects into `apps/desktop/src/lib/types/toolList.ts`
8. **E2E Tests:** Scaffolds assertions in `tools/penta-interface-e2e/src/cases/<name>.rs`
