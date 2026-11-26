use image::imageops::FilterType;
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

/// Options for generating a clean document showcase GIF.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowcaseGifOptions {
    /// Frames per second. If provided, delay is calculated as `(100 / fps).max(1)`.
    pub fps: Option<u8>,
    /// Delay between frames in centiseconds (1/100ths of a second). Default is 25 (0.25s).
    pub delay_centisecs: Option<u16>,
    /// Target canvas width in pixels. Default is 800. Height will scale proportionally (default 16:9).
    pub width: Option<u32>,
    /// Number of animation frames per transition or scan cycle. Default is 8.
    pub transition_frames: Option<u8>,
}

impl Default for ShowcaseGifOptions {
    fn default() -> Self {
        Self {
            fps: Some(4),
            delay_centisecs: Some(25),
            width: Some(800),
            transition_frames: Some(8),
        }
    }
}

fn fill_rect(
    img: &mut image::RgbaImage,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    color: image::Rgba<u8>,
) {
    let max_x = (x + w).min(img.width());
    let max_y = (y + h).min(img.height());
    for py in y..max_y {
        for px in x..max_x {
            img.put_pixel(px, py, color);
        }
    }
}

/// Render all pages of a PDF up to max_pages into RgbaImages
fn render_document_pages(
    input_bytes: &[u8],
    max_pages: usize,
    target_width: u32,
) -> Result<Vec<image::RgbaImage>, PdfError> {
    let pdf = hayro_syntax::Pdf::new(input_bytes.to_vec())
        .map_err(|e| PdfError::ParseError(format!("Failed to parse PDF: {:?}", e)))?;

    let pages = pdf.pages();
    if pages.is_empty() {
        return Err(PdfError::ParseError("PDF contains no pages".to_string()));
    }

    let cache = hayro::RenderCache::new();
    let render_settings = hayro::RenderSettings::default();

    let count = pages.len().min(max_pages);
    let mut rendered_pages = Vec::with_capacity(count);

    for i in 0..count {
        let page = pages
            .get(i)
            .ok_or_else(|| PdfError::ParseError(format!("Failed to get page {}", i + 1)))?;

        let (base_w, _base_h) = page.render_dimensions();
        let scale = if base_w > 0.0 {
            (target_width as f32 / base_w).max(0.01)
        } else {
            1.0
        };

        let mut pixmap_settings = hayro::PixmapSettings::default();
        pixmap_settings.x_scale = scale;
        pixmap_settings.y_scale = scale;

        let pixmap = hayro::render(
            page,
            &cache,
            &Default::default(),
            &render_settings,
            &pixmap_settings,
        );

        let pw = pixmap.width() as u32;
        let ph = pixmap.height() as u32;
        if pw == 0 || ph == 0 {
            continue;
        }

        let raw_data: &[u8] = bytemuck::cast_slice(pixmap.data());
        let rgba_raw = image::RgbaImage::from_raw(pw, ph, raw_data.to_vec()).ok_or_else(|| {
            PdfError::ParseError("Failed to construct RgbaImage from rendered pixmap".to_string())
        })?;

        // Composite over crisp white background
        let mut blended = image::RgbaImage::new(pw, ph);
        for (x, y, pixel) in rgba_raw.enumerate_pixels() {
            let alpha = pixel[3] as f32 / 255.0;
            let r = ((pixel[0] as f32 * alpha) + (255.0 * (1.0 - alpha))).round() as u8;
            let g = ((pixel[1] as f32 * alpha) + (255.0 * (1.0 - alpha))).round() as u8;
            let b = ((pixel[2] as f32 * alpha) + (255.0 * (1.0 - alpha))).round() as u8;
            blended.put_pixel(x, y, image::Rgba([r, g, b, 255]));
        }

        rendered_pages.push(blended);
    }

    if rendered_pages.is_empty() {
        return Err(PdfError::ParseError(
            "Failed to render any pages from PDF".to_string(),
        ));
    }

    Ok(rendered_pages)
}

/// Helper to draw a sleek modern background canvas with subtle radial/vignette gradient
fn create_background_canvas(width: u32, height: u32) -> image::RgbaImage {
    let mut canvas = image::RgbaImage::new(width, height);
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let max_dist = ((center_x * center_x) + (center_y * center_y)).sqrt().max(1.0);

    // Modern neutral slate backdrop: center [246, 248, 252] fading to [226, 232, 240]
    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let dist = ((dx * dx) + (dy * dy)).sqrt();
            let factor = (dist / max_dist).clamp(0.0, 1.0);

            let r = (246.0 * (1.0 - factor) + 226.0 * factor).round() as u8;
            let g = (248.0 * (1.0 - factor) + 232.0 * factor).round() as u8;
            let b = (252.0 * (1.0 - factor) + 240.0 * factor).round() as u8;

            canvas.put_pixel(x, y, image::Rgba([r, g, b, 255]));
        }
    }

    canvas
}

/// Helper to render multi-layer soft drop shadow under the document sheet
fn render_page_shadow(
    canvas: &mut image::RgbaImage,
    doc_x: u32,
    doc_y: u32,
    doc_w: u32,
    doc_h: u32,
) {
    // Progressive soft shadow layers
    let shadow_layers = [
        (4, 4, 18, 28, 38, 25),
        (8, 8, 20, 30, 42, 20),
        (14, 14, 25, 35, 48, 15),
    ];

    for (ox, oy, sr, sg, sb, alpha) in shadow_layers {
        let sx = doc_x + ox;
        let sy = doc_y + oy;
        let max_x = (sx + doc_w).min(canvas.width());
        let max_y = (sy + doc_h).min(canvas.height());
        let shadow_alpha = alpha as f32 / 255.0;

        for py in sy..max_y {
            for px in sx..max_x {
                let current = canvas.get_pixel(px, py);
                let r = ((current[0] as f32 * (1.0 - shadow_alpha)) + (sr as f32 * shadow_alpha)).round() as u8;
                let g = ((current[1] as f32 * (1.0 - shadow_alpha)) + (sg as f32 * shadow_alpha)).round() as u8;
                let b = ((current[2] as f32 * (1.0 - shadow_alpha)) + (sb as f32 * shadow_alpha)).round() as u8;
                canvas.put_pixel(px, py, image::Rgba([r, g, b, 255]));
            }
        }
    }
}

/// Draw a crisp, subtle border around the document sheet
fn render_page_border(
    canvas: &mut image::RgbaImage,
    doc_x: u32,
    doc_y: u32,
    doc_w: u32,
    doc_h: u32,
) {
    let border_color = image::Rgba([203, 213, 225, 255]); // Subtle slate border
    // Top & bottom
    fill_rect(canvas, doc_x, doc_y, doc_w, 1, border_color);
    if doc_h > 1 {
        fill_rect(canvas, doc_x, doc_y + doc_h - 1, doc_w, 1, border_color);
    }
    // Left & right
    fill_rect(canvas, doc_x, doc_y, 1, doc_h, border_color);
    if doc_w > 1 {
        fill_rect(canvas, doc_x + doc_w - 1, doc_y, 1, doc_h, border_color);
    }
}

/// Alpha blend two images of identical dimensions: out = (1 - alpha) * img_a + alpha * img_b
fn blend_images(
    img_a: &image::RgbaImage,
    img_b: &image::RgbaImage,
    alpha: f32,
) -> image::RgbaImage {
    let alpha = alpha.clamp(0.0, 1.0);
    let inv_alpha = 1.0 - alpha;
    let (w, h) = (img_a.width(), img_a.height());
    let mut blended = image::RgbaImage::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let pa = img_a.get_pixel(x, y);
            let pb = img_b.get_pixel(x, y);
            let r = ((pa[0] as f32 * inv_alpha) + (pb[0] as f32 * alpha)).round() as u8;
            let g = ((pa[1] as f32 * inv_alpha) + (pb[1] as f32 * alpha)).round() as u8;
            let b = ((pa[2] as f32 * inv_alpha) + (pb[2] as f32 * alpha)).round() as u8;
            blended.put_pixel(x, y, image::Rgba([r, g, b, 255]));
        }
    }

    blended
}

/// Generate a document-first showcase GIF without mock terminals or developer jargon.
pub fn showcase_gif_bytes(
    input_bytes: &[u8],
    options: &ShowcaseGifOptions,
) -> Result<Vec<u8>, PdfError> {
    if input_bytes.is_empty() {
        return Err(PdfError::ParseError("Input PDF bytes cannot be empty".to_string()));
    }

    let total_width = options.width.unwrap_or(800).max(400);
    let total_height = ((total_width as f32 * 9.0) / 16.0).round() as u32; // 16:9 canvas

    let transition_frames = options.transition_frames.unwrap_or(8).clamp(4, 30) as usize;

    let delay_centisecs = if let Some(fps) = options.fps {
        if fps > 0 {
            (100 / fps as u16).max(1)
        } else {
            options.delay_centisecs.unwrap_or(25).max(1)
        }
    } else {
        options.delay_centisecs.unwrap_or(25).max(1)
    };

    // Render pages from input PDF (limit to at most 8 pages for multi-page loop)
    let max_render_w = total_width.saturating_sub(100).max(200);
    let pages = render_document_pages(input_bytes, 8, max_render_w)?;

    let base_bg = create_background_canvas(total_width, total_height);

    // Available viewport for document inside canvas
    let max_doc_w = (total_width as f32 * 0.82).round() as u32;
    let max_doc_h = (total_height as f32 * 0.84).round() as u32;

    let mut frames: Vec<image::RgbaImage> = Vec::new();

    if pages.len() == 1 {
        // --- Single Page Showcase: Elegant Scan & Inspection Highlight ---
        let page = &pages[0];
        let pw = page.width() as f32;
        let ph = page.height() as f32;

        let scale = (max_doc_w as f32 / pw).min(max_doc_h as f32 / ph).min(1.0);
        let target_pw = (pw * scale).round() as u32;
        let target_ph = (ph * scale).round() as u32;

        let resized_page = image::imageops::resize(
            page,
            target_pw,
            target_ph,
            FilterType::Lanczos3,
        );

        let doc_x = (total_width.saturating_sub(target_pw)) / 2;
        let doc_y = (total_height.saturating_sub(target_ph)) / 2;

        // Render sweep animation frames
        let total_scan_frames = transition_frames * 2;
        for frame_idx in 0..total_scan_frames {
            let mut canvas = base_bg.clone();

            // Soft drop shadow
            render_page_shadow(&mut canvas, doc_x, doc_y, target_pw, target_ph);

            // Document sheet
            image::imageops::overlay(&mut canvas, &resized_page, doc_x as i64, doc_y as i64);

            // Border
            render_page_border(&mut canvas, doc_x, doc_y, target_pw, target_ph);

            // Calculate scan bar position (continuous loop)
            let progress = frame_idx as f32 / total_scan_frames as f32;
            let scan_center_y = doc_y as f32 + (target_ph as f32 * progress);

            // Draw modern soft accent gradient scan beam across document
            let beam_height = 14.0f32;
            let start_y = (scan_center_y - beam_height).max(doc_y as f32) as u32;
            let end_y = (scan_center_y + beam_height).min((doc_y + target_ph) as f32) as u32;

            for py in start_y..end_y {
                let dist = (py as f32 - scan_center_y).abs();
                let factor = (1.0 - (dist / beam_height)).clamp(0.0, 1.0);
                let beam_alpha = (0.28 * factor).clamp(0.0, 1.0);

                // Sleek indigo / sky-blue beam highlight: [59, 130, 246]
                for px in doc_x..(doc_x + target_pw) {
                    let cur = canvas.get_pixel(px, py);
                    let r = ((cur[0] as f32 * (1.0 - beam_alpha)) + (59.0 * beam_alpha)).round() as u8;
                    let g = ((cur[1] as f32 * (1.0 - beam_alpha)) + (130.0 * beam_alpha)).round() as u8;
                    let b = ((cur[2] as f32 * (1.0 - beam_alpha)) + (246.0 * beam_alpha)).round() as u8;
                    canvas.put_pixel(px, py, image::Rgba([r, g, b, 255]));
                }
            }

            // Crisp accent highlight line right at scan_center_y
            let center_py = scan_center_y.round() as u32;
            if center_py >= doc_y && center_py < doc_y + target_ph {
                let line_color = image::Rgba([99, 102, 241, 230]); // Indigo accent
                fill_rect(&mut canvas, doc_x, center_py, target_pw, 2, line_color);
            }

            frames.push(canvas);
        }
    } else {
        // --- Multi-Page Showcase: Smooth Page-Turn / Cross-Dissolve Loop ---
        let pw = pages[0].width() as f32;
        let ph = pages[0].height() as f32;
        let scale = (max_doc_w as f32 / pw).min(max_doc_h as f32 / ph).min(1.0);
        let target_pw = (pw * scale).round() as u32;
        let target_ph = (ph * scale).round() as u32;

        let resized_pages: Vec<image::RgbaImage> = pages
            .iter()
            .map(|p| image::imageops::resize(p, target_pw, target_ph, FilterType::Lanczos3))
            .collect();

        let doc_x = (total_width.saturating_sub(target_pw)) / 2;
        let doc_y = (total_height.saturating_sub(target_ph)) / 2;

        let num_pages = resized_pages.len();
        for page_idx in 0..num_pages {
            let next_page_idx = (page_idx + 1) % num_pages;
            let current_page = &resized_pages[page_idx];
            let next_page = &resized_pages[next_page_idx];

            // 1. Static hold frame for current page (2 steps hold)
            for _ in 0..2 {
                let mut canvas = base_bg.clone();
                render_page_shadow(&mut canvas, doc_x, doc_y, target_pw, target_ph);
                image::imageops::overlay(&mut canvas, current_page, doc_x as i64, doc_y as i64);
                render_page_border(&mut canvas, doc_x, doc_y, target_pw, target_ph);
                frames.push(canvas);
            }

            // 2. Smooth cross-dissolve transition frames into next page
            for t_step in 1..transition_frames {
                let alpha = t_step as f32 / transition_frames as f32;
                let blended_page = blend_images(current_page, next_page, alpha);

                let mut canvas = base_bg.clone();
                render_page_shadow(&mut canvas, doc_x, doc_y, target_pw, target_ph);
                image::imageops::overlay(&mut canvas, &blended_page, doc_x as i64, doc_y as i64);
                render_page_border(&mut canvas, doc_x, doc_y, target_pw, target_ph);
                frames.push(canvas);
            }
        }
    }

    if frames.is_empty() {
        return Err(PdfError::ParseError("Failed to generate animation frames".to_string()));
    }

    let gif_width = total_width as u16;
    let gif_height = total_height as u16;

    let mut gif_bytes = Vec::new();
    {
        let mut encoder = gif::Encoder::new(&mut gif_bytes, gif_width, gif_height, &[])
            .map_err(|e| PdfError::Other(format!("Failed to initialize GIF encoder: {:?}", e)))?;

        encoder
            .set_repeat(gif::Repeat::Infinite)
            .map_err(|e| PdfError::Other(format!("Failed to set GIF repeat mode: {:?}", e)))?;

        for mut frame_img in frames {
            let mut gif_frame = gif::Frame::from_rgba_speed(
                gif_width,
                gif_height,
                frame_img.as_mut(),
                10,
            );
            gif_frame.delay = delay_centisecs;

            encoder
                .write_frame(&gif_frame)
                .map_err(|e| PdfError::Other(format!("Failed to write GIF frame: {:?}", e)))?;
        }
    }

    Ok(gif_bytes)
}

#[derive(Default)]
pub struct ShowcaseGifOperation {
    options: ShowcaseGifOptions,
}

impl ShowcaseGifOperation {
    pub fn new(options: ShowcaseGifOptions) -> Self {
        Self { options }
    }
}

impl PdfOperation for ShowcaseGifOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> paperpilot_core::error::OperationResult<()> {
        let doc = document
            .as_any_mut()
            .downcast_mut::<crate::document::LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be LopdfDocument".to_string())
            })?;

        let mut bytes = Vec::new();
        if let Some(orig) = doc.source_bytes() {
            bytes = orig.to_vec();
        } else {
            doc.inner
                .save_to(&mut bytes)
                .map_err(|e| PdfError::Other(format!("Failed to save PDF: {:?}", e)))?;
        }

        let _gif_bytes = showcase_gif_bytes(&bytes, &self.options)?;

        Err(PdfError::UnsupportedOperation(
            "Use showcase_gif_bytes directly. In-place mutation is unsupported for binary GIF export.".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::dictionary;
    use lopdf::Document;

    fn create_sample_pdf(title: &str, pages_count: usize) -> Vec<u8> {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();

        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        });

        let resources_id = doc.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });

        let mut page_ids = Vec::new();

        for i in 0..pages_count {
            let page_text = format!("{} - Page {}", title, i + 1);
            let content = lopdf::content::Content {
                operations: vec![
                    lopdf::content::Operation::new("BT", vec![]),
                    lopdf::content::Operation::new("Tf", vec!["F1".into(), 16.into()]),
                    lopdf::content::Operation::new("Td", vec![50.into(), 100.into()]),
                    lopdf::content::Operation::new(
                        "Tj",
                        vec![lopdf::Object::string_literal(page_text)],
                    ),
                    lopdf::content::Operation::new("ET", vec![]),
                ],
            };

            let content_id = doc.add_object(lopdf::Stream::new(
                lopdf::Dictionary::new(),
                content.encode().unwrap(),
            ));

            let page_id = doc.add_object(dictionary! {
                "Type" => "Page",
                "Parent" => pages_id,
                "Contents" => content_id,
                "Resources" => resources_id,
                "MediaBox" => vec![0.into(), 0.into(), 400.into(), 300.into()],
            });
            page_ids.push(page_id.into());
        }

        doc.set_object(
            pages_id,
            dictionary! {
                "Type" => "Pages",
                "Kids" => page_ids,
                "Count" => pages_count as i64,
            },
        );

        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });

        doc.trailer.set("Root", catalog_id);

        let mut bytes = Vec::new();
        doc.save_to(&mut bytes).unwrap();
        bytes
    }

    #[test]
    fn test_showcase_gif_empty_document() {
        let options = ShowcaseGifOptions::default();
        let bytes = vec![];
        let result = showcase_gif_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_showcase_gif_single_page() {
        let pdf_bytes = create_sample_pdf("Clean Single Document", 1);
        let options = ShowcaseGifOptions {
            fps: Some(5),
            delay_centisecs: None,
            width: Some(600),
            transition_frames: Some(4),
        };

        let gif_bytes = showcase_gif_bytes(&pdf_bytes, &options).expect("showcase GIF generation failed");

        assert!(gif_bytes.len() > 6);
        assert_eq!(&gif_bytes[0..6], b"GIF89a");
    }

    #[test]
    fn test_showcase_gif_multi_page() {
        let pdf_bytes = create_sample_pdf("Clean Multi Page Document", 3);
        let options = ShowcaseGifOptions {
            fps: Some(4),
            delay_centisecs: None,
            width: Some(600),
            transition_frames: Some(4),
        };

        let gif_bytes = showcase_gif_bytes(&pdf_bytes, &options).expect("multi-page showcase GIF generation failed");

        assert!(gif_bytes.len() > 6);
        assert_eq!(&gif_bytes[0..6], b"GIF89a");
    }
}
