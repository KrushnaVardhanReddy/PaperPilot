# Task 5.3.3: Page Dimensional Scaling Engine (`pdf_scale_pages`)

## Status
- Spec: [Spec 098](file:///home/krushna/Project/PaperPilot/specs/098-pdf-scale-pages/spec.md)
- Target: Wave 2 (`v0.7.1`) Core Operations Sprint
- Status: ⏳ Ready for Jules Submission

---

## Objectives & Work Items

### 1. Scaffolding
- [ ] Run `cargo run -j 2 -p penta-scaffold -- --name pdf_scale_pages --title "Scale Page Dimensions" --category organize --icon "📐" --input-type pdf --output-type pdf --description "Scale page dimensions between standard formats (A4, Letter, Legal) or arbitrary percentages."`
- [ ] Verify clean compilation across all 5 surfaces.

### 2. Core Scaling Algorithm (`paperpilot-pdf/src/operations/pdf_scale_pages.rs`)
- [ ] Implement `PdfScalePagesOptions` with paper format dimensions (A4, Letter, Legal, A3, Tabloid).
- [ ] Extract page `/MediaBox`, calculate scale ratios $(s_x, s_y)$ and centering translation $(dx, dy)$.
- [ ] Update `/MediaBox` and wrap `/Contents` streams with transformation matrix `cm`.
- [ ] Unit tests:
  - `test_scale_to_a4`: Letter input $\rightarrow$ output `/MediaBox` matches A4.
  - `test_scale_factor_half`: 50% scale factor $\rightarrow$ page content scaled.
  - `test_scale_empty_document`: graceful error on empty bytes.

### 3. Verification & Conformance
- [ ] `cargo test -p paperpilot-pdf --lib operations::pdf_scale_pages -j 2`
- [ ] `cargo run -j 2 -p penta-interface-e2e -- --group pdf_scale_pages`
- [ ] `cargo check --workspace -j 2`
