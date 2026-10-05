# Phase 4.FIX.4C.v2 — Master Tri-Interface E2E Test Suite (All 44 Tools, 100% Parity)

## Executive Scorecard
- **Total Tools Verified:** 44
- **CLI Pass Rate:** 44 / 44
- **MCP Pass Rate:** 44 / 44
- **API Pass Rate:** 44 / 44

## Detailed Tool-by-Tool Documentation

### Tool: `pdf_merge`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli merge --input tests/e2e_fixtures/page_1.pdf tests/e2e_fixtures/page_2.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/merged_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.94 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/merged_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 7.56 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/merge -H 'Content-Type: application/json' -d '{"inputs": ["tests/e2e_fixtures/page_1.pdf", "tests/e2e_fixtures/page_2.pdf"], "output": "/app/tests/e2e_fixtures/out/tri_e2e/merged_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 45.96 ms

---

### Tool: `pdf_split`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli split --input tests/e2e_fixtures/multi_page.pdf --pages 1,2 --output /app/tests/e2e_fixtures/out/tri_e2e/split --json
```
- **Status:** ✅ PASS
- **Latency:** 7.75 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_split",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/split"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 8.21 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/split -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "pages": "1,2", "output": "/app/tests/e2e_fixtures/out/tri_e2e/split"}'
```
- **Status:** ✅ PASS
- **Latency:** 2.54 ms

---

### Tool: `pdf_extract_pages`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli extract --input tests/e2e_fixtures/multi_page.pdf --pages 1,3 --output /app/tests/e2e_fixtures/out/tri_e2e/extracted_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.46 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/extracted_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.73 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_extract_pages -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "pages": "1,3", "output": "/app/tests/e2e_fixtures/out/tri_e2e/extracted_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.68 ms

---

### Tool: `pdf_delete_pages`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli delete --input tests/e2e_fixtures/multi_page.pdf --pages 2,4 --output /app/tests/e2e_fixtures/out/tri_e2e/deleted_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.06 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/deleted_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.81 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_delete_pages -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "pages": "2,4", "output": "/app/tests/e2e_fixtures/out/tri_e2e/deleted_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.70 ms

---

### Tool: `pdf_reorder_pages`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli reorder --input tests/e2e_fixtures/multi_page.pdf --order 2,1,3,4,5 --output /app/tests/e2e_fixtures/out/tri_e2e/reordered_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.23 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/reordered_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.27 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_reorder_pages -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "order": "2,1,3,4,5", "output": "/app/tests/e2e_fixtures/out/tri_e2e/reordered_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.86 ms

---

### Tool: `pdf_rotate`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli rotate --input tests/e2e_fixtures/single_page.pdf --degrees 90 --pages 1 --output /app/tests/e2e_fixtures/out/tri_e2e/rotated_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.76 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/rotated_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.91 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_rotate -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "angle": 90, "pages": "1", "output": "/app/tests/e2e_fixtures/out/tri_e2e/rotated_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 2.08 ms

---

### Tool: `pdf_crop`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli crop --input tests/e2e_fixtures/single_page.pdf --rect 10,10,200,200 --output /app/tests/e2e_fixtures/out/tri_e2e/cropped_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.85 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/cropped_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.85 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_crop -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "box": "10,10,200,200", "output": "/app/tests/e2e_fixtures/out/tri_e2e/cropped_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.84 ms

---

### Tool: `pdf_burst`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli burst --input tests/e2e_fixtures/multi_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/burst_dir --json
```
- **Status:** ✅ PASS
- **Latency:** 8.04 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_burst",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output_dir": "/app/tests/e2e_fixtures/out/tri_e2e/burst_dir"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 8.31 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_burst -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "output_dir": "/app/tests/e2e_fixtures/out/tri_e2e/burst_dir"}'
```
- **Status:** ✅ PASS
- **Latency:** 2.35 ms

---

### Tool: `pdf_remove_blank`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli remove-blank --input tests/e2e_fixtures/multi_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/noblank_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.52 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_remove_blank",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/noblank_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.81 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_remove_blank -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/noblank_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.79 ms

---

### Tool: `pdf_compress`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli compress --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/compressed_cli.pdf --quality medium --json
```
- **Status:** ✅ PASS
- **Latency:** 7.80 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_compress",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/compressed_mcp.pdf",
      "quality": "medium"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 7.38 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/compress -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/compressed_api.pdf", "quality": "medium"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.83 ms

---

### Tool: `pdf_repair`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli repair --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/repaired_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.34 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_repair",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/repaired_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.55 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_repair -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/repaired_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.84 ms

---

### Tool: `pdf_linearize`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli linearize --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/linearized_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.66 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_linearize",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/linearized_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 7.11 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_linearize -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/linearized_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.72 ms

---

### Tool: `pdf_encrypt`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli encrypt --input tests/e2e_fixtures/single_page.pdf --user-password secret123 --owner-password secret123 --output /app/tests/e2e_fixtures/out/tri_e2e/encrypted_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.28 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/encrypted_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.92 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_encrypt -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "password": "secret123", "output": "/app/tests/e2e_fixtures/out/tri_e2e/encrypted_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.84 ms

---

### Tool: `pdf_decrypt`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli decrypt --input /app/tests/e2e_fixtures/out/tri_e2e/encrypted_cli.pdf --password secret123 --output /app/tests/e2e_fixtures/out/tri_e2e/decrypted_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 6.78 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_decrypt",
    "arguments": {
      "input": "/app/tests/e2e_fixtures/out/tri_e2e/encrypted_mcp.pdf",
      "password": "secret123",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/decrypted_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.06 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_decrypt -H 'Content-Type: application/json' -d '{"input": "/app/tests/e2e_fixtures/out/tri_e2e/encrypted_api.pdf", "password": "secret123", "output": "/app/tests/e2e_fixtures/out/tri_e2e/decrypted_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.77 ms

---

### Tool: `pdf_watermark`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli watermark --input tests/e2e_fixtures/single_page.pdf --text CONFIDENTIAL --output /app/tests/e2e_fixtures/out/tri_e2e/watermarked_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.36 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/watermarked_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.84 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/watermark -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "text": "CONFIDENTIAL", "output": "/app/tests/e2e_fixtures/out/tri_e2e/watermarked_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.59 ms

---

### Tool: `pdf_redact`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli redact --input tests/e2e_fixtures/single_page.pdf --pages 1 --rect 50,50,200,50 --output /app/tests/e2e_fixtures/out/tri_e2e/redacted_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.34 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/redacted_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.90 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_redact -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "page": 1, "x": 50, "y": 50, "width": 200, "height": 50, "output": "/app/tests/e2e_fixtures/out/tri_e2e/redacted_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.81 ms

---

### Tool: `pdf_metadata`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli metadata --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/metadata.json --json
```
- **Status:** ✅ PASS
- **Latency:** 7.14 ms

#### 🤖 MCP
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
- **Status:** ✅ PASS
- **Latency:** 6.14 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/info -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.45 ms

---

### Tool: `pdf_sign`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli signature --input tests/e2e_fixtures/single_page.pdf --cert tests/e2e_fixtures/out/tri_e2e/dummy.p12 --output /app/tests/e2e_fixtures/out/tri_e2e/signed_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 6.92 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/signed_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.54 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_sign -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "cert": "tests/e2e_fixtures/out/tri_e2e/dummy.p12", "output": "/app/tests/e2e_fixtures/out/tri_e2e/signed_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.84 ms

---

### Tool: `pdf_flatten`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli flatten --input tests/e2e_fixtures/form.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/flattened_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 6.79 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_flatten",
    "arguments": {
      "input": "tests/e2e_fixtures/form.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/flattened_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.45 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_flatten -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/form.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/flattened_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.56 ms

---

### Tool: `pdf_to_pdf_a`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli pdf-a --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/pdf_a_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.02 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_to_pdf_a",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/pdf_a_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.95 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_pdf_a -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/pdf_a_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.60 ms

---

### Tool: `pdf_header_footer`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli header-footer --input tests/e2e_fixtures/multi_page.pdf --text Confidential --output /app/tests/e2e_fixtures/out/tri_e2e/header_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.36 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/header_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.69 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_header_footer -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "header_left": "Confidential", "footer_center": "Page", "output": "/app/tests/e2e_fixtures/out/tri_e2e/header_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.77 ms

---

### Tool: `pdf_bates`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli bates --input tests/e2e_fixtures/multi_page.pdf --prefix CONF- --start 1 --output /app/tests/e2e_fixtures/out/tri_e2e/bates_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.33 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/bates_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.91 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_bates -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "prefix": "CONF-", "start_number": 1, "padding": 6, "output": "/app/tests/e2e_fixtures/out/tri_e2e/bates_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.86 ms

---

### Tool: `pdf_page_numbers`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli page-numbers --input tests/e2e_fixtures/multi_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/numbers_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.30 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/numbers_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.58 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_page_numbers -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/multi_page.pdf", "position": "bottom-right", "start_number": 1, "output": "/app/tests/e2e_fixtures/out/tri_e2e/numbers_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.70 ms

---

### Tool: `pdf_extract_text`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli extract-text --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/text.txt --json
```
- **Status:** ✅ PASS
- **Latency:** 7.03 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_text",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/text.txt"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.41 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/extract-text -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/text.txt"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.66 ms

---

### Tool: `pdf_extract_images`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli extract-images --input tests/e2e_fixtures/image_doc.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/extracted_images --json
```
- **Status:** ✅ PASS
- **Latency:** 7.09 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_images",
    "arguments": {
      "input": "tests/e2e_fixtures/image_doc.pdf",
      "output_dir": "/app/tests/e2e_fixtures/out/tri_e2e/extracted_images"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.29 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_extract_images -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/image_doc.pdf", "output_dir": "/app/tests/e2e_fixtures/out/tri_e2e/extracted_images"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.68 ms

---

### Tool: `pdf_search`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli search --input tests/e2e_fixtures/search_test.pdf --query test --json
```
- **Status:** ✅ PASS
- **Latency:** 7.25 ms

#### 🤖 MCP
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
- **Status:** ✅ PASS
- **Latency:** 6.01 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_search -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/search_test.pdf", "query": "test"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.45 ms

---

### Tool: `pdf_render`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli render --input tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/rendered_cli.png --json
```
- **Status:** ✅ PASS
- **Latency:** 7.04 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/rendered_mcp.png"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.83 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/render-page -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "page": 1, "output": "/app/tests/e2e_fixtures/out/tri_e2e/rendered_api.png"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.63 ms

---

### Tool: `pdf_compare`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli compare --input tests/e2e_fixtures/page_1.pdf --input-b tests/e2e_fixtures/page_2.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.05 ms

#### 🤖 MCP
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
- **Status:** ✅ PASS
- **Latency:** 6.46 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/compare -H 'Content-Type: application/json' -d '{"file1": "tests/e2e_fixtures/page_1.pdf", "file2": "tests/e2e_fixtures/page_2.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.76 ms

---

### Tool: `pdf_ocr`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli ocr --input tests/e2e_fixtures/image_doc.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/ocr_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.21 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_ocr",
    "arguments": {
      "input": "tests/e2e_fixtures/image_doc.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/ocr_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.66 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_ocr -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/image_doc.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/ocr_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.79 ms

---

### Tool: `pdf_bookmarks`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli bookmarks --input tests/e2e_fixtures/large_doc.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 6.93 ms

#### 🤖 MCP
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
- **Status:** ✅ PASS
- **Latency:** 6.90 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_bookmarks -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/large_doc.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.69 ms

---

### Tool: `pdf_images_to_pdf`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli images-to-pdf --images tests/e2e_fixtures/img1.png tests/e2e_fixtures/img2.png --output /app/tests/e2e_fixtures/out/tri_e2e/images_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 6.92 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/images_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.24 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_images_to_pdf -H 'Content-Type: application/json' -d '{"inputs": ["tests/e2e_fixtures/img1.png", "tests/e2e_fixtures/img2.png"], "output": "/app/tests/e2e_fixtures/out/tri_e2e/images_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.66 ms

---

### Tool: `pdf_annotate`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli annotate --input tests/e2e_fixtures/single_page.pdf --data [{"id": "1", "type": "highlight", "page": 1, "x": 50.0, "y": 50.0, "w": 50.0, "h": 50.0, "color": "#ffff00", "content": "Test"}] --output /app/tests/e2e_fixtures/out/tri_e2e/annotated.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.19 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/annotated.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 6.69 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_annotate -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "annotations": [{"id": "1", "type": "highlight", "page": 1, "x": 50.0, "y": 50.0, "w": 50.0, "h": 50.0, "color": "#ffff00", "content": "Test"}], "output": "/app/tests/e2e_fixtures/out/tri_e2e/annotated.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.81 ms

---

### Tool: `pdf_classify_type`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli classify --input tests/e2e_fixtures/single_page.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.38 ms

#### 🤖 MCP
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
- **Status:** ✅ PASS
- **Latency:** 6.44 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_classify_type -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.72 ms

---

### Tool: `pdf_validate`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli validate --input tests/e2e_fixtures/single_page.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.25 ms

#### 🤖 MCP
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
- **Status:** ✅ PASS
- **Latency:** 6.42 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_validate -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.37 ms

---

### Tool: `pdf_hash`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli hash --input tests/e2e_fixtures/single_page.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 6.35 ms

#### 🤖 MCP
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
- **Status:** ✅ PASS
- **Latency:** 6.25 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_hash -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.16 ms

---

### Tool: `pdf_read_form`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli form read tests/e2e_fixtures/form.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.07 ms

#### 🤖 MCP
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
- **Status:** ✅ PASS
- **Latency:** 6.13 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_read_form -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/form.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.55 ms

---

### Tool: `pdf_fill_form`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli form fill tests/e2e_fixtures/form.pdf --data tests/e2e_fixtures/form_data.json --output /app/tests/e2e_fixtures/out/tri_e2e/filled_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.27 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/filled_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 7.71 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_fill_form -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/form.pdf", "values": {"TestText": "Alice"}, "output": "/app/tests/e2e_fixtures/out/tri_e2e/filled_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 1.77 ms

---

### Tool: `pdf_create_form_field`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli form add-field tests/e2e_fixtures/single_page.pdf --name signature --type text --rect 50,50,150,30 --output /app/tests/e2e_fixtures/out/tri_e2e/added_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 7.28 ms

#### 🤖 MCP
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
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/added_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 7.49 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_create_form_field -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "field_name": "signature", "field_type": "text", "x": 50.0, "y": 50.0, "width": 100.0, "height": 30.0, "output": "/app/tests/e2e_fixtures/out/tri_e2e/added_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 2.04 ms

---

### Tool: `pdf_to_docx`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli convert --input tests/e2e_fixtures/single_page.pdf --format docx --output /app/tests/e2e_fixtures/out/tri_e2e/out_cli.docx --json
```
- **Status:** ✅ PASS
- **Latency:** 8.34 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_to_docx",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_mcp.docx"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 8.15 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_api.docx", "format": "docx"}'
```
- **Status:** ✅ PASS
- **Latency:** 2.41 ms

---

### Tool: `pdf_to_xlsx`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli convert --input tests/e2e_fixtures/single_page.pdf --format xlsx --output /app/tests/e2e_fixtures/out/tri_e2e/out_cli.xlsx --json
```
- **Status:** ✅ PASS
- **Latency:** 10.89 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_to_xlsx",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_mcp.xlsx"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 10.43 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_api.xlsx", "format": "xlsx"}'
```
- **Status:** ✅ PASS
- **Latency:** 3.85 ms

---

### Tool: `pdf_to_pptx`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli convert --input tests/e2e_fixtures/single_page.pdf --format pptx --output /app/tests/e2e_fixtures/out/tri_e2e/out_cli.pptx --json
```
- **Status:** ✅ PASS
- **Latency:** 8.49 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_to_pptx",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_mcp.pptx"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 8.06 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/convert -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_api.pptx", "format": "pptx"}'
```
- **Status:** ✅ PASS
- **Latency:** 2.23 ms

---

### Tool: `pdf_convert_html`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli convert --input tests/e2e_fixtures/test.html --format pdf --output /app/tests/e2e_fixtures/out/tri_e2e/out_html_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 2465.84 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_html",
    "arguments": {
      "input": "tests/e2e_fixtures/test.html",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_html_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 2249.62 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_convert_html -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/test.html", "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_html_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 2499.07 ms

---

### Tool: `pdf_convert_markdown`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli convert --input tests/e2e_fixtures/test.md --format pdf --output /app/tests/e2e_fixtures/out/tri_e2e/out_md_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 2645.13 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_markdown",
    "arguments": {
      "input": "tests/e2e_fixtures/test.md",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_md_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 2500.79 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_convert_markdown -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/test.md", "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_md_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 2118.93 ms

---

### Tool: `pdf_convert_excel`

#### 💻 CLI
```bash
/app/target/release/paperpilot-cli convert --input tests/e2e_fixtures/test.csv --format pdf --output /app/tests/e2e_fixtures/out/tri_e2e/out_csv_cli.pdf --json
```
- **Status:** ✅ PASS
- **Latency:** 2406.34 ms

#### 🤖 MCP
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_excel",
    "arguments": {
      "input": "tests/e2e_fixtures/test.csv",
      "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_csv_mcp.pdf"
    }
  }
}
```
- **Status:** ✅ PASS
- **Latency:** 2403.84 ms

#### 🌐 REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_convert_excel -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/test.csv", "output": "/app/tests/e2e_fixtures/out/tri_e2e/out_csv_api.pdf"}'
```
- **Status:** ✅ PASS
- **Latency:** 2131.24 ms

---
