use lopdf::{Dictionary, Document, Object, StringFormat};
use paperpilot_core::error::PdfError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationParams {
    pub id: String,
    pub r#type: String, // 'highlight', 'underline', 'strikethrough', 'note', 'pen'
    pub page: u32,
    pub x: f64,
    pub y: f64,
    pub w: Option<f64>,
    pub h: Option<f64>,
    pub color: String,
    pub content: Option<String>,
}

pub struct AnnotateOperation {
    annotations: Vec<AnnotationParams>,
}

impl Default for AnnotateOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl AnnotateOperation {
    pub fn new() -> Self {
        Self {
            annotations: Vec::new(),
        }
    }

    pub fn with_annotations(mut self, annotations: Vec<AnnotationParams>) -> Self {
        self.annotations = annotations;
        self
    }

    pub fn execute(&self, doc: &mut Document) -> Result<(), PdfError> {
        let pages = doc.get_pages();

        for annot in &self.annotations {
            if let Some(&page_id) = pages.get(&annot.page) {
                // Parse color from hex "#RRGGBB"
                let (r, g, b) = Self::parse_color(&annot.color).unwrap_or((1.0, 1.0, 0.0));

                let mut annot_dict = Dictionary::new();
                annot_dict.set("Type", Object::Name(b"Annot".to_vec()));

                match annot.r#type.as_str() {
                    "highlight" => {
                        annot_dict.set("Subtype", Object::Name(b"Highlight".to_vec()));
                        let rect = Self::create_rect(
                            annot.x,
                            annot.y,
                            annot.w.unwrap_or(100.0),
                            annot.h.unwrap_or(20.0),
                        );
                        annot_dict.set("Rect", Object::Array(rect.clone()));
                        annot_dict.set("QuadPoints", Object::Array(Self::create_quad_points(rect)));
                    }
                    "underline" => {
                        annot_dict.set("Subtype", Object::Name(b"Underline".to_vec()));
                        let rect = Self::create_rect(
                            annot.x,
                            annot.y,
                            annot.w.unwrap_or(100.0),
                            annot.h.unwrap_or(20.0),
                        );
                        annot_dict.set("Rect", Object::Array(rect.clone()));
                        annot_dict.set("QuadPoints", Object::Array(Self::create_quad_points(rect)));
                    }
                    "strikethrough" => {
                        annot_dict.set("Subtype", Object::Name(b"StrikeOut".to_vec()));
                        let rect = Self::create_rect(
                            annot.x,
                            annot.y,
                            annot.w.unwrap_or(100.0),
                            annot.h.unwrap_or(20.0),
                        );
                        annot_dict.set("Rect", Object::Array(rect.clone()));
                        annot_dict.set("QuadPoints", Object::Array(Self::create_quad_points(rect)));
                    }
                    "note" => {
                        annot_dict.set("Subtype", Object::Name(b"Text".to_vec()));
                        annot_dict.set(
                            "Rect",
                            Object::Array(Self::create_rect(annot.x, annot.y, 24.0, 24.0)),
                        );
                        if let Some(content) = &annot.content {
                            annot_dict.set(
                                "Contents",
                                Object::String(content.as_bytes().to_vec(), StringFormat::Literal),
                            );
                        }
                    }
                    "pen" => {
                        annot_dict.set("Subtype", Object::Name(b"Ink".to_vec()));
                        // Approximation for ink bounds
                        let rect = Self::create_rect(
                            annot.x,
                            annot.y,
                            annot.w.unwrap_or(100.0),
                            annot.h.unwrap_or(100.0),
                        );
                        annot_dict.set("Rect", Object::Array(rect));
                    }
                    _ => {
                        continue;
                    }
                }

                // Color array [R, G, B]
                annot_dict.set(
                    "C",
                    Object::Array(vec![
                        Object::Real(r as f32),
                        Object::Real(g as f32),
                        Object::Real(b as f32),
                    ]),
                );

                let annot_id = doc.add_object(Object::Dictionary(annot_dict));

                // Resolve any references first to avoid double mutable borrow
                let mut resolved_annots_ref = None;
                if let Ok(Object::Dictionary(page_dict)) = doc.get_object(page_id)
                    && let Ok(Object::Reference(ref_id)) = page_dict.get(b"Annots")
                {
                    resolved_annots_ref = Some(*ref_id);
                }

                let mut annots_array = vec![];
                if let Some(ref_id) = resolved_annots_ref {
                    if let Ok(Object::Array(arr)) = doc.get_object_mut(ref_id) {
                        annots_array = arr.clone();
                    }
                } else if let Ok(Object::Dictionary(page_dict)) = doc.get_object(page_id)
                    && let Ok(Object::Array(arr)) = page_dict.get(b"Annots")
                {
                    annots_array = arr.clone();
                }

                annots_array.push(Object::Reference(annot_id));

                if let Ok(Object::Dictionary(page_dict)) = doc.get_object_mut(page_id) {
                    page_dict.set("Annots", Object::Array(annots_array));
                }
            }
        }

        Ok(())
    }

    fn parse_color(hex: &str) -> Option<(f64, f64, f64)> {
        if hex.starts_with('#') && hex.len() == 7 {
            let r = u8::from_str_radix(&hex[1..3], 16).ok()? as f64 / 255.0;
            let g = u8::from_str_radix(&hex[3..5], 16).ok()? as f64 / 255.0;
            let b = u8::from_str_radix(&hex[5..7], 16).ok()? as f64 / 255.0;
            Some((r, g, b))
        } else {
            None
        }
    }

    fn create_rect(x: f64, y: f64, w: f64, h: f64) -> Vec<Object> {
        // PDF coordinates are bottom-left origin. For simplicity in mapping we assume the frontend
        // coordinates might need y-inversion based on page height, but here we just store the raw values.
        vec![
            Object::Real(x as f32),
            Object::Real(y as f32),
            Object::Real((x + w) as f32),
            Object::Real((y + h) as f32),
        ]
    }

    fn create_quad_points(rect: Vec<Object>) -> Vec<Object> {
        // QuadPoints are 8 numbers specifying the 4 corners: bottom-left, bottom-right, top-right, top-left
        if rect.len() != 4 {
            return vec![];
        }
        let llx = rect[0].clone();
        let lly = rect[1].clone();
        let urx = rect[2].clone();
        let ury = rect[3].clone();

        vec![
            llx.clone(),
            lly.clone(),
            urx.clone(),
            lly.clone(),
            llx.clone(),
            ury.clone(),
            urx.clone(),
            ury.clone(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::Document;

    #[test]
    fn test_annotate_operation_new() {
        let op = AnnotateOperation::new();
        assert_eq!(op.annotations.len(), 0);
    }

    #[test]
    fn test_parse_color() {
        assert_eq!(
            AnnotateOperation::parse_color("#ff0000"),
            Some((1.0, 0.0, 0.0))
        );
        assert_eq!(
            AnnotateOperation::parse_color("#00ff00"),
            Some((0.0, 1.0, 0.0))
        );
        assert_eq!(
            AnnotateOperation::parse_color("#0000ff"),
            Some((0.0, 0.0, 1.0))
        );
        assert_eq!(AnnotateOperation::parse_color("invalid"), None);
    }

    #[test]
    fn test_execute_adds_annotation() {
        let mut doc = Document::with_version("1.5");

        let mut catalog = Dictionary::new();
        catalog.set("Type", Object::Name(b"Catalog".to_vec()));

        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Count", Object::Integer(1));

        let mut page = Dictionary::new();
        page.set("Type", Object::Name(b"Page".to_vec()));

        let page_id = doc.add_object(Object::Dictionary(page));
        pages_dict.set("Kids", Object::Array(vec![Object::Reference(page_id)]));
        let pages_id = doc.add_object(Object::Dictionary(pages_dict));

        catalog.set("Pages", Object::Reference(pages_id));
        let catalog_id = doc.add_object(Object::Dictionary(catalog));

        doc.trailer.set("Root", Object::Reference(catalog_id));

        if let Ok(Object::Dictionary(page_dict)) = doc.get_object_mut(page_id) {
            page_dict.set("Parent", Object::Reference(pages_id));
        }

        let op = AnnotateOperation::new().with_annotations(vec![AnnotationParams {
            id: "test1".to_string(),
            r#type: "highlight".to_string(),
            page: 1,
            x: 10.0,
            y: 10.0,
            w: Some(50.0),
            h: Some(20.0),
            color: "#ff0000".to_string(),
            content: None,
        }]);

        let result = op.execute(&mut doc);
        assert!(result.is_ok());

        // Verify the page has an Annots array
        let page_obj = doc.get_object(page_id).unwrap();
        if let Object::Dictionary(dict) = page_obj {
            assert!(dict.has(b"Annots"));
        } else {
            panic!("Page is not a dictionary");
        }
    }
}
