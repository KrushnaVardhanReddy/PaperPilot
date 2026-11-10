# Spec 159: Agent Semantic Chunking & Dynamic Overlap (`pdf_chunk`)

## 1. Executive Summary & Problem Statement
When local AI coding agents (Claude Code, Gemini CLI, Cursor, OpenCodeInterpreter) and LLMs interact with medium-to-large PDFs (legal contracts, financial reports, technical manuals), loading the full raw text frequently overflows token budgets, dilutes attention mechanisms, or clips critical sections.
While PaperPilot provides structured markdown extraction and table parsing (`pdf_to_markdown`, `pdf_extract_tables`), it lacks an intelligent semantic chunker tailored specifically for LLM context windows.

Spec 159 introduces **`pdf_chunk`** across all PaperPilot interfaces (Rust Core, CLI, MCP Server, REST Gateway, and Desktop). Instead of naive character-count slicing that splits words or table rows in half, `pdf_chunk` uses document structure (headings `#`, `##`, paragraphs `\n\n`, table boundaries `|...|`, and page breaks) with configurable token targets and token overlap.

---

## 2. Technical Architecture & Data Models

### 2.1 Core Rust Engine (`paperpilot-pdf`)
File: `crates/paperpilot-pdf/src/operations/pdf_chunk.rs` (or `paperpilot-core`)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkOptions {
    /// Target chunk size in estimated tokens (default: 500, range: 100-4000)
    pub target_tokens: usize,
    /// Overlap between contiguous chunks in tokens (default: 50, range: 0-500)
    pub overlap_tokens: usize,
    /// If true, strips repetitive headers/footers before chunking
    pub strip_boilerplate: bool,
    /// Minimum heading level to trigger a hard boundary split (e.g. 1 for #, 2 for ##)
    pub break_on_headings: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    pub chunk_index: usize,
    pub page_start: u32,
    pub page_end: u32,
    pub estimated_tokens: usize,
    pub text: String,
    pub headings_context: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkResult {
    pub total_chunks: usize,
    pub total_tokens: usize,
    pub chunks: Vec<DocumentChunk>,
    pub execution_time_ms: f64,
}
```

### 2.2 Token Estimation & Boundary Splitting
1. **Token Heuristic**: Accurate BPE token estimator (~4 characters per token for English text/markdown).
2. **Structural Prioritization**:
   - Level 1: Page-level markdown headings (`# `, `## `)
   - Level 2: Paragraph breaks (`\n\n`)
   - Level 3: Sentence terminators (`. `, `! `, `? `)
   - Atomic entities: Markdown tables (`| ... |`) are kept intact in a single chunk unless the table itself exceeds `target_tokens`.
3. **Boilerplate Suppression**:
   - Detects recurring header/footer patterns identical across $\ge 3$ consecutive pages and strips them before segmenting.

---

## 3. Penta-Interface Specification

### 3.1 CLI Surface (`paperpilot-cli`)
```bash
paperpilot pdf-chunk \
  --input sample.pdf \
  --target-tokens 500 \
  --overlap 50 \
  --strip-boilerplate \
  --json-out chunks.json
```

### 3.2 MCP Tool (`paperpilot-mcp`)
Tool Name: `pdf_chunk`
- `input`: string (file path)
- `target_tokens`: optional number (default 500)
- `overlap_tokens`: optional number (default 50)
- `strip_boilerplate`: optional boolean (default true)
- `break_on_headings`: optional boolean (default true)

### 3.3 REST Gateway (`paperpilot-gateway`)
- `POST /api/v1/pdf/chunk`
- Payload: `{ "input": "/path/to/doc.pdf", "target_tokens": 500, "overlap_tokens": 50, "strip_boilerplate": true }`

### 3.4 Desktop & Web UI
- Registered in `toolList.ts` under Category `Conversions & Intelligence`.
- Controls in `OperationsPanel.svelte`: Target tokens slider (100–2000), Overlap slider (0–200), Strip boilerplate toggle.

---

## 4. Verification & Testing Standards
- Must pass unit tests in `paperpilot-pdf` with varied markdown fixtures (headings, long tables, lists).
- Memory safety: zero heap bloat on 500+ page test corpus.
- Parallel build requirement: Always verify with `cargo check --workspace -j 4` and `cargo test -p paperpilot-pdf --test chunk_tests -j 4`.
