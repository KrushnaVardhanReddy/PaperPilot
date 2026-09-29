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

#[allow(clippy::collapsible_if)]
impl PdfOperation for ReadFormOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;

        let catalog = doc.inner.catalog().map_err(|_| {
            PdfError::ParseError("Missing Document Catalog".into())
        })?;

        let acroform_ref = match catalog.get(b"AcroForm") {
            Ok(Object::Reference(id)) => *id,
            Ok(Object::Dictionary(_)) => {
                return Err(PdfError::ParseError("Inline AcroForm dict not fully supported".into()));
            }
            _ => {
                let mut map = self.extracted_fields.lock().unwrap();
                *map = Some(HashMap::new());
                return Ok(());
            }
        };

        let acroform = doc.inner.get_dictionary(acroform_ref).map_err(|_| {
            PdfError::ParseError("Failed to get AcroForm dictionary".into())
        })?;

        let mut fields_map = HashMap::new();

        if let Ok(Object::Array(fields)) = acroform.get(b"Fields") {
            for field_obj in fields {
                if let Object::Reference(field_id) = field_obj {
                    if let Ok(field_dict) = doc.inner.get_dictionary(*field_id) {
                        if let Ok(Object::String(name_bytes, _)) = field_dict.get(b"T") {
                            let name = String::from_utf8_lossy(name_bytes).to_string();

                            let value = if let Ok(v) = field_dict.get(b"V") {
                                match v {
                                    Object::String(val_bytes, _) => {
                                        String::from_utf8_lossy(val_bytes).to_string()
                                    }
                                    Object::Name(name_bytes) => {
                                        String::from_utf8_lossy(name_bytes).to_string()
                                    }
                                    _ => "".to_string(),
                                }
                            } else if let Ok(Object::Name(as_bytes)) = field_dict.get(b"AS") {
                                String::from_utf8_lossy(as_bytes).to_string()
                            } else {
                                "".to_string()
                            };

                            fields_map.insert(name, value);
                        }
                    }
                }
            }
        }

        let mut map = self.extracted_fields.lock().unwrap();
        *map = Some(fields_map);

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

#[allow(clippy::collapsible_if)]
impl PdfOperation for FillFormOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;

        let catalog = doc.inner.catalog().map_err(|_| {
            PdfError::ParseError("Missing Document Catalog".into())
        })?;

        let acroform_ref = match catalog.get(b"AcroForm") {
            Ok(Object::Reference(id)) => *id,
            _ => return Err(PdfError::ParseError("AcroForm not found or unsupported format".into())),
        };

        let mut field_ids = Vec::new();
        if let Ok(acroform) = doc.inner.get_dictionary(acroform_ref) {
            if let Ok(Object::Array(fields)) = acroform.get(b"Fields") {
                for field_obj in fields {
                    if let Object::Reference(field_id) = field_obj {
                        field_ids.push(*field_id);
                    }
                }
            }
        }

        let mut fields_to_update = Vec::new();

        for field_id in field_ids {
            if let Ok(field_dict) = doc.inner.get_dictionary(field_id) {
                if let Ok(Object::String(name_bytes, _)) = field_dict.get(b"T") {
                    let name = String::from_utf8_lossy(name_bytes).to_string();
                    if let Some(new_value) = self.values.get(&name) {
                        fields_to_update.push((field_id, new_value.clone()));
                    }
                }
            }
        }

        for (field_id, new_value) in fields_to_update {
            if let Ok(field_dict) = doc.inner.get_object_mut(field_id).and_then(Object::as_dict_mut) {
                let is_checkbox = if let Ok(Object::Name(ft)) = field_dict.get(b"FT") {
                    ft == b"Btn"
                } else {
                    false
                };

                if is_checkbox {
                    field_dict.set("V", Object::Name(new_value.clone().into_bytes()));
                    field_dict.set("AS", Object::Name(new_value.clone().into_bytes()));
                } else {
                    field_dict.set("V", Object::String(new_value.clone().into_bytes(), StringFormat::Literal));
                }
            }
        }

        Ok(())
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

#[allow(clippy::collapsible_if)]
impl PdfOperation for CreateFormFieldOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;

        let pages = doc.inner.get_pages();
        let page_id = pages.get(&(self.page as u32)).ok_or_else(|| {
            PdfError::InvalidInput(format!("Page {} not found", self.page))
        })?;
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

        widget_dict.set("T", Object::String(self.field_name.clone().into_bytes(), StringFormat::Literal));
        widget_dict.set("Rect", vec![
            self.rect[0].into(),
            self.rect[1].into(),
            self.rect[2].into(),
            self.rect[3].into(),
        ]);

        if self.field_type == "checkbox" {
            widget_dict.set("V", Object::Name(b"Off".to_vec()));
        }

        let widget_id = doc.inner.add_object(widget_dict);

        if let Ok(page_dict) = doc.inner.get_object_mut(page_id).and_then(Object::as_dict_mut) {
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
            _ => return Err(PdfError::ParseError("Missing Document Catalog reference in trailer".into())),
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

            if let Ok(catalog) = doc.inner.get_object_mut(catalog_id).and_then(Object::as_dict_mut) {
                catalog.set("AcroForm", Object::Reference(new_id));
            }
            new_id
        };

        if let Ok(af_dict) = doc.inner.get_object_mut(af_id).and_then(Object::as_dict_mut) {
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

        let page_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Page".to_vec())),
        ]));

        let pages_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Kids", Object::Array(vec![Object::Reference(page_id)])),
            ("Count", Object::Integer(1)),
        ]));

        let catalog_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Catalog".to_vec())),
            ("Pages", Object::Reference(pages_id)),
        ]));

        doc.trailer.set("Root", Object::Reference(catalog_id));

        let field_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Annot".to_vec())),
            ("Subtype", Object::Name(b"Widget".to_vec())),
            ("T", Object::String(b"FirstName".to_vec(), StringFormat::Literal)),
            ("V", Object::String(b"John".to_vec(), StringFormat::Literal)),
        ]));

        let acroform_id = doc.add_object(Dictionary::from_iter(vec![
            ("Fields", Object::Array(vec![Object::Reference(field_id)])),
        ]));

        if let Ok(catalog) = doc.get_object_mut(catalog_id).and_then(Object::as_dict_mut) {
            catalog.set("AcroForm", Object::Reference(acroform_id));
        }

        LopdfDocument { inner: doc }
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
