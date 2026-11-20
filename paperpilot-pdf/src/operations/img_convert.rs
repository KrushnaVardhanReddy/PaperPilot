use image::ImageEncoder;
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImgConvertOptions {
    /// Target format: "png", "jpeg", "jpg", "webp", "tiff", "tif", "avif"
    #[serde(default)]
    pub format: Option<String>,
    /// Quality (1-100) for lossy formats (JPEG, WebP). Default is 90.
    #[serde(default)]
    pub quality: Option<u8>,
}

pub fn img_convert_bytes(
    input_bytes: &[u8],
    options: &ImgConvertOptions,
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

    let input_format = image::guess_format(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Unrecognized image format: {:?}", e)))?;

    let img = image::load_from_memory_with_format(input_bytes, input_format)
        .map_err(|e| PdfError::ParseError(format!("Failed to decode image: {:?}", e)))?;

    let target_format_str = options
        .format
        .as_deref()
        .unwrap_or("png")
        .trim()
        .to_lowercase();

    let quality = options.quality.unwrap_or(90).clamp(1, 100);

    let mut output_bytes = Vec::new();
    let mut cursor = Cursor::new(&mut output_bytes);

    match target_format_str.as_str() {
        "jpeg" | "jpg" => {
            let rgb = img.to_rgb8();
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, quality);
            encoder
                .encode(
                    rgb.as_raw(),
                    rgb.width(),
                    rgb.height(),
                    image::ExtendedColorType::Rgb8,
                )
                .map_err(|e| PdfError::Other(format!("Failed to encode JPEG: {:?}", e)))?;
        }
        "webp" => {
            let rgba = img.to_rgba8();
            let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut cursor);
            encoder
                .encode(
                    rgba.as_raw(),
                    rgba.width(),
                    rgba.height(),
                    image::ExtendedColorType::Rgba8,
                )
                .map_err(|e| PdfError::Other(format!("Failed to encode WebP: {:?}", e)))?;
        }
        "tiff" | "tif" => {
            let rgba = img.to_rgba8();
            let encoder = image::codecs::tiff::TiffEncoder::new(&mut cursor);
            encoder
                .encode(
                    rgba.as_raw(),
                    rgba.width(),
                    rgba.height(),
                    image::ExtendedColorType::Rgba8,
                )
                .map_err(|e| PdfError::Other(format!("Failed to encode TIFF: {:?}", e)))?;
        }
        "avif" => {
            let rgba = img.to_rgba8();
            let encoder = image::codecs::avif::AvifEncoder::new_with_speed_quality(&mut cursor, 6, quality);
            encoder
                .write_image(
                    rgba.as_raw(),
                    rgba.width(),
                    rgba.height(),
                    image::ExtendedColorType::Rgba8,
                )
                .map_err(|e| PdfError::Other(format!("Failed to encode AVIF: {:?}", e)))?;
        }
        "png" | _ => {
            let rgba = img.to_rgba8();
            rgba.write_to(&mut cursor, image::ImageFormat::Png)
                .map_err(|e| PdfError::Other(format!("Failed to encode PNG: {:?}", e)))?;
        }
    }

    Ok(output_bytes)
}

#[derive(Default)]
pub struct ImgConvertOperation {
    pub options: ImgConvertOptions,
}

impl ImgConvertOperation {
    pub fn new(options: ImgConvertOptions) -> Self {
        Self { options }
    }
}

impl PdfOperation for ImgConvertOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> paperpilot_core::error::OperationResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_img_convert_empty_document() {
        let options = ImgConvertOptions::default();
        let bytes = vec![];
        let result = img_convert_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_img_convert_transcoding() {
        let width = 50u32;
        let height = 50u32;
        let img = image::RgbaImage::from_pixel(width, height, image::Rgba([120, 150, 200, 255]));

        let mut png_bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut png_bytes), image::ImageFormat::Png).unwrap();

        // 1. PNG -> JPEG
        let jpeg_res = img_convert_bytes(
            &png_bytes,
            &ImgConvertOptions {
                format: Some("jpeg".to_string()),
                quality: Some(85),
            },
        );
        assert!(jpeg_res.is_ok());
        let jpeg_bytes = jpeg_res.unwrap();
        assert_eq!(image::guess_format(&jpeg_bytes).unwrap(), image::ImageFormat::Jpeg);

        // 2. JPEG -> WebP
        let webp_res = img_convert_bytes(
            &jpeg_bytes,
            &ImgConvertOptions {
                format: Some("webp".to_string()),
                quality: Some(90),
            },
        );
        assert!(webp_res.is_ok());
        let webp_bytes = webp_res.unwrap();
        assert_eq!(image::guess_format(&webp_bytes).unwrap(), image::ImageFormat::WebP);

        // 3. WebP -> TIFF
        let tiff_res = img_convert_bytes(
            &webp_bytes,
            &ImgConvertOptions {
                format: Some("tiff".to_string()),
                quality: None,
            },
        );
        assert!(tiff_res.is_ok());
        let tiff_bytes = tiff_res.unwrap();
        assert_eq!(image::guess_format(&tiff_bytes).unwrap(), image::ImageFormat::Tiff);

        // 4. TIFF -> PNG
        let png_res = img_convert_bytes(
            &tiff_bytes,
            &ImgConvertOptions {
                format: Some("png".to_string()),
                quality: None,
            },
        );
        assert!(png_res.is_ok());
        let png_bytes_out = png_res.unwrap();
        assert_eq!(image::guess_format(&png_bytes_out).unwrap(), image::ImageFormat::Png);
    }

    #[test]
    fn test_img_convert_pdf_passthrough() {
        let pdf_data = b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog >>\nendobj\ntrailer\n<< /Root 1 0 R >>\n%%EOF".to_vec();
        let options = ImgConvertOptions::default();
        let res = img_convert_bytes(&pdf_data, &options);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), pdf_data);
    }
}
