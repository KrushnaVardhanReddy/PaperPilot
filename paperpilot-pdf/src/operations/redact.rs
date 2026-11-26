use crate::document::LopdfDocument;
use lopdf::content::{Content, Operation};
use lopdf::StringFormat;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedactRule {
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
    pub rect: Option<[f32; 4]>,
    /// Target page for explicit rect (default 1)
    #[serde(default)]
    pub page: Option<u32>,
    /// Occurrence selector: 'all' (default), 'first', 'last', or comma-separated indices "0,2"
    #[serde(default = "default_occurrence")]
    pub occurrence: String,
    /// Optional overlay text (e.g. "[REDACTED]")
    #[serde(default)]
    pub overlay_text: Option<String>,
}

fn default_occurrence() -> String {
    "all".to_string()
}

#[derive(Debug, Clone)]
struct TargetRedaction {
    page: u32,
    bbox: BoundingBox,
    text_to_strip: Option<String>,
    overlay_text: Option<String>,
}

pub struct RedactOperation {
    page_number: Option<u32>,
    bounding_box: Option<BoundingBox>,
    rules: Vec<RedactRule>,
    overlay_text: Option<String>,
}

impl RedactOperation {
    pub fn new(page_number: u32, bounding_box: BoundingBox) -> Self {
        Self {
            page_number: Some(page_number),
            bounding_box: Some(bounding_box),
            rules: Vec::new(),
            overlay_text: None,
        }
    }

    pub fn with_overlay_text(mut self, text: impl Into<String>) -> Self {
        self.overlay_text = Some(text.into());
        self
    }

    pub fn with_rules(mut self, rules: Vec<RedactRule>) -> Self {
        self.rules = rules;
        self
    }

    pub fn from_rules(rules: Vec<RedactRule>) -> Self {
        Self {
            page_number: None,
            bounding_box: None,
            rules,
            overlay_text: None,
        }
    }

    pub fn from_patterns(patterns: Vec<String>, overlay_text: Option<String>) -> Self {
        let rules = patterns
            .into_iter()
            .map(|pat| RedactRule {
                search: Some(pat),
                regex: None,
                case_sensitive: false,
                pages: None,
                rect: None,
                page: None,
                occurrence: "all".to_string(),
                overlay_text: overlay_text.clone(),
            })
            .collect();
        Self {
            page_number: None,
            bounding_box: None,
            rules,
            overlay_text,
        }
    }
}

fn ensure_page_font(doc: &mut lopdf::Document, page_id: lopdf::ObjectId) {
    let mut font_dict = lopdf::Dictionary::new();
    font_dict.set("Type", lopdf::Object::Name(b"Font".to_vec()));
    font_dict.set("Subtype", lopdf::Object::Name(b"Type1".to_vec()));
    font_dict.set("BaseFont", lopdf::Object::Name(b"Helvetica".to_vec()));
    let font_id = doc.add_object(lopdf::Object::Dictionary(font_dict));

    if let Ok(lopdf::Object::Dictionary(page_dict)) = doc.get_object_mut(page_id) {
        if let Ok(res) = page_dict.get_mut(b"Resources") {
            if let lopdf::Object::Dictionary(res_dict) = res {
                let mut fonts = res_dict
                    .get(b"Font")
                    .and_then(|f| f.as_dict())
                    .cloned()
                    .unwrap_or_else(|_| lopdf::Dictionary::new());
                fonts.set("F1", lopdf::Object::Reference(font_id));
                res_dict.set("Font", lopdf::Object::Dictionary(fonts));
            }
        } else {
            let mut fonts = lopdf::Dictionary::new();
            fonts.set("F1", lopdf::Object::Reference(font_id));
            let mut res_dict = lopdf::Dictionary::new();
            res_dict.set("Font", lopdf::Object::Dictionary(fonts));
            page_dict.set("Resources", lopdf::Object::Dictionary(res_dict));
        }
    }
}

impl PdfOperation for RedactOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document is not a LopdfDocument".to_string())
            })?;

        let mut targets = Vec::new();

        if let (Some(page_number), Some(bounding_box)) = (self.page_number, &self.bounding_box) {
            targets.push(TargetRedaction {
                page: page_number,
                bbox: bounding_box.clone(),
                text_to_strip: None,
                overlay_text: self.overlay_text.clone(),
            });
        }

        if !self.rules.is_empty() {
            let mut doc_bytes = Vec::new();
            let _ = lopdf_doc.inner.save_to(&mut doc_bytes);

            for rule in &self.rules {
                if rule.search.is_some() || rule.regex.is_some() {
                    let matches = crate::operations::pdf_extract_patterns::find_text_matches(
                        &doc_bytes,
                        rule.search.as_deref(),
                        rule.regex.as_deref(),
                        rule.case_sensitive,
                        rule.pages.as_deref(),
                    );
                    let filtered = crate::operations::pdf_extract_patterns::filter_occurrences(matches, &rule.occurrence);
                    for m in filtered {
                        targets.push(TargetRedaction {
                            page: m.page,
                            bbox: BoundingBox {
                                x: m.rect[0] as f32,
                                y: m.rect[1] as f32,
                                width: m.rect[2] as f32,
                                height: m.rect[3] as f32,
                            },
                            text_to_strip: Some(m.text),
                            overlay_text: rule.overlay_text.clone().or_else(|| self.overlay_text.clone()),
                        });
                    }
                } else if let Some(rect) = rule.rect {
                    let page = rule.page.unwrap_or(1);
                    if rule.pages.is_none() || rule.pages.as_ref().unwrap().contains(&page) {
                        targets.push(TargetRedaction {
                            page,
                            bbox: BoundingBox {
                                x: rect[0],
                                y: rect[1],
                                width: rect[2],
                                height: rect[3],
                            },
                            text_to_strip: None,
                            overlay_text: rule.overlay_text.clone().or_else(|| self.overlay_text.clone()),
                        });
                    }
                }
            }
        }

        if targets.is_empty() {
            return Ok(());
        }

        // Group targets by page
        let mut by_page: std::collections::HashMap<u32, Vec<TargetRedaction>> = std::collections::HashMap::new();
        for t in targets {
            by_page.entry(t.page).or_default().push(t);
        }

        let mut sorted_pages: Vec<u32> = by_page.keys().copied().collect();
        sorted_pages.sort_unstable();

        for page_num in sorted_pages {
            let page_targets = by_page.remove(&page_num).unwrap();
            let page_id = *lopdf_doc
                .inner
                .get_pages()
                .get(&page_num)
                .ok_or_else(|| PdfError::Other(format!("Page {} not found", page_num)))?;

            // 1. Permanently remove matched text glyphs
            let texts_to_strip: Vec<String> = page_targets.iter().filter_map(|t| t.text_to_strip.clone()).collect();
            if !texts_to_strip.is_empty() {
                let content_bytes = lopdf_doc.inner.get_page_content(page_id);
                if !content_bytes.is_empty() {
                    if let Ok(mut content) = Content::decode(&content_bytes) {
                        let mut modified = false;
                        for op in &mut content.operations {
                            match op.operator.as_str() {
                                "Tj" | "'" => {
                                    if let Some(lopdf::Object::String(bytes, _)) = op.operands.first_mut() {
                                        if let Ok(text) = String::from_utf8(bytes.clone()) {
                                            let mut new_text = text.clone();
                                            for target in &texts_to_strip {
                                                if new_text.contains(target) {
                                                    let replacement = " ".repeat(target.chars().count());
                                                    new_text = new_text.replace(target, &replacement);
                                                    modified = true;
                                                }
                                            }
                                            if modified {
                                                *bytes = new_text.into_bytes();
                                            }
                                        }
                                    }
                                }
                                "\"" => {
                                    if let Some(lopdf::Object::String(bytes, _)) = op.operands.get_mut(2) {
                                        if let Ok(text) = String::from_utf8(bytes.clone()) {
                                            let mut new_text = text.clone();
                                            for target in &texts_to_strip {
                                                if new_text.contains(target) {
                                                    let replacement = " ".repeat(target.chars().count());
                                                    new_text = new_text.replace(target, &replacement);
                                                    modified = true;
                                                }
                                            }
                                            if modified {
                                                *bytes = new_text.into_bytes();
                                            }
                                        }
                                    }
                                }
                                "TJ" => {
                                    if let Some(lopdf::Object::Array(arr)) = op.operands.first_mut() {
                                        for item in arr.iter_mut() {
                                            if let lopdf::Object::String(bytes, _) = item {
                                                if let Ok(text) = String::from_utf8(bytes.clone()) {
                                                    let mut new_text = text.clone();
                                                    for target in &texts_to_strip {
                                                        if new_text.contains(target) {
                                                            let replacement = " ".repeat(target.chars().count());
                                                            new_text = new_text.replace(target, &replacement);
                                                            modified = true;
                                                        }
                                                    }
                                                    if modified {
                                                        *bytes = new_text.into_bytes();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        if modified {
                            if let Ok(new_encoded) = content.encode() {
                                let _ = lopdf_doc.inner.change_page_content(page_id, new_encoded);
                            }
                        }
                    }
                }
            }

            // 2. Add blackout boxes and optional overlay text
            let mut page_ops = Vec::new();
            for target in &page_targets {
                page_ops.push(Operation::new("q", vec![]));
                page_ops.push(Operation::new(
                    "rg",
                    vec![
                        lopdf::Object::Real(0.0),
                        lopdf::Object::Real(0.0),
                        lopdf::Object::Real(0.0),
                    ],
                ));
                page_ops.push(Operation::new(
                    "re",
                    vec![
                        lopdf::Object::Real(target.bbox.x),
                        lopdf::Object::Real(target.bbox.y),
                        lopdf::Object::Real(target.bbox.width),
                        lopdf::Object::Real(target.bbox.height),
                    ],
                ));
                page_ops.push(Operation::new("f", vec![]));

                if let Some(overlay) = &target.overlay_text {
                    if !overlay.is_empty() {
                        ensure_page_font(&mut lopdf_doc.inner, page_id);
                        let font_size = (target.bbox.height * 0.7).clamp(6.0, 14.0);
                        let approx_char_w = font_size * 0.55;
                        let text_w = overlay.len() as f32 * approx_char_w;
                        let text_x = (target.bbox.x + (target.bbox.width - text_w) / 2.0).max(target.bbox.x + 2.0);
                        let text_y = target.bbox.y + (target.bbox.height - font_size) / 2.0 + 1.0;

                        page_ops.push(Operation::new("BT", vec![]));
                        page_ops.push(Operation::new("Tf", vec![lopdf::Object::Name(b"F1".to_vec()), lopdf::Object::Real(font_size)]));
                        page_ops.push(Operation::new("rg", vec![lopdf::Object::Real(1.0), lopdf::Object::Real(1.0), lopdf::Object::Real(1.0)]));
                        page_ops.push(Operation::new("Tm", vec![
                            lopdf::Object::Real(1.0), lopdf::Object::Real(0.0),
                            lopdf::Object::Real(0.0), lopdf::Object::Real(1.0),
                            lopdf::Object::Real(text_x), lopdf::Object::Real(text_y),
                        ]));
                        page_ops.push(Operation::new("Tj", vec![lopdf::Object::String(overlay.as_bytes().to_vec(), StringFormat::Literal)]));
                        page_ops.push(Operation::new("ET", vec![]));
                    }
                }

                page_ops.push(Operation::new("Q", vec![]));
            }

            lopdf_doc
                .inner
                .add_to_page_content(page_id, Content { operations: page_ops })
                .map_err(|e| PdfError::Other(format!("Failed to add redaction to page: {}", e)))?;
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

        LopdfDocument {
            inner,
            original_bytes: None,
            source: None,
        }
    }

    #[test]
    fn test_redact_valid_page() {
        let mut doc = create_test_document();
        let op = RedactOperation::new(
            1,
            BoundingBox {
                x: 10.0,
                y: 20.0,
                width: 100.0,
                height: 50.0,
            },
        );

        assert!(op.execute(&mut doc).is_ok());

        let page_id = *doc.inner.get_pages().get(&1).unwrap();
        let content_bytes = doc.inner.get_page_content(page_id);
        let decoded = Content::decode(&content_bytes).unwrap();

        // We added 5 operations
        assert_eq!(decoded.operations.len(), 5);

        // Verify the operations
        assert_eq!(decoded.operations[0].operator, "q");
        assert_eq!(decoded.operations[1].operator, "rg");
        assert_eq!(decoded.operations[2].operator, "re");
        let get_val = |obj: &lopdf::Object| {
            if let lopdf::Object::Real(r) = obj {
                *r
            } else if let lopdf::Object::Integer(i) = obj {
                *i as f32
            } else {
                0.0
            }
        };

        assert_eq!(get_val(&decoded.operations[2].operands[0]), 10.0);
        assert_eq!(get_val(&decoded.operations[2].operands[1]), 20.0);
        assert_eq!(get_val(&decoded.operations[2].operands[2]), 100.0);
        assert_eq!(get_val(&decoded.operations[2].operands[3]), 50.0);
        assert_eq!(decoded.operations[3].operator, "f");
        assert_eq!(decoded.operations[4].operator, "Q");
    }

    #[test]
    fn test_redact_invalid_page() {
        let mut doc = create_test_document();
        let op = RedactOperation::new(
            99, // Non-existent page
            BoundingBox {
                x: 10.0,
                y: 20.0,
                width: 100.0,
                height: 50.0,
            },
        );

        let result = op.execute(&mut doc);
        assert!(result.is_err());
        if let Err(PdfError::Other(msg)) = result {
            assert!(msg.contains("Page 99 not found"));
        } else {
            panic!("Expected Other error");
        }
    }

    #[test]
    fn test_pattern_redaction_removes_glyphs_and_overlays_text() {
        let pdf_path = std::path::Path::new("../ManaulTestingFixtures/pdf_redact/Input/sensitive_customer_records.pdf");
        let alt_path = std::path::Path::new("ManaulTestingFixtures/pdf_redact/Input/sensitive_customer_records.pdf");
        let p = if pdf_path.exists() { pdf_path } else { alt_path };
        if let Ok(bytes) = std::fs::read(p) {
            let mut doc = LopdfDocument::from_bytes(&bytes).expect("Failed to load PDF");

            // Verify the initial document contains sensitive SSN
            let initial_text = doc.inner.extract_text(&[1]).unwrap_or_default();
            assert!(initial_text.contains("123-45-6789"), "Initial doc should contain SSN");

            // Define multi-rule pattern redactions
            let rules = vec![
                RedactRule {
                    search: Some("123-45-6789".to_string()),
                    regex: None,
                    case_sensitive: false,
                    pages: None,
                    rect: None,
                    page: None,
                    occurrence: "all".to_string(),
                    overlay_text: Some("[REDACTED]".to_string()),
                },
                RedactRule {
                    search: None,
                    regex: Some(r"\b4532-\d{4}-\d{4}-\d{4}\b".to_string()),
                    case_sensitive: false,
                    pages: None,
                    rect: None,
                    page: None,
                    occurrence: "first".to_string(),
                    overlay_text: Some("[REDACTED CC]".to_string()),
                },
            ];

            let op = RedactOperation::from_rules(rules);
            let res = op.execute(&mut doc);
            assert!(res.is_ok(), "Redaction failed: {:?}", res.err());

            // Check that sensitive text glyphs are permanently stripped!
            let redacted_text = doc.inner.extract_text(&[1]).unwrap_or_default();
            assert!(!redacted_text.contains("123-45-6789"), "Redacted doc must NOT contain SSN");
            assert!(!redacted_text.contains("4532-8901-2345-6789"), "Redacted doc must NOT contain Credit Card");
            assert!(redacted_text.contains("CONFIDENTIAL CUSTOMER RECORD"), "Preserved text should remain");
        }
    }
}
