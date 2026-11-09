# Master Instructions: 5 Tools Per Session Protocol for Tester 2 (Security, Forms, Legal Stamping & Annotations)

## Context & Objective
We are standardizing automated test execution and UI alignment for **Tester 2 (Tools 31–60)** in `qa/manual_desktop_testing/TESTER_2_SECURITY_FORMS.md`.

For each session:
1. **Fix any underlying UI/Desktop/ArgBuilder bug** in `apps/desktop/src/lib/components/layout/OperationsPanel.svelte` or `operationArgsBuilder.ts`.
2. **Use pre-made fixtures** from `tests/e2e_fixtures/real/`.
3. **Update Playwright Desktop E2E** in `apps/desktop/tests/e2e_suite_2_security_forms.spec.ts` with deep assertions.
4. **Update Rust E2E Harness** in `tests/e2e/test_wave2_penta_interface.rs` or `test_wave5_penta_interface.rs` with identical assertions.
5. **Compile & test with `-j 4`**:
   ```bash
   cargo check --workspace -j 4
   pnpm --filter @paperpilot/desktop test:e2e:suite2
   ```

---

## ⭐️ Phase 0: Prerequisite Fixture Generation (Run FIRST before any session)
Generate/verify the following fixtures in `tests/e2e_fixtures/real/`:
1. `encrypted.pdf`: Password-protected (`Secret123`) PDF document for `pdf_decrypt` testing.
2. `form.pdf` & `form_filled.pdf`: Standard AcroForm with text fields, checkboxes, and radio buttons.
3. `form_data.json` & `batch_data.csv`: JSON payload and multi-row CSV for form filling.
4. `dummy.p12`: Self-signed PKCS#12 test certificate with password `test` for digital signing.
5. `text_pii.pdf`: PDF containing fake SSNs, credit card numbers, and emails for PII detection and redaction.
6. `test.html` & `test.md`: Formatted HTML and Markdown source files for document synthesis.
7. `stationery.pdf`: 1-page Letter layout with background letterhead for overlay/underlay testing.

---

## 6 Tool-Execution Sessions (5 Tools Per Session)

### Session 1: Tools 31–35 (Decryption, Sanitization, Metadata, Permissions & Timestamps)
- **Tool 31 (`pdf_decrypt`)**: Decrypt `encrypted.pdf` with `Secret123`. Assert output `/Encrypt` dictionary is absent.
- **Tool 32 (`pdf_sanitize`)**: Deep sanitization on JavaScript/launch actions. Assert 0 active script dictionaries remain.
- **Tool 33 (`pdf_metadata`)**: Update title, author, subject, keywords. Assert persisted metadata dictionary values.
- **Tool 34 (`pdf_change_permissions`)**: Set user permissions to read-only (disable printing/copying). Assert `/P` permissions flag.
- **Tool 35 (`pdf_timestamp`)**: RFC 3161 TSA timestamping. Assert timestamp token structure present.

### Session 2: Tools 36–40 (Signatures, Hashing & Conformance)
- **Tool 36 (`pdf_sign`)**: Sign document using `dummy.p12`. Assert `/ByteRange` and signature `/Sig` dictionary.
- **Tool 37 (`pdf_validate_signatures`)**: Verify signatures on signed document. Assert valid signature status.
- **Tool 38 (`pdf_remove_signatures`)**: Strip digital signatures. Assert clean PDF without signature dictionary.
- **Tool 39 (`pdf_hash`)**: Compute SHA-256 and MD5 checksums. Assert exact hex string match.
- **Tool 40 (`pdf_validate`)**: Validate PDF syntax and ISO conformance. Assert report contains 0 critical syntax errors.

### Session 3: Tools 41–45 (PII Detection, Overlays, Stamps, Flattening & Form Reading)
- **Tool 41 (`document_pii_scan`)**: Scan `text_pii.pdf`. Assert SSN, credit card, and email detection findings.
- **Tool 42 (`pdf_overlay_underlay`)**: Underlay letterhead stationery beneath page 1. Assert combined page streams.
- **Tool 43 (`pdf_stamp`)**: Stamp visual approved badge at (x: 100, y: 100). Assert image stream stamped.
- **Tool 44 (`pdf_flatten`)**: Flatten form fields and annotations into static page content. Assert 0 interactive widget annotations remain.
- **Tool 45 (`pdf_read_form`)**: Extract field values from `form.pdf`. Assert JSON structure containing all field names.

### Session 4: Tools 46–50 (Form Filling, Batch CSV & Watermarking)
- **Tool 46 (`pdf_fill_form`)**: Populate `form.pdf` with `form_data.json`. Assert field values rendered.
- **Tool 47 (`pdf_create_form_field`)**: Create text field `ApplicantName` at specified rect. Assert widget added.
- **Tool 48 (`batch_fill`)**: Generate 3 filled documents from `batch_data.csv`. Assert 3 distinct output PDFs created.
- **Tool 49 (`cli_fill_batch`)**: Batch command line execution with receipt generation. Assert receipt report JSON created.
- **Tool 50 (`pdf_watermark`)**: Apply diagonal semi-transparent "CONFIDENTIAL" watermark. Assert opacity & rotation.

### Session 5: Tools 51–55 (Bates Stamping, Headers/Footers, Annotations & Redaction)
- **Tool 51 (`pdf_bates`)**: Stamp legal Bates numbers `CONF-000001` across pages. Assert sequential bates strings.
- **Tool 52 (`pdf_page_numbers`)**: Insert dynamic "Page X of Y" bottom-center. Assert page numbering rendered.
- **Tool 53 (`pdf_header_footer`)**: Add custom headers and footers to all pages. Assert text streams injected.
- **Tool 54 (`pdf_annotate`)**: Add highlight and sticky note annotations. Assert `/Annots` dictionary items.
- **Tool 55 (`pdf_redact`)**: Permanently redact target text rect. Assert underlying characters excised from content stream.

### Session 6: Tools 56–60 (Document Synthesis from JSON, Markdown & HTML)
- **Tool 56 (`json_to_pdf`)**: Synthesize PDF invoice from structured JSON. Assert formatted invoice layout.
- **Tool 57 (`md_to_pdf`)**: Render `test.md` into styled PDF. Assert heading and list styles.
- **Tool 58 (`html_to_pdf`)**: Convert `test.html` into paginated PDF. Assert CSS styling preserved.
- **Tool 59 (`pdf_convert_html`)**: Parse raw HTML snippet to PDF. Assert valid PDF output.
- **Tool 60 (`pdf_convert_markdown`)**: Parse markdown string to PDF. Assert valid PDF output.
