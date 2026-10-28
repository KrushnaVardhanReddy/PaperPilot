#![allow(clippy::collapsible_if)]
use crate::document::LopdfDocument;
use lopdf::{Dictionary, Object, StringFormat};
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
}

impl CreateFormFieldOperation {
    pub fn new(field_name: String, field_type: String, page: i32, rect: [f32; 4]) -> Self {
        Self {
            field_name,
            field_type,
            page,
            rect,
        }
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

        let ft = if self.field_type == "checkbox" {
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

        if self.field_type == "checkbox" {
            widget_dict.set("V", Object::Name(b"Off".to_vec()));
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
