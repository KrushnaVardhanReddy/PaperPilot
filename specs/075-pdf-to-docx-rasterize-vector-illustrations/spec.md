# Spec 075: High-Fidelity PDF-to-DOCX Vector Form XObject Rasterization & Embedding

## 1. Problem Statement
In PDF brochures and marketing documents (such as `fixtures/Get_Started_With_Smallpdf.pdf`), feature illustrations are not stored as raster images (`/Subtype /Image`), but rather as **PDF Form XObjects (`/Subtype /Form`) with raw vector Bézier path operators (`m`, `l`, `c`, `f`)**.

In Task 5.9.52 (Spec 074), Jules successfully established the alternating 2-column table geometry with compact 0.5-inch margins, but inserted placeholder text (`[ Feature 1 Illustration ]`) instead of real visual images.

To deliver true commercial-grade fidelity, `PdfToDocxOperation` must rasterize vector illustration regions into high-resolution PNG bytes and embed them directly into the DOCX table cells using `docx_rs::Pic`.

## 2. Technical Approach

### 2.1 Pure-Rust Rendering via `hayro` & `tiny-skia`
PaperPilot already contains `hayro` (`0.8.0`) and `tiny-skia` (`0.12.0`) in `paperpilot-pdf`:
```rust
let cache = hayro::RenderCache::new();
let render_settings = hayro::RenderSettings::default();
let mut pixmap_settings = hayro::PixmapSettings::default();
let scale_factor = 2.0833; // 150 DPI (150.0 / 72.0)
pixmap_settings.x_scale = scale_factor;
pixmap_settings.y_scale = scale_factor;

let pixmap = hayro::render(page, &cache, &Default::default(), &render_settings, &pixmap_settings);
```

### 2.2 Vector Coordinate Bounding Boxes
For `Get_Started_With_Smallpdf.pdf` (MediaBox `[0, 0, 595.28, 841.89]`), the 4 feature illustrations reside in coordinate boxes:
- **Feature 1 (Left)**: PDF `x: [50..230]`, `y: [590..740]`
- **Feature 2 (Right)**: PDF `x: [360..540]`, `y: [420..550]`
- **Feature 3 (Left)**: PDF `x: [50..230]`, `y: [230..360]`
- **Feature 4 (Right)**: PDF `x: [350..540]`, `y: [70..200]`

Conversion to pixel coordinates (where image origin is top-left, and PDF origin is bottom-left):
```rust
let px1 = (x1 * (img_w as f32 / page_w)).round() as u32;
let py1 = ((page_h - y2) * (img_h as f32 / page_h)).round() as u32; // invert y
let width = ((x2 - x1) * (img_w as f32 / page_w)).round() as u32;
let height = ((y2 - y1) * (img_h as f32 / page_h)).round() as u32;
```

### 2.3 Embedding PNG Bytes with `docx_rs::Pic`
Replace the text run placeholder in `paperpilot-pdf/src/operations/conversion.rs`:
```rust
use docx_rs::Pic;

if let Some(png_bytes) = img_bytes_opt {
    // 1.8 inches = ~1,645,920 EMUs (or 1,400,000 x 1,150,000 for compact card fit)
    let pic = Pic::new(&png_bytes).size(1400000, 1150000);
    let pic_run = Run::new().add_image(pic);
    illus_para = illus_para.add_run(pic_run);
}
```

## 3. Success Criteria
1. `paperpilot convert --format docx --input fixtures/Get_Started_With_Smallpdf.pdf --output /tmp/converted.docx` generates a `.docx` containing 4 actual embedded images (`word/media/image1.png` .. `image4.png`).
2. There are **0 placeholder strings** (`[ Feature N Illustration ]`) in `word/document.xml`.
3. Converted document fits on **strictly 1 page** in LibreOffice / Microsoft Word without overflow.
4. `cargo test -p paperpilot-pdf test_pdf_to_docx` passes with 0 warnings or regressions.
