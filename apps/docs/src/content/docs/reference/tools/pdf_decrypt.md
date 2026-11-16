---
title: "pdf_decrypt"
description: "Reference, CLI, MCP, and REST API commands for pdf_decrypt."
sidebar:
  order: 19
---

**Status**: ✅ Verified  |  **Format**: PDF $\to$ PDF

### What it does
**Removes password protection and usage restrictions from an unlocked PDF.**  
Decrypts the document using the supplied user or owner password and outputs an unrestricted PDF that can be opened, printed, and edited freely.

- **Password Verification**: Validates the input password against the document's encryption dictionary before stripping security handlers.
- **Full Structure Preservation**: Parses the catalog, page tree, streams, annotations, and metadata intact before serializing into clean, decrypted objects.
- **Downstream Pipeline Ready**: Produces an unencrypted standard PDF suitable for subsequent automated tasks like indexing, compression (`pdf_compress`), or OCR.
- **Fail-Safe Rejection**: Rejects incorrect passwords with descriptive error codes rather than producing corrupt or incomplete files.

### CLI
```bash
# Decrypt an encrypted PDF using the document password
paperpilot decrypt \
  --input tests/e2e_fixtures/real/encrypted.pdf \
  --output tests/e2e_fixtures/out/penta_e2e_security/decrypt_cli_Simple.pdf \
  --password secret123 \
  --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_decrypt",
  "arguments": {
    "input": "tests/e2e_fixtures/real/encrypted.pdf",
    "password": "secret123",
    "output": "tests/e2e_fixtures/out/penta_e2e/decrypted.pdf"
  }
}
```

### REST API
```bash
# Via dedicated REST route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_decrypt \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/encrypted.pdf", "password": "secret123", "output": "tests/e2e_fixtures/out/penta_e2e/decrypted.pdf"}'

# Via generic tools endpoint
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_decrypt \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/encrypted.pdf", "password": "secret123", "output": "tests/e2e_fixtures/out/penta_e2e/decrypted.pdf"}'
```

### WebAssembly (WASM)
In browser and client runtimes via `@paperpilot/wasm`:
```typescript
import { decrypt } from "@paperpilot/wasm";

const decryptedBytes = decrypt(encryptedPdfBytes, "secret123");
```

### Parameter Reference
| Option | CLI Flag | MCP Field | REST Field | Type | Default | Description |
|---|---|---|---|---|---|---|
| Input File | `--input` | `input` | `input` | `string` | *(required)* | Path to the password-protected input PDF file. |
| Password | `--password` | `password` | `password` | `string` | *(required)* | Password (user or owner) required to unlock the document. |
| Output File | `--output` | `output` | `output` | `string` | *(required)* | Destination path for the decrypted, unrestricted PDF. |

### Output Response
```json
{
  "success": true,
  "operation": "Decrypt",
  "error": null
}
```

### Troubleshooting & Known Limitations
- **Invalid Password**: Providing an incorrect password fails execution with:
  ```json
  {
    "success": false,
    "operation": "Decrypt",
    "error": "Unsupported operation: Decryption failed or invalid password: invalid password for encrypted PDF"
  }
  ```
- **Unencrypted Files**: Passing an already unencrypted PDF returns a success status or saves the file without encryption modifications.
