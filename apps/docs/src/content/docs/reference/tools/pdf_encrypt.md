---
title: "pdf_encrypt"
description: "Reference, CLI, MCP, and REST API commands for pdf_encrypt."
sidebar:
  order: 21
---

**Status**: ✅ Verified  |  **Format**: PDF $\to$ PDF

### What it does
**Secures a PDF document with standard password encryption.**  
Protects document contents by establishing encryption keys with user opening passwords and optional owner administrative passwords to restrict printing, copying text, or editing.

- **Password Protection**: Prevents unauthorized access by requiring a password to open and view the document (`--user-password` / `--password`).
- **Administrative Rights**: Sets optional owner passwords (`--owner-password`) to separate document ownership and permission modifications from general viewing.
- **Standards Compliant**: Generates standard PDF encryption dictionaries adhering to Adobe / ISO PDF encryption specifications.

### CLI
```bash
# Basic password encryption using user password
paperpilot encrypt \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e_security/encrypt_cli_Simple.pdf \
  --user-password secret123 \
  --json

# Encrypt specifying both user password and owner administrative password
paperpilot encrypt \
  --input document.pdf \
  --output encrypted_doc.pdf \
  --user-password userpass \
  --owner-password adminpass \
  --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_encrypt",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "password": "secret123",
    "output": "tests/e2e_fixtures/out/penta_e2e/encrypted.pdf"
  }
}
```
*Note: MCP schema accepts `password` (required) and supports `user_password` / `owner_password`.*

### REST API
```bash
# Via dedicated REST endpoint
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_encrypt \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/encrypted.pdf", "user_password": "secret123"}'

# Via generic tools endpoint
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_encrypt \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/encrypted.pdf", "password": "secret123"}'
```

### WebAssembly (WASM)
In browser and client runtimes via `@paperpilot/wasm`:
```typescript
import { encrypt } from "@paperpilot/wasm";

const encryptedBytes = encrypt(inputPdfBytes, "secret123");
```

### Parameter Reference
| Option | CLI Flag | MCP Field | REST Field | Type | Default | Description |
|---|---|---|---|---|---|---|
| Input File | `--input` | `input` | `input` | `string` | *(required)* | Path to source unencrypted PDF file. |
| Output File | `--output` | `output` | `output` | `string` | *(required)* | Path where the encrypted PDF will be written. |
| User Password | `--user-password` (alias: `--password`) | `password` / `user_password` | `password` / `user_password` | `string` | *(required)* | Password required to unlock and read the document. |
| Owner Password | `--owner-password` | `owner_password` | `owner_password` | `string` | `null` | Administrative password managing document permissions. |

### Output Response
```json
{
  "success": true,
  "operation": "Encrypt",
  "error": null
}
```

### Troubleshooting & Known Limitations
- **Encrypted Source**: Running `pdf_encrypt` on a PDF that is already encrypted will return an error or overwrite encryption keys. Ensure the document is decrypted using `pdf_decrypt` prior to applying new encryption credentials.
- **Loopback Proxy**: In environments setting `http_proxy` / `HTTP_PROXY`, ensure loopback addresses bypass proxy (`--noproxy "*"`) when making REST calls to local gateway ports.
