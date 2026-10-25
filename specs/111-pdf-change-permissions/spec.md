# Spec 111: Modify PDF Permissions & Rights Management (`pdf_change_permissions`)

## 1. Overview
Enables modifying standard PDF security permission flags without requiring full document re-encryption or altering passwords. In the PDF specification (ISO 32000-1 § 7.6.3.2), user and owner permissions are governed by an integer bitmask (`/P`) in the document's `/Encrypt` dictionary.

This operation allows users and automation pipelines to:
- Grant or revoke printing permissions (disabled, low-resolution, high-resolution).
- Grant or revoke content copying (clipboard text/graphics, accessibility extract).
- Grant or revoke modification rights (form field filling, annotations, page assembly).

---

## 2. Technical Architecture

### 2.1 ISO 32000-1 Permission Bitmask Flags
The `/P` 32-bit signed integer defines permission bits (bits 1-indexed according to Table 22):
- **Bit 3**: Print (1 = allow printing).
- **Bit 4**: Modify contents (1 = allow modifying contents other than annotations/forms).
- **Bit 5**: Copy (1 = allow copying or extracting text and graphics).
- **Bit 6**: Annotations / Forms (1 = allow adding or modifying text annotations and interactive form fields).
- **Bit 9**: Fill Forms (1 = allow filling existing interactive form fields).
- **Bit 10**: Accessibility Extract (1 = allow extracting text/graphics for accessibility purposes).
- **Bit 11**: Assemble Document (1 = allow assemble document: insert, rotate, delete pages).
- **Bit 12**: High-Quality Print (1 = allow high-resolution printing; Bit 3 must also be 1).

### 2.2 Parameters (`PdfChangePermissionsOptions`)
```rust
pub struct PdfChangePermissionsOptions {
    pub allow_printing: Option<bool>,
    pub allow_copying: Option<bool>,
    pub allow_modifying: Option<bool>,
    pub allow_annotations: Option<bool>,
    pub allow_form_filling: Option<bool>,
    pub allow_accessibility: Option<bool>,
    pub owner_password: Option<String>,
    pub user_password: Option<String>,
}
```

### 2.3 Operation Semantics
1. Parse the input PDF bytes via `lopdf::Document::load_mem`.
2. If document is encrypted:
   - Check if owner or user password is required/supplied.
   - Read existing `/Encrypt` dictionary and its `/P` value.
   - Apply bitwise operations to toggle the requested bits.
   - Update `/P` in the encryption dictionary.
3. If document is unencrypted:
   - If permissions are specified, generate standard Rev 2/3 `/Encrypt` dictionary with default empty/specified password to enforce rights, or warn/return modified document.
4. Serialize and return the modified PDF bytes.

---

## 3. Penta-Interface Parity
- **Core Engine**: `paperpilot-pdf/src/operations/pdf_change_permissions.rs`
- **CLI**: `paperpilot pdf-change-permissions --input in.pdf --output out.pdf --allow-printing=false --allow-copying=true`
- **MCP Server**: `pdf_change_permissions` with full boolean flags
- **REST Gateway**: `POST /api/v1/pdf/change-permissions`
- **Docs & Wiki**: `wiki/111-Pdf-Change-Permissions.md`, `apps/docs/src/content/docs/reference/tools/pdf_change_permissions.md`
