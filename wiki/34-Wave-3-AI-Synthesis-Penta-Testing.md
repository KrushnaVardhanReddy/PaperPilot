# Wave 3: AI Document Parsing & Synthesis Penta-Interface Testing Guide

## Overview
Wave 3 covers document-to-web conversions, structured JSON/Markdown extraction, semantic chunking for RAG, and query-based documentation lookup.

## The 6 Wave 3 Operations Covered
1. **`pdf_to_markdown`**: Converts documents into clean, LLM-ready layout markdown.
2. **`pdf_to_html`**: Synthesizes semantic, responsive HTML5 documents from PDF.
3. **`pdf_to_json`**: Extracts multimodal layout and text JSON with coordinates.
4. **`json_to_pdf`**: Bidirectional synthesis of publication-ready PDFs from structured JSON.
5. **`pdf_rag_chunks`**: Layout-aware semantic chunker for RAG indexing.
6. **`doc_rag_query`**: Documentation RAG lookup engine for instant tool matching.

## Running Tests
```bash
make test-wave3
# or
cargo test -p paperpilot-mcp --test wave3_ai_synthesis_mcp_tests -- --nocapture
```
*Outputs auto-generated scorecard to `reports/WAVE_3_AI_SYNTHESIS_PENTA_SCORECARD.md`.*
