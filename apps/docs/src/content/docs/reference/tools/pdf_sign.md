---
title: "pdf_sign"
description: "Reference, CLI, MCP, and REST API commands for pdf_sign."
sidebar:
  order: 44
---

**Status**: ⚠️ Stub / Preview  |  **Format**: PDF $\to$ PDF

### What it does
**Applies digital signature metadata and AcroForm signature flags to a PDF document.**  
Configures `/AcroForm` signature dictionary structures and flags (`SigFlags = 3`) within the document catalog, reserving signature space and preparing the document for cryptographic workflow validation.

- **Signature Flag Configuration**: Updates catalog root dictionaries with ISO 32000-compliant `/AcroForm` and `/SigFlags` entries.
- **Workflow Preparation**: Prepares documents for downstream signature indexing and signature-field extraction workflows.
- **Certificate Compatibility**: Accepts certificate input paths (`--cert`) in CLI invocations for integration pipelines.

### CLI
```bash
# Apply signature field structure with certificate path
paperpilot signature \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --cert tests/e2e_fixtures/real/dummy.p12 \
  --output tests/e2e_fixtures/out/penta_e2e_security/sign_cli_Simple.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_sign",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/signed.pdf"
  }
}
```

### REST API
```bash
# Via generic tools endpoint
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_sign \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/signed.pdf"}'
```

### Parameter Reference
| Option | CLI Flag | MCP Field | REST Field | Type | Default | Description |
|---|---|---|---|---|---|---|
| Input File | `--input` | `input` | `input` | `string` | *(required)* | Path to the source PDF to sign. |
| Certificate | `--cert` | *(n/a)* | *(n/a)* | `string` | *(CLI required)* | Path to certificate file (`.p12`, `.pem`, `.pfx`). |
| Output File | `--output` | `output` | `output` | `string` | *(required)* | Destination path for the signed PDF output. |

### Output Response
```json
{
  "success": true,
  "operation": "Signature",
  "error": null
}
```

### Troubleshooting & Known Limitations
> [!NOTE] Implementation Architecture & PKI Stub Status
> - In PaperPilot version 0.7.0, `pdf_sign` operates as a structural signature placeholder: it attaches an AcroForm dictionary with `SigFlags: 3` (SignaturesExist and AppendOnly) to the document catalog.
> - Full asymmetric PKI cryptographic signing (RSA/ECDSA byte-range hashing and CMS/PKCS#7 detached signature injection) is in development. The `--cert` parameter is parsed and accepted in CLI invocations to guarantee forwards compatibility, but full cryptographic chain signing requires hardware tokens or external signing services.
- **WebAssembly**: `pdf_sign` is not currently exposed in `@paperpilot/wasm` due to native cryptographic signing dependencies.
