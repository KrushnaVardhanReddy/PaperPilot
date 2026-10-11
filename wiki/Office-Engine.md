# Office Engine

The PaperPilot Office Engine is a robust, pure-Rust system designed to parse, extract, and convert Microsoft Office formats into text, Markdown, and modern OOXML variants.

## Core Capabilities
The engine relies on the `office_oxide` crate to accomplish tasks that previously required heavyweight external dependencies like LibreOffice.

It supports 6 distinct file formats:
- **OOXML**: `.docx`, `.xlsx`, `.pptx`
- **Legacy Binary**: `.doc`, `.xls`, `.ppt`

## Architecture (`paperpilot-pdf/src/operations/office.rs`)
The engine is split into three primary operations, all returning a standard `OperationResult<()>`:

1. **OfficeToMarkdownOperation**:
   - Converts formatted structures like tables in Excel, slide headers in PowerPoint, and headings in Word into clean Markdown.
   - Useful for LLM ingestion and rendering via our HTML/PDF pipelines.

2. **OfficeExtractTextOperation**:
   - Executes sub-5ms raw text extraction.
   - Perfect for quick RAG pipeline embedding steps.

3. **OfficeModernizeOperation**:
   - Converts legacy binaries (`.doc`, `.xls`, `.ppt`) from old legal discovery dumps directly into their modern `x` counterparts without modifying content integrity.

## Usage Guide

### CLI
The `paperpilot-cli` introduces a dedicated `office` subcommand root.
```bash
paperpilot office to-markdown --input data.xlsx --output data.md
paperpilot office extract --input presentation.pptx --output out.txt
paperpilot office modernize --input 1999_case_file.doc --output 1999_case_file.docx
```

### Model Context Protocol (MCP)
Through the `paperpilot-mcp` server, agents can interact with the engine using three registered tools:
- `office_to_markdown`
- `office_extract_text`
- `office_modernize_legacy`

Each tool accepts an `input` path and `output` destination path.

### REST API
Via `paperpilot-gateway`, conversion operations can be accessed over HTTP.
```http
POST /api/v1/office/convert
Content-Type: application/json

{
    "input": "/path/to/legacy.doc",
    "output": "/path/to/modern.docx"
}
```

## Performance & Constraints
- **Pure Rust**: Contains zero C/C++ bindings.
- **In-Memory**: Standard files require <35MB RSS.
- **WASM Compatibility**: Due to the lack of native bindings, this engine is inherently WASM-compatible for future web-only execution variants.
