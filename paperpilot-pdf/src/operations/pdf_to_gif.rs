use image::imageops::FilterType;
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

/// Options for converting PDF pages to an animated looping GIF.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfToGifOptions {
    /// Frames per second. If provided, delay is calculated as `(100 / fps).max(1)`.
    pub fps: Option<u8>,
    /// Delay between frames in centiseconds (1/100ths of a second). Default is 100 (1.0 second).
    pub delay_centisecs: Option<u16>,
    /// Maximum number of pages to render. None means render all pages.
    pub max_pages: Option<usize>,
    /// Target frame width in pixels. Default is 800. Height will scale proportionally.
    pub width: Option<u32>,
}

impl Default for PdfToGifOptions {
    fn default() -> Self {
        Self {
            fps: Some(1),
            delay_centisecs: Some(100),
            max_pages: None,
            width: Some(800),
        }
    }
}

/// Convert multi-page PDF documents into an animated looping GIF preview.
pub fn pdf_to_gif_bytes(
    input_bytes: &[u8],
    options: &PdfToGifOptions,
) -> Result<Vec<u8>, PdfError> {
    if input_bytes.is_empty() {
        return Err(PdfError::ParseError("Input PDF bytes cannot be empty".to_string()));
    }

    let pdf = hayro_syntax::Pdf::new(input_bytes.to_vec())
        .map_err(|e| PdfError::ParseError(format!("Failed to parse PDF: {:?}", e)))?;

    let pages = pdf.pages();
    let total_pages = pages.len();
    if total_pages == 0 {
        return Err(PdfError::ParseError("PDF contains no pages".to_string()));
    }

    let page_limit = options.max_pages.unwrap_or(total_pages).min(total_pages);
    if page_limit == 0 {
        return Err(PdfError::ParseError("Page limit is zero".to_string()));
    }

    let target_width = options.width.unwrap_or(800).max(1);

    // Determine frame delay in centiseconds (1/100 s)
    let delay_centisecs = if let Some(fps) = options.fps {
        if fps > 0 {
            (100 / fps as u16).max(1)
        } else {
            options.delay_centisecs.unwrap_or(100).max(1)
        }
    } else {
        options.delay_centisecs.unwrap_or(100).max(1)
    };

    // Render each page into an RgbaImage (blended on white background)
    let mut frames_rgba: Vec<image::RgbaImage> = Vec::with_capacity(page_limit);

    for i in 0..page_limit {
        let page = pages
            .get(i)
            .ok_or_else(|| PdfError::ParseError(format!("Failed to get page {}", i + 1)))?;

        let (base_w, _base_h) = page.render_dimensions();
        let scale = if base_w > 0.0 {
            (target_width as f32 / base_w).max(0.01)
        } else {
            1.0
        };

        let cache = hayro::RenderCache::new();
        let render_settings = hayro::RenderSettings::default();
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
        let rgba_raw = image::RgbaImage::from_raw(pw, ph, raw_data.to_vec())
            .ok_or_else(|| PdfError::ParseError("Failed to construct RgbaImage from rendered pixmap".to_string()))?;

        // Composite over white background (PDF pages are typically transparent by default in hayro)
        let mut blended = image::RgbaImage::new(pw, ph);
        for (x, y, pixel) in rgba_raw.enumerate_pixels() {
            let alpha = pixel[3] as f32 / 255.0;
            let r = ((pixel[0] as f32 * alpha) + (255.0 * (1.0 - alpha))).round() as u8;
            let g = ((pixel[1] as f32 * alpha) + (255.0 * (1.0 - alpha))).round() as u8;
            let b = ((pixel[2] as f32 * alpha) + (255.0 * (1.0 - alpha))).round() as u8;
            blended.put_pixel(x, y, image::Rgba([r, g, b, 255]));
        }

        // Downscale or ensure exact target width if needed
        let final_frame = if pw != target_width {
            let target_height = ((ph as f32 * (target_width as f32 / pw as f32)).round() as u32).max(1);
            image::imageops::resize(&blended, target_width, target_height, FilterType::Lanczos3)
        } else {
            blended
        };

        frames_rgba.push(final_frame);
    }

    if frames_rgba.is_empty() {
        return Err(PdfError::ParseError("No frames were successfully rendered from PDF".to_string()));
    }

    // Canvas size equals first frame size
    let gif_width = frames_rgba[0].width() as u16;
    let gif_height = frames_rgba[0].height() as u16;

    let mut gif_bytes = Vec::new();
    {
        let mut encoder = gif::Encoder::new(&mut gif_bytes, gif_width, gif_height, &[])
            .map_err(|e| PdfError::Other(format!("Failed to initialize GIF encoder: {:?}", e)))?;

        encoder
            .set_repeat(gif::Repeat::Infinite)
            .map_err(|e| PdfError::Other(format!("Failed to set GIF repeat mode: {:?}", e)))?;

        for mut frame_img in frames_rgba {
            // Resize frame to match canvas dimensions if subsequent pages vary slightly
            let (fw, fh) = (frame_img.width() as u16, frame_img.height() as u16);
            if fw != gif_width || fh != gif_height {
                frame_img = image::imageops::resize(
                    &frame_img,
                    gif_width as u32,
                    gif_height as u32,
                    FilterType::Triangle,
                );
            }

            // High-quality NeuQuant palette quantization (speed = 10 for balanced visual quality and speed)
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
pub struct PdfToGifOperation {
    options: PdfToGifOptions,
}

impl PdfToGifOperation {
    pub fn new(options: PdfToGifOptions) -> Self {
        Self { options }
    }
}

impl PdfOperation for PdfToGifOperation {
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

        let _gif_bytes = pdf_to_gif_bytes(&bytes, &self.options)?;

        Err(PdfError::UnsupportedOperation(
            "Use pdf_to_gif_bytes directly. In-place mutation is unsupported for binary GIF export.".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::dictionary;
    use lopdf::Document;

    fn create_test_multipage_pdf(page_texts: &[&str]) -> Vec<u8> {
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

        for text in page_texts {
            let content = lopdf::content::Content {
                operations: vec![
                    lopdf::content::Operation::new("BT", vec![]),
                    lopdf::content::Operation::new("Tf", vec!["F1".into(), 16.into()]),
                    lopdf::content::Operation::new("Td", vec![50.into(), 100.into()]),
                    lopdf::content::Operation::new(
                        "Tj",
                        vec![lopdf::Object::string_literal(*text)],
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
                "MediaBox" => vec![0.into(), 0.into(), 300.into(), 200.into()],
            });

            page_ids.push(page_id.into());
        }

        doc.set_object(
            pages_id,
            dictionary! {
                "Type" => "Pages",
                "Kids" => page_ids,
                "Count" => page_texts.len() as i64,
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
    fn test_pdf_to_gif_empty_document() {
        let options = PdfToGifOptions::default();
        let bytes = vec![];
        let result = pdf_to_gif_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_pdf_to_gif_multipage_success() {
        let pdf_bytes = create_test_multipage_pdf(&["Page 1 Preview", "Page 2 Content", "Page 3 Summary"]);
        let options = PdfToGifOptions {
            fps: Some(2),
            delay_centisecs: None,
            max_pages: Some(3),
            width: Some(300),
        };

        let gif_bytes = pdf_to_gif_bytes(&pdf_bytes, &options).expect("GIF encoding should succeed");

        // Verify valid GIF header (GIF89a)
        assert!(gif_bytes.len() > 6);
        assert_eq!(&gif_bytes[0..6], b"GIF89a");

        // Verify compactness: 3 synthetic pages at 300px should easily be < 100 KB
        assert!(gif_bytes.len() < 100_000, "GIF size {} was too large", gif_bytes.len());
    }

    #[test]
    fn test_pdf_to_gif_max_pages_limit() {
        let pdf_bytes = create_test_multipage_pdf(&["Page 1", "Page 2", "Page 3", "Page 4"]);
        let options = PdfToGifOptions {
            fps: None,
            delay_centisecs: Some(50),
            max_pages: Some(2),
            width: Some(200),
        };

        let gif_bytes = pdf_to_gif_bytes(&pdf_bytes, &options).expect("GIF encoding should succeed");
        assert_eq!(&gif_bytes[0..6], b"GIF89a");
    }
}
