# Wave 8: Analysis, Intelligence & Search Penta-Testing

This wave covers tests across CLI, MCP, REST API, WASM, and Desktop for Analysis & Intelligence tools:

## Tools covered
1. `pdf_extract_text`
2. `pdf_ocr`
3. `pdf_extract_tables`
4. `document_pii_scan`
5. `pdf_bookmarks`
6. `pdf_bates`
7. `pdf_header_footer`
8. `pdf_classify_type`
9. `pdf_compare`
10. `pdf_hash`
11. `pdf_validate`
12. `pdf_render`
13. `pdf_rag_chunks`
14. `workspace_search`
15. `semantic_search`

## Testing Command

To run the automated penta test suite against these tools:
```bash
cargo test -p paperpilot-mcp --test wave8_analysis_intelligence_mcp_tests -j 2 -- --nocapture
```

## Manual testing commands

### 1. `pdf_extract_text`
**CLI**:
```bash
paperpilot extract-text --input tests/e2e_fixtures/single_page.pdf --json
```

**MCP**:
```json
{
  "name": "pdf_extract_text",
  "arguments": {
    "input": "tests/e2e_fixtures/single_page.pdf"
  }
}
```

**REST API**:
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_extract_text -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf"}'
```

### 2. `pdf_ocr`
**CLI**:
```bash
paperpilot ocr --input tests/e2e_fixtures/real/sample_photo_doc.pdf --output tests/e2e_fixtures/out/penta_e2e/ocr.pdf --json
```

**MCP**:
```json
{
  "name": "pdf_ocr",
  "arguments": {
    "input": "tests/e2e_fixtures/real/sample_photo_doc.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/ocr.pdf"
  }
}
```

**REST API**:
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_ocr -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/sample_photo_doc.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/ocr.pdf"}'
```

### 3. `pdf_extract_tables`
**CLI**:
```bash
paperpilot extract-tables --input tests/e2e_fixtures/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/tables.csv --format csv --pages all
```

**MCP**:
```json
{
  "name": "pdf_extract_tables",
  "arguments": {
    "input": "tests/e2e_fixtures/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/tables.json",
    "format": "json",
    "pages": "all"
  }
}
```

**REST API**:
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_extract_tables -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/tables.csv", "format": "csv", "pages": "all"}'
```

### 4. `document_pii_scan`
**CLI**:
```bash
paperpilot pii-scan --input tests/e2e_fixtures/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/pii_report.json --json
```

**MCP**:
```json
{
  "name": "document_pii_scan",
  "arguments": {
    "input": "tests/e2e_fixtures/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/pii_report.json"
  }
}
```

**REST API**:
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/document_pii_scan -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/pii_report.json"}'
```

### 5. `pdf_bookmarks`
**CLI**:
```bash
paperpilot bookmarks --input fixtures/Get_Started_With_Smallpdf.pdf --json
```

**MCP**:
```json
{
  "name": "pdf_bookmarks",
  "arguments": {
    "input": "fixtures/Get_Started_With_Smallpdf.pdf"
  }
}
```

**REST API**:
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_bookmarks -H 'Content-Type: application/json' -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf"}'
```

### 6. `pdf_bates`
**CLI**:
```bash
paperpilot bates --input fixtures/Get_Started_With_Smallpdf.pdf --output tests/e2e_fixtures/out/penta_e2e/bates.pdf --prefix BATES --start 1 --json
```

**MCP**:
```json
{
  "name": "pdf_bates",
  "arguments": {
    "input": "fixtures/Get_Started_With_Smallpdf.pdf",
    "prefix": "CONF-",
    "start": 1,
    "output": "tests/e2e_fixtures/out/penta_e2e/bates.pdf"
  }
}
```

**REST API**:
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_bates -H 'Content-Type: application/json' -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "prefix": "CONF-", "start": 1, "output": "tests/e2e_fixtures/out/penta_e2e/bates.pdf"}'
```

### 7. `pdf_header_footer`
**CLI**:
```bash
paperpilot header-footer --input tests/e2e_fixtures/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/header.pdf --text HEADER --json
```

**MCP**:
```json
{
  "name": "pdf_header_footer",
  "arguments": {
    "input": "tests/e2e_fixtures/single_page.pdf",
    "text": "Confidential",
    "output": "tests/e2e_fixtures/out/penta_e2e/header.pdf"
  }
}
```

**REST API**:
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_header_footer -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "text": "Confidential", "output": "tests/e2e_fixtures/out/penta_e2e/header.pdf"}'
```

### 8. `pdf_classify_type`
**CLI**:
```bash
paperpilot classify --input tests/e2e_fixtures/single_page.pdf --json
```

**MCP**:
```json
{
  "name": "pdf_classify_type",
  "arguments": {
    "input": "tests/e2e_fixtures/single_page.pdf"
  }
}
```

**REST API**:
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_classify_type -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf"}'
```

### 9. `pdf_compare`
**CLI**:
```bash
paperpilot compare --input tests/e2e_fixtures/single_page.pdf --input-b tests/e2e_fixtures/single_page.pdf --json
```

**MCP**:
```json
{
  "name": "pdf_compare",
  "arguments": {
    "input_a": "tests/e2e_fixtures/single_page.pdf",
    "input_b": "tests/e2e_fixtures/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/comparison.json"
  }
}
```

**REST API**:
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_compare -H 'Content-Type: application/json' -d '{"file1": "tests/e2e_fixtures/single_page.pdf", "file2": "tests/e2e_fixtures/single_page.pdf"}'
```

### 10. `pdf_hash`
**CLI**:
```bash
paperpilot hash --input tests/e2e_fixtures/single_page.pdf --json
```

**MCP**:
```json
{
  "name": "pdf_hash",
  "arguments": {
    "input": "tests/e2e_fixtures/single_page.pdf"
  }
}
```

**REST API**:
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_hash -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf"}'
```

### 11. `pdf_validate`
**CLI**:
```bash
paperpilot validate --input tests/e2e_fixtures/single_page.pdf --json
```

**MCP**:
```json
{
  "name": "pdf_validate",
  "arguments": {
    "input": "tests/e2e_fixtures/single_page.pdf"
  }
}
```

**REST API**:
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_validate -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf"}'
```

### 12. `pdf_render`
**CLI**:
```bash
paperpilot render --input tests/e2e_fixtures/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/rendered.png --json
```

**MCP**:
```json
{
  "name": "pdf_render",
  "arguments": {
    "input": "tests/e2e_fixtures/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/rendered.png"
  }
}
```

**REST API**:
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_render -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/rendered.png"}'
```

### 13. `pdf_rag_chunks`
**CLI**:
```bash
paperpilot rag-chunks --input tests/e2e_fixtures/single_page.pdf --json
```

**MCP**:
```json
{
  "name": "pdf_rag_chunks",
  "arguments": {
    "input": "tests/e2e_fixtures/single_page.pdf"
  }
}
```

**REST API**:
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_rag_chunks -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf"}'
```

### 14. `workspace_search`
**CLI**:
```bash
paperpilot search --query 'test' --dir tests/e2e_fixtures/real
```

**MCP**:
```json
{
  "name": "workspace_search",
  "arguments": {
    "query": "test",
    "dir": "tests/e2e_fixtures/real"
  }
}
```

**REST API**:
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/workspace_search -H 'Content-Type: application/json' -d '{"query": "test", "dir": "tests/e2e_fixtures/real"}'
```

### 15. `semantic_search`
**CLI**:
```bash
paperpilot search --query "quarterly revenue trends" --dir ./docs --json
```

**MCP**:
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "quarterly revenue trends",
    "limit": 5
  }
}
```

**REST API**:
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/search/semantic -H 'Content-Type: application/json' -d '{"query": "quarterly revenue trends", "limit": 5}'
```
