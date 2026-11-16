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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationRule {
    #[serde(default)]
    pub id: Option<String>,
    /// 'highlight', 'underline', 'strikethrough', 'note', 'pen'
    #[serde(default = "default_annot_type")]
    pub r#type: String,
    /// Hex color code, e.g. "#FFFF00"
    #[serde(default = "default_annot_color")]
    pub color: String,
    /// Plain keyword text to search
    #[serde(default)]
    pub search: Option<String>,
    /// Regex pattern to search
    #[serde(default)]
    pub regex: Option<String>,
    /// Case-sensitive matching (default false)
    #[serde(default)]
    pub case_sensitive: bool,
    /// Target pages filter (None = all pages)
    #[serde(default)]
    pub pages: Option<Vec<u32>>,
    /// Explicit bounding rect [x, y, w, h]
    #[serde(default)]
    pub rect: Option<[f64; 4]>,
    /// Target page for explicit rect (default 1)
    #[serde(default)]
    pub page: Option<u32>,
    /// Occurrence selector: 'all' (default), 'first', 'last', or comma-separated indices "0,2"
    #[serde(default = "default_occurrence")]
    pub occurrence: String,
    /// Note content / comment text
    #[serde(default)]
    pub content: Option<String>,
}

fn default_annot_type() -> String {
    "highlight".to_string()
}

fn default_annot_color() -> String {
    "#FFFF00".to_string()
}

fn default_occurrence() -> String {
    "all".to_string()
}

pub struct AnnotateOperation {
    annotations: Vec<AnnotationParams>,
    rules: Vec<AnnotationRule>,
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
            rules: Vec::new(),
        }
    }

    pub fn with_annotations(mut self, annotations: Vec<AnnotationParams>) -> Self {
        self.annotations = annotations;
        self
    }

    pub fn with_rules(mut self, rules: Vec<AnnotationRule>) -> Self {
        self.rules = rules;
        self
    }

    pub fn execute(&self, doc: &mut Document) -> Result<(), PdfError> {
        let mut all_annotations = self.annotations.clone();

        if !self.rules.is_empty() {
            let mut doc_bytes = Vec::new();
            let _ = doc.save_to(&mut doc_bytes);

            for (rule_idx, rule) in self.rules.iter().enumerate() {
                if rule.search.is_some() || rule.regex.is_some() {
                    let matches = crate::operations::pdf_extract_patterns::find_text_matches(
                        &doc_bytes,
                        rule.search.as_deref(),
                        rule.regex.as_deref(),
                        rule.case_sensitive,
                        rule.pages.as_deref(),
                    );
                    let filtered = crate::operations::pdf_extract_patterns::filter_occurrences(matches, &rule.occurrence);
                    for (match_idx, m) in filtered.into_iter().enumerate() {
                        let id = rule.id.clone().unwrap_or_else(|| format!("rule_{}_{}", rule_idx + 1, match_idx + 1));
                        let content = rule.content.clone().or(Some(m.text));
                        all_annotations.push(AnnotationParams {
                            id,
                            r#type: rule.r#type.clone(),
                            page: m.page,
                            x: m.rect[0],
                            y: m.rect[1],
                            w: Some(m.rect[2]),
                            h: Some(m.rect[3]),
                            color: rule.color.clone(),
                            content,
                        });
                    }
                } else if let Some(rect) = rule.rect {
                    let page = rule.page.unwrap_or(1);
                    if rule.pages.is_none() || rule.pages.as_ref().unwrap().contains(&page) {
                        let id = rule.id.clone().unwrap_or_else(|| format!("rule_{}_rect", rule_idx + 1));
                        all_annotations.push(AnnotationParams {
                            id,
                            r#type: rule.r#type.clone(),
                            page,
                            x: rect[0],
                            y: rect[1],
                            w: Some(rect[2]),
                            h: Some(rect[3]),
                            color: rule.color.clone(),
                            content: rule.content.clone(),
                        });
                    }
                }
            }
        }

        let pages = doc.get_pages();

        for annot in &all_annotations {
            if let Some(&page_id) = pages.get(&annot.page) {
                // Parse color from hex "#RRGGBB"
                let (r, g, b) = Self::parse_color(&annot.color).unwrap_or((1.0, 1.0, 0.0));

                let mut annot_dict = Dictionary::new();
                annot_dict.set("Type", Object::Name(b"Annot".to_vec()));

                match annot.r#type.as_str() {
                    "highlight" => {
                        annot_dict.set("Subtype", Object::Name(b"Highlight".to_vec()));
                        // Expand highlight bounds slightly (padding) based on text font size
                        let h = annot.h.unwrap_or(12.0);
                        let w = annot.w.unwrap_or(100.0);
                        let pad_x = (w * 0.02).clamp(1.0, 3.0);
                        let pad_y = (h * 0.12).clamp(1.0, 2.5);
                        let rect = Self::create_rect(
                            annot.x - pad_x,
                            annot.y - pad_y,
                            w + (pad_x * 2.0),
                            h + (pad_y * 2.0),
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
                            annot.h.unwrap_or(12.0),
                        );
                        annot_dict.set("Rect", Object::Array(rect.clone()));
                        annot_dict.set("QuadPoints", Object::Array(Self::create_quad_points(rect)));
                    }
                    "strikethrough" => {
                        annot_dict.set("Subtype", Object::Name(b"StrikeOut".to_vec()));
                        // Shift strikethrough down slightly so it cuts directly through the optical middle of glyphs
                        let h = annot.h.unwrap_or(12.0);
                        let shift_y = h * 0.15;
                        let rect = Self::create_rect(
                            annot.x,
                            annot.y - shift_y,
                            annot.w.unwrap_or(100.0),
                            h,
                        );
                        annot_dict.set("Rect", Object::Array(rect.clone()));
                        annot_dict.set("QuadPoints", Object::Array(Self::create_quad_points(rect)));
                    }
                    "note" => {
                        annot_dict.set("Subtype", Object::Name(b"Text".to_vec()));
                        // Place sticky note pin neatly beside the word (to the right of target word)
                        // so it doesn't cover or obscure the document text!
                        let target_w = annot.w.unwrap_or(0.0);
                        let note_x = annot.x + target_w + 4.0;
                        let note_y = annot.y;
                        annot_dict.set(
                            "Rect",
                            Object::Array(Self::create_rect(note_x, note_y, 20.0, 20.0)),
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
        // PDF Specification ISO 32000-1 §12.5.6.10:
        // QuadPoints are 8 numbers specifying the 4 vertices of the quadrilateral in counter-clockwise order:
        // x1, y1 (top-left), x2, y2 (top-right), x3, y3 (bottom-left), x4, y4 (bottom-right).
        // For Underline, PDF viewers draw the underline along the bottom edge (between bottom-left and bottom-right).
        if rect.len() != 4 {
            return vec![];
        }
        let llx = rect[0].clone();
        let lly = rect[1].clone();
        let urx = rect[2].clone();
        let ury = rect[3].clone();

        vec![
            llx.clone(),
            ury.clone(), // x1, y1 = top-left
            urx.clone(),
            ury.clone(), // x2, y2 = top-right
            llx.clone(),
            lly.clone(), // x3, y3 = bottom-left
            urx.clone(),
            lly.clone(), // x4, y4 = bottom-right
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

    #[test]
    fn test_execute_multi_rule_pattern_annotations_fixture() {
        let pdf_path = std::path::Path::new("../ManaulTestingFixtures/pdf_annotate/Input/research_paper_to_annotate.pdf");
        let alt_path = std::path::Path::new("ManaulTestingFixtures/pdf_annotate/Input/research_paper_to_annotate.pdf");
        let p = if pdf_path.exists() { pdf_path } else { alt_path };
        if let Ok(bytes) = std::fs::read(p) {
            let mut doc = lopdf::Document::load_mem(&bytes).expect("Failed to load PDF");

            let rules = vec![
                AnnotationRule {
                    id: Some("rule-highlight-ai".to_string()),
                    r#type: "highlight".to_string(),
                    color: "#FFFF00".to_string(),
                    search: Some("Artificial Intelligence".to_string()),
                    regex: None,
                    case_sensitive: false,
                    pages: Some(vec![1]),
                    rect: None,
                    page: None,
                    occurrence: "all".to_string(),
                    content: None,
                },
                AnnotationRule {
                    id: Some("rule-underline-year".to_string()),
                    r#type: "underline".to_string(),
                    color: "#FF0000".to_string(),
                    search: None,
                    regex: Some(r"\b2026\b".to_string()),
                    case_sensitive: false,
                    pages: None,
                    rect: None,
                    page: None,
                    occurrence: "first".to_string(),
                    content: None,
                },
                AnnotationRule {
                    id: Some("rule-strikethrough-auto".to_string()),
                    r#type: "strikethrough".to_string(),
                    color: "#0000FF".to_string(),
                    search: Some("autonomous".to_string()),
                    regex: None,
                    case_sensitive: false,
                    pages: None,
                    rect: None,
                    page: None,
                    occurrence: "all".to_string(),
                    content: None,
                },
                AnnotationRule {
                    id: Some("rule-note-pipeline".to_string()),
                    r#type: "note".to_string(),
                    color: "#00FF00".to_string(),
                    search: Some("pipeline".to_string()),
                    regex: None,
                    case_sensitive: false,
                    pages: None,
                    rect: None,
                    page: None,
                    occurrence: "first".to_string(),
                    content: Some("Review this pipeline logic".to_string()),
                },
            ];

            let op = AnnotateOperation::new().with_rules(rules);
            let res = op.execute(&mut doc);
            assert!(res.is_ok(), "Operation failed: {:?}", res.err());

            let pages = doc.get_pages();
            let page_id = *pages.get(&1).unwrap();
            let page_obj = doc.get_object(page_id).unwrap();
            if let Object::Dictionary(dict) = page_obj {
                assert!(dict.has(b"Annots"));
                if let Ok(Object::Array(arr)) = dict.get(b"Annots") {
                    assert_eq!(arr.len(), 4, "Expected 4 annotations created from 4 rules");
                }
            }
        }
    }
}
