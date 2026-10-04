# Tri-Interface E2E Verification & Interactive Living Documentation

## Executive Scorecard
- Total Tools Verified: 44/44
- CLI Pass Rate: 5/14
- MCP Pass Rate: 14/14
- REST API Pass Rate: 13/14

## Detailed Tool-by-Tool Documentation

### 31. `pdf_bookmarks`: Extract document outline and bookmarks.
- **Input**: `multi_page.pdf` (1409 bytes)

#### Working Examples
**💻 CLI Example**
```bash
paperpilot bookmarks --input /app/tests/e2e_fixtures/multi_page.pdf --json
```
**🤖 MCP Example**
```json
{
  "input": "/app/tests/e2e_fixtures/multi_page.pdf"
}
```
**🌐 REST API Example**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_bookmarks \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/multi_page.pdf"}'
```

#### Verification Scorecard
| Interface | Status | Latency (ms) | Output Details |
|-----------|--------|--------------|----------------|
| CLI | PASS | 22 | JSON metrics |
| MCP | PASS | 92 | Success |
| API | PASS | 145 | Success |


### 32. `pdf_compress`: Optimize and compress PDF streams.
- **Input**: `large_doc.pdf` (4737 bytes)

#### Working Examples
**💻 CLI Example**
```bash
paperpilot compress --input /app/tests/e2e_fixtures/large_doc.pdf --quality medium --output /app/tests/e2e_fixtures/out/tri_e2e/compressed.pdf --json
```
**🤖 MCP Example**
```json
{
  "input": "/app/tests/e2e_fixtures/large_doc.pdf",
  "quality": "medium",
  "output": "/app/tests/e2e_fixtures/out/tri_e2e/compressed.pdf"
}
```
**🌐 REST API Example**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/compress \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/large_doc.pdf", "quality": "medium", "output": "/app/tests/e2e_fixtures/out/tri_e2e/compressed.pdf"}'
```

#### Verification Scorecard
| Interface | Status | Latency (ms) | Output Details |
|-----------|--------|--------------|----------------|
| CLI | PASS | 29 | Output: `compressed.pdf` (4742 bytes) |
| MCP | PASS | 28 | Success |
| API | PASS | 22 | Success |


### 33. `pdf_linearize`: Linearize PDF for fast web viewing.
- **Input**: `single_page.pdf` (539 bytes)

#### Working Examples
**💻 CLI Example**
```bash
paperpilot linearize --input /app/tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/linearized.pdf --json
```
**🤖 MCP Example**
```json
{
  "input": "/app/tests/e2e_fixtures/single_page.pdf",
  "output": "/app/tests/e2e_fixtures/out/tri_e2e/linearized.pdf"
}
```
**🌐 REST API Example**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_linearize \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/linearized.pdf"}'
```

#### Verification Scorecard
| Interface | Status | Latency (ms) | Output Details |
|-----------|--------|--------------|----------------|
| CLI | PASS | 17 | Output: `linearized.pdf` (543 bytes) |
| MCP | PASS | 14 | Success |
| API | PASS | 6 | Success |


### 34. `pdf_flatten`: Flatten interactive annotations into page content.
- **Input**: `form_filled.pdf` (857 bytes)

#### Working Examples
**💻 CLI Example**
```bash
paperpilot flatten --input /app/tests/e2e_fixtures/form_filled.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/flattened.pdf --json
```
**🤖 MCP Example**
```json
{
  "input": "/app/tests/e2e_fixtures/form_filled.pdf",
  "output": "/app/tests/e2e_fixtures/out/tri_e2e/flattened.pdf"
}
```
**🌐 REST API Example**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_flatten \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/form_filled.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/flattened.pdf"}'
```

#### Verification Scorecard
| Interface | Status | Latency (ms) | Output Details |
|-----------|--------|--------------|----------------|
| CLI | PASS | 15 | Output: `flattened.pdf` (842 bytes) |
| MCP | PASS | 13 | Success |
| API | PASS | 5 | Success |


### 35. `pdf_to_docx`: Convert PDF to editable Word .docx.
- **Input**: `search_test.pdf` (3499 bytes)

#### Working Examples
**💻 CLI Example**
```bash
paperpilot to-docx --input /app/tests/e2e_fixtures/search_test.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/document.docx --json
```
**🤖 MCP Example**
```json
{
  "input": "/app/tests/e2e_fixtures/search_test.pdf",
  "output": "/app/tests/e2e_fixtures/out/tri_e2e/document.docx"
}
```
**🌐 REST API Example**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/convert \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/search_test.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/document.docx", "format": "docx"}'
```

#### Verification Scorecard
| Interface | Status | Latency (ms) | Output Details |
|-----------|--------|--------------|----------------|
| CLI | FAIL | 11 | Output: `document.docx` (17945 bytes) |
| MCP | PASS | 34 | Success |
| API | FAIL | 5 | HTTP Error 500: Internal Server Error |


### 36. `pdf_to_xlsx`: Convert PDF tables to Excel .xlsx.
- **Input**: `search_test.pdf` (3499 bytes)

#### Working Examples
**💻 CLI Example**
```bash
paperpilot to-xlsx --input /app/tests/e2e_fixtures/search_test.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/document.xlsx --json
```
**🤖 MCP Example**
```json
{
  "input": "/app/tests/e2e_fixtures/search_test.pdf",
  "output": "/app/tests/e2e_fixtures/out/tri_e2e/document.xlsx"
}
```
**🌐 REST API Example**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_xlsx \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/search_test.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/document.xlsx"}'
```

#### Verification Scorecard
| Interface | Status | Latency (ms) | Output Details |
|-----------|--------|--------------|----------------|
| CLI | FAIL | 11 | Output: `document.xlsx` (5081 bytes) |
| MCP | PASS | 49 | Success |
| API | PASS | 52 | Success |


### 37. `pdf_to_pptx`: Convert PDF to PowerPoint .pptx.
- **Input**: `single_page.pdf` (539 bytes)

#### Working Examples
**💻 CLI Example**
```bash
paperpilot to-pptx --input /app/tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/presentation.pptx --json
```
**🤖 MCP Example**
```json
{
  "input": "/app/tests/e2e_fixtures/single_page.pdf",
  "output": "/app/tests/e2e_fixtures/out/tri_e2e/presentation.pptx"
}
```
**🌐 REST API Example**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_pptx \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/presentation.pptx"}'
```

#### Verification Scorecard
| Interface | Status | Latency (ms) | Output Details |
|-----------|--------|--------------|----------------|
| CLI | FAIL | 11 | Output: `presentation.pptx` (645 bytes) |
| MCP | PASS | 16 | Success |
| API | PASS | 10 | Success |


### 38. `pdf_to_pdf_a`: Convert PDF to archival standard PDF/A.
- **Input**: `single_page.pdf` (539 bytes)

#### Working Examples
**💻 CLI Example**
```bash
paperpilot to-pdf-a --input /app/tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/archival.pdf --json
```
**🤖 MCP Example**
```json
{
  "input": "/app/tests/e2e_fixtures/single_page.pdf",
  "output": "/app/tests/e2e_fixtures/out/tri_e2e/archival.pdf"
}
```
**🌐 REST API Example**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_pdf_a \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/archival.pdf"}'
```

#### Verification Scorecard
| Interface | Status | Latency (ms) | Output Details |
|-----------|--------|--------------|----------------|
| CLI | FAIL | 11 | Output: `archival.pdf` (607 bytes) |
| MCP | PASS | 12 | Success |
| API | PASS | 5 | Success |


### 39. `pdf_classify_type`: Classify PDF document category.
- **Input**: `form.pdf` (832 bytes)

#### Working Examples
**💻 CLI Example**
```bash
paperpilot classify --input /app/tests/e2e_fixtures/form.pdf --json
```
**🤖 MCP Example**
```json
{
  "input": "/app/tests/e2e_fixtures/form.pdf"
}
```
**🌐 REST API Example**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_classify_type \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/form.pdf"}'
```

#### Verification Scorecard
| Interface | Status | Latency (ms) | Output Details |
|-----------|--------|--------------|----------------|
| CLI | PASS | 29 | JSON metrics |
| MCP | PASS | 19 | Success |
| API | PASS | 12 | Success |


### 40. `pdf_compare`: Compare two PDFs and flag visual/text diffs.
- **Input**: `page_1.pdf` (539 bytes)

#### Working Examples
**💻 CLI Example**
```bash
paperpilot compare --file1 /app/tests/e2e_fixtures/page_1.pdf --file2 /app/tests/e2e_fixtures/page_2.pdf --json
```
**🤖 MCP Example**
```json
{
  "file1": "/app/tests/e2e_fixtures/page_1.pdf",
  "file2": "/app/tests/e2e_fixtures/page_2.pdf"
}
```
**🌐 REST API Example**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/compare \
  -H "Content-Type: application/json" \
  -d '{"file1": "/app/tests/e2e_fixtures/page_1.pdf", "file2": "/app/tests/e2e_fixtures/page_2.pdf"}'
```

#### Verification Scorecard
| Interface | Status | Latency (ms) | Output Details |
|-----------|--------|--------------|----------------|
| CLI | FAIL | 11 | JSON metrics |
| MCP | PASS | 15 | Success |
| API | PASS | 8 | Success |


### 41. `pdf_annotate`: Add sticky notes, highlights, and markup.
- **Input**: `single_page.pdf` (539 bytes)

#### Working Examples
**💻 CLI Example**
```bash
paperpilot annotate --input /app/tests/e2e_fixtures/single_page.pdf --annotations '[{"type": "note", "page": 1, "x": 50, "y": 50, "content": "Test", "w": 100, "h": 100, "color": "yellow", "id": "1", "rect": [50, 50, 150, 150]}]' --output /app/tests/e2e_fixtures/out/tri_e2e/annotated.pdf --json
```
**🤖 MCP Example**
```json
{
  "input": "/app/tests/e2e_fixtures/single_page.pdf",
  "annotations": [
    {
      "type": "note",
      "page": 1,
      "x": 50,
      "y": 50,
      "content": "Test",
      "w": 100,
      "h": 100,
      "color": "yellow",
      "id": "1",
      "rect": [
        50,
        50,
        150,
        150
      ]
    }
  ],
  "output": "/app/tests/e2e_fixtures/out/tri_e2e/annotated.pdf"
}
```
**🌐 REST API Example**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_annotate \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/single_page.pdf", "annotations": [{"type": "note", "page": 1, "x": 50, "y": 50, "content": "Test", "w": 100, "h": 100, "color": "yellow", "id": "1", "rect": [50, 50, 150, 150]}], "output": "/app/tests/e2e_fixtures/out/tri_e2e/annotated.pdf"}'
```

#### Verification Scorecard
| Interface | Status | Latency (ms) | Output Details |
|-----------|--------|--------------|----------------|
| CLI | FAIL | 11 | Output: `annotated.pdf` (650 bytes) |
| MCP | PASS | 15 | Success |
| API | PASS | 8 | Success |


### 42. `pdf_convert_html`: Convert HTML to styled PDF with CSS presets.
- **Input**: `test.html` (27 bytes)

#### Working Examples
**💻 CLI Example**
```bash
paperpilot from-html --input /app/tests/e2e_fixtures/test.html --output /app/tests/e2e_fixtures/out/tri_e2e/html_out.pdf --preset elegant --json
```
**🤖 MCP Example**
```json
{
  "input": "/app/tests/e2e_fixtures/test.html",
  "output": "/app/tests/e2e_fixtures/out/tri_e2e/html_out.pdf",
  "preset": "elegant"
}
```
**🌐 REST API Example**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_convert_html \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/test.html", "output": "/app/tests/e2e_fixtures/out/tri_e2e/html_out.pdf", "preset": "elegant"}'
```

#### Verification Scorecard
| Interface | Status | Latency (ms) | Output Details |
|-----------|--------|--------------|----------------|
| CLI | FAIL | 11 | Output: `html_out.pdf` (13621 bytes) |
| MCP | PASS | 15669 | Success |
| API | PASS | 2621 | Success |


### 43. `pdf_convert_markdown`: Convert Markdown to styled PDF with CSS presets.
- **Input**: `test.md` (17 bytes)

#### Working Examples
**💻 CLI Example**
```bash
paperpilot from-markdown --input /app/tests/e2e_fixtures/test.md --output /app/tests/e2e_fixtures/out/tri_e2e/md_out.pdf --preset github --json
```
**🤖 MCP Example**
```json
{
  "input": "/app/tests/e2e_fixtures/test.md",
  "output": "/app/tests/e2e_fixtures/out/tri_e2e/md_out.pdf",
  "preset": "github"
}
```
**🌐 REST API Example**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_convert_markdown \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/test.md", "output": "/app/tests/e2e_fixtures/out/tri_e2e/md_out.pdf", "preset": "github"}'
```

#### Verification Scorecard
| Interface | Status | Latency (ms) | Output Details |
|-----------|--------|--------------|----------------|
| CLI | FAIL | 11 | Output: `md_out.pdf` (14554 bytes) |
| MCP | PASS | 2222 | Success |
| API | PASS | 2646 | Success |


### 44. `pdf_convert_excel`: Convert CSV/Excel to semantic HTML styled PDF.
- **Input**: `test.csv` (18 bytes)

#### Working Examples
**💻 CLI Example**
```bash
paperpilot from-excel --input /app/tests/e2e_fixtures/test.csv --output /app/tests/e2e_fixtures/out/tri_e2e/excel_out.pdf --preset minimal --json
```
**🤖 MCP Example**
```json
{
  "input": "/app/tests/e2e_fixtures/test.csv",
  "output": "/app/tests/e2e_fixtures/out/tri_e2e/excel_out.pdf",
  "preset": "minimal"
}
```
**🌐 REST API Example**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_convert_excel \
  -H "Content-Type: application/json" \
  -d '{"input": "/app/tests/e2e_fixtures/test.csv", "output": "/app/tests/e2e_fixtures/out/tri_e2e/excel_out.pdf", "preset": "minimal"}'
```

#### Verification Scorecard
| Interface | Status | Latency (ms) | Output Details |
|-----------|--------|--------------|----------------|
| CLI | FAIL | 15 | Output: `excel_out.pdf` (13182 bytes) |
| MCP | PASS | 2327 | Success |
| API | PASS | 2552 | Success |
