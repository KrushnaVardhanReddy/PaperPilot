# Request Plan Review

The objective is to implement Stage 2 of the decoupled PDF-to-DOCX conversion pipeline in `paperpilot-pdf/src/operations/conversion.rs` as specified in Spec 077.

## Approach
1.  **Remove legacy extraction:** The current `PdfToDocxOperation::execute` extracts images and runs `unpdf::to_markdown`. I will remove the block handling `unpdf::to_markdown` and Form/Image scanning.
2.  **Use `PdfToMarkdownOperation`:** Invoke `PdfToMarkdownOperation::execute(document)` to retrieve the rich Markdown string containing inline Base64 data URIs from `extracted_markdown`.
3.  **Use `pulldown_cmark` to parse Markdown:**
    - Initialize `Parser::new_ext(&markdown, Options::all())`.
    - Maintain state variables like `pending_image: Option<(Vec<u8>, u32, u32)>` (for PNG bytes and EMU sizes), and layout states.
4.  **Handle Markdown AST events:**
    - `Event::Start(Tag::Image { dest_url, .. })`:
        - Decode data URIs (`data:image/...;base64,...`).
        - Use `image::load_from_memory` to get pixel dimensions.
        - Convert pixels to EMUs: 1 inch = 914,400 EMUs, at 150 DPI => scale is 6,096 EMUs/px.
        - Bound width to 4,500,000 EMUs max (maintaining aspect ratio).
        - Store in `pending_image`.
    - `Event::End(TagEnd::Image)`:
        - If `pending_image` exists, construct `docx_rs::Pic::new(bytes).size(...)` inside a `docx_rs::Run`.
        - Add it to a `Paragraph` and hold it as `pending_image_paragraph` (or similar).
    - `Event::Start(Tag::Heading)` and `Event::End(TagEnd::Heading)`:
        - Map heading levels (H1=44, H2=32, H3/H4=24) and set styling.
    - `Event::Text(text)`:
        - Append text with appropriate styling (e.g. 19pt, color #525C68 for body text).
    - **Card Layout Synthesis:**
        - Instead of relying on `card_index` and `is_form_array`, if a `pending_image` was recently added, and a heading/paragraph follows, combine them into a 2-column borderless `docx_rs::Table`.
        - Column 1: Image, Column 2: Text.
5.  **Remove `[ Feature {} Illustration ]`:** All instances of this placeholder text will be deleted.
6.  **Add `decode_data_uri` and constants:** Create constants for EMU conversions and a helper for data URI decoding as outlined in Spec 077.
7.  **Write Tests:** Write or update tests in `operations::conversion::tests` to ensure functionality.
8.  **Complete pre-commit steps.**
