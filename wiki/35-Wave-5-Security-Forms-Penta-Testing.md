# Wave 5: Security, Signatures & Forms Penta-Interface Testing

This document contains copy-pasteable commands to manually test the Wave 5 tools across CLI, MCP, and REST API interfaces.

## 1. `pdf_watermark`
**CLI**:
```bash
cargo run --bin paperpilot -- watermark --input fixtures/Get_Started_With_Smallpdf.pdf --output out.pdf --text "CONFIDENTIAL" --json
```
**MCP**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_watermark",
    "arguments": {
      "input": "fixtures/Get_Started_With_Smallpdf.pdf",
      "output": "out.pdf",
      "text": "CONFIDENTIAL"
    }
  }
}
```
**API**:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_watermark \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.pdf", "text": "CONFIDENTIAL"}'
```

## 2. `pdf_encrypt`
**CLI**:
```bash
cargo run --bin paperpilot -- encrypt --input fixtures/Get_Started_With_Smallpdf.pdf --output out.pdf --user-password "pass" --owner-password "pass" --json
```
**MCP**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_encrypt",
    "arguments": {
      "input": "fixtures/Get_Started_With_Smallpdf.pdf",
      "output": "out.pdf",
      "user_password": "pass",
      "owner_password": "pass"
    }
  }
}
```
**API**:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_encrypt \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.pdf", "user_password": "pass", "owner_password": "pass"}'
```

## 3. `pdf_decrypt`
**CLI**:
```bash
cargo run --bin paperpilot -- decrypt --input tests/e2e_fixtures/real/encrypted.pdf --output out.pdf --password "password" --json
```
**MCP**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_decrypt",
    "arguments": {
      "input": "tests/e2e_fixtures/real/encrypted.pdf",
      "output": "out.pdf",
      "password": "password"
    }
  }
}
```
**API**:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_decrypt \
  -H "Content-Type: application/json" \
  -d '{"input": "tests/e2e_fixtures/real/encrypted.pdf", "output": "out.pdf", "password": "password"}'
```

## 4. `pdf_sanitize`
**CLI**:
```bash
cargo run --bin paperpilot -- sanitize --input fixtures/Get_Started_With_Smallpdf.pdf --output out.pdf --json
```
**MCP**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_sanitize",
    "arguments": {
      "input": "fixtures/Get_Started_With_Smallpdf.pdf",
      "output": "out.pdf"
    }
  }
}
```
**API**:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_sanitize \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.pdf"}'
```

## 5. `pdf_redact`
**CLI**:
```bash
cargo run --bin paperpilot -- redact --input fixtures/Get_Started_With_Smallpdf.pdf --output out.pdf --query "Smallpdf" --json
```
**MCP**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_redact",
    "arguments": {
      "input": "fixtures/Get_Started_With_Smallpdf.pdf",
      "output": "out.pdf",
      "query": "Smallpdf"
    }
  }
}
```
**API**:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_redact \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.pdf", "query": "Smallpdf"}'
```

## 6. `pdf_read_form`
**CLI**:
```bash
cargo run --bin paperpilot -- read-form --input tests/e2e_fixtures/real/form.pdf --json
```
**MCP**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_read_form",
    "arguments": {
      "input": "tests/e2e_fixtures/real/form.pdf"
    }
  }
}
```
**API**:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_read_form \
  -H "Content-Type: application/json" \
  -d '{"input": "tests/e2e_fixtures/real/form.pdf"}'
```

## 7. `pdf_fill_form`
**CLI**:
```bash
cargo run --bin paperpilot -- fill-form --input tests/e2e_fixtures/real/form.pdf --output out.pdf --data '{"Name": "John Doe"}' --json
```
**MCP**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_fill_form",
    "arguments": {
      "input": "tests/e2e_fixtures/real/form.pdf",
      "output": "out.pdf",
      "data": {"Name": "John Doe"}
    }
  }
}
```
**API**:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_fill_form \
  -H "Content-Type: application/json" \
  -d '{"input": "tests/e2e_fixtures/real/form.pdf", "output": "out.pdf", "data": {"Name": "John Doe"}}'
```

## 8. `pdf_flatten`
**CLI**:
```bash
cargo run --bin paperpilot -- flatten --input tests/e2e_fixtures/real/form.pdf --output out.pdf --json
```
**MCP**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_flatten",
    "arguments": {
      "input": "tests/e2e_fixtures/real/form.pdf",
      "output": "out.pdf"
    }
  }
}
```
**API**:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_flatten \
  -H "Content-Type: application/json" \
  -d '{"input": "tests/e2e_fixtures/real/form.pdf", "output": "out.pdf"}'
```

## 9. `pdf_stamp`
**CLI**:
```bash
cargo run --bin paperpilot -- stamp --input tests/e2e_fixtures/real/single_page.pdf --output out.pdf --image tests/e2e_fixtures/real/img1.png --x 100.0 --y 100.0 --json
```
**MCP**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_stamp",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "out.pdf",
      "image": "tests/e2e_fixtures/real/img1.png",
      "x": 100.0,
      "y": 100.0
    }
  }
}
```
**API**:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_stamp \
  -H "Content-Type: application/json" \
  -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "out.pdf", "image": "tests/e2e_fixtures/real/img1.png", "x": 100.0, "y": 100.0}'
```

## 10. `pdf_validate_signatures`
**CLI**:
```bash
cargo run --bin paperpilot -- validate-signatures --input fixtures/Get_Started_With_Smallpdf.pdf --json
```
**MCP**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_validate_signatures",
    "arguments": {
      "input": "fixtures/Get_Started_With_Smallpdf.pdf"
    }
  }
}
```
**API**:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_validate_signatures \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf"}'
```

## 11. `pdf_remove_signatures`
**CLI**:
```bash
cargo run --bin paperpilot -- remove-signatures --input fixtures/Get_Started_With_Smallpdf.pdf --output out.pdf --json
```
**MCP**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_remove_signatures",
    "arguments": {
      "input": "fixtures/Get_Started_With_Smallpdf.pdf",
      "output": "out.pdf"
    }
  }
}
```
**API**:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_remove_signatures \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.pdf"}'
```

## 12. `pdf_change_permissions`
**CLI**:
```bash
cargo run --bin paperpilot -- change-permissions --input fixtures/Get_Started_With_Smallpdf.pdf --output out.pdf --allow-print true --json
```
**MCP**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_change_permissions",
    "arguments": {
      "input": "fixtures/Get_Started_With_Smallpdf.pdf",
      "output": "out.pdf",
      "allow_print": true
    }
  }
}
```
**API**:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_change_permissions \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.pdf", "allow_print": true}'
```

## 13. `pdf_create_form_field`
**CLI**:
```bash
cargo run --bin paperpilot -- create-form-field --input tests/e2e_fixtures/real/single_page.pdf --output out.pdf --name "Signature" --type "text" --json
```
**MCP**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_create_form_field",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "out.pdf",
      "name": "Signature",
      "type": "text"
    }
  }
}
```
**API**:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_create_form_field \
  -H "Content-Type: application/json" \
  -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "out.pdf", "name": "Signature", "type": "text"}'
```

## 14. `pdf_timestamp`
**CLI**:
```bash
cargo run --bin paperpilot -- timestamp --input fixtures/Get_Started_With_Smallpdf.pdf --output out.pdf --server-url "http://test.tsa" --json
```
**MCP**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_timestamp",
    "arguments": {
      "input": "fixtures/Get_Started_With_Smallpdf.pdf",
      "output": "out.pdf",
      "server_url": "http://test.tsa"
    }
  }
}
```
**API**:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_timestamp \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.pdf", "server_url": "http://test.tsa"}'
```

## 15. `pdf_bates`
**CLI**:
```bash
cargo run --bin paperpilot -- bates --input fixtures/Get_Started_With_Smallpdf.pdf --output out.pdf --prefix "EXH-" --json
```
**MCP**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "pdf_bates",
    "arguments": {
      "input": "fixtures/Get_Started_With_Smallpdf.pdf",
      "output": "out.pdf",
      "prefix": "EXH-"
    }
  }
}
```
**API**:
```bash
curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_bates \
  -H "Content-Type: application/json" \
  -d '{"input": "fixtures/Get_Started_With_Smallpdf.pdf", "output": "out.pdf", "prefix": "EXH-"}'
```
