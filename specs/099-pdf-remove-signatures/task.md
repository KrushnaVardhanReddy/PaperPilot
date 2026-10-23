# Task 5.3.13: Digital Signature Removal Engine (`pdf_remove_signatures`)

## Status
- Spec: [Spec 099](file:///home/krushna/Project/PaperPilot/specs/099-pdf-remove-signatures/spec.md)
- Target: Wave 2 (`v0.7.1`) Core Operations Sprint
- Status: ⏳ Ready for Jules Submission

---

## Objectives & Work Items

### 1. Scaffolding
- [ ] Run `cargo run -j 2 -p penta-scaffold -- --name pdf_remove_signatures --title "Remove Digital Signatures" --category security --icon "🔓" --input-type pdf --output-type pdf --description "Strip digital signature fields, cryptographic locks, and widget annotations."`
- [ ] Verify clean compilation across all 5 surfaces.

### 2. Core Removal Logic (`paperpilot-pdf/src/operations/pdf_remove_signatures.rs`)
- [ ] Implement `PdfRemoveSignaturesOptions`.
- [ ] Traverse `/Root` $\rightarrow$ `/AcroForm` $\rightarrow$ `/Fields` and remove `/FT == /Sig` references.
- [ ] Traverse `/Annots` on all pages and remove corresponding signature widget annotations.
- [ ] Clear `/SigFlags` if no signatures remain.
- [ ] Unit tests:
  - `test_remove_signatures_from_signed_pdf`: Removes synthetic signature and verifies `/SigFlags` and signature widgets are gone.
  - `test_remove_signatures_from_unsigned_pdf`: Leaves unsigned PDF unharmed.
  - `test_remove_signatures_empty_bytes`: Gracefully returns error on empty input.

### 3. Verification & Conformance
- [ ] `cargo test -p paperpilot-pdf --lib operations::pdf_remove_signatures -j 2`
- [ ] `cargo run -j 2 -p penta-interface-e2e -- --group pdf_remove_signatures`
- [ ] `cargo check --workspace -j 2`
