# Spec 099: Digital Signature Removal Engine (`pdf_remove_signatures`)

## 1. Background & Executive Summary
Documents that have been digitally signed or certified become cryptographically locked; adding annotations, reorganizing pages, or appending text invalidates the cryptographic hash. When a document needs to be edited, re-signed, or sanitized before redrafting, existing `/Sig` widgets, `/ByteRange` records, and `/SigFlags` must be stripped cleanly without corrupting the document tree.

**Spec 099** implements `pdf_remove_signatures` (Task 5.3.13) across all 5 deployment surfaces, removing all digital signature form fields, associated widget annotations, and clearing the `/SigFlags` entry in `/AcroForm`.

---

## 2. Technical Architecture & Stripping Strategy

### 2.1 Object Tree Cleanup
1. **Traverse Catalog `/AcroForm`**:
   - Inspect `/Fields` array.
   - Remove fields where `/FT == /Sig` or the field value dictionary `/V` has `/Type == /Sig`.
   - Remove orphan signature field objects from `doc.objects`.
   - If no remaining signature fields exist, clear `/SigFlags` or remove the AcroForm if empty.
2. **Page Annotations (`/Annots`)**:
   - For each page, inspect `/Annots` array.
   - Filter out any annotation objects whose `/Subtype == /Widget` and whose parent or `/FT == /Sig`.
3. **Save Normalized Document**:
   - Save the cleaned document to bytes. The resulting PDF is an unsigned, fully editable document.

---

## 3. Data Model

In `paperpilot-pdf/src/operations/pdf_remove_signatures.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PdfRemoveSignaturesOptions {
    /// If true, also strips empty unsigned signature placeholder fields
    #[serde(default = "default_true")]
    pub remove_empty_signature_fields: bool,
}

fn default_true() -> bool {
    true
}
```

---

## 4. Verification & Testing
1. Take a signed synthetic PDF (or output from `pdf_sign`):
   - Run `pdf_remove_signatures`.
   - Run `pdf_validate_signatures` on the output $\rightarrow$ returns `total_signatures == 0`.
2. Running on an unsigned PDF leaves the document intact without error.
3. Empty document input returns error.
