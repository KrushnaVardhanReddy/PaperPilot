## Phase 10 — Universal Document Operating System (Beyond PDF: Cross-Format Engine) `[FUTURE VISION]`

> **Strategic Vision:** Evolve PaperPilot from a best-in-class PDF workstation into the **Universal Document Operating System** for humans and AI agents.
> Bypasses heavy legacy runtimes (LibreOffice, Chromium/Puppeteer, Pandoc) using a unified, pure-Rust Universal Document AST that executes in-memory across Desktop, Mobile, Browser WASM, and Cloudflare Edge.

```
                  ┌──────────────────────────────────────────────┐
                  │          Universal Document AST              │
                  │   (Semantic Text, Layout, Styles, Media)     │
                  └──────────────────────┬───────────────────────┘
          ┌──────────────────────────────┼──────────────────────────────┐
          ▼                              ▼                              ▼
 📄 Rich Documents              📊 Data & Sheets               📑 Web & Publishing
 - PDF / PDF/A                  - XLSX / CSV                   - Markdown / CommonMark
 - DOCX (Word)                  - JSON / NDJSON / Parquet      - HTML5 / CSS Paged Media
 - RTF / Plain Text             - SQLite / libSQL Tables       - EPUB / Typst Research Docs
```

| # | Milestone | Scope & Deliverables |
|---|---|---|
| 10.1 | Universal Document Abstract Syntax Tree (`DocAST`) | Unify text spans, bounding boxes, typographic hierarchies, tables, vectors, and embedded images into a single in-memory DOM representation in `paperpilot-core`. |
| 10.2 | Any-to-Any Format Conversion Matrix | Pure-Rust bidirectional transformations without external runtimes: `DOCX ⇄ Markdown ⇄ HTML ⇄ PDF ⇄ EPUB` and `XLSX ⇄ CSV ⇄ JSON ⇄ PDF Table`. |
| 10.3 | Native Non-PDF Operation Suite | Native document hygiene outside PDF: `docx_strip_tracking` (remove revision metadata & comments), `docx_redact`, `xlsx_deduplicate`, `xlsx_sanitize_hidden_sheets`, `markdown_lint_format`. |
| 10.4 | Multi-Format Penta-Interface Parity | Expose the full cross-format catalog across CLI (`paperpilot convert/sanitize`), MCP (`universal_document_read/write`), REST Gateway, client WASM, and Cloudflare Edge. |
| 10.5 | Universal AI Document Gateway | Become the default document ingestion & synthesis layer for AI agents (Claude, ChatGPT, LangChain) with automated multimodal asset extraction, table normalization, and zero-leak air-gapped operation. |
| 10.6 | Enterprise Universal Document Studio UI | Unified visual workbench in `apps/desktop` and `apps/web` allowing users to preview, edit, annotate, split, and merge any document format side-by-side with real-time visual diffing. |




