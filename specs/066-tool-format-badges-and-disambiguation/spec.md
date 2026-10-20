# Spec 066 — Universal Penta-Interface Tool Disambiguation & Explicit Input/Output Format Badges

## 1. Overview & Problem Statement
Currently across PaperPilot (Desktop UI, MCP Server, CLI, and REST Gateway), several core tools carry ambiguous, overlapping names:
- Searching `"extr"` in the desktop UI or listing MCP tools displays 5+ similar tools: *"Extract Plain Text"*, *"Extract Text"*, *"Extract Pages"*, *"Extract Embedded Images"*, and *"OCR Text Recognition"*.
- Neither human users in the desktop UI nor LLM agents connected via MCP have explicit guarantees of:
  1. What file extensions are supported as input (e.g. `.pdf` vs `.docx`, `.xlsx`, `.pptx`).
  2. What kind of output artifact will be written or returned (e.g. `.txt`, `.docx`, a single `.pdf`, a folder of `.png`s, or `.json`).

**Objective:**
Upgrade PaperPilot across all interfaces according to the **Penta-Interface Mandate**:
1. **Desktop Studio UI**: Disambiguate all tool titles and render sleek `INPUT ➔ OUTPUT` flow badges on cards in `OperationsPanel.svelte` and `+page.svelte`.
2. **MCP Server (`paperpilot-mcp`)**: Enrich all 51 tool descriptions with explicit `[Input: ... | Output: ...]` constraints so LLMs choose the exact tool without hallucination.
3. **CLI (`paperpilot-cli`)**: Add clear doc comments with `[Input: ... | Output: ...]` to all clap subcommands and arguments.
4. **REST Gateway (`paperpilot-gateway`)**: Document endpoints in Swagger/OpenAPI with clear format contracts.

---

## 2. Technical Architecture & Precise Implementations

### 2.1 Interface 1: Desktop Studio (`apps/desktop`)
1. **`apps/desktop/src/lib/types/tools.ts`**:
   Extend `ToolDefinition`:
   ```typescript
   export interface ToolDefinition {
     id: string;
     title: string;
     description: string;
     icon: string;
     category: 'quick' | 'pages' | 'edit' | 'optimize' | 'security' | 'convert' | 'ai';
     context: 'pdf' | 'office' | 'ai';
     tags: string[];
     inputFormat: string;   // e.g. "PDF (.pdf)", "Office (.docx, .xlsx)", "Images"
     outputFormat: string;  // e.g. ".TXT", ".DOCX", ".PDF", "PNG Images", ".JSON"
     rule?: ToolConditionRule;
   }
   ```

2. **`apps/desktop/src/lib/types/toolList.ts`**:
   Disambiguate titles and add `inputFormat`/`outputFormat` across all tools:
   - `extract_text` $\to$ **PDF to Plain Text** (`PDF` ➔ `.TXT`)
   - `office_extract_text` $\to$ **Office to Plain Text** (`DOCX, XLSX, PPTX` ➔ `.TXT`)
   - `ocr` $\to$ **OCR Text Recognition** (`Scanned PDF / Image` ➔ `.TXT / PDF`)
   - `extract_pages` $\to$ **Extract PDF Pages** (`PDF` ➔ `.PDF`)
   - `extract_images` $\to$ **Extract Embedded Images** (`PDF` ➔ `PNG Images`)
   - `pdf_to_docx` $\to$ **PDF to Word (DOCX)** (`PDF` ➔ `.DOCX`)
   - `pdf_to_xlsx` $\to$ **PDF to Excel (XLSX)** (`PDF` ➔ `.XLSX`)
   - `pdf_to_markdown` $\to$ **PDF to Markdown** (`PDF` ➔ `.MD`)
   - `pdf_to_html` $\to$ **PDF to HTML** (`PDF` ➔ `.HTML`)
   - `excel_to_json` $\to$ **Excel to JSON** (`XLSX, XLS, CSV` ➔ `.JSON`)
   - `excel_to_csv` $\to$ **Excel to CSV** (`XLSX, XLS` ➔ `.CSV`)
   - `img_to_pdf` $\to$ **Images to PDF** (`JPG, PNG, WebP` ➔ `.PDF`)
   - `md_to_pdf` $\to$ **Markdown to PDF** (`Markdown (.md)` ➔ `.PDF`)
   - `compress` $\to$ **Compress PDF** (`PDF` ➔ `.PDF`)
   - `merge` $\to$ **Merge PDFs** (`2+ PDFs` ➔ `.PDF`)
   - `split` $\to$ **Split PDF** (`PDF` ➔ `Multiple .PDFs`)

3. **`OperationsPanel.svelte` & `+page.svelte`**:
   Render the `.format-pill`:
   ```svelte
   <div class="tool-card-title-row">
     <span class="tool-card-title">{tool.title}</span>
     {#if tool.inputFormat && tool.outputFormat}
       <span class="format-pill">
         <span class="fmt-in">{tool.inputFormat}</span>
         <span class="fmt-arrow">➔</span>
         <span class="fmt-out">{tool.outputFormat}</span>
       </span>
     {/if}
   </div>
   ```

---

### 2.2 Interface 2: MCP Server (`paperpilot-mcp/src/server.rs`)
In `list_tools()`:
Update tool descriptions to explicitly declare expected input and output formats:
- `pdf_extract_text`:
  `"Extract raw text content from PDF. [Input: PDF (.pdf) | Output: Plain Text (.txt)]"`
- `office_extract_text`:
  `"Extract text content from Microsoft Office documents. [Input: Office (.docx, .xlsx, .pptx) | Output: Plain Text (.txt)]"`
- `extract_pages`:
  `"Extract selected page numbers into a new PDF. [Input: PDF (.pdf) | Output: PDF (.pdf)]"`
- `extract_images`:
  `"Extract raster images from a PDF. [Input: PDF (.pdf) | Output: PNG/JPEG images]"`
- `pdf_to_docx`:
  `"Convert PDF into editable Microsoft Word format. [Input: PDF (.pdf) | Output: Word (.docx)]"`
- `excel_to_json`:
  `"Extract spreadsheet tabular data as JSON. [Input: Excel (.xlsx, .xls) | Output: JSON array]"`
*(And all remaining tools with matching standardized `[Input: ... | Output: ...]` tags)*.

---

### 2.3 Interface 3: CLI (`paperpilot-cli/src/cli.rs`)
Update clap doc comments for subcommands:
```rust
/// PDF to Plain Text: Extract all raw textual content from a PDF [Input: .pdf | Output: .txt]
ExtractText {
    #[arg(long, help = "Path to input PDF file (.pdf)")]
    input: PathBuf,
    #[arg(long, help = "Path to output text file (.txt)")]
    output: PathBuf,
    ...
},

/// Office to Plain Text: Extract all text from Office documents [Input: .docx, .xlsx, .pptx | Output: .txt]
OfficeExtractText {
    #[arg(long, help = "Path to input Office file (.docx, .xlsx, .pptx)")]
    input: PathBuf,
    #[arg(long, help = "Path to output text file (.txt)")]
    output: PathBuf,
},
```

---

### 2.4 Interface 4: REST Gateway (`paperpilot-gateway/src/handlers.rs`)
Update endpoint route docstrings and Swagger summary descriptions to include `[Input: ... -> Output: ...]`.

---

## 3. Verification Protocol
1. **Desktop Verification**:
   - Run `npm run check` in `apps/desktop` (0 errors).
   - Search `"extr"` in Operations Panel and confirm all cards render with unambiguous titles and high-contrast `INPUT ➔ OUTPUT` pills.
2. **MCP Verification**:
   - Run `cargo test -p paperpilot-mcp`.
   - Verify tool list JSON includes `[Input: ... | Output: ...]` in tool descriptions.
3. **CLI Verification**:
   - Run `cargo run -p paperpilot-cli -- --help` and verify updated descriptions.
   - Run `cargo test -p paperpilot-cli`.
