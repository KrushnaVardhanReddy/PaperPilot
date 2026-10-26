# Task 118A: Implement `pdf_split_by_outline`

## Objective
Implement outline-based chapter splitting in `paperpilot-pdf` and expose across CLI, MCP, and Gateway.

## Implementation Details
1. **Core**: `paperpilot-pdf/src/operations/pdf_split_by_outline.rs`
   - Outline bookmark traversal via `lopdf::Document` `/Outlines`.
   - Resolve `/Dest` and `/GoTo` actions to 1-indexed page indices.
   - Slice chapters into separate PDF buffers with safe filenames.
   - Comprehensive unit tests using real PDF fixtures with bookmarks.
2. **CLI**: `paperpilot-cli/src/commands/pdf_split_by_outline.rs`
   - Command `split-by-outline`.
3. **MCP**: `paperpilot-mcp/src/server.rs`
   - Tool `pdf_split_by_outline`.
4. **Gateway**: `paperpilot-gateway/src/handlers.rs`
   - Route `POST /api/v1/pdf/split-by-outline`.
5. **Documentation**:
   - `wiki/118-Pdf-Split-By-Outline.md` and reference docs.
