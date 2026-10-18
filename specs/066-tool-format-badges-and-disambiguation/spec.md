# Spec 066 — Explicit Input/Output Format Badges & Disambiguated Tool Typography

## 1. Overview & Problem Statement
Currently, tools in the Desktop Operations Panel and Studio Hub display generic, overlapping names:
- Searching `"extr"` yields 5+ similarly named cards: *"Extract Plain Text"*, *"Extract Text"*, *"Extract Pages"*, *"Extract Embedded Images"*, and *"OCR Text Recognition"*.
- Users cannot tell at a glance what file types can be supplied as input (e.g. PDF vs Word/Excel), nor what kind of file will be generated as output (e.g. a single `.txt` file, a new `.pdf`, a ZIP of `.png` images, or an editable `.docx`).
- This leads to cognitive friction, misapplied operations, and confusion.

**Objective:**
Upgrade `ToolDefinition` and the Desktop UI components (`OperationsPanel.svelte`, `+page.svelte`, and `toolList.ts`) to:
1. Disambiguate all tool titles (e.g., *"PDF to Plain Text"* vs *"Office to Plain Text"* vs *"Extract PDF Pages"*).
2. Add explicit `inputFormat` and `outputFormat` metadata across all 50+ tool definitions.
3. Render sleek, high-contrast, compact `INPUT ➔ OUTPUT` flow pills on every tool card in both the Operations Panel and Studio Hub.

---

## 2. Technical Architecture & Interface Schema

### 2.1 ToolDefinition Schema Extension (`apps/desktop/src/lib/types/tools.ts`)
```typescript
export interface ToolDefinition {
  id: string;
  title: string;
  description: string;
  icon: string;
  category: 'quick' | 'pages' | 'edit' | 'optimize' | 'security' | 'convert' | 'ai';
  context: 'pdf' | 'office' | 'ai';
  tags: string[];
  inputFormat: string;   // e.g. "PDF", "Office (DOCX/XLSX/PPTX)", "Scanned PDF / Images", "Images"
  outputFormat: string;  // e.g. ".TXT", ".DOCX", ".PDF", "PNGs", "JSON"
  rule?: ToolConditionRule;
}
```

### 2.2 Title & Format Disambiguation Catalog (`toolList.ts`)
Update existing ambiguous titles and equip all tools with explicit flow badges:

| Tool ID | Cleaned Disambiguated Title | `inputFormat` | `outputFormat` | Description |
| :--- | :--- | :--- | :--- | :--- |
| `extract_text` | **PDF to Plain Text** | `PDF` | `.TXT` | Extract all raw textual content from PDF into a text file |
| `ocr` | **OCR Text Recognition** | `Scanned PDF / Image` | `.TXT / PDF` | Extract text from scanned pages using local on-device neural OCR |
| `office_extract_text` | **Office to Plain Text** | `DOCX, XLSX, PPTX` | `.TXT` | Extract textual content from Microsoft Office documents |
| `extract_images` | **Extract Embedded Images** | `PDF` | `PNG Images` | Extract all photos, illustrations, and raster media saved in the PDF |
| `extract_pages` | **Extract PDF Pages** | `PDF` | `.PDF` | Cut out selected page ranges into a separate PDF |
| `pdf_to_docx` | **PDF to Word (DOCX)** | `PDF` | `.DOCX` | Export document as editable Microsoft Word file |
| `pdf_to_xlsx` | **PDF to Excel (XLSX)** | `PDF` | `.XLSX` | Extract tabular data into editable Excel spreadsheet |
| `pdf_to_markdown` | **PDF to Markdown** | `PDF` | `.MD` | Convert layout into LLM-ready markdown text |
| `pdf_to_html` | **PDF to HTML** | `PDF` | `.HTML` | Export document as responsive, styled HTML5 web page |
| `excel_to_json` | **Excel to JSON** | `XLSX, XLS, CSV` | `.JSON` | Convert spreadsheet sheets into structured tabular JSON |
| `excel_to_csv` | **Excel to CSV** | `XLSX, XLS` | `.CSV` | Export spreadsheet sheets into standard RFC 4180 CSV |
| `img_to_pdf` | **Images to PDF** | `JPG, PNG, WebP` | `.PDF` | Combine multiple images into a multi-page PDF |
| `md_to_pdf` | **Markdown to PDF** | `Markdown (.md)` | `.PDF` | Render formatted markdown text into styled PDF |
| `compress` | **Compress PDF** | `PDF` | `.PDF` | Lossy & lossless file size reduction with downsampling |
| `merge` | **Merge PDFs** | `2+ PDFs` | `.PDF` | Combine multiple PDF files into one continuous document |
| `split` | **Split PDF** | `PDF` | `Multiple .PDFs` | Divide document into separate pages or ranges |

*(And complete matching pairs for all remaining tools in `toolList.ts`)*.

### 2.3 UI Card Rendering (`OperationsPanel.svelte` & `+page.svelte`)
In the tool card template:
```svelte
<div class="tool-card-info">
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
  <span class="tool-card-desc">{tool.description}</span>
</div>
```

**Styling**:
- `.format-pill`: Compact pill badge with subtle glass background (`rgba(255, 255, 255, 0.08)` or accent tint), 10px font size, uppercase tracking, rounded radius.
- `.fmt-arrow`: Slight accent color (e.g. vibrant blue/violet `text-indigo-400`).
- Responsive: Wraps cleanly on smaller screens or mobile sheets without cutting off tool titles.

---

## 3. Verification Criteria
1. **TypeScript Compilation**: `npm run check` in `apps/desktop` passes with zero type errors.
2. **Search Verification**: Typing `"extr"` in the Operations Panel search bar displays distinct, instantly identifiable tools:
   - `PDF to Plain Text` (`PDF ➔ .TXT`)
   - `Office to Plain Text` (`DOCX, XLSX, PPTX ➔ .TXT`)
   - `Extract PDF Pages` (`PDF ➔ .PDF`)
   - `Extract Embedded Images` (`PDF ➔ PNG Images`)
   - `OCR Text Recognition` (`Scanned PDF / Image ➔ .TXT / PDF`)
3. **No Layout Regressions**: Operations panel cards and Studio Hub grid cards remain aligned and visually polished in dark and light themes.
