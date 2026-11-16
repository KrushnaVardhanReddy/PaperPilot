---
title: "pdf_change_permissions"
description: "Reference, CLI, MCP, and REST API commands for pdf_change_permissions."
sidebar:
  order: 54
---

**Status**: ✅ Verified  |  **Format**: PDF $\to$ PDF

### What it does
**Configures user and owner permissions for printing, copying, and modification.**  
Enables granular control over document permissions across PDF viewers according to ISO 32000-1 specification encryption dictionaries (`/P` bitmask flags).

- **Granular Permissions Control**: Revoke or allow printing (`allow_printing`), copying (`allow_copying`), content modifications (`allow_modifying`), annotations (`allow_annotations`), form filling (`allow_form_filling`), and accessibility screen reading (`allow_accessibility`).
- **Cryptographic Bitmask Management**: Dynamically recalculates the PDF permission integer flags (`/P`) and re-encrypts the document under standard 128-bit key schemes.
- **Credential Segregation**: Supports distinct user opening passwords and administrative owner passwords.

### CLI
```bash
# Revoke printing and text copying permissions
paperpilot pdf-change-permissions \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e_security/perm_restricted.pdf \
  --allow-print false \
  --allow-copy false \
  --json

# Change permissions with custom owner password
paperpilot pdf-change-permissions \
  --input document.pdf \
  --output secured.pdf \
  --allow-modify false \
  --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_change_permissions",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/perm_restricted.pdf",
    "allow_print": false,
    "allow_copy": false,
    "owner_password": "adminsecret"
  }
}
```

### REST API
```bash
# Via dedicated REST endpoint
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/change-permissions \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/perm_restricted.pdf", "allow_print": false, "allow_copy": false}'

# Via dedicated endpoint (tool alias)
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_change_permissions \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/perm_restricted.pdf", "allow_print": false, "allow_copy": false}'
```

### WebAssembly (WASM)
In browser and client runtimes via `@paperpilot/wasm`:
```typescript
import { pdf_change_permissions } from "@paperpilot/wasm";

const restrictedPdfBytes = pdf_change_permissions(
  inputPdfBytes,
  JSON.stringify({
    allow_printing: false,
    allow_copying: false,
    owner_password: "adminsecret"
  })
);
```

### Parameter Reference
| Option | CLI Flag | MCP Field | REST Field | Type | Default | Description |
|---|---|---|---|---|---|---|
| Input File | `--input` | `input` | `input` | `string` | *(required)* | Path to the source PDF document. |
| Output File | `--output` | `output` | `output` | `string` | *(required)* | Destination path for the updated PDF. |
| Allow Printing | `--allow-print` | `allow_print` / `allow_printing` | `allow_print` / `allow_printing` | `boolean` | `true` | Grant or revoke printing permissions. |
| Allow Copying | `--allow-copy` | `allow_copy` / `allow_copying` | `allow_copy` / `allow_copying` | `boolean` | `true` | Grant or revoke text extraction / clipboard copying permissions. |
| Allow Modifying | `--allow-modify` | `allow_modify` / `allow_modifying` | `allow_modify` / `allow_modifying` | `boolean` | `true` | Grant or revoke document editing / assembly permissions. |
| Allow Annotations | *(library)* | `allow_annotations` | `allow_annotations` | `boolean` | `true` | Grant or revoke adding and editing annotations. |
| Allow Form Filling | *(library)* | `allow_form_filling` | `allow_form_filling` | `boolean` | `true` | Grant or revoke interactive form filling. |
| Allow Accessibility | *(library)* | `allow_accessibility` | `allow_accessibility` | `boolean` | `true` | Grant or revoke text extraction for screen readers. |
| Owner Password | *(library)* | `owner_password` | `owner_password` | `string` | `null` | Configure owner administrative password. |
| User Password | *(library)* | `user_password` | `user_password` | `string` | `null` | Configure opening user password. |

### Output Response
```json
{
  "success": true,
  "operation": "PdfChangePermissions",
  "error": null
}
```

### Troubleshooting & Known Limitations
- **CLI Options Scoping**: The CLI flags expose `--allow-print`, `--allow-copy`, and `--allow-modify`. Granular permission flags (`allow_annotations`, `allow_form_filling`, `allow_accessibility`, passwords) can be configured via MCP JSON-RPC, REST API, or the WASM engine.
- **Viewer Enforcement**: PDF permission flags (`/P`) are enforced client-side by compliant PDF readers according to the ISO standard.
