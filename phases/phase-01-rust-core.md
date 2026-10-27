## Phase 1 — Rust Core Engine

**Goal:** A reliable, well-tested PDF manipulation engine with a working CLI.
No AI. No GUI. Just a solid foundation everything else will build on.

**Exit condition:** All PDF operations listed below work correctly on real-world PDFs, have passing tests, and are accessible via the CLI.

---

### 1.1 — Project Setup `[PARALLEL]`

These can all be done on day one simultaneously.

| # | Task | Notes |
|---|---|---|
| ~~1.1.1~~ ✓ | ~~Initialize Cargo workspace~~ | Root `Cargo.toml` with workspace members |
| ~~1.1.2~~ ✓ | ~~Create crate stubs~~ | `paperpilot-core`, `paperpilot-pdf`, `paperpilot-cli` empty crates |
| ~~1.1.3~~ ✓ | ~~Set up CI pipeline~~ | Completed by Jules (Session 367767207543843406) |
| 1.1.4 | Set up test fixture corpus ✅ | Merged (Session 4769317803713179485) |
| ~~1.1.5~~ ✓ | ~~Choose and document PDF library decision~~ | Completed by Jules (Session 901308598875262977) |
| ~~1.1.6~~ ✓ | ~~Set up linting and formatting~~ | Completed by Jules (Session 367767207543843406) |

---

### 1.2 — Core Traits and Abstractions

**[BLOCKED BY 1.1.1, 1.1.2]**

Design the internal interfaces before implementing anything. This prevents tight coupling to a specific PDF library.

| # | Task | Notes |
|---|---|---|
| ~~1.2.1~~ ✓ | ~~Define `PdfDocument` trait~~ | Completed by Jules (Session 803802133789731818) |
| ~~1.2.2~~ ✓ | ~~Define `PdfOperation` trait~~ | Completed by Jules (Session 803802133789731818) |
| ~~1.2.3~~ ✓ | ~~Define `OperationResult` and error types~~ | Completed by Jules (Session 6267567952319900834) |
| ~~1.2.4~~ ✓ | ~~Define `JobProgress` event type~~ | Completed by Jules (Session 6267567952319900834) |

---

### 1.3 — PDF Operations `[PARALLEL within group]`

**[BLOCKED BY 1.2]**

All operations are independent of each other and can be built in parallel once the traits are in place.

#### Group A — Page Manipulation
| # | Task | Notes |
|---|---|---|
| 1.3.1 | Merge ✅ | Merged (Session 2375117738102953150) |
| 1.3.2 | Split ✅ | Merged (Session 2375117738102953150) |
| 1.3.3 | Extract pages ✅ | Merged (Session 1757891711996002541) |
| 1.3.4 | Delete pages ✅ | Merged (Session 1757891711996002541) |
| 1.3.5 | Reorder pages ✅ | Merged (Session 1757891711996002541) |
| 1.3.6 | Rotate pages ✅ | Merged (Session 12203497870162599469) |
| 1.3.7 | Crop pages ✅ | Merged (Session 12203497870162599469) |
| 1.3.8 | Burst ✅ | Merged (Session 2375117738102953150) |

#### Group B — Document Operations
| # | Task | Notes |
|---|---|---|
| 1.3.9 | Compress ✅ | Merged (Session 6923800200121241243) |
| 1.3.10 | Repair ✅ | Merged (Session 6923800200121241243) |
| 1.3.11 | Metadata read/write ✅ | Merged (Session 2384599268814762867) |
| 1.3.12 | Encrypt ✅ | Merged (Session 2384599268814762867) |
| 1.3.13 | Decrypt ✅ | Merged (Session 2384599268814762867) |
| 1.3.14 | Watermark ✅ | Merged (Session 8470929282963705394) |
| 1.3.15 | E-Signature ✅ | Merged (Session 12850003959773306225) |
| 1.3.16 | Redact ✅ | Merged (Session 12850003959773306225) |
| 1.3.17 | Linearize ✅ | Merged (Session 6923800200121241243) |
| 1.3.18 | Flatten ✅ | Merged (Session 12850003959773306225) |
| 1.3.19 | PDF/A conversion ✅ | Merged (Session 12850003959773306225) |
| 1.3.20 | Header/Footer ✅ | Merged (Session 8470929282963705394) |
| 1.3.21 | Bates numbering ✅ | Merged (Session 12785657976803298608) |
| 1.3.22 | Integrity hash ✅ | Merged (Session 10906031722199501492) |

#### Group C — Extraction & Search
| # | Task | Notes |
|---|---|---|
| 1.3.22 | Extract text ✅ | Merged (Session 2668342604059292265) |
| 1.3.23 | Extract images ✅ | Merged (Session 10564378390682319461) |
| 1.3.24 | Render pages ✅ | Merged (Session 12785657976803298608) |
| 1.3.25 | Images → PDF ✅ | Merged (Session 10564378390682319461) |
| 1.3.26 | Search ✅ | Merged (Session 2668342604059292265) |
| 1.3.27 | Compare ✅ | Merged (Session 12785657976803298608) |
| 1.3.28 | Bookmarks ✅ | Merged (Session 2668342604059292265) |
| 1.3.29 | OCR ✅ | Merged (Session 12785657976803298608) |

#### Group D — Conversion
| # | Task | Notes |
|---|---|---|
| 1.3.30 | PDF → Word (DOCX) ✅ | Merged (Session 8203167166999327475) |
| 1.3.31 | PDF → Excel (XLSX) ✅ | Merged (Session 8203167166999327475) |
| 1.3.32 | PDF → PowerPoint (PPTX) ✅ | Merged (Session 8203167166999327475) |
| 1.3.33 | PDF → HTML ✅ | Merged (Session 10906031722199501492) |
| 1.3.34 | PDF → Markdown ✅ | Merged (Session 10906031722199501492) |
| 1.3.38 | PDF → LLM-Ready Export ✅ | Merged (Session 10906031722199501492) |
| 1.3.39 | Document type classifier ✅ | Merged (Session 8203167166999327475) |
| 1.3.46 | PDF → Structured JSON ✅ | Merged (Session 10906031722199501492) |

#### Group E — Forms
| # | Task | Notes |
|---|---|---|
| 1.3.35 | AcroForm read ✅ | Merged (Session 996394326205831326) |
| 1.3.36 | AcroForm fill ✅ | Merged (Session 996394326205831326) |
| 1.3.37 | Form field creation ✅ | Merged (Session 996394326205831326) |

#### Group F — PDF Creation (from scratch)
| # | Task | Notes |
|---|---|---|
| 1.3.40 | Markdown → PDF ✅ | Merged |
| 1.3.41 | HTML → PDF ✅ | Merged |
| 1.3.42 | Template engine ✅ | Merged |
| 1.3.43 | Built-in template library ✅ | Merged |
| 1.3.44 | Template → PDF CLI ✅ | Merged |
| 1.3.45 | Custom page layout ✅ | Merged |

---

#### Group G — Smart Scan Compression (DjVu-inspired)
> Inspired by DjVu's 3-layer decomposition (background + foreground text mask + OCR). Target: 80–90% size reduction on scanned PDFs vs. standard Flate/JPEG. See `INSPIRATION.md §1`.

| # | Task | Notes |
|---|---|---|
| 1.3.50 | Scanned page detector | Heuristic: if >80% of page content stream is raster image data with no vector paths → classify as "scanned page" |
| 1.3.51 | Background layer extraction | Downsample page raster to ~72 DPI; apply IW44-style wavelet compression (Rust `image` crate wavelets) for paper texture / colour wash |
| 1.3.52 | Foreground text mask | Binarise the full-res page raster (adaptive threshold); detect connected components corresponding to text glyphs; encode as JBIG2 or CCITT Group 4 bitonal stream |
| 1.3.53 | Hidden text layer injection | After OCR (Phase 5.1), inject the text layer as a `ActualText` / invisible-text overlay within the compressed page for copy-paste and search |
| 1.3.54 | 3-layer page reassembly | Combine background (low-res colour) + foreground (bitonal hi-res mask) + hidden text into a single lopdf page object; result opens normally in any PDF viewer |
| 1.3.55 | `SmartCompressOperation` | Wraps tasks 1.3.50–1.3.54; called by `pdf_compress` when `--mode smart-scan` is passed; falls back to standard Flate for non-scanned pages |
| 1.3.56 | Compression ratio benchmarks | Automated test comparing output size vs. standard `pdf_compress --quality low/medium` on a fixture set of 10 scanned PDFs; must achieve ≥70% reduction |

---

### 1.4 — Testing `[PARALLEL with 1.3]`

Write tests alongside each operation, not after.

| # | Task | Notes |
|---|---|---|
| 1.4.1 | Unit tests per operation ✅ | Written inline in every operation (`#[cfg(test)]`) |
| 1.4.2 | Round-trip tests ✅ | Merged (`fixture_tests_group_a/b/c.rs`) |
| 1.4.3 | Fixture-based integration tests ✅ | Merged (`fixture_tests_group_a/b/c.rs`) |
| 1.4.4 | Property-based tests ✅ | Merged |
| 1.4.5 | Error handling tests ✅ | Merged |
| 1.4.6 | PDF Validation layer ✅ | Merged |

---

### 1.5 — CLI `[BLOCKED BY 1.3]`

**[BLOCKED BY 1.3 — each command blocked only by its own operation]**

| # | Task | Notes |
|---|---|---|
| 1.5.1 | CLI scaffold ✅ | Merged |
| 1.5.2 | `merge` command ✅ | Merged |
| 1.5.3 | `split` command ✅ | Merged |
| 1.5.4 | `extract` command ✅ | Merged |
| 1.5.5 | `remove` command ✅ | Merged |
| 1.5.6 | `reorder` command ✅ | Merged |
| 1.5.7 | `rotate` command ✅ | Merged |
| 1.5.8 | `compress` command ✅ | Merged |
| 1.5.9 | `extract-text` command ✅ | Merged |
| 1.5.10 | `extract-images` command ✅ | Merged |
| 1.5.11 | `watermark` command ✅ | Merged |
| 1.5.12 | `encrypt` / `decrypt` commands ✅ | Merged |
| 1.5.13 | `search` command ✅ | Merged |
| 1.5.14 | `validate` command ✅ | Merged |
| 1.5.15 | Progress output ✅ | Merged |
| 1.5.16 | JSON output mode ✅ | Merged |
| 1.5.17 | Webhook support ✅ | Merged |
| 1.5.18 | Webhook HMAC signing ✅ | Merged (Session 3236745347341611735) |
| 1.5.19 | Webhook filter flags ✅ | Merged (Session 3236745347341611735) |
| 1.5.20 | `hash` command ✅ | Merged (Session 3236745347341611735) |
| 1.5.21 | `verify` command ✅ | Merged (Session 3236745347341611735) |
| 1.5.22 | CLI integration tests ✅ | Merged (Session 3236745347341611735) |
| 1.5.23 | E2E CLI pipeline tests ✅ | Merged (Session 3236745347341611735) |
| 1.5.24 | `extract-text --format json` flag ✅ | Merged (Session 3236745347341611735) |

---

