use crate::document::LopdfDocument;
use lopdf::dictionary;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct WatermarkOperation {
    pub text: String,
    pub angle: f32,
    pub opacity: f32,
    pub font_size: Option<f32>,
    pub color: Option<String>,
    pub pages: Option<Vec<u32>>,
}

impl WatermarkOperation {
    pub fn new(text: String) -> Self {
        Self {
            text,
            angle: 45.0,
            opacity: 0.2,
            font_size: None,
            color: None,
            pages: None,
        }
    }
}

fn parse_color(color: &str) -> (f32, f32, f32) {
    match color.to_lowercase().as_str() {
        "red" => (1.0, 0.0, 0.0),
        "blue" => (0.0, 0.0, 1.0),
        "green" => (0.0, 1.0, 0.0),
        "black" => (0.0, 0.0, 0.0),
        "white" => (1.0, 1.0, 1.0),
        "gray" => (0.5, 0.5, 0.5),
        _ => (0.5, 0.5, 0.5), // default to gray
    }
}

#[allow(clippy::collapsible_if)]
impl PdfOperation for WatermarkOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document is not a LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;

        let font_id = inner.add_object(lopdf::dictionary!(
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        ));

        let extgstate_id = inner.add_object(lopdf::dictionary!(
            "Type" => "ExtGState",
            "ca" => self.opacity,
            "CA" => self.opacity,
        ));

        let mut pages_to_update = Vec::new();
        for (page_number, object_id) in inner.get_pages() {
            if let Some(pages) = &self.pages {
                if !pages.contains(&page_number) {
                    continue;
                }
            }
            pages_to_update.push(object_id);
        }

        for object_id in pages_to_update {
            let mut page_width = 612.0;
            let mut page_height = 792.0;

            if let Ok(lopdf::Object::Dictionary(page_dict)) = inner.get_object_mut(object_id) {
                if let Ok(media_box) = page_dict.get(b"MediaBox") {
                    if let Ok(arr) = media_box.as_array() {
                        if arr.len() == 4 {
                            if let (Ok(x2), Ok(y2)) = (arr[2].as_f32().or_else(|_| arr[2].as_i64().map(|v| v as f32)), arr[3].as_f32().or_else(|_| arr[3].as_i64().map(|v| v as f32))) {
                                page_width = x2;
                                page_height = y2;
                            }
                        }
                    }
                }

                let mut new_resources = None;
                if let Ok(res) = page_dict.get_mut(b"Resources") {
                    if let lopdf::Object::Dictionary(res_dict) = res {
                        let mut fonts = res_dict
                            .get(b"Font")
                            .and_then(|f| f.as_dict())
                            .cloned()
                            .unwrap_or_else(|_| lopdf::Dictionary::new());
                        fonts.set("F1", lopdf::Object::Reference(font_id));
                        res_dict.set("Font", lopdf::Object::Dictionary(fonts));

                        let mut ext_gstates = res_dict
                            .get(b"ExtGState")
                            .and_then(|e| e.as_dict())
                            .cloned()
                            .unwrap_or_else(|_| lopdf::Dictionary::new());
                        ext_gstates.set("GS1", lopdf::Object::Reference(extgstate_id));
                        res_dict.set("ExtGState", lopdf::Object::Dictionary(ext_gstates));
                    }
                } else {
                    let mut fonts = lopdf::Dictionary::new();
                    fonts.set("F1", lopdf::Object::Reference(font_id));
                    let mut ext_gstates = lopdf::Dictionary::new();
                    ext_gstates.set("GS1", lopdf::Object::Reference(extgstate_id));
                    let mut res_dict = lopdf::Dictionary::new();
                    res_dict.set("Font", lopdf::Object::Dictionary(fonts));
                    res_dict.set("ExtGState", lopdf::Object::Dictionary(ext_gstates));
                    new_resources = Some(lopdf::Object::Dictionary(res_dict));
                }
                if let Some(res) = new_resources {
                    page_dict.set("Resources", res);
                }
            }

            let escaped_text = self.text.replace("(", "\\(").replace(")", "\\)");

            let cx = page_width / 2.0;
            let cy = page_height / 2.0;

            let font_size = self.font_size.unwrap_or(54.0);
            let approx_text_width = (self.text.len() as f32) * (font_size * 0.5);
            let offset_x = -approx_text_width / 2.0;
            let offset_y = -font_size / 2.0;

            let theta = self.angle * std::f32::consts::PI / 180.0;
            let cos_t = theta.cos();
            let sin_t = theta.sin();

            let color = self.color.as_deref().unwrap_or("gray");
            let (r, g, b) = parse_color(color);

            let content = format!(
                "q
/GS1 gs
{r} {g} {b} rg
BT
/F1 {font_size} Tf
{cos_t} {sin_t} {minus_sin_t} {cos_t} {cx} {cy} Tm
{offset_x} {offset_y} Td
({text}) Tj
ET
Q
",
                r = r,
                g = g,
                b = b,
                font_size = font_size,
                cos_t = cos_t,
                sin_t = sin_t,
                minus_sin_t = -sin_t,
                cx = cx,
                cy = cy,
                offset_x = offset_x,
                offset_y = offset_y,
                text = escaped_text
            );

            let content_stream =
                lopdf::Stream::new(lopdf::Dictionary::new(), content.as_bytes().to_vec());
            let stream_id = inner.add_object(content_stream);

            if let Ok(lopdf::Object::Dictionary(dict)) = inner.get_object_mut(object_id) {
                if let Ok(contents) = dict.get_mut(b"Contents") {
                    match contents {
                        lopdf::Object::Reference(ref_id) => {
                            let old_ref = *ref_id;
                            *contents = lopdf::Object::Array(vec![
                                lopdf::Object::Reference(old_ref),
                                lopdf::Object::Reference(stream_id),
                            ]);
                        }
                        lopdf::Object::Array(arr) => {
                            arr.push(lopdf::Object::Reference(stream_id));
                        }
                        _ => {}
                    }
                } else {
                    dict.set("Contents", lopdf::Object::Reference(stream_id));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::Document as LopdfInnerDocument;

    fn create_test_document() -> LopdfDocument {
        let mut inner = LopdfInnerDocument::with_version("1.5");
        let pages_id = inner.new_object_id();
        let mut page_dict = lopdf::Dictionary::new();
        page_dict.set("Type", lopdf::Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", lopdf::Object::Reference(pages_id));
        let page_id = inner.add_object(page_dict);

        let mut pages_dict = lopdf::Dictionary::new();
        pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
        pages_dict.set(
            "Kids",
            lopdf::Object::Array(vec![lopdf::Object::Reference(page_id)]),
        );
        pages_dict.set("Count", lopdf::Object::Integer(1));
        inner.set_object(pages_id, pages_dict);

        let mut catalog_dict = lopdf::Dictionary::new();
        catalog_dict.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
        catalog_dict.set("Pages", lopdf::Object::Reference(pages_id));
        let catalog_id = inner.add_object(catalog_dict);

        inner
            .trailer
            .set("Root", lopdf::Object::Reference(catalog_id));

        LopdfDocument { inner }
    }

    #[test]
    fn test_watermark_applied() {
        let mut doc = create_test_document();
        let op = WatermarkOperation::new("CONFIDENTIAL".to_string());

        assert!(op.execute(&mut doc).is_ok());

        let page_id = *doc.inner.get_pages().get(&1).unwrap();
        let dict = doc.inner.get_object(page_id).unwrap().as_dict().unwrap();

        assert!(dict.has(b"Resources"));

        let contents = dict.get(b"Contents").unwrap();
        match contents {
            lopdf::Object::Reference(ref_id) => {
                let stream = doc.inner.get_object(*ref_id).unwrap().as_stream().unwrap();
                let content_str = std::str::from_utf8(&stream.content).unwrap();
                assert!(content_str.contains("CONFIDENTIAL"));
                // Default 45 degrees
                assert!(content_str.contains("0.7071"));
                // Default opacity ExtGState GS1
                assert!(content_str.contains("/GS1 gs"));
                // Default gray color
                assert!(content_str.contains("0.5 0.5 0.5 rg"));
            }
            lopdf::Object::Array(arr) => {
                assert_eq!(arr.len(), 1);
                let stream_id = arr[0].as_reference().unwrap();
                let stream = doc.inner.get_object(stream_id).unwrap().as_stream().unwrap();
                let content_str = std::str::from_utf8(&stream.content).unwrap();
                assert!(content_str.contains("CONFIDENTIAL"));
            }
            _ => panic!("Contents is not Reference or Array"),
        }
    }

    #[test]
    fn test_watermark_horizontal() {
        let mut doc = create_test_document();
        let mut op = WatermarkOperation::new("HORIZONTAL".to_string());
        op.angle = 0.0;
        assert!(op.execute(&mut doc).is_ok());

        let page_id = *doc.inner.get_pages().get(&1).unwrap();
        let dict = doc.inner.get_object(page_id).unwrap().as_dict().unwrap();
        let contents = dict.get(b"Contents").unwrap();

        let stream_id = match contents {
            lopdf::Object::Reference(id) => *id,
            lopdf::Object::Array(arr) => arr[0].as_reference().unwrap(),
            _ => panic!(),
        };
        let stream = doc.inner.get_object(stream_id).unwrap().as_stream().unwrap();
        let content_str = std::str::from_utf8(&stream.content).unwrap();
        // cos(0) = 1, sin(0) = 0 => 1 0 0 1
        assert!(content_str.contains("1 0 -0 1") || content_str.contains("1 0 0 1"));
    }

    #[test]
    fn test_watermark_custom_config() {
        let mut doc = create_test_document();
        let mut op = WatermarkOperation::new("CUSTOM".to_string());
        op.color = Some("red".to_string());
        op.font_size = Some(100.0);
        op.opacity = 0.8;
        assert!(op.execute(&mut doc).is_ok());

        let page_id = *doc.inner.get_pages().get(&1).unwrap();
        let dict = doc.inner.get_object(page_id).unwrap().as_dict().unwrap();
        let contents = dict.get(b"Contents").unwrap();

        let stream_id = match contents {
            lopdf::Object::Reference(id) => *id,
            lopdf::Object::Array(arr) => arr[0].as_reference().unwrap(),
            _ => panic!(),
        };
        let stream = doc.inner.get_object(stream_id).unwrap().as_stream().unwrap();
        let content_str = std::str::from_utf8(&stream.content).unwrap();
        // Red color
        assert!(content_str.contains("1 0 0 rg"));
        // Font size 100
        assert!(content_str.contains("/F1 100 Tf"));
    }
}
