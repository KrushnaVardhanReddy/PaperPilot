# Master Instructions: 5 Tools Per Session Protocol for Tester 3 (Conversions, Intelligence, RAG & Watchers)

## Context & Objective
We are standardizing automated test execution and UI alignment for **Tester 3 (Tools 61–90)** in `qa/manual_desktop_testing/TESTER_3_CONVERSIONS_INTELLIGENCE.md`.

For each session:
1. **Fix any underlying UI/Desktop/ArgBuilder bug** in `apps/desktop/src/lib/components/layout/OperationsPanel.svelte` or `operationArgsBuilder.ts`.
2. **Use pre-made fixtures** from `tests/e2e_fixtures/real/`.
3. **Update Playwright Desktop E2E** in `apps/desktop/tests/e2e_suite_3_conversions_ai.spec.ts` with deep assertions.
4. **Update Rust E2E Harness** in `tests/e2e/test_wave2_penta_interface.rs` or `test_wave5_penta_interface.rs` with identical assertions.
5. **Compile & test with `-j 4`**:
   ```bash
   cargo check --workspace -j 4
   pnpm --filter @paperpilot/desktop test:e2e:suite3
   ```

---

## ⭐️ Phase 0: Prerequisite Fixture Generation (Run FIRST before any session)
Generate/verify the following fixtures in `tests/e2e_fixtures/real/`:
1. `financial_tables.pdf`: PDF containing structured multi-column balance sheets and financial tables.
2. `test.docx`, `test.xlsx`, `test.pptx`: Native modern Office documents.
3. `legacy.doc`, `legacy.xls`: Legacy binary Office documents for modernization tests.
4. `ocr.pdf`: Scanned low-resolution invoice image document without native text streams.
5. `research_paper.pdf`: 10-page academic PDF with abstract, sections, and references for RAG chunking and classification.
6. `sample.ppflow`: Declarative PaperPilot pipeline flow definition.

---

## 6 Tool-Execution Sessions (5 Tools Per Session)

### Session 1: Tools 61–65 (Excel, Comic Archives, EPUB, PDF/A & Image Extraction)
- **Tool 61 (`pdf_convert_excel`)**: Convert spreadsheet to PDF. Assert grid borders and values rendered.
- **Tool 62 (`pdf_comic_archive`)**: Package PDF pages into `.cbz` archive. Assert valid ZIP structure containing page images.
- **Tool 63 (`pdf_to_epub`)**: Convert fixed layout PDF into reflowable EPUB 3.0. Assert valid `.epub` archive with chapter files.
- **Tool 64 (`pdf_to_pdf_a`)**: Convert to archival PDF/A. Assert PDF/A metadata and color profile conformance.
- **Tool 65 (`extract_images`)**: Extract all embedded raster images. Assert individual image files dumped.

### Session 2: Tools 66–70 (Office & Web Conversions)
- **Tool 66 (`pdf_extract_images`)**: Filtered image extraction with min-DPI threshold. Assert image dimensions.
- **Tool 67 (`pdf_to_docx`)**: Convert PDF into Word `.docx`. Assert valid docx zip archive containing `document.xml`.
- **Tool 68 (`pdf_to_xlsx`)**: Extract tables into Excel `.xlsx`. Assert valid xlsx zip archive containing `sheet1.xml`.
- **Tool 69 (`pdf_to_pptx`)**: Export PDF pages as slide deck `.pptx`. Assert slide count matches page count.
- **Tool 70 (`pdf_to_html`)**: Convert PDF to responsive HTML5 page. Assert semantic tags and embedded CSS.

### Session 3: Tools 71–75 (AI Markdown, Multimodal JSON, Tables & OCR)
- **Tool 71 (`pdf_to_markdown`)**: Extract clean LLM-ready markdown. Assert `# ` headings and formatted tables.
- **Tool 72 (`pdf_to_json`)**: Extract multimodal JSON layout with bounding boxes. Assert token coordinates.
- **Tool 73 (`pdf_extract_text`)**: Fast plain text extraction. Assert string contents dumped without markup.
- **Tool 74 (`pdf_extract_tables`)**: Extract table matrix from `financial_tables.pdf`. Assert 2D array representation.
- **Tool 75 (`pdf_ocr`)**: Perform neural OCR on `ocr.pdf`. Assert extracted text contains scanned keywords.

### Session 4: Tools 76–80 (Office Extraction, Legacy Modernization & Spreadsheets)
- **Tool 76 (`office_to_markdown`)**: Convert `.docx`/`.xlsx` to markdown. Assert table and text structure.
- **Tool 77 (`office_extract_text`)**: Extract raw text from Office docs. Assert clean string output.
- **Tool 78 (`office_modernize_legacy`)**: Upgrade `.doc` to `.docx`. Assert modernized XML packaging.
- **Tool 79 (`excel_to_json`)**: Convert spreadsheet sheets into JSON objects. Assert column key mapping.
- **Tool 80 (`excel_to_csv`)**: Export spreadsheet to RFC 4180 CSV. Assert comma-separated output rows.

### Session 5: Tools 81–85 (Search, Workspace Indexing & RAG Retrieval)
- **Tool 81 (`pdf_search`)**: Search keyword query. Assert hit page numbers and bounding box coordinates.
- **Tool 82 (`workspace_search`)**: Multi-format recursive workspace search. Assert file matches.
- **Tool 83 (`semantic_search`)**: Vector semantic retrieval. Assert cosine similarity ranking.
- **Tool 84 (`pdf_classify_type`)**: Classify `research_paper.pdf`. Assert category returns `Academic/Technical`.
- **Tool 85 (`pdf_rag_chunks`)**: Generate semantic chunks for LLM RAG. Assert token chunks with overlap.

### Session 6: Tools 86–90 (Workflows, Recipes, Natural Language & Watchers)
- **Tool 86 (`document_workflow`)**: Execute declarative `.ppflow` recipe. Assert sequential step outputs.
- **Tool 87 (`recipe_run`)**: Run instant multi-tool pipeline. Assert intermediate files passed between steps.
- **Tool 88 (`cli_ask`)**: Natural language query against document. Assert grounded response.
- **Tool 89 (`watcher_start`)**: Start hot-folder watcher daemon on test folder. Assert auto-processing on file drop.
- **Tool 90 (`watcher_stop`)**: Stop hot-folder watcher. Assert daemon shuts down cleanly and releases file locks.
