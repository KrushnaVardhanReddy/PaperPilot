# PaperPilot: Real Assertions for All 44 Tools

## Global rules (apply to every tool, every interface)

1. **Output opens:** file parses with an independent library (pypdf / pdfium / qpdf --check), not your own engine.
2. **Output differs from input** (hash not equal) unless the tool is a no-op by design (hash, validate, search).
3. **Input file is not modified** (hash before == hash after).
4. **Parity:** compare *normalized content*, not bytes: page count, extracted text per page, page sizes, metadata. Byte-size differences alone are not failures, but content differences are.
5. **Error paths:** missing file, corrupt file, bad arguments, and wrong password each return a clear error, not a crash or a silent PASS.
6. **Tests use realistic fixtures** (see list at bottom), not 500-byte files.

---

## A. Page operations

| Tool | Fixture | Must assert |
|---|---|---|
| pdf_merge | page_1 (text "AAA"), page_2 (text "BBB") | Page count = 2; page 1 text contains "AAA", page 2 contains "BBB"; order preserved; merging 3+ files works; merging a file with itself works |
| pdf_split | multi_page (5 pages, each with unique text "P1".."P5") | With `1,2`: exactly the requested pages are produced, not all 5; each output has correct text; with `each` produces 5 files; invalid range (`9`) returns error |
| pdf_extract_pages | multi_page | Output has 2 pages; text is "P1" then "P3"; original untouched; out-of-range page errors |
| pdf_delete_pages | multi_page | Output has 3 pages; text is "P1","P3","P5"; deleting all pages returns error; deleting nonexistent page errors |
| pdf_reorder_pages | multi_page | Page count = 5; text order is P2,P1,P3,P4,P5; order list with duplicates or missing pages returns error |
| pdf_rotate | single_page | Page `/Rotate` = 90 for target page; other pages unchanged; rotating twice by 90 gives 180; invalid angle (45) errors; text still extractable |
| pdf_crop | single_page (612x792) | New CropBox/MediaBox equals requested rect; page count unchanged; rect outside page errors or clamps (document which) |
| pdf_burst | multi_page | Exactly 5 files; each has 1 page; file N has text "PN"; filenames sort in page order |
| pdf_remove_blank | doc with pages: text, blank, text, near-blank (scan noise), text | Blank page removed; near-blank handled per threshold; all text pages kept in order; doc with no blanks returns identical page count; all-blank doc returns clear error. **CLI and MCP must give the same result.** |

## B. Optimization and repair

| Tool | Fixture | Must assert |
|---|---|---|
| pdf_compress | image-heavy PDF (5+ MB) | Output size < input size (e.g. at least 20% at `medium`); low < medium < high quality in size order; page count and text unchanged; already-small file does not grow by more than a small margin |
| pdf_repair | PDF with truncated xref table / missing `endobj` (damaged on purpose) | Original fails `qpdf --check`; repaired file passes; page count and text recovered; valid input returns unchanged content; unrecoverable file returns error |
| pdf_linearize | multi_page | Output contains `/Linearized` in first object; `qpdf --check-linearization` passes; page content identical to input |

## C. Security

| Tool | Fixture | Must assert |
|---|---|---|
| pdf_encrypt | single_page | Opening without password fails; opening with wrong password fails; opening with correct password works; `/Encrypt` present; algorithm is AES-256 (or document it); user vs owner password permissions behave differently |
| pdf_decrypt | encrypted file from above | Correct password gives a PDF with no `/Encrypt`; page count and text **identical to the original**; wrong password returns error; size is not drastically smaller than the original |
| pdf_redact | PDF with text "SECRET 12345" at known coordinates | After redaction, extracted text **does not contain** "SECRET" or "12345" anywhere (also check raw content streams and hidden layers); other text remains; visible black box present at rect; copy-paste and search cannot find the string; works on all 5 interfaces, not via crop |
| pdf_sign | single_page + test .p12 | Output contains `/Sig` with `/ByteRange`; signature validates with an external verifier (pdfsig / Adobe); modifying a byte after signing breaks validation; wrong cert password errors; signer name matches cert |
| pdf_hash | single_page | Same file gives same hash twice; one-byte change gives different hash; hash matches `sha256sum` of the file; algorithm is stated in the output |

## D. Stamping and numbering

| Tool | Fixture | Must assert |
|---|---|---|
| pdf_watermark | multi_page | Extracted text on **every** page contains "CONFIDENTIAL"; original text still present; page count unchanged; opacity/rotation options respected; empty text errors |
| pdf_header_footer | multi_page | Header text appears on every page at top region, footer at bottom region (check text coordinates); original text not overlapped or lost; page count unchanged |
| pdf_bates | multi_page | Page 1 contains `CONF-000001`, page 5 contains `CONF-000005`; padding respected; `start` offset works (start 100 gives 000100); numbers sequential with no gaps; **output differs from header_footer output** |
| pdf_page_numbers | multi_page | Each page shows its number in the requested corner (check coordinates); `start_number` works; format `{page}/{total}` gives "3/5" on page 3; CLI and MCP outputs have the same text |
| pdf_annotate | single_page | Output contains `/Annots` with a `/Highlight` at the given coordinates; color matches; content/comment text present; annotation count equals input count; re-reading with pypdf returns the annotation |

## E. Forms

| Tool | Fixture | Must assert |
|---|---|---|
| pdf_read_form | form.pdf (text, checkbox, dropdown, radio) | Returns every field with correct name, type, and current value; PDF with no form returns empty list (not error); field count matches an independent reader |
| pdf_fill_form | form.pdf | After fill, `read_form` returns the new values; unknown field name returns error or warning (document which); checkbox/dropdown values accepted; unicode ("Zoë", Hindi text) renders; other fields unchanged; **CLI and MCP produce identical field values** |
| pdf_flatten | filled form.pdf | After flatten, `read_form` returns zero fields; filled values are still visible as page text; page count unchanged; flattening a non-form PDF returns the same content |
| pdf_create_form_field | single_page | New field appears in `read_form` with correct name, type, and rect; field is fillable by `pdf_fill_form`; duplicate name errors; each type (text, checkbox, dropdown) works |

## F. Extraction and analysis

| Tool | Fixture | Must assert |
|---|---|---|
| pdf_extract_text | real 3-page invoice with known strings | Output contains the known strings in reading order; multi-column and table pages are not scrambled; unicode preserved; scanned PDF (no text layer) returns empty or a clear "no text layer" message; byte size is realistic |
| pdf_extract_images | PDF with 3 known embedded images | Exactly 3 files; each decodes as valid PNG/JPEG; dimensions match the originals; PDF with no images returns an empty directory, not an error |
| pdf_search | search_test.pdf with "test" on pages 1 and 3 | Returns exactly hits on pages 1 and 3 with correct counts; case-insensitive option works; no-match returns an empty list; hit coordinates fall inside the page; multi-word query works |
| pdf_render | single_page, multi_page | PNG decodes; dimensions equal page size x scale (612x792 at 1.0); file size > 5 KB for a text page; not a single solid color (blank-image check); page 3 renders differently from page 1; invalid page errors |
| pdf_compare | page_1 vs page_2, and a file vs itself | Same file returns "no differences"; different files list the differing pages and text; swapping inputs gives mirrored output; page-count differences reported |
| pdf_metadata | PDF with known Title/Author/Dates | Returns exactly the known Title, Author, creation date, page count; PDF with no metadata returns empty fields; **the tool reads metadata and never writes** (WASM must not call `set_metadata`) |
| pdf_bookmarks | large_doc with known outline (3 levels) | Returns the correct titles, hierarchy, and target pages; PDF with no outline returns an empty list |
| pdf_classify_type | text PDF, scanned PDF, form PDF, image-only PDF | Text PDF classified "text"; scanned classified "scanned/image"; form classified "form"; result includes a confidence or reason |
| pdf_validate | valid PDF, corrupt PDF, PDF/A file | Valid file returns valid; corrupt file returns invalid **with specific error messages**; PDF/A claim matches veraPDF result |
| pdf_ocr | scanned PDF of a known page (e.g., a photographed invoice with "Invoice 4821, Total $1,250.00") | Output has a text layer; `extract_text` on output contains "4821" and "$1,250.00"; word accuracy >= 95% on a clean scan; page count unchanged; visual appearance unchanged; already-searchable PDF skipped or handled; `lang` option works; output size larger than input text-only size |

## G. Conversion

| Tool | Fixture | Must assert |
|---|---|---|
| pdf_images_to_pdf | img1.png (800x600), img2.png | Page count = 2; page order matches input order; each page contains one image with the correct aspect ratio; unsupported file returns error; JPEG, PNG both work |
| pdf_to_pdf_a | single_page with non-embedded font | **veraPDF reports compliant** for the claimed level (1b/2b); fonts embedded; no `/Encrypt`; XMP metadata contains PDF/A identification; text content unchanged; encrypted input errors or asks for password |
| pdf_to_docx | text PDF with heading, paragraph, table | Opens in python-docx/LibreOffice; contains the same text; table converted to a docx table; page count within reason; 18 KB file for a one-liner is OK but verify the text |
| pdf_to_xlsx | PDF with a 4x5 table of known numbers | Opens in openpyxl; cell values match the table exactly (numbers stay numeric, not text); header row correct; multiple tables go to separate sheets or ranges |
| pdf_to_pptx | multi_page | Slide count = page count; each slide contains that page's text or image; opens in python-pptx. **A 645-byte file likely means empty slides, so verify content** |
| pdf_convert_html | test.html with heading, image, CSS | PDF text contains the heading; CSS applied (e.g., bold/color check via render); image present; external-resource behavior documented; page size and margins correct |
| pdf_convert_markdown | test.md with heading, list, code block, table | Headings, list items, code, table text all present in the PDF; heading hierarchy visible (font size larger); special characters survive |
| pdf_convert_excel | test.csv (50 rows) and a real .xlsx | All rows present in the PDF text; columns not cut off; multi-page when rows overflow; **a 2.2 MB output for a 1-page result is suspicious: check for embedded fonts or bloat** |

---

## Parity test (run once per tool)

For each tool, run the same input through CLI, MCP, API, WASM and Edge, then compare:

```
page_count equal
extracted_text_per_page equal (normalized whitespace)
page_sizes equal
metadata equal (ignoring dates/producer)
for forms: field names and values equal
for images: dimensions equal
```

If a tool is N/A on WASM/Edge, record it as `N/A`, and exclude it from the parity percentage. Report parity as "X of Y comparable tool/interface pairs match."

## Fixtures you need

- `invoice_text.pdf` (real layout, 3 pages, tables, known strings)
- `invoice_scan.pdf` (photographed/skewed scan, 150 to 300 dpi)
- `multi_page.pdf` (5 pages, unique text per page "P1".."P5")
- `with_blank.pdf` (text, blank, text, noisy-blank, text)
- `redact_test.pdf` (known secret strings)
- `form.pdf` (text, checkbox, dropdown, radio)
- `image_heavy.pdf` (5+ MB)
- `broken_xref.pdf` (intentionally damaged)
- `outline.pdf` (nested bookmarks)
- `unicode.pdf` (accented + non-Latin text)
- `encrypted.pdf` (known password)

## Reporting format

Replace "PASS/FAIL" with one line per check, for example:
`pdf_redact | CLI | text "SECRET" absent after redact | FAIL`
Then publish: **"44 tools, N assertions, X% passing on CLI/MCP/API."** This is a far stronger launch claim than "44/44."