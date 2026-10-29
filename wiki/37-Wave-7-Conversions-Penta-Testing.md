# Wave 7: Conversions, Transformations & Document Packaging Penta-Interface Testing

This document contains testing procedures and verifiable snippets for the 13 tools in Wave 7 (Conversions, Transformations, and Document Packaging) across the Penta-Interface (CLI, MCP JSON-RPC, REST API).

## Tools Audited
1. `pdf_to_html`
2. `pdf_to_markdown`
3. `pdf_to_json`
4. `json_to_pdf`
5. `pdf_to_epub`
6. `pdf_to_pdf_a`
7. `pdf_comic_archive`
8. `excel_to_json`
9. `excel_to_csv`
10. `pdf_dedup_streams`
11. `pdf_invert_colors`
12. `pdf_linearize`
13. `pdf_repair`

---

## 1. `pdf_to_html`
**CLI:**
`paperpilot convert --format html --input fixtures/Get_Started_With_Smallpdf.pdf --output out.html`

**MCP:**
```json
{
  "name": "pdf_to_html",
  "arguments": {
    "input": "fixtures/Get_Started_With_Smallpdf.pdf",
    "output": "out.html"
  }
}
```

**API:**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_html \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.html"}'
```

---

## 2. `pdf_to_markdown`
**CLI:**
`paperpilot convert --format markdown --input fixtures/Get_Started_With_Smallpdf.pdf --output out.md`

**MCP:**
```json
{
  "name": "pdf_to_markdown",
  "arguments": {
    "input": "fixtures/Get_Started_With_Smallpdf.pdf",
    "output": "out.md"
  }
}
```

**API:**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_markdown \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.md"}'
```

---

## 3. `pdf_to_json`
**CLI:**
`paperpilot convert --format json --input fixtures/Get_Started_With_Smallpdf.pdf --output out.json`

**MCP:**
```json
{
  "name": "pdf_to_json",
  "arguments": {
    "input": "fixtures/Get_Started_With_Smallpdf.pdf",
    "output": "out.json"
  }
}
```

**API:**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_json \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.json"}'
```

---

## 4. `json_to_pdf`
**CLI:**
`paperpilot convert --format pdf --input tests/e2e_fixtures/real/json_doc.json --output out.pdf`

**MCP:**
```json
{
  "name": "json_to_pdf",
  "arguments": {
    "input": "tests/e2e_fixtures/real/json_doc.json",
    "output": "out.pdf"
  }
}
```

**API:**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/json_to_pdf \
  -H "Content-Type: application/json" \
  -d '{"input": "tests/e2e_fixtures/real/json_doc.json", "output": "out.pdf"}'
```

---

## 5. `pdf_to_epub`
**CLI:**
`paperpilot convert --format epub --input fixtures/Get_Started_With_Smallpdf.pdf --output out.epub`

**MCP:**
```json
{
  "name": "pdf_to_epub",
  "arguments": {
    "input": "fixtures/Get_Started_With_Smallpdf.pdf",
    "output": "out.epub"
  }
}
```

**API:**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_epub \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.epub"}'
```

---

## 6. `pdf_to_pdf_a`
**CLI:**
`paperpilot pdf-a --input fixtures/Get_Started_With_Smallpdf.pdf --output out.pdf`

**MCP:**
```json
{
  "name": "pdf_to_pdf_a",
  "arguments": {
    "input": "fixtures/Get_Started_With_Smallpdf.pdf",
    "output": "out.pdf"
  }
}
```

**API:**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_pdf_a \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.pdf"}'
```

---

## 7. `pdf_comic_archive`
**CLI:**
`paperpilot pdf-comic-archive --input fixtures/Get_Started_With_Smallpdf.pdf --output out.cbz`

**MCP:**
```json
{
  "name": "pdf_comic_archive",
  "arguments": {
    "input": "fixtures/Get_Started_With_Smallpdf.pdf",
    "output": "out.cbz"
  }
}
```

**API:**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_comic_archive \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.cbz"}'
```

---

## 8. `excel_to_json`
**CLI:**
`paperpilot office excel-to-json --input tests/e2e_fixtures/real/test.xlsx --output out.json`

**MCP:**
```json
{
  "name": "excel_to_json",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.xlsx",
    "output": "out.json"
  }
}
```

**API:**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/excel_to_json \
  -H "Content-Type: application/json" \
  -d '{"input": "tests/e2e_fixtures/real/test.xlsx", "output": "out.json"}'
```

---

## 9. `excel_to_csv`
**CLI:**
`paperpilot office excel-to-csv --input tests/e2e_fixtures/real/test.xlsx --output out.csv`

**MCP:**
```json
{
  "name": "excel_to_csv",
  "arguments": {
    "input": "tests/e2e_fixtures/real/test.xlsx",
    "output": "out.csv"
  }
}
```

**API:**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/excel_to_csv \
  -H "Content-Type: application/json" \
  -d '{"input": "tests/e2e_fixtures/real/test.xlsx", "output": "out.csv"}'
```

---

## 10. `pdf_dedup_streams`
**CLI:**
`paperpilot pdf-dedup-streams --input fixtures/Get_Started_With_Smallpdf.pdf --output out.pdf`

**MCP:**
```json
{
  "name": "pdf_dedup_streams",
  "arguments": {
    "input": "fixtures/Get_Started_With_Smallpdf.pdf",
    "output": "out.pdf"
  }
}
```

**API:**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_dedup_streams \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.pdf"}'
```

---

## 11. `pdf_invert_colors`
**CLI:**
`paperpilot invert-colors --input fixtures/Get_Started_With_Smallpdf.pdf --output out.pdf`

**MCP:**
```json
{
  "name": "pdf_invert_colors",
  "arguments": {
    "input": "fixtures/Get_Started_With_Smallpdf.pdf",
    "output": "out.pdf"
  }
}
```

**API:**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_invert_colors \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.pdf"}'
```

---

## 12. `pdf_linearize`
**CLI:**
`paperpilot linearize --input fixtures/Get_Started_With_Smallpdf.pdf --output out.pdf`

**MCP:**
```json
{
  "name": "pdf_linearize",
  "arguments": {
    "input": "fixtures/Get_Started_With_Smallpdf.pdf",
    "output": "out.pdf"
  }
}
```

**API:**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_linearize \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.pdf"}'
```

---

## 13. `pdf_repair`
**CLI:**
`paperpilot repair --input fixtures/Get_Started_With_Smallpdf.pdf --output out.pdf`

**MCP:**
```json
{
  "name": "pdf_repair",
  "arguments": {
    "input": "fixtures/Get_Started_With_Smallpdf.pdf",
    "output": "out.pdf"
  }
}
```

**API:**
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_repair \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.pdf"}'
```
