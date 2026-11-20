# Specification: Glama MCP TDQS Grade A+ Enhancement

## Context & Objectives
Glama.ai assesses Model Context Protocol (MCP) servers using the **Tool Definition Quality Score (TDQS)** framework across 6 key dimensions:
1. **Purpose Clarity**: Plain, self-contained summary of the tool's core action.
2. **Usage Guidelines**: Explicit "When to use:" trigger scenarios so agents know exact situations to call the tool.
3. **Behavioral Transparency**: Explicit description of side-effects (e.g. disk write destination, memory return, overwrite behavior).
4. **Parameter Semantics**: Comprehensive schemas where every property defines an unambiguous description, concrete type, enums, and defaults.
5. **Conciseness**: Clean, compact prose with high signal-to-noise ratio.
6. **Contextual Completeness & Disambiguation**: Clear "Difference from related tools: unlike X, do NOT use this for Y" statements and formatted `[Input: ... | Output: ...]` signatures.

### Target Quality Metrics
- Upgrade Glama MCP Quality Grade from **C / B** to **A / A+**.
- Eliminate parameter schema ambiguities and enforce `"additionalProperties": false` on root tool schemas.
- Ensure 100% of registered MCP tools (`paperpilot-mcp/src/server.rs`) contain:
  - Concise purpose overview
  - `When to use:` trigger section
  - `Difference from related tools:` / `unlike:` section with explicit negative constraints
  - `Key parameters:` summary
  - Standardized `[Input: <Type> | Output: <Type>]` bracketed trailer

---

## Scope of Changes

### 1. Tool Schema Hardening
- Add `"additionalProperties": false` to all MCP tool input schemas where appropriate to prevent LLM hallucinations.
- Ensure all properties across all tool schemas provide explicit `"description"` fields.
- Populate `"enum"` and `"default"` fields for constrained parameters (e.g., formats, positions, orientations, sort orders).

### 2. Disambiguation & Separation
- Maintain clear boundaries between overlapping operations:
  - Search: `pdf_search` (regex/keyword in PDF content) vs. `semantic_search` (vector embeddings) vs. `workspace_search` (file repository indexing).
  - Text & Structured Extraction: `pdf_extract_text` (plain text) vs. `pdf_chunk` (fixed token chunks) vs. `pdf_rag_chunks` (structure-aware RAG chunks) vs. `pdf_to_markdown` (formatted markdown) vs. `pdf_to_json` (document AST).
- Ensure deprecated aliases remain suppressed in `tools/list` while safely executing in `tools/call`.

### 3. Verification & CI Compatibility
- All unit tests in `paperpilot-mcp` must pass: `cargo test -p paperpilot-mcp -j 4`.
- Compilation and schema checks must run with `-j 4` to preserve system stability.
