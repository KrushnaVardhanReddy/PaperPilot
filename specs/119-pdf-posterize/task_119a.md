# Task 119A: Implement Multi-Sheet Poster Tile Slicing Engine (`pdf_posterize`)

## Status
Ready for Implementation.

## Description
Implement the `pdf_posterize` operation across the entire PaperPilot stack according to `specs/119-pdf-posterize/spec.md`.

## Deliverables
1. **Core Engine**: `paperpilot-pdf/src/operations/pdf_posterize.rs`:
   - Scaffolding and types (`PdfPosterizeOptions`, `pdf_posterize_bytes`) are already generated.
   - Slices large source pages across a grid (e.g. 2x2, 3x3) with configurable glue gutters (`overlap_pt`) and optional trim marks.
   - Unit tests with real synthesized PDF fixtures in `paperpilot-pdf/src/operations/pdf_posterize.rs`.
2. **CLI**: `paperpilot-cli/src/commands/pdf_posterize.rs`:
   - Subcommand `posterize` with `--input`, `--output`, `--target-sheet`, `--grid`, `--overlap`, `--scale`, `--guides`.
3. **MCP Server**: `paperpilot-mcp/src/server.rs`:
   - Tool `pdf_posterize` with complete JSON schema validation.
4. **REST Gateway**: `paperpilot-gateway/src/handlers.rs` & `server.rs`:
   - Handler and schema struct `PdfPosterizeJsonRequest` at `/api/v1/pdf/posterize`.
5. **WASM & Edge**:
   - `paperpilot-wasm/src/operations.rs`: `posterize` method.
   - `apps/edge/src/index.ts`: routing and parameter parsing.
6. **Desktop & Documentation**:
   - `apps/desktop/src/lib/types/toolList.ts`: Add `pdf_posterize` tool definition.
   - `apps/desktop/src/lib/services/operationArgsBuilder.ts`: Add arguments mapping.
   - `wiki/119-Pdf-Posterize.md` & `apps/docs/src/content/docs/reference/tools/pdf_posterize.md`.
