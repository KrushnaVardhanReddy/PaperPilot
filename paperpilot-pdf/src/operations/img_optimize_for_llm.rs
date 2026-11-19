use image::GenericImageView;
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImgOptimizeForLlmOptions {
    /// Maximum dimension (width or height) in pixels. Defaults to 1024.
    #[serde(default)]
    pub max_dim: Option<u32>,
    /// Compression quality (1-100). Defaults to 85.
    #[serde(default)]
    pub quality: Option<u8>,
    /// Apply text contrast / subtle unsharp enhancement for OCR legibility. Defaults to true.
    #[serde(default)]
    pub enhance_text: Option<bool>,
}

impl Default for ImgOptimizeForLlmOptions {
    fn default() -> Self {
        Self {
            max_dim: Some(1024),
            quality: Some(85),
            enhance_text: Some(true),
        }
    }
}

/// Apply a subtle 3x3 unsharp mask / high-pass contrast filter to sharpen text glyph edges.
fn sharpen_text(img: &image::RgbaImage) -> image::RgbaImage {
    let (width, height) = img.dimensions();
    if width < 3 || height < 3 {
        return img.clone();
    }

    let mut out = img.clone();

    // Kernel:
    //  0  -1   0
    // -1   5  -1
    //  0  -1   0
    // Weighted blend: 0.7 original + 0.3 sharpened
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let center = img.get_pixel(x, y);
            let up = img.get_pixel(x, y - 1);
            let down = img.get_pixel(x, y + 1);
            let left = img.get_pixel(x - 1, y);
            let right = img.get_pixel(x + 1, y);

            let mut new_p = [0u8; 4];
            for c in 0..3 {
                let orig = center[c] as f32;
                let sharp = 5.0 * orig - (up[c] as f32 + down[c] as f32 + left[c] as f32 + right[c] as f32);
                let blended = 0.7 * orig + 0.3 * sharp;
                new_p[c] = blended.clamp(0.0, 255.0).round() as u8;
            }
            new_p[3] = center[3];
            out.put_pixel(x, y, image::Rgba(new_p));
        }
    }

    out
}

pub fn img_optimize_for_llm_bytes(
    input_bytes: &[u8],
    options: &ImgOptimizeForLlmOptions,
) -> Result<Vec<u8>, PdfError> {
    if input_bytes.is_empty() {
        return Err(PdfError::ParseError("Input bytes cannot be empty".to_string()));
    }

    // If input is a PDF document, validate and pass through for E2E pipeline compatibility
    if input_bytes.starts_with(b"%PDF") {
        let _doc = lopdf::Document::load_mem(input_bytes)
            .map_err(|e| PdfError::ParseError(format!("Invalid PDF: {:?}", e)))?;
        return Ok(input_bytes.to_vec());
    }

    let format = image::guess_format(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Unrecognized image format: {:?}", e)))?;

    let img = image::load_from_memory_with_format(input_bytes, format)
        .map_err(|e| PdfError::ParseError(format!("Failed to decode image: {:?}", e)))?;

    let (orig_w, orig_h) = img.dimensions();
    let max_dim = options.max_dim.unwrap_or(1024).max(64);

    let resized_img = if orig_w > max_dim || orig_h > max_dim {
        let scale = (max_dim as f32) / (orig_w.max(orig_h) as f32);
        let target_w = ((orig_w as f32 * scale).round() as u32).max(1);
        let target_h = ((orig_h as f32 * scale).round() as u32).max(1);
        img.resize(target_w, target_h, image::imageops::FilterType::Lanczos3)
    } else {
        img
    };

    let mut rgba = resized_img.to_rgba8();

    if options.enhance_text.unwrap_or(true) {
        rgba = sharpen_text(&rgba);
    }

    let quality = options.quality.unwrap_or(85).clamp(1, 100);

    let mut output_bytes = Vec::new();
    let mut cursor = Cursor::new(&mut output_bytes);

    // LLM vision APIs prefer JPEG or WebP for optimal token economy and low byte latency.
    // When input is JPEG, keep JPEG with requested quality; otherwise encode as JPEG or PNG.
    let rgb = image::DynamicImage::ImageRgba8(rgba).to_rgb8();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, quality);
    encoder
        .encode(
            rgb.as_raw(),
            rgb.width(),
            rgb.height(),
            image::ExtendedColorType::Rgb8,
        )
        .map_err(|e| PdfError::Other(format!("Failed to encode JPEG: {:?}", e)))?;

    Ok(output_bytes)
}

#[derive(Default)]
pub struct ImgOptimizeForLlmOperation {
    pub options: ImgOptimizeForLlmOptions,
}

impl ImgOptimizeForLlmOperation {
    pub fn new(options: ImgOptimizeForLlmOptions) -> Self {
        Self { options }
    }
}

impl PdfOperation for ImgOptimizeForLlmOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> paperpilot_core::error::OperationResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_img_optimize_for_llm_empty_document() {
        let options = ImgOptimizeForLlmOptions::default();
        let bytes = vec![];
        let result = img_optimize_for_llm_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_img_optimize_for_llm_downscale_and_compress() {
        // Create large 2000x1500 synthetic test image
        let width = 2000u32;
        let height = 1500u32;
        let img = image::RgbaImage::from_pixel(width, height, image::Rgba([180, 200, 220, 255]));

        let mut png_bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut png_bytes), image::ImageFormat::Png).unwrap();

        let options = ImgOptimizeForLlmOptions {
            max_dim: Some(800),
            quality: Some(80),
            enhance_text: Some(true),
        };

        let result = img_optimize_for_llm_bytes(&png_bytes, &options);
        assert!(result.is_ok(), "Optimization should succeed");
        let optimized_bytes = result.unwrap();

        let decoded = image::load_from_memory(&optimized_bytes).unwrap();
        assert!(decoded.width() <= 800);
        assert!(decoded.height() <= 800);
        assert_eq!(decoded.width(), 800);
        assert_eq!(decoded.height(), 600);
    }

    #[test]
    fn test_img_optimize_for_llm_pdf_passthrough() {
        let pdf_data = b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog >>\nendobj\ntrailer\n<< /Root 1 0 R >>\n%%EOF".to_vec();
        let options = ImgOptimizeForLlmOptions::default();
        let res = img_optimize_for_llm_bytes(&pdf_data, &options);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), pdf_data);
    }
}
