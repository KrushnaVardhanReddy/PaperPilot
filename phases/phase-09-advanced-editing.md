## Phase 9 — Advanced Editing & Enterprise Features

**Goal:** Close the most common gaps vs. Adobe Acrobat with achievable, well-defined capabilities. WYSIWYG text reflow is deliberately deferred to Phase 9.3 (Moonshot) due to extreme complexity.

### 9.1 — Annotation & Markup (Achievable)
| # | Task | Notes |
|---|---|---|
| 9.1.1 | Highlight Tool | Select text regions and apply colour highlight overlays |
| 9.1.2 | Sticky Notes / Comments | Attach pop-up comment annotations to any page location |
| 9.1.3 | Free-Draw Markup | Freehand pen/pencil drawing over pages (stored as SVG overlay) |
| 9.1.4 | Redaction Tool | Select text/areas, burn a black box over them, and permanently strip underlying content stream data |

### 9.2 — Forms & E-Signatures (Achievable)
| # | Task | Notes |
|---|---|---|
| 9.2.1 | Form Field Builder | Drag-and-drop text boxes, checkboxes, and radio buttons to create fillable PDFs |
| 9.2.2 | Data Extraction | Extract filled form data to JSON/CSV |
| 9.2.3 | Cryptographic Signing UI | Select a certificate, draw a signature, and apply a cryptographic seal |
| 9.2.4 | Signature Verification | Automatically verify document integrity and show a green/red seal indicator |

### 9.3 — WYSIWYG Canvas Editing (Moonshot)
> [!WARNING]
> This is the hardest unsolved problem in PDF engineering. PDFs have no reflow model — text is stored as absolute `x,y` positioned glyphs. True in-place editing requires parsing content streams, having access to embedded font subsets, and re-encoding glyph data. Even Adobe Acrobat struggles with this on some PDFs. Estimated scope: 12–24 engineering months.

| # | Task | Notes |
|---|---|---|
| 9.3.1 | Content Stream Parser | Parse PDF page content streams to extract text bounding boxes and glyph sequences |
| 9.3.2 | Font Subsetting Engine | Expand embedded font subsets to include any new characters the user wants to type |
| 9.3.3 | In-Place Text Edit | Click a text run, edit it, re-encode back into the content stream |
| 9.3.4 | Image Drag & Replace | Click an embedded image, drag to reposition, or replace with a new file |

---

