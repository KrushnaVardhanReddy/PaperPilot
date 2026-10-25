# 111 - PDF Change Permissions

## Overview

The `pdf_change_permissions` operation is part of the PaperPilot PDF manipulation engine. It implements the ability to read, modify, and enforce user access permissions according to the ISO 32000-1 specification.

By modifying the integer bitmask of the `/P` value in the `/Encrypt` dictionary, PaperPilot can revoke or grant various permissions such as printing, copying, modifying, and more.

## Architecture

The operation resides in `paperpilot-pdf/src/operations/pdf_change_permissions.rs`.
- Input: Takes a byte array representing the PDF file and a `PdfChangePermissionsOptions` struct containing `allow_printing`, `allow_copying`, `allow_modifying`, `allow_annotations`, `allow_form_filling`, `allow_accessibility`, `owner_password`, `user_password`.
- Processing: It uses `lopdf`'s `EncryptionState` and `Document::encrypt` to securely modify permissions and recalculate the encryption keys.
- Output: Returns a `Result<Vec<u8>, PdfError>` containing the manipulated PDF bytes.

## Endpoints / CLI Integration

- **CLI**: `paperpilot pdf-change-permissions --input <file> --output <file> --allow-printing false --allow-copying true`
- **MCP**: `pdf_change_permissions` tool using JSON schema configuration mapping directly to the `PdfChangePermissionsOptions`.
- **Gateway**: Available at `/api/v1/pdf/change-permissions` as a `POST` route.
