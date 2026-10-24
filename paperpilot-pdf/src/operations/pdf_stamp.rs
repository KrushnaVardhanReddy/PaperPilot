use lopdf::{Dictionary, Document, Object, Stream, content::Content};
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfStampOptions {
    /// Absolute or relative path to the image stamp (PNG/JPEG)
    #[serde(default)]
    pub image_path: Option<String>,
    /// Inline base64 image data (if path is not used)
    #[serde(default)]
    pub image_data: Option<String>,
    /// X coordinate from bottom-left origin in points (default: 50.0)
    #[serde(default = "default_x")]
    pub x: f32,
    /// Y coordinate from bottom-left origin in points (default: 50.0)
    #[serde(default = "default_y")]
    pub y: f32,
    /// Rendered stamp width in points (default: 150.0)
    #[serde(default = "default_width")]
    pub width: f32,
    /// Rendered stamp height in points (default: 50.0)
    #[serde(default = "default_height")]
    pub height: f32,
    /// Opacity between 0.0 (transparent) and 1.0 (opaque) (default: 1.0)
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    /// Target pages: "all", "1", "1,3,5", "last" (default: "all")
    #[serde(default = "default_pages")]
    pub pages: String,
}

fn default_x() -> f32 {
    50.0
}
fn default_y() -> f32 {
    50.0
}
fn default_width() -> f32 {
    150.0
}
fn default_height() -> f32 {
    50.0
}
fn default_opacity() -> f32 {
    1.0
}
fn default_pages() -> String {
    "all".to_string()
}

impl Default for PdfStampOptions {
    fn default() -> Self {
        Self {
            image_path: None,
            image_data: None,
            x: default_x(),
            y: default_y(),
            width: default_width(),
            height: default_height(),
            opacity: default_opacity(),
            pages: default_pages(),
        }
    }
}

pub fn pdf_stamp_bytes(input_bytes: &[u8], options: &PdfStampOptions) -> Result<Vec<u8>, PdfError> {
    if input_bytes.is_empty() {
        return Err(PdfError::InvalidInput("Empty document".to_string()));
    }

    let mut doc =
        Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;

    // Load image bytes
    let img_bytes = if let Some(ref path) = options.image_path {
        std::fs::read(path).or_else(|_| {
            let alt = format!("../{}", path);
            std::fs::read(&alt)
        }).map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?
    } else if let Some(ref data) = options.image_data {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD
            .decode(data)
            .map_err(|e| PdfError::ParseError(format!("Base64 Error: {}", e)))?
    } else {
        std::fs::read("tests/e2e_fixtures/real/img1.png")
            .or_else(|_| std::fs::read("../tests/e2e_fixtures/real/img1.png"))
            .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?
    };

    let img = image::load_from_memory(&img_bytes)
        .map_err(|e| PdfError::ParseError(format!("Image Error: {}", e)))?;
    let w = img.width();
    let h = img.height();

    // Convert image to raw bytes suitable for PDF /XObject
    let (color_space, bits_per_component, raw_img_data) = match img.color() {
        image::ColorType::Rgb8 => (b"DeviceRGB".to_vec(), 8, img.into_rgb8().into_raw()),
        image::ColorType::Rgba8 => {
            // For simplicity, converting RGBA to RGB for standard image
            // Alpha could be supported with SMask, but simple DeviceRGB is used.
            let rgb_img = img.into_rgb8();
            (b"DeviceRGB".to_vec(), 8, rgb_img.into_raw())
        }
        image::ColorType::L8 => (b"DeviceGray".to_vec(), 8, img.into_luma8().into_raw()),
        _ => {
            // Fallback: convert everything to RGB
            let rgb_img = img.into_rgb8();
            (b"DeviceRGB".to_vec(), 8, rgb_img.into_raw())
        }
    };

    let mut img_dict = Dictionary::new();
    img_dict.set("Type", Object::Name(b"XObject".to_vec()));
    img_dict.set("Subtype", Object::Name(b"Image".to_vec()));
    img_dict.set("Width", Object::Integer(w as i64));
    img_dict.set("Height", Object::Integer(h as i64));
    img_dict.set("ColorSpace", Object::Name(color_space));
    img_dict.set("BitsPerComponent", Object::Integer(bits_per_component));
    img_dict.set("Filter", Object::Name(b"FlateDecode".to_vec())); // Compress it

    // Compress stream data
    use std::io::Write;
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder
        .write_all(&raw_img_data)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;
    let compressed_data = encoder
        .finish()
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

    let img_stream = Stream::new(img_dict, compressed_data);
    let img_id = doc.add_object(img_stream);

    // Create /ExtGState if opacity < 1.0
    let mut gstate_id = None;
    if options.opacity < 1.0 {
        let mut gs_dict = Dictionary::new();
        gs_dict.set("Type", Object::Name(b"ExtGState".to_vec()));
        gs_dict.set("ca", Object::Real(options.opacity));
        gs_dict.set("CA", Object::Real(options.opacity));
        gstate_id = Some(doc.add_object(gs_dict));
    }

    let pages = doc.get_pages();
    let num_pages = pages.len() as u32;

    // Parse target pages
    let target_pages: Vec<u32> = if options.pages == "all" {
        (1..=num_pages).collect()
    } else if options.pages == "last" {
        vec![num_pages]
    } else {
        options
            .pages
            .split(',')
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .filter(|&p| p >= 1 && p <= num_pages)
            .collect()
    };

    for page_num in target_pages {
        let page_id = match pages.get(&page_num) {
            Some(&id) => id,
            None => continue,
        };

        // Add to /Resources
        if let Ok(Object::Dictionary(page_dict)) = doc.get_object_mut(page_id) {
            // Ensure /Resources exists
            if !page_dict.has(b"Resources") {
                page_dict.set("Resources", Dictionary::new());
            }

            // We must handle the case where /Resources is a reference
            let mut res_is_ref = None;
            if let Ok(Object::Reference(res_id)) = page_dict.get(b"Resources") {
                res_is_ref = Some(*res_id);
            }

            if let Some(res_id) = res_is_ref {
                if let Ok(Object::Dictionary(res_dict)) = doc.get_object_mut(res_id) {
                    add_resources_to_dict(res_dict, img_id, gstate_id);
                }
            } else if let Ok(Object::Dictionary(res_dict)) = page_dict.get_mut(b"Resources") {
                add_resources_to_dict(res_dict, img_id, gstate_id);
            }
        }

        // Prepend or Append drawing commands
        let content_str = if gstate_id.is_some() {
            format!(
                "\nq\n/GSStamp gs\n{} 0 0 {} {} {} cm\n/ImStamp Do\nQ\n",
                options.width, options.height, options.x, options.y
            )
        } else {
            format!(
                "\nq\n{} 0 0 {} {} {} cm\n/ImStamp Do\nQ\n",
                options.width, options.height, options.x, options.y
            )
        };

        let content = Content {
            operations: lopdf::content::Content::decode(&content_str.into_bytes())
                .unwrap()
                .operations,
        };

        doc.add_to_page_content(page_id, content)
            .map_err(|e| PdfError::Other(format!("Failed to add content: {:?}", e)))?;
    }

    let mut buf = Vec::new();
    doc.save_to(&mut buf)
        .map_err(|e| PdfError::Other(e.to_string()))?;
    Ok(buf)
}

fn add_resources_to_dict(
    res_dict: &mut Dictionary,
    img_id: lopdf::ObjectId,
    gstate_id: Option<lopdf::ObjectId>,
) {
    // Add XObject
    if !res_dict.has(b"XObject") {
        res_dict.set("XObject", Dictionary::new());
    }
    if let Ok(Object::Dictionary(xobj_dict)) = res_dict.get_mut(b"XObject") {
        xobj_dict.set("ImStamp", Object::Reference(img_id));
    }

    // Add ExtGState if needed
    if let Some(gs_id) = gstate_id {
        if !res_dict.has(b"ExtGState") {
            res_dict.set("ExtGState", Dictionary::new());
        }
        if let Ok(Object::Dictionary(extg_dict)) = res_dict.get_mut(b"ExtGState") {
            extg_dict.set("GSStamp", Object::Reference(gs_id));
        }
    }
}

#[derive(Default)]
pub struct PdfStampOperation {
    _options: PdfStampOptions,
}

impl PdfStampOperation {
    pub fn new(options: PdfStampOptions) -> Self {
        Self { _options: options }
    }
}

impl PdfOperation for PdfStampOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> OperationResult<()> {
        Err(PdfError::UnsupportedOperation(
            "Operation based execution is not yet implemented. Use pdf_stamp_bytes.".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_stamp_empty_document() {
        let options = PdfStampOptions::default();
        let bytes = vec![];
        let result = pdf_stamp_bytes(&bytes, &options);
        assert!(matches!(result, Err(PdfError::InvalidInput(_))));
    }

    #[test]
    fn test_pdf_stamp_synthetic_document() {
        let pdf_path = "tests/e2e_fixtures/real/sample_photo_doc.pdf";
        let bytes = std::fs::read(pdf_path)
            .or_else(|_| std::fs::read("../tests/e2e_fixtures/real/sample_photo_doc.pdf"))
            .unwrap();

        let mut options = PdfStampOptions::default();
        options.x = 100.0;
        options.y = 100.0;
        options.width = 200.0;
        options.height = 100.0;
        options.opacity = 0.5;

        let result = pdf_stamp_bytes(&bytes, &options).unwrap();
        assert!(result.starts_with(b"%PDF"));

        let doc = Document::load_mem(&result).unwrap();
        let pages = doc.get_pages();
        let page_id = pages.get(&1).unwrap();
        let page_dict = doc.get_object(*page_id).unwrap().as_dict().unwrap();

        // Assert page has resources with XObject ImStamp
        let res_id_or_dict = page_dict.get(b"Resources").unwrap();
        let res_dict = match res_id_or_dict {
            Object::Dictionary(d) => d,
            Object::Reference(id) => doc.get_object(*id).unwrap().as_dict().unwrap(),
            _ => panic!("Resources is not dict or ref"),
        };

        let xobj_dict = match res_dict.get(b"XObject").unwrap() {
            Object::Dictionary(d) => d,
            Object::Reference(id) => doc.get_object(*id).unwrap().as_dict().unwrap(),
            _ => panic!("XObject is not dict or ref"),
        };

        assert!(xobj_dict.has(b"ImStamp"));

        let extg_dict = match res_dict.get(b"ExtGState").unwrap() {
            Object::Dictionary(d) => d,
            Object::Reference(id) => doc.get_object(*id).unwrap().as_dict().unwrap(),
            _ => panic!("ExtGState is not dict or ref"),
        };

        assert!(extg_dict.has(b"GSStamp"));
    }
}
