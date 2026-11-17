# Specification: Glama MCP Quality Grade Improvement (C -> A)

## Context & Objectives
Glama's automated evaluation system scored PaperPilot MCP:
- **License Grade**: `A`
- **Maintenance Grade**: `B`
- **Quality Grade**: `C` (Coherence: 2.8, Description Quality: 2.7, Disambiguation: 2/5, Completeness: 4/5)

### Glama Evaluation Findings
1. **Tool Count & Redundant Aliases**: 95 tools is excessive because of near-duplicate aliases (`pdf_convert_html`/`html_to_pdf`, `pdf_convert_markdown`/`md_to_pdf`, `pdf_images_to_pdf`/`img_to_pdf`, `batch_fill`/`cli_fill_batch`).
2. **Disambiguation (2/5)**: LLM agent misselection occurs when multiple tools appear to do the same task. Text extraction and search blur across `pdf_extract_text`, `pdf_to_markdown`, `pdf_to_json`, `pdf_chunk`, `pdf_rag_chunks`, `semantic_search`, `workspace_search`, and `pdf_search`.
3. **Naming Inconsistencies**: Mixing `pdf_`, `office_`, `excel_`, `cli_`, `watcher_`, `execution_receipt_`, and bare names like `semantic_search`. Uneven noun vs. verb styles (`pdf_info`, `pdf_metadata`).
4. **Description Quality (2.7)**: Descriptions lack consistent disambiguation cues ("Use this tool when...", "Do NOT use this for...").

## Goals
1. Retire or suppress duplicate legacy aliases from `tools/list` while keeping backwards-compatible execution in `tools/call`.
2. Group or namespace tools consistently (`pdf_` canonical prefix or clear domain groupings).
3. Enhance tool descriptions across the MCP registry with structured disambiguation guidance (When to use, Inputs, Outputs, Related tools).
4. Clearly distinguish search and extraction surfaces (`pdf_search` vs `semantic_search` vs `workspace_search`; `pdf_extract_text` vs `pdf_chunk` vs `pdf_rag_chunks`).
5. Verify with automated tests that all tools still work, compile without errors, and pass CI.
