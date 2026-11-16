# Spec: Unmocked Real-Engine E2E Tests — All 90 Tools

**Branch**: `feature/dev`  
**Output files**: 3 new Playwright spec files in `apps/desktop/tests/`  
**Philosophy**: Zero IPC mocks on engine calls. Real Rust binary runs. Real output verified at byte level.

---

## Architecture

```
Playwright (headless: false, slowMo: 400ms)
  ↓ navigates to http://127.0.0.1:1420
  ↓ uploads REAL fixture files via mocked dialog (OS picker only mocked)
  ↓ clicks tool card, fills args, clicks Run
  ↓ awaits success toast
  ↓ real Tauri IPC → real paperpilot Rust binary → real output file on disk
  ↓ PdfInspector / fs byte checks → domain invariants from SPEC_152
```

**What IS mocked (OS dialogs only):**
- `plugin:dialog|open` → returns absolute path to real fixture file
- `plugin:dialog|save` → returns absolute path to sandbox output dir

**What is NOT mocked (runs real):**
- `invoke_mcp_tool` → real Tauri IPC → real paperpilot binary

---

## Shared Setup (all 3 suites)

```ts
// In beforeEach — partial mock, engine passthrough
await page.addInitScript(() => {
  localStorage.setItem('paperpilot_onboarding_completed', 'true');
  (window as any).__TAURI_INTERNALS__ = {
    invoke: async (cmd: string, args: any) => {
      // OS file dialogs only — returns real fixture paths
      if (cmd === 'plugin:dialog|open') {
        return (window as any).__dialog_fixture || ['/PATH/TO/FIXTURE'];
      }
      if (cmd === 'plugin:dialog|save') {
        const ext = args?.filters?.[0]?.extensions?.[0] || 'pdf';
        const ts = Date.now();
        return `/PATH/TO/SANDBOX/out_${ts}.${ext}`;
      }
      // ALL other commands pass through to real Tauri
      return window.__REAL_TAURI__.invoke(cmd, args);
    }
  };
});
```

> **Note**: The `__REAL_TAURI__` reference is captured BEFORE the initScript overwrites it, so engine calls are genuinely forwarded to the native backend.

---

## Per-Tool Test Pattern

```ts
test('pdf_merge — Page Addition Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_merge');
  
  // 1. Set fixture
  await page.evaluate((paths) => { (window as any).__dialog_fixture = paths; },
    [fixture('merge_a.pdf'), fixture('merge_b.pdf')]);
  await page.evaluate((out) => { (window as any).__dialog_save = out; },
    path.join(sandbox, 'merged.pdf'));

  // 2. Navigate & load
  await page.goto('http://127.0.0.1:1420/');
  await page.waitForSelector('.tool-search-input, [data-testid="search"]', { timeout: 10000 });

  // 3. Search & click tool
  await page.fill('.tool-search-input', 'merge');
  await page.waitForTimeout(400);
  await page.click('[data-tool-name="pdf_merge"]');

  // 4. Upload via dialog trigger
  await page.click('[data-testid="open-files-btn"], .upload-btn, button:has-text("Open")');
  await page.waitForTimeout(600);

  // 5. Run
  await page.click('.run-btn, button:has-text("Run"), button:has-text("Execute")');
  await page.waitForSelector('.toast-success, [data-testid="success-toast"]', { timeout: 30000 });

  // 6. Post-validate
  const outFile = path.join(sandbox, 'merged.pdf');
  expect(fs.existsSync(outFile)).toBe(true);
  const pdf = await PdfInspector.inspect(outFile);
  expect(pdf.numPages).toBe(2);                          // Page Addition Invariant
  expect(pdf.textByPage[0]).toContain('Page A content'); // Doc A text on page 1
  expect(pdf.textByPage[1]).toContain('Page B content'); // Doc B text on page 2

  sandboxManager.cleanup(sandbox);
});
```

---

## Suite 1 — Tools 01–30: Page Operations, Imposition, Optimization

**File**: `apps/desktop/tests/e2e_unmocked_suite1_page_ops.spec.ts`

| # | Tool | Fixture(s) | Key Args | SPEC_152 Invariant |
|---|---|---|---|---|
| 01 | `pdf_merge` | `merge_a.pdf` + `merge_b.pdf` | — | Pages(out)=2; page1 text=A, page2 text=B |
| 02 | `pdf_merge_alternate` | `merge_a.pdf` + `merge_b.pdf` | — | Pages(out)=2; interleaved content order |
| 03 | `pdf_split` | `multi_page.pdf` (5 pages) | split at p1 | Output: pages=1, text matches page 1 of input |
| 04 | `pdf_split_by_outline` | `bookmarks.json`→use `research_paper.pdf` | — | Output ZIP contains multiple PDFs, one per bookmark |
| 05 | `pdf_split_scanned_book` | `two_up_book.pdf` | — | Pages(out)=2×Pages(in); portrait dimensions |
| 06 | `pdf_burst` | `multi_page.pdf` (5 pages) | — | 5 output files, each 1 page |
| 07 | `pdf_crop` | `single_page.pdf` | margin=50 | CropBox changes in PDF dict |
| 08 | `pdf_delete_pages` | `multi_page.pdf` | pages=1 | Pages(out)=4; original p1 text absent from p1 |
| 09 | `pdf_extract_pages` | `multi_page.pdf` | pages=1 | Pages(out)=1; text matches input page 1 |
| 10 | `pdf_reorder_pages` | `four_pages.pdf` | order=4,3,2,1 | Page 1 text = original page 4 text |
| 11 | `pdf_rotate` | `single_page.pdf` | angle=90 | Raw: `/Rotate 90` in PDF dict |
| 12 | `pdf_auto_rotate` | `rotated_landscape.pdf` | — | Output `/Rotate 0` (normalized) |
| 13 | `pdf_remove_blank` | `with_blank_pages.pdf` | — | Pages(out) < Pages(in); no empty pages |
| 14 | `pdf_scale_pages` | `single_page.pdf` | scale=0.5 | MediaBox width×height both halved |
| 15 | `pdf_nup` | `four_pages.pdf` | n=4 | Pages(out)=1 (4→1 sheet) |
| 16 | `pdf_booklet` | `four_pages.pdf` | — | Pages(out)=2; saddle-stitch reorder |
| 17 | `pdf_posterize` | `single_page.pdf` | grid=2x2 | Pages(out)=4 tiles |
| 18 | `pdf_stitch_canvas` | `four_pages.pdf` | — | Pages(out)=1; height = sum of all pages |
| 19 | `pdf_bookmarks` | `research_paper.pdf` | — | Output JSON array with `title`+`page` keys |
| 20 | `pdf_attachments` | `single_page.pdf` | — | Output JSON lists embedded files |
| 21 | `pdf_images_to_pdf` | `img1.png` + `img2.png` | — | Pages(out)=2; each has `/XObject /Image` |
| 22 | `img_to_pdf` | `test.png` | — | Pages(out)=1; has `/XObject /Image` |
| 23 | `pdf_compress` | `large_doc.pdf` | — | size(out) ≤ size(in) |
| 24 | `pdf_dedup_streams` | `images.pdf` | — | Valid PDF; raw has fewer unique object refs |
| 25 | `pdf_linearize` | `single_page.pdf` | — | Raw first 1024 bytes contains `/Linearized` |
| 26 | `pdf_repair` | `malformed.pdf` | — | Valid PDF header `%PDF-`; no parse error |
| 27 | `pdf_invert_colors` | `single_page.pdf` | — | Valid PDF; colorspace inverted |
| 28 | `pdf_render` | `single_page.pdf` | dpi=150 | Output `.png`; dimensions ≈ 1240×1754 at 150dpi |
| 29 | `pdf_compare` | `merge_a.pdf` + `merge_b.pdf` | — | Output JSON has `differences` array |
| 30 | `pdf_encrypt` | `single_page.pdf` | password=test123 | Raw: `/Encrypt` dict present |

---

## Suite 2 — Tools 31–60: Security, Signatures, AcroForms, Watermarks & Synthesis

**File**: `apps/desktop/tests/e2e_unmocked_suite2_security_forms.spec.ts`

| # | Tool | Fixture(s) | Key Args | SPEC_152 Invariant |
|---|---|---|---|---|
| 31 | `pdf_decrypt` | `encrypted.pdf` | password=test | Raw: no `/Encrypt`; text accessible |
| 32 | `pdf_sign` | `unsigned.pdf` | use `dummy.p12` | Raw: `/Sig` or `/ByteRange` present |
| 33 | `pdf_verify_signature` | `signed.pdf` | — | Output JSON: `valid: true` |
| 34 | `pdf_flatten` | `form.pdf` | — | Output: no `/AcroForm /Fields`; text frozen in content stream |
| 35 | `pdf_remove_signature` | `signed.pdf` | — | Raw: no `/Sig` present |
| 36 | `pdf_read_form` | `form.pdf` | — | Output JSON: `{field_name: field_value}` pairs |
| 37 | `pdf_fill_form` | `form.pdf` | field data JSON | Output: AcroForm fields contain new values |
| 38 | `pdf_create_form_field` | `single_page.pdf` | name=MyField | Raw: `/Widget` annotation present |
| 39 | `batch_fill` | `form.pdf` + `batch_data.csv` | — | N output PDFs = N CSV rows |
| 40 | `pdf_watermark` | `single_page.pdf` | text=CONFIDENTIAL, angle=45 | Content stream contains "CONFIDENTIAL" |
| 41 | `pdf_bates` | `multi_page.pdf` | prefix=CONF, start=100 | Pages contain "CONF-000100", "CONF-000101"… |
| 42 | `pdf_page_numbers` | `four_pages.pdf` | — | Each page contains "1 of 4"…"4 of 4" |
| 43 | `pdf_header_footer` | `single_page.pdf` | header=TOP, footer=BOTTOM | Content stream has "TOP" and "BOTTOM" |
| 44 | `pdf_annotate` | `single_page.pdf` | type=highlight | Raw: `/Highlight` in `/Annots` array |
| 45 | `pdf_redact` | `text_pii.pdf` | target=SSN | Target text absent from decoded stream |
| 46 | `pdf_remove_metadata` | `metadata.pdf` | — | Output: no `/Author`, `/Creator` in XMP/Info |
| 47 | `pdf_set_metadata` | `single_page.pdf` | author=Test | Raw: `/Author (Test)` in Info dict |
| 48 | `pdf_validate` | `single_page.pdf` | — | Output JSON: `valid: true`, no errors |
| 49 | `pdf_validate_pdfa` | `pdf_a.pdf` | — | Output JSON: `conformance: PDF/A-1b` |
| 50 | `pdf_hash` | `single_page.pdf` | algo=sha256 | Output JSON: 64-char hex SHA-256 string |
| 51 | `pdf_classify_type` | `form.pdf` | — | Output JSON: `type: "form"` |
| 52 | `pdf_search` | `search_test.pdf` | query=Invoice | Output JSON: array of `{page, x, y, text}` matches |
| 53 | `pdf_to_html` | `single_page.pdf` | — | Output `.html` with `<div>` or `<span>` tags, UTF-8 text |
| 54 | `pdf_to_markdown` | `single_page.pdf` | — | Output `.md` with `#` headings or `**bold**` |
| 55 | `pdf_to_json` | `single_page.pdf` | — | Output JSON array: each item has `text`, `page`, `x`, `y` |
| 56 | `json_to_pdf` | `form_data.json` | — | Valid PDF; raw `%PDF-`; contains JSON field values as text |
| 57 | `md_to_pdf` | `test.md` | — | Valid PDF; contains Markdown-rendered headings |
| 58 | `html_to_pdf` | `test.html` | — | Valid PDF; contains HTML-rendered text content |
| 59 | `pdf_add_blank_page` | `single_page.pdf` | — | Pages(out) = Pages(in) + 1 |
| 60 | `pdf_duplicate_pages` | `single_page.pdf` | — | Pages(out) = 2×Pages(in) |

---

## Suite 3 — Tools 61–90: Conversions, Office, AI & Workflows

**File**: `apps/desktop/tests/e2e_unmocked_suite3_conversions_ai.spec.ts`

| # | Tool | Fixture(s) | Key Args | SPEC_152 Invariant |
|---|---|---|---|---|
| 61 | `pdf_to_docx` | `single_page.pdf` | — | Valid .docx PKZIP; `word/document.xml` has source text |
| 62 | `pdf_to_xlsx` | `financial_tables.pdf` | — | Valid .xlsx PKZIP; `xl/worksheets/sheet1.xml` has cell data |
| 63 | `pdf_to_pptx` | `four_pages.pdf` | — | Valid .pptx PKZIP; `ppt/slides/slide1.xml` exists |
| 64 | `pdf_to_epub` | `single_page.pdf` | — | Valid .epub PKZIP; `mimetype` file = `application/epub+zip` |
| 65 | `pdf_to_pdf_a` | `single_page.pdf` | — | Raw: `/GTS_PDFA1` or `/OutputIntents` in dict |
| 66 | `pdf_extract_images` | `images.pdf` | — | Output: PNG/JPEG files, count > 0 |
| 67 | `pdf_extract_text` | `single_page.pdf` | — | Output `.txt`: non-empty, contains document text |
| 68 | `pdf_extract_tables` | `financial_tables.pdf` | — | Output JSON/CSV: row×col matrix with numeric values |
| 69 | `pdf_comic_archive` | `four_pages.pdf` | — | Output `.cbz` PKZIP; contains `001.png`…`00N.png` |
| 70 | `pdf_grayscale` | `single_page.pdf` | — | Valid PDF; colorspace changed to DeviceGray |
| 71 | `pdf_convert_excel` | `test.csv` | — | Valid PDF; tabular grid with CSV row content |
| 72 | `excel_to_json` | `test.xlsx` | — | Output JSON array; rows match spreadsheet rows |
| 73 | `excel_to_csv` | `test.xlsx` | — | Output `.csv`; comma-delimited rows match sheet |
| 74 | `excel_to_pdf` | `test.xlsx` | — | Valid PDF; contains cell text from spreadsheet |
| 75 | `office_modernize_legacy` | `test.xls` | — | Output `.xlsx` PKZIP; valid OpenXML format |
| 76 | `word_to_pdf` | `test.docx` | — | Valid PDF; contains Word document text |
| 77 | `powerpoint_to_pdf` | `test.pptx` | — | Valid PDF; contains slide text |
| 78 | `pdf_ocr` | `ocr.pdf` | lang=eng | Output PDF: extracted text layer present in content stream |
| 79 | `pdf_classify_pages` | `multimodal.pdf` | — | Output JSON: `[{page: 1, type: "text/image/table"}]` |
| 80 | `pdf_summarize` | `research_paper.pdf` | — | Output JSON/text: summary string length > 50 chars |
| 81 | `pdf_translate` | `single_page.pdf` | target=es | Output PDF/text: contains Spanish words |
| 82 | `pdf_chat` | `single_page.pdf` | query=hello | Output JSON: `answer` field non-empty string |
| 83 | `pdf_reverse_order` | `four_pages.pdf` | — | Pages(out)=4; page1 text = original page4 text |
| 84 | `pdf_interleave` | `merge_a.pdf` + `merge_b.pdf` | — | Pages(out)=2; alternating A/B content |
| 85 | `pdf_deinterleave` | `four_pages.pdf` | — | 2 output files; odd pages in one, even in other |
| 86 | `pdf_adjust_page_box` | `single_page.pdf` | box=TrimBox | Raw: `/TrimBox` added to page dict |
| 87 | `pdf_render_page_selection` | `multi_page.pdf` | pages=1,3 | Output: 2 PNG files, one per selected page |
| 88 | `pdf_extract_embedded_images` | `images.pdf` | — | Output ZIP with PNG files = image count in PDF |
| 89 | `pdf_split_by_size` | `large_doc.pdf` | size=100kb | Multiple PDFs, each ≤ 100KB |
| 90 | `pdf_split_half` | `four_pages.pdf` | — | 2 output PDFs, each with Pages(in)/2 pages |

---

## Playwright Config Additions

```ts
// playwright.config.ts — add a "real" project
{
  name: 'real-engine-90tools',
  use: {
    ...devices['Desktop Chrome'],
    headless: false,       // visible — user can watch
    slowMo: 400,           // 400ms between actions
    video: 'on',           // saved to test-results/
    screenshot: 'on',      // on every step
    launchOptions: {
      args: ['--proxy-bypass-list=<-loopback>'],
    },
  },
  testMatch: 'tests/e2e_unmocked_suite*.spec.ts',
}
```

## Makefile Target to Add

```makefile
test-e2e-real-90:
	cd apps/desktop && npx playwright test e2e_unmocked_suite1_page_ops e2e_unmocked_suite2_security_forms e2e_unmocked_suite3_conversions_ai --project=real-engine-90tools

test-e2e-real-s1:
	cd apps/desktop && npx playwright test e2e_unmocked_suite1_page_ops --project=real-engine-90tools --headed

test-e2e-real-s2:
	cd apps/desktop && npx playwright test e2e_unmocked_suite2_security_forms --project=real-engine-90tools --headed

test-e2e-real-s3:
	cd apps/desktop && npx playwright test e2e_unmocked_suite3_conversions_ai --project=real-engine-90tools --headed
```

---

## Done Criteria

Each agent is DONE when:
1. The spec file is written and compiles cleanly (`npx tsc --noEmit`)
2. Each tool has its own `test()` block (no mega-loop)
3. Every test has realistic post-validation matching the SPEC_152 invariant
4. The file is committed to `feature/dev`
5. A brief scorecard summary is posted to the board

## References

- Invariants: `qa/SPEC_152_TOOL_SPECIFIC_INVARIANTS.md`
- Helpers: `apps/desktop/tests/helpers/pdfInspector.ts`, `sandboxManager.ts`
- Fixtures: `tests/e2e_fixtures/real/` (60+ files available — use the right one per tool)
- Existing pattern: `apps/desktop/tests/e2e_suite_1_page_operations.spec.ts` (for UI interaction pattern)
- Real fixture paths: always use `path.resolve(process.cwd(), '../../tests/e2e_fixtures/real/<file>')`

