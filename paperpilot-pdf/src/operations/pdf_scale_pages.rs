use lopdf::{Dictionary, Document, Object, Stream};
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfScalePagesOptions {
    #[serde(default = "default_target_size")]
    pub target_size: String,
    #[serde(default = "default_scale_factor")]
    pub scale_factor: f32,
    #[serde(default = "default_true")]
    pub maintain_aspect_ratio: bool,
    #[serde(default = "default_true")]
    pub center_content: bool,
}

fn default_target_size() -> String {
    "A4".to_string()
}
fn default_scale_factor() -> f32 {
    1.0
}
fn default_true() -> bool {
    true
}

impl Default for PdfScalePagesOptions {
    fn default() -> Self {
        Self {
            target_size: default_target_size(),
            scale_factor: default_scale_factor(),
            maintain_aspect_ratio: default_true(),
            center_content: default_true(),
        }
    }
}

pub fn pdf_scale_pages_bytes(
    input_bytes: &[u8],
    options: &PdfScalePagesOptions,
) -> Result<Vec<u8>, PdfError> {
    if input_bytes.is_empty() {
        return Err(PdfError::InvalidInput("Empty document".to_string()));
    }

    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to load PDF: {}", e)))?;

    let get_target_dimensions = |size: &str| -> Option<(f32, f32)> {
        match size.to_uppercase().as_str() {
            "A4" => Some((595.28, 841.89)),
            "LETTER" => Some((612.0, 792.0)),
            "LEGAL" => Some((612.0, 1008.0)),
            "A3" => Some((841.89, 1190.55)),
            "A5" => Some((419.53, 595.28)),
            "TABLOID" => Some((792.0, 1224.0)),
            _ => None,
        }
    };

    let page_ids = doc.get_pages();
    for (page_num, page_id) in page_ids.into_iter() {
        let (orig_w, orig_h, x0, y0, _x1, _y1) = {
            let page_dict = doc
                .get_object(page_id)
                .and_then(|o: &Object| o.as_dict())
                .map_err(|_| {
                    PdfError::ParseError(format!("Failed to get page {} dictionary", page_num))
                })?;

            let media_box = if let Ok(Object::Array(rect)) = page_dict.get(b"MediaBox") {
                if rect.len() == 4 {
                    let get_f32 = |obj: &Object| -> f32 {
                        match obj {
                            Object::Real(f) => *f,
                            Object::Integer(i) => *i as f32,
                            _ => 0.0,
                        }
                    };
                    let mx0 = get_f32(&rect[0]);
                    let my0 = get_f32(&rect[1]);
                    let mx1 = get_f32(&rect[2]);
                    let my1 = get_f32(&rect[3]);
                    Some((mx0, my0, mx1, my1))
                } else {
                    None
                }
            } else {
                None
            };

            let (mx0, my0, mx1, my1) = media_box.unwrap_or((0.0, 0.0, 612.0, 792.0));
            (mx1 - mx0, my1 - my0, mx0, my0, mx1, my1)
        };

        if orig_w <= 0.0 || orig_h <= 0.0 {
            continue;
        }

        let target_dims = get_target_dimensions(&options.target_size);
        let (target_w, target_h) = if let Some((tw, th)) = target_dims {
            if options.scale_factor != 1.0 {
                (tw * options.scale_factor, th * options.scale_factor)
            } else {
                (tw, th)
            }
        } else {
            (orig_w * options.scale_factor, orig_h * options.scale_factor)
        };

        let mut sx = target_w / orig_w;
        let mut sy = target_h / orig_h;

        if options.maintain_aspect_ratio {
            let s = sx.min(sy);
            sx = s;
            sy = s;
        }

        let scaled_w = orig_w * sx;
        let scaled_h = orig_h * sy;

        let mut tx = 0.0;
        let mut ty = 0.0;

        if options.center_content {
            tx = (target_w - scaled_w) / 2.0;
            ty = (target_h - scaled_h) / 2.0;
        }

        let matrix_str = format!(
            "q\n{} 0 0 {} {} {} cm\n",
            sx,
            sy,
            tx - x0 * sx,
            ty - y0 * sy
        );

        let prepend_stream_id = doc.add_object(Stream::new(
            Dictionary::new(),
            matrix_str.as_bytes().to_vec(),
        ));
        let append_stream_id = doc.add_object(Stream::new(Dictionary::new(), b"\nQ\n".to_vec()));

        let page_dict = doc
            .get_object_mut(page_id)
            .and_then(|o: &mut Object| o.as_dict_mut())
            .unwrap();

        let new_box = vec![
            Object::Real(0.0),
            Object::Real(0.0),
            Object::Real(target_w),
            Object::Real(target_h),
        ];
        page_dict.set("MediaBox", Object::Array(new_box.clone()));

        if page_dict.has(b"CropBox") {
            page_dict.set("CropBox", Object::Array(new_box));
        }

        let existing_contents = page_dict.get(b"Contents").cloned();

        let mut new_contents_array = vec![Object::Reference(prepend_stream_id)];

        if let Ok(contents_obj) = existing_contents {
            match contents_obj {
                Object::Reference(id) => {
                    new_contents_array.push(Object::Reference(id));
                }
                Object::Array(arr) => {
                    new_contents_array.extend(arr);
                }
                _ => {}
            }
        }

        new_contents_array.push(Object::Reference(append_stream_id));

        page_dict.set("Contents", Object::Array(new_contents_array));
    }

    let mut buf = Vec::new();
    doc.save_to(&mut buf)
        .map_err(|e| PdfError::Other(format!("Failed to save scaled PDF: {}", e)))?;

    Ok(buf)
}

#[derive(Default)]
pub struct PdfScalePagesOperation {
    #[allow(dead_code)]
    options: PdfScalePagesOptions,
}

impl PdfScalePagesOperation {
    pub fn new(options: PdfScalePagesOptions) -> Self {
        Self { options }
    }
}

impl PdfOperation for PdfScalePagesOperation {
    fn execute(
        &self,
        _document: &mut dyn PdfDocument,
    ) -> paperpilot_core::error::OperationResult<()> {
        Err(PdfError::UnsupportedOperation(
            "Not yet implemented".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_pdf() -> Vec<u8> {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let font_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", "Font".into()),
            ("Subtype", "Type1".into()),
            ("BaseFont", "Helvetica".into()),
        ]));

        let resources_id = doc.add_object(Dictionary::from_iter(vec![
            ("Font", Dictionary::from_iter(vec![("F1", font_id.into())]).into()),
        ]));

        let content_id = doc.add_object(Stream::new(Dictionary::new(), b"BT /F1 12 Tf 100 100 Td (Hello World) Tj ET".to_vec()));

        let page_dict = Dictionary::from_iter(vec![
            ("Type", "Page".into()),
            ("Parent", pages_id.into()),
            ("Resources", resources_id.into()),
            ("MediaBox", vec![0.into(), 0.into(), 612.into(), 792.into()].into()),
            ("Contents", content_id.into()),
        ]);

        let page_id = doc.add_object(page_dict);

        let pages_dict = Dictionary::from_iter(vec![
            ("Type", "Pages".into()),
            ("Count", 1.into()),
            ("Kids", vec![page_id.into()].into()),
        ]);
        doc.objects.insert(pages_id, pages_dict.into());

        let catalog_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", "Catalog".into()),
            ("Pages", pages_id.into()),
        ]));

        doc.trailer.set("Root", catalog_id);

        let mut buf = Vec::new();
        doc.save_to(&mut buf).unwrap();
        buf
    }

    #[test]
    fn test_scale_to_a4() {
        let pdf_bytes = create_test_pdf();
        let options = PdfScalePagesOptions {
            target_size: "A4".to_string(),
            ..Default::default()
        };

        let result = pdf_scale_pages_bytes(&pdf_bytes, &options).expect("Should scale PDF");

        let doc = Document::load_mem(&result).expect("Output should be valid PDF");
        let page_id = doc.get_pages().into_values().next().unwrap();
        let page_dict = doc.get_object(page_id).unwrap().as_dict().unwrap();
        let media_box = page_dict.get(b"MediaBox").unwrap().as_array().unwrap();

        let get_f32 = |obj: &Object| -> f32 {
            match obj {
                Object::Real(f) => *f,
                Object::Integer(i) => *i as f32,
                _ => 0.0,
            }
        };

        assert!((get_f32(&media_box[0]) - 0.0).abs() < 0.01, "Expected 0.0, got {}", get_f32(&media_box[0]));
        assert!((get_f32(&media_box[1]) - 0.0).abs() < 0.01, "Expected 0.0, got {}", get_f32(&media_box[1]));
        assert!((get_f32(&media_box[2]) - 595.28).abs() < 0.01, "Expected ~595.28, got {}", get_f32(&media_box[2]));
        assert!((get_f32(&media_box[3]) - 841.89).abs() < 0.01, "Expected ~841.89, got {}", get_f32(&media_box[3]));
    }

    #[test]
    fn test_scale_factor_half() {
        let pdf_bytes = create_test_pdf();
        let options = PdfScalePagesOptions {
            target_size: "Original".to_string(),
            scale_factor: 0.5,
            ..Default::default()
        };

        let result = pdf_scale_pages_bytes(&pdf_bytes, &options).expect("Should scale PDF");

        let doc = Document::load_mem(&result).expect("Output should be valid PDF");
        let page_id = doc.get_pages().into_values().next().unwrap();
        let page_dict = doc.get_object(page_id).unwrap().as_dict().unwrap();
        let media_box = page_dict.get(b"MediaBox").unwrap().as_array().unwrap();

        let get_f32 = |obj: &Object| -> f32 {
            match obj {
                Object::Real(f) => *f,
                Object::Integer(i) => *i as f32,
                _ => 0.0,
            }
        };

        assert!((get_f32(&media_box[0]) - 0.0).abs() < 0.01, "Expected 0.0, got {}", get_f32(&media_box[0]));
        assert!((get_f32(&media_box[1]) - 0.0).abs() < 0.01, "Expected 0.0, got {}", get_f32(&media_box[1]));
        assert!((get_f32(&media_box[2]) - 306.0).abs() < 0.01, "Expected 306.0, got {}", get_f32(&media_box[2]));
        assert!((get_f32(&media_box[3]) - 396.0).abs() < 0.01, "Expected 396.0, got {}", get_f32(&media_box[3]));
    }

    #[test]
    fn test_scale_empty_document() {
        let options = PdfScalePagesOptions::default();
        let bytes = vec![];
        let result = pdf_scale_pages_bytes(&bytes, &options);
        assert!(result.is_err());
    }
}
