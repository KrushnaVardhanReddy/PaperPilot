# Task 111A: Implement PDF Change Permissions (`pdf_change_permissions`)

## Metadata
- **Spec**: [Spec 111: Modify PDF Permissions & Rights Management (`pdf_change_permissions`)](file:///home/krushna/Project/PaperPilot/specs/111-pdf-change-permissions/spec.md)
- **Phase**: Phase 5.3 / Task 5.3.15 & 5.4.5
- **Primary Language**: Rust 2021
- **Impacted Crates**: `paperpilot-pdf`, `paperpilot-cli`, `paperpilot-mcp`, `paperpilot-gateway`

---

## Detailed Requirements

### 1. Core Logic (`paperpilot-pdf/src/operations/pdf_change_permissions.rs`)
- Expand `PdfChangePermissionsOptions`:
  - `pub allow_printing: Option<bool>`
  - `pub allow_copying: Option<bool>`
  - `pub allow_modifying: Option<bool>`
  - `pub allow_annotations: Option<bool>`
  - `pub allow_form_filling: Option<bool>`
  - `pub allow_accessibility: Option<bool>`
  - `pub owner_password: Option<String>`
  - `pub user_password: Option<String>`
- Implement `pdf_change_permissions_bytes(&[u8], &PdfChangePermissionsOptions) -> Result<Vec<u8>, PdfError>`:
  - Load document bytes with `lopdf::Document::load_mem`.
  - Manipulate the `/P` integer bitmask in the `/Encrypt` dictionary according to ISO 32000-1 Table 22.
  - Implement fallback handling for unencrypted documents adding a standard encryption dictionary with empty password or user password to enforce permissions.
  - Re-save document bytes.

### 2. Unit Tests
- `test_pdf_change_permissions_revoke_printing`: Verify `/P` bit 3 is toggled off.
- `test_pdf_change_permissions_allow_copying`: Verify bit 5 toggled on/off.
- `test_pdf_change_permissions_roundtrip`: Re-open output document and confirm `/Encrypt` `/P` integer reflects changed bits.

### 3. Interface Integration
- **CLI** (`paperpilot-cli/src/commands/pdf_change_permissions.rs`): Wire up `--allow-printing`, `--allow-copying`, etc.
- **MCP** (`paperpilot-mcp/src/server.rs`): Ensure schema properties for permissions match options.
- **Documentation**:
  - `wiki/111-Pdf-Change-Permissions.md`
  - `apps/docs/src/content/docs/reference/tools/pdf_change_permissions.md`

### 4. Verification
- `cargo test -p paperpilot-pdf --lib operations::pdf_change_permissions -j 2`
- `cargo check --workspace -j 2`
