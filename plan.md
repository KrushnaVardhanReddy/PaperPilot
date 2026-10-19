1. **Modify `PdfToDocxOperation::execute` in `paperpilot-pdf/src/operations/conversion.rs`**:
   - Initialize `docx` with 0.5-inch margins.
   - Introduce `pdf_oxide` logic (similar to `PdfToHtmlOperation`) to extract semantic HTML from `lopdf_doc.original_bytes`.
   - Parse the generated HTML.
   - For `<h1>` and `<h3>`, create centered paragraphs with specific formatting (font size, bold, color).
   - For `<h4>` (Card Heading) + `<p>` (Card Description), handle feature cards by organizing them into alternating 2-column borderless tables:
     - Keep track of card count (`card_idx`) to alternate the side of the graphic.
     - Insert a placeholder illustration badge (`✦`) if a real raster image isn't available for the graphic cell.
   - Implement a fallback using `doc.extract_text()` if `original_bytes` is unavailable or parsing fails, ensuring the fallback closely matches the original behavior.

2. **Verify changes**:
   - Run `cargo check -p paperpilot-pdf -j 4`.
   - Run `cargo test -p paperpilot-pdf --lib test_pdf_to_docx -j 4`.
   - Use `paperpilot-cli` to convert `fixtures/Get_Started_With_Smallpdf.pdf` to docx and use Python `zipfile` script to verify `<w:tbl>` content as per the verification protocol.

3. **Complete pre-commit steps**:
   - Complete pre-commit steps to make sure proper testing, verifications, reviews, and reflections are done.

4. **Submit changes**.
