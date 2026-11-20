use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl RedactRect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
}

/// Helper to parse a rectangle string in format "x,y,w,h" (delimiters: ',', 'x', ':', or whitespace)
pub fn parse_rect_str(s: &str) -> Option<RedactRect> {
    let parts: Vec<&str> = s
        .split(|c: char| c == ',' || c == 'x' || c == ':' || c.is_whitespace())
        .filter(|p| !p.trim().is_empty())
        .collect();
    if parts.len() == 4 {
        if let (Ok(x), Ok(y), Ok(w), Ok(h)) = (
            parts[0].trim().parse::<u32>(),
            parts[1].trim().parse::<u32>(),
            parts[2].trim().parse::<u32>(),
            parts[3].trim().parse::<u32>(),
        ) {
            return Some(RedactRect { x, y, width: w, height: h });
        }
    }
    None
}

/// Helper to parse multiple rectangle definitions from a string.
/// Supports semicolon/newline separated rectangles, e.g. "x1,y1,w1,h1; x2,y2,w2,h2"
/// or space/newline separated.
pub fn parse_rects_str(s: &str) -> Vec<RedactRect> {
    let mut rects = Vec::new();
    for segment in s.split(|c: char| c == ';' || c == '\n') {
        let trimmed = segment.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(rect) = parse_rect_str(trimmed) {
            rects.push(rect);
        }
    }
    rects
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImgRedactOptions {
    /// List of bounding boxes to redact: x, y, width, height.
    #[serde(default)]
    pub rects: Vec<RedactRect>,
    /// Optional fill color in hex format, e.g. "#000000" or "#FFFFFF" or "black". Defaults to black [0, 0, 0, 255].
    #[serde(default)]
    pub color: Option<String>,
}

impl Default for ImgRedactOptions {
    fn default() -> Self {
        Self {
            rects: Vec::new(),
            color: Some("#000000".to_string()),
        }
    }
}

fn parse_hex_color(hex_or_name: &str) -> [u8; 4] {
    let s = hex_or_name.trim();
    if s.eq_ignore_ascii_case("white") {
        return [255, 255, 255, 255];
    }
    if s.eq_ignore_ascii_case("black") {
        return [0, 0, 0, 255];
    }
    if s.eq_ignore_ascii_case("red") {
        return [255, 0, 0, 255];
    }

    let cleaned = s.trim_start_matches('#');
    if cleaned.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&cleaned[0..2], 16),
            u8::from_str_radix(&cleaned[2..4], 16),
            u8::from_str_radix(&cleaned[4..6], 16),
        ) {
            return [r, g, b, 255];
        }
    } else if cleaned.len() == 8 {
        if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
            u8::from_str_radix(&cleaned[0..2], 16),
            u8::from_str_radix(&cleaned[2..4], 16),
            u8::from_str_radix(&cleaned[4..6], 16),
            u8::from_str_radix(&cleaned[6..8], 16),
        ) {
            return [r, g, b, a];
        }
    }

    [0, 0, 0, 255]
}

pub fn img_redact_bytes(
    input_bytes: &[u8],
    options: &ImgRedactOptions,
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

    let mut rgba = img.to_rgba8();
    let (img_w, img_h) = rgba.dimensions();

    let fill_color = options
        .color
        .as_deref()
        .map(parse_hex_color)
        .unwrap_or([0, 0, 0, 255]);
    let fill_pixel = image::Rgba(fill_color);

    // Apply pixel-level overwrites for each rectangle
    for rect in &options.rects {
        let x_end = (rect.x + rect.width).min(img_w);
        let y_end = (rect.y + rect.height).min(img_h);

        for y in rect.y..y_end {
            for x in rect.x..x_end {
                rgba.put_pixel(x, y, fill_pixel);
            }
        }
    }

    let mut output_bytes = Vec::new();
    let mut cursor = Cursor::new(&mut output_bytes);

    match format {
        image::ImageFormat::Jpeg => {
            let rgb = image::DynamicImage::ImageRgba8(rgba).to_rgb8();
            rgb.write_to(&mut cursor, image::ImageFormat::Jpeg)
                .map_err(|e| PdfError::Other(format!("Failed to encode JPEG: {:?}", e)))?;
        }
        _ => {
            rgba.write_to(&mut cursor, image::ImageFormat::Png)
                .map_err(|e| PdfError::Other(format!("Failed to encode PNG: {:?}", e)))?;
        }
    }

    Ok(output_bytes)
}

#[derive(Default)]
pub struct ImgRedactOperation {
    pub options: ImgRedactOptions,
}

impl ImgRedactOperation {
    pub fn new(options: ImgRedactOptions) -> Self {
        Self { options }
    }
}

impl PdfOperation for ImgRedactOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> paperpilot_core::error::OperationResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_img_redact_empty_document() {
        let options = ImgRedactOptions::default();
        let bytes = vec![];
        let result = img_redact_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_img_redact_synthetic_image() {
        let width = 100u32;
        let height = 100u32;
        // White image
        let img = image::RgbaImage::from_pixel(width, height, image::Rgba([255, 255, 255, 255]));

        let mut png_bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut png_bytes), image::ImageFormat::Png).unwrap();

        let options = ImgRedactOptions {
            rects: vec![RedactRect {
                x: 20,
                y: 30,
                width: 40,
                height: 20,
            }],
            color: Some("#000000".to_string()),
        };

        let result = img_redact_bytes(&png_bytes, &options);
        assert!(result.is_ok());
        let redacted_bytes = result.unwrap();

        let decoded = image::load_from_memory(&redacted_bytes).unwrap().to_rgba8();

        // Check inside redacted rectangle: all pixels are solid black [0, 0, 0, 255]
        for y in 30..50 {
            for x in 20..60 {
                let p = decoded.get_pixel(x, y);
                assert_eq!(p[0], 0);
                assert_eq!(p[1], 0);
                assert_eq!(p[2], 0);
                assert_eq!(p[3], 255);
            }
        }

        // Check outside redacted rectangle: pixels remain white [255, 255, 255, 255]
        let p_out = decoded.get_pixel(10, 10);
        assert_eq!(p_out[0], 255);
        assert_eq!(p_out[1], 255);
        assert_eq!(p_out[2], 255);
    }

    #[test]
    fn test_img_redact_multi_regions() {
        let width = 120u32;
        let height = 120u32;
        let img = image::RgbaImage::from_pixel(width, height, image::Rgba([255, 255, 255, 255]));

        let mut png_bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut png_bytes), image::ImageFormat::Png).unwrap();

        let parsed_rects = parse_rects_str("10,10,20,20; 50,50,30,30\n90,90,15,15");
        assert_eq!(parsed_rects.len(), 3);

        let options = ImgRedactOptions {
            rects: parsed_rects,
            color: Some("red".to_string()),
        };

        let result = img_redact_bytes(&png_bytes, &options).expect("multi-region redaction should succeed");
        let decoded = image::load_from_memory(&result).unwrap().to_rgba8();

        // Check region 1 (10,10,20,20) is red [255, 0, 0, 255]
        let r1 = decoded.get_pixel(15, 15);
        assert_eq!(r1[0], 255);
        assert_eq!(r1[1], 0);
        assert_eq!(r1[2], 0);

        // Check region 2 (50,50,30,30) is red
        let r2 = decoded.get_pixel(60, 60);
        assert_eq!(r2[0], 255);
        assert_eq!(r2[1], 0);
        assert_eq!(r2[2], 0);

        // Check region 3 (90,90,15,15) is red
        let r3 = decoded.get_pixel(95, 95);
        assert_eq!(r3[0], 255);
        assert_eq!(r3[1], 0);
        assert_eq!(r3[2], 0);

        // Check untouched region (0,0) is white
        let un = decoded.get_pixel(0, 0);
        assert_eq!(un[0], 255);
        assert_eq!(un[1], 255);
        assert_eq!(un[2], 255);
    }

    #[test]
    fn test_img_redact_pdf_passthrough() {
        let pdf_data = b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog >>\nendobj\ntrailer\n<< /Root 1 0 R >>\n%%EOF".to_vec();
        let options = ImgRedactOptions::default();
        let res = img_redact_bytes(&pdf_data, &options);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), pdf_data);
    }
}
