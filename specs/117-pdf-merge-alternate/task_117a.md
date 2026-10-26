# Task 117A: Implement `pdf_merge_alternate`

## Objective
Implement `pdf_merge_alternate` across Core, CLI, MCP, and Gateway with 100% real byte testing.

## Implementation Details
1. **Core**: `paperpilot-pdf/src/operations/pdf_merge_alternate.rs`
   - Implement `merge_alternate_bytes(a_bytes: &[u8], b_bytes: &[u8], options: &PdfMergeAlternateOptions) -> Result<Vec<u8>, OperationError>`.
   - Support `reverse_b`, `reverse_a`, `step`, `step_b`.
   - Write comprehensive unit tests with real fixtures.
2. **CLI**: `paperpilot-cli/src/commands/pdf_merge_alternate.rs`
   - Command `merge-alternate`.
3. **MCP**: `paperpilot-mcp/src/server.rs`
   - Tool `pdf_merge_alternate`.
4. **Gateway**: `paperpilot-gateway/src/handlers.rs`
   - Route `POST /api/v1/pdf/merge-alternate`.
5. **Documentation**:
   - `wiki/117-Pdf-Merge-Alternate.md` and docs reference.
