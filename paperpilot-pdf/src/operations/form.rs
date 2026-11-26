#![allow(clippy::collapsible_if)]
use crate::document::LopdfDocument;
use lopdf::{Dictionary, Object, Stream, StringFormat};
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Operation to read AcroForm fields and extract their values.
pub struct ReadFormOperation {
    pub extracted_fields: Arc<Mutex<Option<HashMap<String, String>>>>,
}

impl ReadFormOperation {
    pub fn new() -> Self {
        Self {
            extracted_fields: Arc::new(Mutex::new(None)),
        }
    }
}

impl Default for ReadFormOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfOperation for ReadFormOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;
        let fields = super::form_fill::read_fields(&doc.inner)?;
        *self.extracted_fields.lock().unwrap() = Some(fields);
        Ok(())
    }
}

/// Operation to fill AcroForm fields with provided values.
pub struct FillFormOperation {
    pub values: HashMap<String, String>,
}

impl FillFormOperation {
    pub fn new(values: HashMap<String, String>) -> Self {
        Self { values }
    }
}

impl PdfOperation for FillFormOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;
        super::form_fill::fill_fields(&mut doc.inner, &self.values)
    }
}

/// Operation to add a new form field.
pub struct CreateFormFieldOperation {
    pub field_name: String,
    pub field_type: String, // "text" or "checkbox"
    pub page: i32,          // 1-based index
    pub rect: [f32; 4],     // [llx, lly, urx, ury]
    pub label: Option<String>,
}

impl CreateFormFieldOperation {
    pub fn new(field_name: String, field_type: String, page: i32, rect: [f32; 4]) -> Self {
        Self {
            field_name,
            field_type,
            page,
            rect,
            label: None,
        }
    }

    pub fn with_label(mut self, label: Option<String>) -> Self {
        self.label = label;
        self
    }
}

impl PdfOperation for CreateFormFieldOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;

        let pages = doc.inner.get_pages();
        let page_id = pages
            .get(&(self.page as u32))
            .ok_or_else(|| PdfError::InvalidInput(format!("Page {} not found", self.page)))?;
        let page_id = *page_id;

        let mut widget_dict = Dictionary::new();
        widget_dict.set("Type", Object::Name(b"Annot".to_vec()));
        widget_dict.set("Subtype", Object::Name(b"Widget".to_vec()));
        widget_dict.set("F", Object::Integer(4)); // Print flag

        let ft = if self.field_type == "checkbox" {
            b"Btn".to_vec()
        } else if self.field_type == "choice" || self.field_type == "dropdown" {
            b"Ch".to_vec()
        } else if self.field_type == "radio" {
            b"Btn".to_vec()
        } else {
            b"Tx".to_vec()
        };
        widget_dict.set("FT", Object::Name(ft));

        widget_dict.set(
            "T",
            Object::String(self.field_name.clone().into_bytes(), StringFormat::Literal),
        );
        widget_dict.set(
            "Rect",
            vec![
                self.rect[0].into(),
                self.rect[1].into(),
                self.rect[2].into(),
                self.rect[3].into(),
            ],
        );

        // Standard visual appearance characteristics (/BS and /MK) for PDF viewers (Evince, Acrobat, Preview)
        let mut bs_dict = Dictionary::new();
        bs_dict.set("W", Object::Integer(1));
        bs_dict.set("S", Object::Name(b"S".to_vec())); // Solid border
        widget_dict.set("BS", Object::Dictionary(bs_dict));

        let mut mk_dict = Dictionary::new();
        // Border color: subtle slate grey [0.58, 0.64, 0.72]
        mk_dict.set(
            "BC",
            vec![
                0.58f32.into(),
                0.64f32.into(),
                0.72f32.into(),
            ],
        );
        // Background fill: soft light blue [0.93, 0.95, 1.0]
        mk_dict.set(
            "BG",
            vec![
                0.93f32.into(),
                0.95f32.into(),
                1.0f32.into(),
            ],
        );
        widget_dict.set("MK", Object::Dictionary(mk_dict));

        // Default Appearance string (/DA) for text rendering
        widget_dict.set("DA", Object::String(b"/Helv 11 Tf 0.1 0.1 0.1 rg".to_vec(), StringFormat::Literal));

        if self.field_type == "checkbox" {
            widget_dict.set("V", Object::Name(b"Off".to_vec()));
        } else if self.field_type == "choice" || self.field_type == "dropdown" {
            widget_dict.set("Ff", Object::Integer(131072)); // Combo box
            widget_dict.set(
                "Opt",
                Object::Array(vec![
                    Object::string_literal("Enterprise"),
                    Object::string_literal("Individual"),
                    Object::string_literal("Government"),
                ]),
            );
            widget_dict.set("V", Object::string_literal("Enterprise"));
            widget_dict.set("DV", Object::string_literal("Enterprise"));
        } else if self.field_type == "radio" {
            widget_dict.set("Ff", Object::Integer(32768)); // Radio
            widget_dict.set("V", Object::Name(b"Enterprise".to_vec()));
        }

        let widget_id = doc.inner.add_object(widget_dict);

        if let Ok(page_dict) = doc
            .inner
            .get_object_mut(page_id)
            .and_then(Object::as_dict_mut)
        {
            match page_dict.get_mut(b"Annots") {
                Ok(Object::Array(annots)) => {
                    annots.push(Object::Reference(widget_id));
                }
                _ => {
                    page_dict.set("Annots", vec![Object::Reference(widget_id)]);
                }
            }
        }

        // If a label was provided, stamp it cleanly onto the page text stream before the field
        if let Some(ref label_text) = self.label {
            if !label_text.trim().is_empty() {
                // Ensure page resources has standard Font /F1
                let font_obj_id = {
                    let mut font_dict = Dictionary::new();
                    font_dict.set("Type", Object::Name(b"Font".to_vec()));
                    font_dict.set("Subtype", Object::Name(b"Type1".to_vec()));
                    font_dict.set("BaseFont", Object::Name(b"Helvetica-Bold".to_vec()));
                    font_dict.set("Encoding", Object::Name(b"WinAnsiEncoding".to_vec()));
                    doc.inner.add_object(font_dict)
                };

                if let Ok(page_dict) = doc.inner.get_object_mut(page_id).and_then(Object::as_dict_mut) {
                    if let Ok(res) = page_dict.get_mut(b"Resources") {
                        if let Object::Dictionary(res_dict) = res {
                            if let Ok(font_res) = res_dict.get_mut(b"Font") {
                                if let Object::Dictionary(fonts) = font_res {
                                    fonts.set("F1", Object::Reference(font_obj_id));
                                }
                            } else {
                                let mut fonts = Dictionary::new();
                                fonts.set("F1", Object::Reference(font_obj_id));
                                res_dict.set("Font", Object::Dictionary(fonts));
                            }
                        }
                    } else {
                        let mut res_dict = Dictionary::new();
                        let mut fonts = Dictionary::new();
                        fonts.set("F1", Object::Reference(font_obj_id));
                        res_dict.set("Font", Object::Dictionary(fonts));
                        page_dict.set("Resources", Object::Dictionary(res_dict));
                    }
                }

                let escaped_label = label_text
                    .replace('\\', "\\\\")
                    .replace('(', "\\(")
                    .replace(')', "\\)");

                // Position label: vertically centered with the rect, placed either to the left or just above the box
                // Default: 12pt font, placed at left of field (llx - text_width) or at llx if placed above
                let label_y = self.rect[1] + (self.rect[3] - self.rect[1] - 10.0) / 2.0;
                let text_approx_width = escaped_label.len() as f32 * 6.5;
                let label_x = (self.rect[0] - text_approx_width - 8.0).max(54.0);

                let label_stream_content = format!(
                    "q\nBT\n/F1 11 Tf\n0 0 0 rg\n1 0 0 1 {} {} Tm\n({}) Tj\nET\nQ\n",
                    label_x,
                    label_y,
                    escaped_label
                );

                let stream_id = doc.inner.add_object(Stream::new(
                    Dictionary::new(),
                    label_stream_content.into_bytes(),
                ));

                if let Ok(page_dict) = doc.inner.get_object_mut(page_id).and_then(Object::as_dict_mut) {
                    match page_dict.get_mut(b"Contents") {
                        Ok(Object::Array(contents)) => {
                            contents.push(Object::Reference(stream_id));
                        }
                        Ok(Object::Reference(existing_id)) => {
                            let existing_ref = *existing_id;
                            page_dict.set("Contents", vec![Object::Reference(existing_ref), Object::Reference(stream_id)]);
                        }
                        _ => {
                            page_dict.set("Contents", vec![Object::Reference(stream_id)]);
                        }
                    }
                }
            }
        }

        // To manipulate the catalog safely, we need its ObjectId.
        let trailer = &doc.inner.trailer;
        let catalog_id = match trailer.get(b"Root") {
            Ok(Object::Reference(id)) => *id,
            _ => {
                return Err(PdfError::ParseError(
                    "Missing Document Catalog reference in trailer".into(),
                ));
            }
        };

        let mut acroform_ref = None;

        if let Ok(catalog) = doc.inner.get_object(catalog_id).and_then(Object::as_dict) {
            if let Ok(Object::Reference(id)) = catalog.get(b"AcroForm") {
                acroform_ref = Some(*id);
            }
        }

        let af_id = if let Some(id) = acroform_ref {
            id
        } else {
            let mut af_dict = Dictionary::new();
            af_dict.set("Fields", vec![]);
            let new_id = doc.inner.add_object(af_dict);

            if let Ok(catalog) = doc
                .inner
                .get_object_mut(catalog_id)
                .and_then(Object::as_dict_mut)
            {
                catalog.set("AcroForm", Object::Reference(new_id));
            }
            new_id
        };

        if let Ok(af_dict) = doc
            .inner
            .get_object_mut(af_id)
            .and_then(Object::as_dict_mut)
        {
            af_dict.set("NeedAppearances", Object::Boolean(true));
            match af_dict.get_mut(b"Fields") {
                Ok(Object::Array(fields)) => {
                    fields.push(Object::Reference(widget_id));
                }
                _ => {
                    af_dict.set("Fields", vec![Object::Reference(widget_id)]);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::Document;
    use paperpilot_core::traits::PdfOperation;

    fn create_test_doc_with_form() -> LopdfDocument {
        let mut doc = Document::with_version("1.5");

        let page_id = doc.add_object(Dictionary::from_iter(vec![(
            "Type",
            Object::Name(b"Page".to_vec()),
        )]));
        doc.get_object_mut(page_id)
            .unwrap()
            .as_dict_mut()
            .unwrap()
            .set("MediaBox", vec![0.into(), 0.into(), 612.into(), 792.into()]);

        let pages_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Kids", Object::Array(vec![Object::Reference(page_id)])),
            ("Count", Object::Integer(1)),
        ]));
        doc.get_object_mut(page_id)
            .unwrap()
            .as_dict_mut()
            .unwrap()
            .set("Parent", pages_id);

        let catalog_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Catalog".to_vec())),
            ("Pages", Object::Reference(pages_id)),
        ]));

        doc.trailer.set("Root", Object::Reference(catalog_id));

        let field_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Annot".to_vec())),
            ("Subtype", Object::Name(b"Widget".to_vec())),
            (
                "T",
                Object::String(b"FirstName".to_vec(), StringFormat::Literal),
            ),
            ("V", Object::String(b"John".to_vec(), StringFormat::Literal)),
        ]));
        let field = doc.get_object_mut(field_id).unwrap().as_dict_mut().unwrap();
        field.set("FT", Object::Name(b"Tx".to_vec()));
        field.set("Rect", vec![10.into(), 20.into(), 210.into(), 45.into()]);
        field.set("P", page_id);
        doc.get_object_mut(page_id)
            .unwrap()
            .as_dict_mut()
            .unwrap()
            .set("Annots", vec![Object::Reference(field_id)]);

        let acroform_id = doc.add_object(Dictionary::from_iter(vec![(
            "Fields",
            Object::Array(vec![Object::Reference(field_id)]),
        )]));

        if let Ok(catalog) = doc.get_object_mut(catalog_id).and_then(Object::as_dict_mut) {
            catalog.set("AcroForm", Object::Reference(acroform_id));
        }

        LopdfDocument {
            inner: doc,
            original_bytes: None,
            source: None,
        }
    }

    #[test]
    fn test_read_form_operation() {
        let mut doc = create_test_doc_with_form();
        let op = ReadFormOperation::new();

        assert!(op.execute(&mut doc).is_ok());

        let fields = op.extracted_fields.lock().unwrap();
        assert!(fields.is_some());
        let map = fields.as_ref().unwrap();

        assert_eq!(map.get("FirstName").unwrap(), "John");
    }

    #[test]
    fn test_fill_form_operation() {
        let mut doc = create_test_doc_with_form();
        let mut values = HashMap::new();
        values.insert("FirstName".to_string(), "Jane".to_string());

        let op = FillFormOperation::new(values);
        assert!(op.execute(&mut doc).is_ok());

        let read_op = ReadFormOperation::new();
        assert!(read_op.execute(&mut doc).is_ok());

        let fields = read_op.extracted_fields.lock().unwrap();
        let map = fields.as_ref().unwrap();

        assert_eq!(map.get("FirstName").unwrap(), "Jane");
    }

    #[test]
    fn test_create_form_field_operation() {
        let mut doc = create_test_doc_with_form();
        let op = CreateFormFieldOperation::new(
            "LastName".to_string(),
            "text".to_string(),
            1,
            [10.0, 20.0, 110.0, 40.0],
        );

        assert!(op.execute(&mut doc).is_ok());

        let read_op = ReadFormOperation::new();
        assert!(read_op.execute(&mut doc).is_ok());

        let fields = read_op.extracted_fields.lock().unwrap();
        let map = fields.as_ref().unwrap();

        assert!(map.contains_key("LastName"));
    }
}
