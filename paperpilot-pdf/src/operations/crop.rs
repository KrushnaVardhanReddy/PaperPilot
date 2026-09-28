use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct CropBox {
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
}

pub struct CropPagesOperation {
    crop_box: CropBox,
}

impl CropPagesOperation {
    pub fn new(crop_box: CropBox) -> Self {
        Self { crop_box }
    }
}

impl PdfOperation for CropPagesOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document is not a LopdfDocument".to_string())
            })?;

        if self.crop_box.left >= self.crop_box.right || self.crop_box.bottom >= self.crop_box.top {
            return Err(PdfError::UnsupportedOperation(
                "Invalid crop box dimensions".to_string(),
            ));
        }

        let mut pages_to_update = Vec::new();
        for (_page_number, object_id) in lopdf_doc.inner.get_pages() {
            pages_to_update.push(object_id);
        }

        let new_crop_box_array = vec![
            lopdf::Object::Real(self.crop_box.left),
            lopdf::Object::Real(self.crop_box.bottom),
            lopdf::Object::Real(self.crop_box.right),
            lopdf::Object::Real(self.crop_box.top),
        ];

        for object_id in pages_to_update {
            if let Ok(lopdf::Object::Dictionary(dict)) = lopdf_doc.inner.get_object_mut(object_id) {
                dict.set("CropBox", lopdf::Object::Array(new_crop_box_array.clone()));
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
    fn test_crop_valid() {
        let mut doc = create_test_document();
        let op = CropPagesOperation::new(CropBox {
            left: 10.0,
            bottom: 20.0,
            right: 100.0,
            top: 200.0,
        });

        assert!(op.execute(&mut doc).is_ok());

        let page_id = *doc.inner.get_pages().get(&1).unwrap();
        let dict = doc.inner.get_object(page_id).unwrap().as_dict().unwrap();

        let crop_box = dict.get(b"CropBox").unwrap().as_array().unwrap();
        assert_eq!(crop_box.len(), 4);

        let get_val = |obj: &lopdf::Object| {
            if let lopdf::Object::Real(r) = obj {
                *r
            } else if let lopdf::Object::Integer(i) = obj {
                *i as f32
            } else {
                0.0
            }
        };

        assert_eq!(get_val(&crop_box[0]), 10.0);
        assert_eq!(get_val(&crop_box[1]), 20.0);
        assert_eq!(get_val(&crop_box[2]), 100.0);
        assert_eq!(get_val(&crop_box[3]), 200.0);
    }

    #[test]
    fn test_crop_invalid_dimensions() {
        let mut doc = create_test_document();

        // left >= right
        let op1 = CropPagesOperation::new(CropBox {
            left: 100.0,
            bottom: 20.0,
            right: 10.0,
            top: 200.0,
        });
        assert!(op1.execute(&mut doc).is_err());

        // bottom >= top
        let op2 = CropPagesOperation::new(CropBox {
            left: 10.0,
            bottom: 200.0,
            right: 100.0,
            top: 20.0,
        });
        assert!(op2.execute(&mut doc).is_err());
    }
}
