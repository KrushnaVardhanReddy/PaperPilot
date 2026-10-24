# PDF Remove Signatures Architecture

The `pdf_remove_signatures` operation provides a robust mechanism to strip digital signatures and their associated visual widgets from PDF documents using the pure-Rust `lopdf` engine.

## Core Mechanism

The engine processes PDFs via an in-memory byte pipeline (`pdf_remove_signatures_bytes`), making it fully compatible with Penta-Interface constraints (CLI, MCP, Gateway, WASM, Cloudflare Edge).

The removal process operates in three distinct phases:

1. **AcroForm Inspection:**
   - The engine iterates through the `/Root` -> `/AcroForm` -> `/Fields` array.
   - It identifies signature fields either directly via `/FT == /Sig` or indirectly by inspecting the referenced value dictionary `/V` for `/Type == /Sig`.
   - Identified signature fields are removed from the `/Fields` array.
   - If `remove_empty_signature_fields` (default: true) is enabled and the resulting `/Fields` array becomes empty, `/AcroForm` is appropriately cleaned or pruned from the Document Catalog.

2. **Page Annotation Sweeping:**
   - The engine sweeps all document pages via `doc.get_pages()`.
   - For each page, the `/Annots` array is inspected.
   - Any widget annotations (`/Subtype == /Widget`) mapping to the previously identified signature fields or explicitly marked as signatures (`/FT == /Sig`) are stripped.
   - Empty `/Annots` arrays are safely purged from the page dictionary.

3. **Object Pruning:**
   - The engine collects the internal `ObjectId`s of all deleted signature fields, value dictionaries, and annotation widgets.
   - These dormant objects are completely wiped from `doc.objects`, preventing residual data leakage.
   - Finally, the document is serialized and returned.

## Integration

The tool adheres strictly to the single-core runner pattern:
- The core logic resides exclusively in `paperpilot-pdf/src/operations/pdf_remove_signatures.rs`.
- Standard interfaces execute this logic seamlessly, without duplication.
- Strong typed errors (`PdfError::IoError`, `PdfError::ParseError`) guarantee crash-free cross-platform execution.
