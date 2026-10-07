use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct RemoveBlankPagesOperation {
    pub sensitivity: u8,
}

impl RemoveBlankPagesOperation {
    pub fn new(sensitivity: u8) -> Self {
        Self { sensitivity }
    }
}

impl PdfOperation for RemoveBlankPagesOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let mut pages_to_delete = Vec::new();

        let threshold = (100 - self.sensitivity.min(100)) as usize * 5;

        for (page_num, object_id) in inner.get_pages() {
            if let Ok(lopdf::Object::Dictionary(page_dict)) = inner.get_object(object_id) {
                if let Ok(contents) = page_dict.get(b"Contents") {
                    let stream_len = match contents {
                        lopdf::Object::Reference(ref_id) => {
                            if let Ok(lopdf::Object::Stream(stream)) = inner.get_object(*ref_id) {
                                stream.content.len()
                            } else {
                                0
                            }
                        }
                        lopdf::Object::Array(arr) => {
                            let mut total_len = 0;
                            for obj in arr {
                                if let lopdf::Object::Reference(ref_id) = obj
                                    && let Ok(lopdf::Object::Stream(stream)) =
                                        inner.get_object(*ref_id)
                                {
                                    total_len += stream.content.len();
                                }
                            }
                            total_len
                        }
                        _ => 0,
                    };

                    if stream_len <= threshold {
                        pages_to_delete.push(page_num);
                    }
                } else {
                    pages_to_delete.push(page_num);
                }
            }
        }

        if !pages_to_delete.is_empty() {
            inner.delete_pages(&pages_to_delete);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{Dictionary, Document, Object, Stream};

    fn create_dummy_doc_with_streams() -> LopdfDocument {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let mut page_ids = vec![];

        let mut page1 = Dictionary::new();
        page1.set("Type", Object::Name(b"Page".to_vec()));
        page1.set("Parent", Object::Reference(pages_id));
        let page1_id = doc.add_object(page1);
        page_ids.push(Object::Reference(page1_id));

        let stream2 = Stream::new(Dictionary::new(), b"q Q".to_vec());
        let stream2_id = doc.add_object(stream2);
        let mut page2 = Dictionary::new();
        page2.set("Type", Object::Name(b"Page".to_vec()));
        page2.set("Parent", Object::Reference(pages_id));
        page2.set("Contents", Object::Reference(stream2_id));
        let page2_id = doc.add_object(page2);
        page_ids.push(Object::Reference(page2_id));

        let large_content = vec![b'q'; 1000];
        let stream3 = Stream::new(Dictionary::new(), large_content);
        let stream3_id = doc.add_object(stream3);
        let mut page3 = Dictionary::new();
        page3.set("Type", Object::Name(b"Page".to_vec()));
        page3.set("Parent", Object::Reference(pages_id));
        page3.set("Contents", Object::Reference(stream3_id));
        let page3_id = doc.add_object(page3);
        page_ids.push(Object::Reference(page3_id));

        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Count", Object::Integer(3));
        pages_dict.set("Kids", Object::Array(page_ids));

        doc.objects.insert(pages_id, Object::Dictionary(pages_dict));
        doc.trailer
            .set("Root", Object::Dictionary(Dictionary::new()));

        let mut catalog = Dictionary::new();
        catalog.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog.set("Pages", Object::Reference(pages_id));
        let catalog_id = doc.add_object(catalog);
        doc.trailer.set("Root", Object::Reference(catalog_id));

        LopdfDocument { inner: doc }
    }

    #[test]
    fn test_remove_blank_pages() {
        let mut doc = create_dummy_doc_with_streams();

        let op = RemoveBlankPagesOperation::new(99);
        op.execute(&mut doc).unwrap();

        assert_eq!(doc.page_count().unwrap(), 1);
    }
}
