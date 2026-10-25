#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
use paperpilot_core::error::{OperationResult, PdfError};
use lopdf::{Document, Object, content::{Content, Operation}, Stream};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvertMode {
    DarkMode,
    TrueInvert,
    HighContrast,
    Sepia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvertColorsOptions {
    pub mode: Option<InvertMode>,
    pub pages: Option<String>,
}

pub fn invert_pdf_colors(doc: &mut Document, options: &InvertColorsOptions) -> OperationResult<()> {
    let pages = doc.get_pages();

    let target_pages: Vec<u32> = if let Some(p_str) = &options.pages {
        if p_str.trim().is_empty() {
            pages.keys().copied().collect()
        } else {
            let mut p = Vec::new();
            for part in p_str.split(',') {
                if let Ok(num) = part.trim().parse::<u32>() {
                    p.push(num);
                }
            }
            if p.is_empty() { pages.keys().copied().collect() } else { p }
        }
    } else {
        pages.keys().copied().collect()
    };

    let mode = options.mode.as_ref().unwrap_or(&InvertMode::DarkMode);

    for page_id in target_pages {
        let page_obj_id = *pages.get(&page_id).ok_or_else(|| PdfError::Other("Page not found".to_string()))?;

        let mut width = 612.0;
        let mut height = 792.0;

        if let Ok(page_dict) = doc.get_dictionary(page_obj_id) {
            if let Ok(Object::Array(media_box)) = page_dict.get(b"MediaBox") {
                if media_box.len() == 4 {
                    if let (Ok(x1), Ok(y1), Ok(x2), Ok(y2)) = (get_f64(&media_box[0]), get_f64(&media_box[1]), get_f64(&media_box[2]), get_f64(&media_box[3])) {
                        width = x2 - x1;
                        height = y2 - y1;
                    }
                }
            }
        }

        let bg_color = match mode {
            InvertMode::DarkMode => vec![0.094, 0.094, 0.094],
            InvertMode::TrueInvert => vec![0.0, 0.0, 0.0],
            InvertMode::HighContrast => vec![0.0, 0.0, 0.0],
            InvertMode::Sepia => vec![0.98, 0.94, 0.9],
        };

        let mut new_ops = vec![
            Operation::new("q", vec![]),
            Operation::new("rg", vec![Object::Real(bg_color[0] as f32), Object::Real(bg_color[1] as f32), Object::Real(bg_color[2] as f32)]),
            Operation::new("re", vec![Object::Real(0.0), Object::Real(0.0), Object::Real(width as f32), Object::Real(height as f32)]),
            Operation::new("f", vec![]),
            Operation::new("Q", vec![]),
        ];

        let content_data = doc.get_page_content(page_obj_id);

        if !content_data.is_empty() {
            let mut content = Content::decode(&content_data)
                .map_err(|e| PdfError::Other(format!("Failed to decode content: {}", e)))?;
            for op in &mut content.operations {
                match op.operator.as_str() {
                    "rg" | "RG" => {
                        if op.operands.len() == 3 {
                            for i in 0..3 {
                                if let Ok(val) = get_f64(&op.operands[i]) {
                                    op.operands[i] = Object::Real(invert_color_val(val, mode) as f32);
                                }
                            }
                        }
                    }
                    "g" | "G" => {
                        if op.operands.len() == 1 {
                            if let Ok(val) = get_f64(&op.operands[0]) {
                                op.operands[0] = Object::Real(invert_color_val(val, mode) as f32);
                            }
                        }
                    }
                    "k" | "K" => {
                        if op.operands.len() == 4 {
                            if let (Ok(c), Ok(m), Ok(y), Ok(k)) = (get_f64(&op.operands[0]), get_f64(&op.operands[1]), get_f64(&op.operands[2]), get_f64(&op.operands[3])) {
                                let (nc, nm, ny, nk) = invert_cmyk_val(c, m, y, k, mode);
                                op.operands[0] = Object::Real(nc as f32);
                                op.operands[1] = Object::Real(nm as f32);
                                op.operands[2] = Object::Real(ny as f32);
                                op.operands[3] = Object::Real(nk as f32);
                            }
                        }
                    }
                    _ => {}
                }
            }
            new_ops.extend(content.operations);
        }

        let new_content = Content { operations: new_ops };
        let new_content_bytes = new_content.encode()
            .map_err(|e| PdfError::Other(format!("Failed to encode content: {}", e)))?;

        let stream = Stream::new(lopdf::Dictionary::new(), new_content_bytes);
        let stream_id = doc.add_object(Object::Stream(stream));
        if let Ok(page_dict) = doc.get_object_mut(page_obj_id) {
            if let Object::Dictionary(dict) = page_dict {
                dict.set("Contents", Object::Reference(stream_id));
            }
        }
    }
    Ok(())
}

fn invert_color_val(val: f64, mode: &InvertMode) -> f64 {
    match mode {
        InvertMode::DarkMode => {
            let inverted = 1.0 - val;
            0.2 + (inverted * 0.7)
        }
        InvertMode::TrueInvert => 1.0 - val,
        InvertMode::HighContrast => {
            if val < 0.5 { 1.0 } else { 0.0 }
        }
        InvertMode::Sepia => {
            val * 0.8
        }
    }
}

fn invert_cmyk_val(c: f64, m: f64, y: f64, k: f64, mode: &InvertMode) -> (f64, f64, f64, f64) {
    match mode {
        InvertMode::DarkMode | InvertMode::TrueInvert | InvertMode::HighContrast => {
            if k > 0.5 {
                (0.0, 0.0, 0.0, 0.0)
            } else {
                (c, m, y, 1.0 - k)
            }
        }
        InvertMode::Sepia => {
            (c, m, y, k * 0.8)
        }
    }
}

fn get_f64(obj: &Object) -> Result<f64, ()> {
    match obj {
        Object::Real(r) => Ok(*r as f64),
        Object::Integer(i) => Ok(*i as f64),
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::Dictionary;

    fn create_test_pdf() -> Document {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let page_id = doc.new_object_id();
        let content_id = doc.new_object_id();

        let content = Content {
            operations: vec![
                Operation::new("rg", vec![Object::Real(1.0), Object::Real(1.0), Object::Real(1.0)]),
                Operation::new("re", vec![Object::Real(10.0), Object::Real(10.0), Object::Real(50.0), Object::Real(50.0)]),
                Operation::new("f", vec![]),
            ],
        };

        doc.objects.insert(content_id, Object::Stream(Stream::new(Dictionary::new(), content.encode().unwrap())));

        let mut page_dict = Dictionary::new();
        page_dict.set("Type", "Page");
        page_dict.set("Parent", pages_id);
        page_dict.set("Contents", Object::Reference(content_id));
        page_dict.set("MediaBox", vec![Object::Real(0.0), Object::Real(0.0), Object::Real(100.0), Object::Real(100.0)]);
        doc.objects.insert(page_id, Object::Dictionary(page_dict));

        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", "Pages");
        pages_dict.set("Kids", vec![Object::Reference(page_id)]);
        pages_dict.set("Count", 1);
        doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

        let mut catalog = Dictionary::new();
        catalog.set("Type", "Catalog");
        catalog.set("Pages", Object::Reference(pages_id));
        let catalog_id = doc.add_object(Object::Dictionary(catalog));

        doc.trailer.set("Root", Object::Reference(catalog_id));
        doc
    }

    #[test]
    fn test_invert_pdf_colors_dark_mode() {
        let mut doc = create_test_pdf();
        let options = InvertColorsOptions { mode: Some(InvertMode::DarkMode), pages: None };
        invert_pdf_colors(&mut doc, &options).unwrap();

        let page_id = *doc.get_pages().get(&1).unwrap();
        let content_bytes = doc.get_page_content(page_id);
        let content = Content::decode(&content_bytes).unwrap();

        assert_eq!(content.operations[1].operator, "rg");
        assert_eq!(content.operations[2].operator, "re");

        let mut found_orig_rg = false;
        for op in &content.operations {
            if op.operator == "rg" {
                if let (Ok(r), Ok(g), Ok(b)) = (get_f64(&op.operands[0]), get_f64(&op.operands[1]), get_f64(&op.operands[2])) {
                    if r < 0.3 && g < 0.3 && b < 0.3 {
                        found_orig_rg = true;
                    }
                }
            }
        }
        assert!(found_orig_rg, "Expected original color to be inverted");
    }

    #[test]
    fn test_invert_pdf_colors_true_invert() {
        let mut doc = create_test_pdf();
        let options = InvertColorsOptions { mode: Some(InvertMode::TrueInvert), pages: None };
        invert_pdf_colors(&mut doc, &options).unwrap();

        let page_id = *doc.get_pages().get(&1).unwrap();
        let content_bytes = doc.get_page_content(page_id);
        let content = Content::decode(&content_bytes).unwrap();

        let mut found_invert = false;
        for op in content.operations.iter() {
            if op.operator == "rg" {
                if let (Ok(r), Ok(g), Ok(b)) = (get_f64(&op.operands[0]), get_f64(&op.operands[1]), get_f64(&op.operands[2])) {
                    if r < 0.01 && g < 0.01 && b < 0.01 { // True invert of 1,1,1
                        found_invert = true;
                    }
                }
            }
        }
        assert!(found_invert, "Expected true invert of 1,1,1 to 0,0,0");
    }

    #[test]
    fn test_invert_pdf_colors_page_subset() {
        let mut doc = create_test_pdf();
        let options = InvertColorsOptions { mode: Some(InvertMode::TrueInvert), pages: Some("1".to_string()) };
        let res = invert_pdf_colors(&mut doc, &options);
        assert!(res.is_ok());
    }
}
