# Tasks: Glama MCP Quality Grade Improvement (C -> A)

- [x] 1. Identify all duplicate aliases in MCP tools registration (`crates/paperpilot-mcp` or server registry) <!-- id: 1 -->
  - Identify redundant pairs: `html_to_pdf`/`pdf_convert_html`, `md_to_pdf`/`pdf_convert_markdown`, `img_to_pdf`/`pdf_images_to_pdf`, `cli_fill_batch`/`batch_fill`.
  - Suppress duplicate aliases from `tools/list` while retaining fallback execution in `tools/call`.

- [x] 2. Disambiguate search & extraction tool descriptions <!-- id: 2 -->
  - `pdf_extract_text`: Raw plain-text extraction from a PDF.
  - `pdf_to_markdown`: Document layout-preserving text & tables to markdown.
  - `pdf_to_json`: Structured AST/JSON representation of PDF document elements.
  - `pdf_chunk`: Simple text chunking by tokens/words.
  - `pdf_rag_chunks`: Semantic, structure-aware chunking optimized for RAG embeddings.
  - `pdf_search`: Regex/keyword pattern searching across PDF page contents.
  - `semantic_search`: Vector/embedding search over indexed document chunks.
  - `workspace_search`: Multi-file repository/workspace document discovery.

- [x] 3. Standardize naming and tool descriptions <!-- id: 3 -->
  - Ensure uniform description structure: Summary, When to use, Key parameters, Output format.
  - Add explicit "Do not use for..." notes where tools have subtle boundaries.

- [x] 4. Verify compilation, unit tests, and MCP runner compatibility <!-- id: 4 -->
  - Run `cargo test -p paperpilot-mcp`.
  - Validate output of `paperpilot-mcp --client-config codex` and list tools.
  - Verify zero runtime regressions.

- [x] 5. Run graphify-rs and commit changes <!-- id: 5 -->
  - Run `graphify-rs build`.
  - Commit to `feature/mcp-quality-improvements`.
