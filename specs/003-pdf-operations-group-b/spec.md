# Specification: PDF Operations (Group B)

## Feature Description
Group B contains document-level operations such as encryption, compression, metadata management, and legal formatting. These operations will be implemented in the `paperpilot-pdf` crate, continuing to use the `PdfOperation` interface.

## Scope (Group B)
1. **Compress**: Optimize file size via image downsampling and stream compression.
2. **Repair**: Best-effort recovery of malformed PDFs.
3. **Metadata**: Read/Write PDF Info dictionary and XMP metadata.
4. **Encrypt/Decrypt**: Password protection and removal.
5. **Watermark**: Overlay text or image onto pages.
6. **E-Signature**: Digital signing via local crypto and DocuSign integration.
7. **Redact**: Permanently remove text and image data under redaction boxes.
8. **Linearize**: Fast Web View optimization.
9. **Flatten**: Bake interactive annotations/forms into static content.
10. **PDF/A**: Convert to archival formats (PDF/A-1b, PDF/A-2b).
11. **Header/Footer**: Add repeating text/page numbers to margins.
12. **Bates Numbering**: Legal document stamping (e.g. EXHIBIT-0001).

## Technical Constraints
- Continue leveraging `lopdf` where applicable (e.g., metadata, linearize, watermarks).
- Encryption/Decryption may require exploring specialized cryptographic crates if `lopdf` is insufficient.
- E-Signature will require `reqwest` for the DocuSign API, and `ring` or `rcgen` for local crypto.
