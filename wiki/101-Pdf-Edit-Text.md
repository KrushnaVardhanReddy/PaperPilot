# Tool 101: In-Place PDF Text Replacement (`pdf_edit_text`)

## 1. Overview & Capabilities
**Tool 101 (`pdf_edit_text`)** provides surgical, in-place search and replacement of text strings directly within PDF `/Contents` streams.

Unlike crude approaches that paste white rectangles over text or require round-trip conversions through DOCX/HTML, `pdf_edit_text` directly modifies the internal PDF content stream operators (`Tj`, `TJ`, and `'`).

### Key Highlights
- **Preserves Document Layout**: Font metrics, vector paths, embedded graphics, tables, and form fields remain 100% intact.
- **Granular Targeting**:
  - Replace all occurrences document-wide.
  - Target a specific 1-indexed page (`--page <N>`).
  - Target a specific match occurrence (`--occurrence <N>`).
  - Case-sensitive or case-insensitive search (`--match-case`).
- **Penta-Interface Parity**: Available across Core Rust, CLI (`edit-text`), REST API Gateway (`/api/v1/pdf/edit-text`), MCP Server (`pdf_edit_text`), WebAssembly, and Cloudflare Edge Workers.

---

## 2. Technical Architecture & Stream Parsing

```
[ PDF Document ]
       │
       ├──► 1. Locate Page Dictionary & Resolve `/Contents`
       │      • Handles both single stream objects and composite stream arrays `[ ref ref ]`
       │
       ├──► 2. Decode Stream Filters
       │      • Decompress FlateDecode (`miniz_oxide`) to access raw PDF graphic tokens
       │
       ├──► 3. Extract Font Resources & Mappings
       │      • Inspect `/Font` dictionary in page `/Resources`
       │      • Resolve character encodings (WinAnsiEncoding, StandardEncoding, Identity-H)
       │
       ├──► 4. Content Stream Token Modification
       │      • Detect text operators (`BT ... ET`)
       │      • Parse string literals `(Hello) Tj` and kerning arrays `[(Hel) -20 (lo)] TJ`
       │      • Match search needle and substitute replacement bytes using proper encoding
       │
       └──► 5. Recompress & AST Synchronization
              • Re-compress stream with FlateDecode
              • Update stream `/Length` and re-serialize document trailer
```

---

## 3. Interface Usage Examples

### 3.1 CLI
```bash
# Correct typo across entire document
paperpilot edit-text \
  --input report.pdf \
  --output report_corrected.pdf \
  --find "Confidntial" \
  --replace "Confidential"

# Replace only the 1st occurrence on page 3
paperpilot edit-text \
  --input contract.pdf \
  --output contract_v2.pdf \
  --find "May 2025" \
  --replace "June 2025" \
  --page 3 \
  --occurrence 1
```

### 3.2 REST API Gateway
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/edit-text \
  -H "Content-Type: application/json" \
  -d '{
    "input": "document.pdf",
    "output": "document_edited.pdf",
    "find": "Draft",
    "replace": "Final",
    "match_case": true
  }'
```

### 3.3 MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_edit_text",
    "arguments": {
      "input": "document.pdf",
      "output": "document_edited.pdf",
      "find": "Old Term",
      "replace": "New Term",
      "page": 1,
      "occurrence": 1
    }
  },
  "id": 101
}
```

### 3.4 WebAssembly & Browser Studio (`paperpilot-wasm`)
```typescript
import { pdf_edit_text } from "@usepaperpilot/wasm";

const modifiedPdfBytes = pdf_edit_text(
  sourcePdfBytes,
  "Acme Inc.",
  "Acme Global Inc.",
  1, // page
  1, // occurrence
  true // match_case
);
```

---

## 4. Edge Cases & Constraints
1. **Font Subset Limitations**: If the PDF font is heavily subsetted (e.g. only containing glyphs present in the original document), replacement characters outside the subset glyph repertoire may not render correctly. For radical text replacements, use OCR reconstruction (`pdf_ocr` $\to$ `pdf_to_docx`).
2. **Justified Text Alignment**: In-place replacement modifies character strings but does not recalculate paragraph line-breaks or justification margins. Replacing short words with substantially longer phrases may push text into margins.
