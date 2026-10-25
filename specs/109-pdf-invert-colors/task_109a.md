# Task 109A: Implement Vector PDF Invert Colors & Dark Mode (`pdf_invert_colors`)

## Metadata
- **Spec**: [Spec 109: Invert PDF Colors & Dark Mode](file:///home/krushna/Project/PaperPilot/specs/109-pdf-invert-colors/spec.md)
- **Phase**: Phase 5.3 / Task 5.3.18
- **Primary Language**: Rust 2021
- **Impacted Crates**: `paperpilot-pdf`, `paperpilot-cli`, `paperpilot-mcp`, `paperpilot-gateway`

---

## Detailed Requirements

### 1. Core Implementation in `paperpilot-pdf`
Create `paperpilot-pdf/src/operations/pdf_invert_colors.rs`:
- Define `InvertMode`: `DarkMode`, `TrueInvert`, `HighContrast`, `Sepia`.
- Define `InvertColorsOptions` with `mode: Option<InvertMode>` and `pages: Option<String>`.
- Function:
  ```rust
  pub fn invert_pdf_colors(
      doc: &mut lopdf::Document,
      options: &InvertColorsOptions,
  ) -> Result<(), PaperPilotError>
  ```
- Inject page background rectangle according to `/MediaBox` at start of `/Contents`.
- Invert or adjust text and path color operations (`rg`, `RG`, `g`, `G`, `k`, `K`).
- Write comprehensive unit tests in `operations::pdf_invert_colors::tests`.

### 2. Export & Re-export
- Expose in `paperpilot-pdf/src/operations/mod.rs` and `paperpilot-pdf/src/lib.rs`.

### 3. CLI Integration in `paperpilot-cli`
- Add command `invert-colors` to `Commands` in `paperpilot-cli/src/cli.rs`.
- Wire handler in `paperpilot-cli/src/commands/mod.rs`.

### 4. MCP Server Integration in `paperpilot-mcp`
- Add `pdf_invert_colors` tool definition to `paperpilot-mcp/src/server.rs`.
- Wire execution handler matching tool parameters.

### 5. REST Gateway Integration in `paperpilot-gateway`
- Add handler `handlers::pdf_invert_colors` in `paperpilot-gateway/src/handlers.rs`.
- Route `POST /api/v1/pdf/invert-colors` in `paperpilot-gateway/src/server.rs`.

### 6. Verification
- `cargo check --workspace -j 2`
- `cargo test -p paperpilot-pdf --lib operations::pdf_invert_colors -j 2`
- `cargo test -p paperpilot-cli -j 2`
