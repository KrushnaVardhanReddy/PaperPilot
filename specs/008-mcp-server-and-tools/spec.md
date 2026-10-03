# Spec: Phase 2 — MCP Server & Tools Infrastructure

## Objective
Provide a Model Context Protocol (MCP) server interface (`paperpilot-mcp`) exposing all core PDF manipulation capabilities to AI assistants, LLM agents (Claude Desktop, Cursor, Goose, Ollama), and local automated pipelines via standard JSON-RPC 2.0.

## Architecture Decision (Locked)
- **SDK**: Official Anthropic `rmcp` 3.5.0 (`rmcp = { version = "3.5.0", features = ["macros", "transport-io"] }`).
- **Transports**:
  - `stdio`: For CLI piping and local AI host integrations (Claude Desktop, Cursor).
  - `sse`: Hosted via `paperpilot-gateway` at `/mcp/sse` and `/mcp/messages`.
- **Headless CLI Compatibility**: The `stdio` transport intercepts raw `tools/call` JSON-RPC lines on standard input to execute tool commands directly without requiring an explicit client handshake.

## Tool Registry (44 Operations)
All tools implement `rmcp` handler methods returning JSON responses with `{ "success": bool, "output_path"?: string, "message"?: string }`.

### Categories:
1. **Page Operations**: `pdf_merge`, `pdf_split`, `pdf_extract_pages`, `pdf_delete_pages`, `pdf_reorder_pages`, `pdf_rotate`, `pdf_crop`, `pdf_burst`.
2. **Optimization & Security**: `pdf_compress`, `pdf_repair`, `pdf_linearize`, `pdf_encrypt`, `pdf_decrypt`, `pdf_watermark`, `pdf_redact`, `pdf_metadata`, `pdf_sign`, `pdf_flatten`.
3. **Extraction & Analysis**: `pdf_extract_text`, `pdf_extract_images`, `pdf_search`, `pdf_compare`, `pdf_bookmarks`, `pdf_render`, `pdf_ocr`, `pdf_validate`, `pdf_hash`.
4. **Creation & Conversions**: `pdf_markdown_to_pdf`, `pdf_html_to_pdf`, `pdf_images_to_pdf`, `pdf_to_docx`, `pdf_to_xlsx`, `pdf_to_markdown`, `pdf_to_pptx`.
5. **Forms & Bates**: `pdf_read_form`, `pdf_fill_form`, `pdf_create_form_field`, `pdf_bates`, `pdf_header_footer`, `pdf_page_numbers`, `pdf_remove_blank`.

## Verification & Testing
- Unit tests via `cargo test -p paperpilot-mcp`.
- Schema generation tests validating input/output argument shapes against JSON schema standards.
- E2E stdio verification asserting JSON-RPC responses via CLI pipes.
