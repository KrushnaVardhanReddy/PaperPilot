---
title: 44-Tools Handbook
description: Comprehensive reference for all 44 PaperPilot tools across CLI, REST API, and MCP.
---

# Master 44-Tool Reference

This document provides exact, copy-pasteable snippets for invoking every tool across our three primary interfaces.

## Index of Tools

- [pdf_merge](#tool-pdf-merge)
- [pdf_split](#tool-pdf-split)
- [pdf_extract_pages](#tool-pdf-extract-pages)
- [pdf_delete_pages](#tool-pdf-delete-pages)
- [pdf_reorder_pages](#tool-pdf-reorder-pages)
- [pdf_rotate](#tool-pdf-rotate)
- [pdf_crop](#tool-pdf-crop)
- [pdf_burst](#tool-pdf-burst)
- [pdf_remove_blank](#tool-pdf-remove-blank)
- [pdf_compress](#tool-pdf-compress)
- [pdf_repair](#tool-pdf-repair)
- [pdf_linearize](#tool-pdf-linearize)
- [pdf_encrypt](#tool-pdf-encrypt)
- [pdf_decrypt](#tool-pdf-decrypt)
- [pdf_watermark](#tool-pdf-watermark)
- [pdf_redact](#tool-pdf-redact)
- [pdf_metadata](#tool-pdf-metadata)
- [pdf_sign](#tool-pdf-sign)
- [pdf_flatten](#tool-pdf-flatten)
- [pdf_to_pdf_a](#tool-pdf-to-pdf-a)
- [pdf_header_footer](#tool-pdf-header-footer)
- [pdf_bates](#tool-pdf-bates)
- [pdf_page_numbers](#tool-pdf-page-numbers)
- [pdf_extract_text](#tool-pdf-extract-text)
- [pdf_extract_images](#tool-pdf-extract-images)
- [pdf_search](#tool-pdf-search)
- [pdf_render](#tool-pdf-render)
- [pdf_compare](#tool-pdf-compare)
- [pdf_ocr](#tool-pdf-ocr)
- [pdf_bookmarks](#tool-pdf-bookmarks)
- [pdf_images_to_pdf](#tool-pdf-images-to-pdf)
- [pdf_annotate](#tool-pdf-annotate)
- [pdf_classify_type](#tool-pdf-classify-type)
- [pdf_validate](#tool-pdf-validate)
- [pdf_hash](#tool-pdf-hash)
- [pdf_read_form](#tool-pdf-read-form)
- [pdf_fill_form](#tool-pdf-fill-form)
- [pdf_create_form_field](#tool-pdf-create-form-field)
- [pdf_to_docx](#tool-pdf-to-docx)
- [pdf_to_xlsx](#tool-pdf-to-xlsx)
- [pdf_to_pptx](#tool-pdf-to-pptx)
- [pdf_convert_html](#tool-pdf-convert-html)
- [pdf_convert_markdown](#tool-pdf-convert-markdown)
- [pdf_convert_excel](#tool-pdf-convert-excel)

---

## Tool: `pdf_merge` <a id="tool-pdf-merge"></a>

### CLI
```bash
paperpilot merge --input tests/e2e_fixtures/page_1.pdf tests/e2e_fixtures/page_2.pdf --output out/merged_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_merge",
    "arguments": {
      "inputs": [
        "tests/e2e_fixtures/page_1.pdf",
        "tests/e2e_fixtures/page_2.pdf"
      ],
      "output": "out/merged_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/merge -H 'Content-Type: application/json' -d '{"inputs": ["tests/e2e_fixtures/page_1.pdf", "tests/e2e_fixtures/page_2.pdf"], "output": "out/merged_api.pdf"}'
```

---

## Tool: `pdf_split` <a id="tool-pdf-split"></a>

### CLI
```bash
paperpilot split --input tests/e2e_fixtures/multi_page.pdf --pages 1,2 --output out/split --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_split",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "out/split"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/split -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "pages": "1,2", "output": "out/split"}'
```

---

## Tool: `pdf_extract_pages` <a id="tool-pdf-extract-pages"></a>

### CLI
```bash
paperpilot extract --input tests/e2e_fixtures/multi_page.pdf --pages 1,3 --output out/extracted_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "pages": "1,3",
      "output": "out/extracted_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_extract_pages -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "pages": "1,3", "output": "out/extracted_api.pdf"}'
```

---

## Tool: `pdf_delete_pages` <a id="tool-pdf-delete-pages"></a>

### CLI
```bash
paperpilot delete --input tests/e2e_fixtures/multi_page.pdf --pages 2,4 --output out/deleted_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_delete_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "pages": "2,4",
      "output": "out/deleted_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_delete_pages -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "pages": "2,4", "output": "out/deleted_api.pdf"}'
```

---

## Tool: `pdf_reorder_pages` <a id="tool-pdf-reorder-pages"></a>

### CLI
```bash
paperpilot reorder --input tests/e2e_fixtures/multi_page.pdf --order 2,1,3,4,5 --output out/reordered_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_reorder_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "order": "2,1,3,4,5",
      "output": "out/reordered_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_reorder_pages -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "order": "2,1,3,4,5", "output": "out/reordered_api.pdf"}'
```

---

## Tool: `pdf_rotate` <a id="tool-pdf-rotate"></a>

### CLI
```bash
paperpilot rotate --input tests/e2e_fixtures/single_page.pdf --degrees 90 --pages 1 --output out/rotated_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_rotate",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "angle": 90,
      "pages": "1",
      "output": "out/rotated_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_rotate -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "angle": 90, "pages": "1", "output": "out/rotated_api.pdf"}'
```

---

## Tool: `pdf_crop` <a id="tool-pdf-crop"></a>

### CLI
```bash
paperpilot crop --input tests/e2e_fixtures/single_page.pdf --rect 10,10,200,200 --output out/cropped_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_crop",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "box": "10,10,200,200",
      "output": "out/cropped_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_crop -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "box": "10,10,200,200", "output": "out/cropped_api.pdf"}'
```

---

## Tool: `pdf_burst` <a id="tool-pdf-burst"></a>

### CLI
```bash
paperpilot burst --input tests/e2e_fixtures/multi_page.pdf --output out/burst_dir --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_burst",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output_dir": "out/burst_dir"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_burst -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "output_dir": "out/burst_dir"}'
```

---

## Tool: `pdf_remove_blank` <a id="tool-pdf-remove-blank"></a>

### CLI
```bash
paperpilot remove-blank --input tests/e2e_fixtures/multi_page.pdf --output out/noblank_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_remove_blank",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "out/noblank_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_remove_blank -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "output": "out/noblank_api.pdf"}'
```

---

## Tool: `pdf_compress` <a id="tool-pdf-compress"></a>

### CLI
```bash
# Quality accepts numeric values 1-100 or presets: low, medium, high
paperpilot compress --input tests/e2e_fixtures/single_page.pdf --quality 75 --output out/compressed_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_compress",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "out/compressed_mcp.pdf",
      "quality": "75"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/compress \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "out/compressed_api.pdf", "quality": "75"}'
```

---

## Tool: `pdf_repair` <a id="tool-pdf-repair"></a>

### CLI
```bash
paperpilot repair --input tests/e2e_fixtures/single_page.pdf --output out/repaired_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_repair",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "out/repaired_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_repair -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "out/repaired_api.pdf"}'
```

---

## Tool: `pdf_linearize` <a id="tool-pdf-linearize"></a>

### CLI
```bash
paperpilot linearize --input tests/e2e_fixtures/single_page.pdf --output out/linearized_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_linearize",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "out/linearized_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_linearize -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "out/linearized_api.pdf"}'
```

---

## Tool: `pdf_encrypt` <a id="tool-pdf-encrypt"></a>

### CLI
```bash
paperpilot encrypt --input tests/e2e_fixtures/single_page.pdf --user-password secret123 --owner-password secret123 --output out/encrypted_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_encrypt",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "password": "secret123",
      "output": "out/encrypted_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_encrypt -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "password": "secret123", "output": "out/encrypted_api.pdf"}'
```

---

## Tool: `pdf_decrypt` <a id="tool-pdf-decrypt"></a>

### CLI
```bash
paperpilot decrypt --input out/encrypted_cli.pdf --password secret123 --output out/decrypted_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_decrypt",
    "arguments": {
      "input": "out/encrypted_mcp.pdf",
      "password": "secret123",
      "output": "out/decrypted_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_decrypt -H 'Content-Type: application/json' -d '{"input": "out/encrypted_api.pdf", "password": "secret123", "output": "out/decrypted_api.pdf"}'
```

---

## Tool: `pdf_watermark` <a id="tool-pdf-watermark"></a>

### CLI
```bash
paperpilot watermark --input tests/e2e_fixtures/single_page.pdf --text CONFIDENTIAL --output out/watermarked_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_watermark",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "text": "CONFIDENTIAL",
      "output": "out/watermarked_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/watermark -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "text": "CONFIDENTIAL", "output": "out/watermarked_api.pdf"}'
```

---

## Tool: `pdf_redact` <a id="tool-pdf-redact"></a>

### CLI
```bash
paperpilot redact --input tests/e2e_fixtures/single_page.pdf --pages 1 --rect 50,50,200,50 --output out/redacted_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_redact",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "page": 1,
      "x": 50,
      "y": 50,
      "width": 200,
      "height": 50,
      "output": "out/redacted_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_redact -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "page": 1, "x": 50, "y": 50, "width": 200, "height": 50, "output": "out/redacted_api.pdf"}'
```

---

## Tool: `pdf_metadata` <a id="tool-pdf-metadata"></a>

### CLI
```bash
paperpilot metadata --input tests/e2e_fixtures/single_page.pdf --output out/metadata.json --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_metadata",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/info -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf"}'
```

---

## Tool: `pdf_sign` <a id="tool-pdf-sign"></a>

### CLI
```bash
paperpilot signature --input tests/e2e_fixtures/single_page.pdf --cert tests/e2e_fixtures/out/tri_e2e/dummy.p12 --output out/signed_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_sign",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "cert": "tests/e2e_fixtures/out/tri_e2e/dummy.p12",
      "output": "out/signed_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_sign -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "cert": "tests/e2e_fixtures/out/tri_e2e/dummy.p12", "output": "out/signed_api.pdf"}'
```

---

## Tool: `pdf_flatten` <a id="tool-pdf-flatten"></a>

### CLI
```bash
paperpilot flatten --input tests/e2e_fixtures/form.pdf --output out/flattened_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_flatten",
    "arguments": {
      "input": "tests/e2e_fixtures/form.pdf",
      "output": "out/flattened_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_flatten -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/form.pdf", "output": "out/flattened_api.pdf"}'
```

---

## Tool: `pdf_to_pdf_a` <a id="tool-pdf-to-pdf-a"></a>

### CLI
```bash
paperpilot pdf-a --input tests/e2e_fixtures/single_page.pdf --output out/pdf_a_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_to_pdf_a",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "out/pdf_a_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_pdf_a -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "out/pdf_a_api.pdf"}'
```

---

## Tool: `pdf_header_footer` <a id="tool-pdf-header-footer"></a>

### CLI
```bash
paperpilot header-footer --input tests/e2e_fixtures/multi_page.pdf --text Confidential --output out/header_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_header_footer",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "header_left": "Confidential",
      "footer_center": "Page",
      "output": "out/header_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_header_footer -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "header_left": "Confidential", "footer_center": "Page", "output": "out/header_api.pdf"}'
```

---

## Tool: `pdf_bates` <a id="tool-pdf-bates"></a>

### CLI
```bash
paperpilot bates --input tests/e2e_fixtures/multi_page.pdf --prefix CONF- --start 1 --output out/bates_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_bates",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "prefix": "CONF-",
      "start_number": 1,
      "padding": 6,
      "output": "out/bates_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_bates -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "prefix": "CONF-", "start_number": 1, "padding": 6, "output": "out/bates_api.pdf"}'
```

---

## Tool: `pdf_page_numbers` <a id="tool-pdf-page-numbers"></a>

### CLI
```bash
paperpilot page-numbers --input tests/e2e_fixtures/multi_page.pdf --output out/numbers_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_page_numbers",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "position": "bottom-right",
      "start_number": 1,
      "output": "out/numbers_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_page_numbers -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "position": "bottom-right", "start_number": 1, "output": "out/numbers_api.pdf"}'
```

---

## Tool: `pdf_extract_text` <a id="tool-pdf-extract-text"></a>

### CLI
```bash
paperpilot extract-text --input tests/e2e_fixtures/single_page.pdf --output out/text.txt --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_text",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "out/text.txt"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/extract-text -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "out/text.txt"}'
```

---

## Tool: `pdf_extract_images` <a id="tool-pdf-extract-images"></a>

### CLI
```bash
paperpilot extract-images --input tests/e2e_fixtures/image_doc.pdf --output out/extracted_images --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_images",
    "arguments": {
      "input": "tests/e2e_fixtures/image_doc.pdf",
      "output_dir": "out/extracted_images"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_extract_images -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/image_doc.pdf", "output_dir": "out/extracted_images"}'
```

---

## Tool: `pdf_search` <a id="tool-pdf-search"></a>

### CLI
```bash
paperpilot search --input tests/e2e_fixtures/search_test.pdf --query test --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_search",
    "arguments": {
      "input": "tests/e2e_fixtures/search_test.pdf",
      "query": "test"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_search -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/search_test.pdf", "query": "test"}'
```

---

## Tool: `pdf_render` <a id="tool-pdf-render"></a>

### CLI
```bash
paperpilot render --input tests/e2e_fixtures/single_page.pdf --output out/rendered_cli.png --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_render",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "page": 1,
      "output": "out/rendered_mcp.png"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/render-page -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "page": 1, "output": "out/rendered_api.png"}'
```

---

## Tool: `pdf_compare` <a id="tool-pdf-compare"></a>

### CLI
```bash
paperpilot compare --input tests/e2e_fixtures/page_1.pdf --input-b tests/e2e_fixtures/page_2.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_compare",
    "arguments": {
      "file1": "tests/e2e_fixtures/page_1.pdf",
      "file2": "tests/e2e_fixtures/page_2.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/compare -H 'Content-Type: application/json' -d '{"file1": "tests/e2e_fixtures/page_1.pdf", "file2": "tests/e2e_fixtures/page_2.pdf"}'
```

---

## Tool: `pdf_ocr` <a id="tool-pdf-ocr"></a>

### CLI
```bash
paperpilot ocr --input tests/e2e_fixtures/image_doc.pdf --output out/ocr_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_ocr",
    "arguments": {
      "input": "tests/e2e_fixtures/image_doc.pdf",
      "output": "out/ocr_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_ocr -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/image_doc.pdf", "output": "out/ocr_api.pdf"}'
```

---

## Tool: `pdf_bookmarks` <a id="tool-pdf-bookmarks"></a>

### CLI
```bash
paperpilot bookmarks --input tests/e2e_fixtures/large_doc.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_bookmarks",
    "arguments": {
      "input": "tests/e2e_fixtures/large_doc.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_bookmarks -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/large_doc.pdf"}'
```

---

## Tool: `pdf_images_to_pdf` <a id="tool-pdf-images-to-pdf"></a>

### CLI
```bash
paperpilot images-to-pdf --images tests/e2e_fixtures/img1.png tests/e2e_fixtures/img2.png --output out/images_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_images_to_pdf",
    "arguments": {
      "inputs": [
        "tests/e2e_fixtures/img1.png",
        "tests/e2e_fixtures/img2.png"
      ],
      "output": "out/images_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_images_to_pdf -H 'Content-Type: application/json' -d '{"inputs": ["tests/e2e_fixtures/img1.png", "tests/e2e_fixtures/img2.png"], "output": "out/images_api.pdf"}'
```

---

## Tool: `pdf_annotate` <a id="tool-pdf-annotate"></a>

### CLI
```bash
paperpilot annotate --input tests/e2e_fixtures/single_page.pdf --data [{"id": "1", "type": "highlight", "page": 1, "x": 50.0, "y": 50.0, "w": 50.0, "h": 50.0, "color": "#ffff00", "content": "Test"}] --output out/annotated.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_annotate",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "annotations": [
        {
          "id": "1",
          "type": "highlight",
          "page": 1,
          "x": 50.0,
          "y": 50.0,
          "w": 50.0,
          "h": 50.0,
          "color": "#ffff00",
          "content": "Test"
        }
      ],
      "output": "out/annotated.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_annotate -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "annotations": [{"id": "1", "type": "highlight", "page": 1, "x": 50.0, "y": 50.0, "w": 50.0, "h": 50.0, "color": "#ffff00", "content": "Test"}], "output": "out/annotated.pdf"}'
```

---

## Tool: `pdf_classify_type` <a id="tool-pdf-classify-type"></a>

### CLI
```bash
paperpilot classify --input tests/e2e_fixtures/single_page.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_classify_type",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_classify_type -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf"}'
```

---

## Tool: `pdf_validate` <a id="tool-pdf-validate"></a>

### CLI
```bash
paperpilot validate --input tests/e2e_fixtures/single_page.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_validate",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_validate -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf"}'
```

---

## Tool: `pdf_hash` <a id="tool-pdf-hash"></a>

### CLI
```bash
paperpilot hash --input tests/e2e_fixtures/single_page.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_hash",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_hash -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf"}'
```

---

## Tool: `pdf_read_form` <a id="tool-pdf-read-form"></a>

### CLI
```bash
paperpilot form read tests/e2e_fixtures/form.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_read_form",
    "arguments": {
      "input": "tests/e2e_fixtures/form.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_read_form -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/form.pdf"}'
```

---

## Tool: `pdf_fill_form` <a id="tool-pdf-fill-form"></a>

### CLI
```bash
paperpilot form fill tests/e2e_fixtures/form.pdf --data tests/e2e_fixtures/form_data.json --output out/filled_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_fill_form",
    "arguments": {
      "input": "tests/e2e_fixtures/form.pdf",
      "values": {
        "TestText": "Alice"
      },
      "output": "out/filled_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_fill_form -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/form.pdf", "values": {"TestText": "Alice"}, "output": "out/filled_api.pdf"}'
```

---

## Tool: `pdf_create_form_field` <a id="tool-pdf-create-form-field"></a>

### CLI
```bash
paperpilot form add-field tests/e2e_fixtures/single_page.pdf --name signature --type text --rect 50,50,150,30 --output out/added_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_create_form_field",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "field_name": "signature",
      "field_type": "text",
      "x": 50.0,
      "y": 50.0,
      "width": 100.0,
      "height": 30.0,
      "output": "out/added_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_create_form_field -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "field_name": "signature", "field_type": "text", "x": 50.0, "y": 50.0, "width": 100.0, "height": 30.0, "output": "out/added_api.pdf"}'
```

---

## Tool: `pdf_to_docx` <a id="tool-pdf-to-docx"></a>

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/single_page.pdf --format docx --output out/out_cli.docx --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_to_docx",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "out/out_mcp.docx"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "out/out_api.docx", "format": "docx"}'
```

---

## Tool: `pdf_to_xlsx` <a id="tool-pdf-to-xlsx"></a>

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/single_page.pdf --format xlsx --output out/out_cli.xlsx --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_to_xlsx",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "out/out_mcp.xlsx"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "out/out_api.xlsx", "format": "xlsx"}'
```

---

## Tool: `pdf_to_pptx` <a id="tool-pdf-to-pptx"></a>

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/single_page.pdf --format pptx --output out/out_cli.pptx --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_to_pptx",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "out/out_mcp.pptx"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "out/out_api.pptx", "format": "pptx"}'
```

---

## Tool: `pdf_convert_html` <a id="tool-pdf-convert-html"></a>

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/test.html --format pdf --output out/out_html_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_html",
    "arguments": {
      "input": "tests/e2e_fixtures/test.html",
      "output": "out/out_html_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_convert_html -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/test.html", "output": "out/out_html_api.pdf"}'
```

---

## Tool: `pdf_convert_markdown` <a id="tool-pdf-convert-markdown"></a>

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/test.md --format pdf --output out/out_md_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_markdown",
    "arguments": {
      "input": "tests/e2e_fixtures/test.md",
      "output": "out/out_md_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_convert_markdown -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/test.md", "output": "out/out_md_api.pdf"}'
```

---

## Tool: `pdf_convert_excel` <a id="tool-pdf-convert-excel"></a>

### CLI
```bash
paperpilot convert --input tests/e2e_fixtures/test.csv --format pdf --output out/out_csv_cli.pdf --json
```

### MCP (JSON-RPC)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_excel",
    "arguments": {
      "input": "tests/e2e_fixtures/test.csv",
      "output": "out/out_csv_mcp.pdf"
    }
  }
}
```

### REST API (cURL)
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_convert_excel -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/test.csv", "output": "out/out_csv_api.pdf"}'
```

---
