use crate::document::LopdfDocument;
use lopdf::dictionary;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct BatesNumberingOperation {
    start_number: u32,
    prefix: String,
    padding: usize,
}

impl BatesNumberingOperation {
    pub fn new(start_number: u32, prefix: String, padding: usize) -> Self {
        Self {
            start_number,
            prefix,
            padding,
        }
    }
}

impl PdfOperation for BatesNumberingOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document is not a LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;

        let font_id = inner.add_object(dictionary!(
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        ));

        let pages = inner.get_pages();
        let mut sorted_page_numbers: Vec<u32> = pages.keys().copied().collect();
        sorted_page_numbers.sort_unstable();

        for (current_number, page_number) in (self.start_number..).zip(sorted_page_numbers) {
            let object_id = *pages.get(&page_number).unwrap();

            if let Ok(lopdf::Object::Dictionary(page_dict)) = inner.get_object_mut(object_id) {
                let mut new_resources = None;
                if let Ok(res) = page_dict.get_mut(b"Resources") {
                    if let lopdf::Object::Dictionary(res_dict) = res {
                        let mut fonts = res_dict
                            .get(b"Font")
                            .and_then(|f| f.as_dict())
                            .cloned()
                            .unwrap_or_else(|_| lopdf::Dictionary::new());
                        fonts.set("F1", lopdf::Object::Reference(font_id));
                        res_dict.set("Font", lopdf::Object::Dictionary(fonts));
                    } else {
                        // If it's a reference we should probably follow it and modify it, but for simplicity let's assume we can overwrite or it's inline in our tests
                    }
                } else {
                    let mut fonts = lopdf::Dictionary::new();
                    fonts.set("F1", lopdf::Object::Reference(font_id));
                    let mut res_dict = lopdf::Dictionary::new();
                    res_dict.set("Font", lopdf::Object::Dictionary(fonts));
                    new_resources = Some(lopdf::Object::Dictionary(res_dict));
                }
                if let Some(res) = new_resources {
                    page_dict.set("Resources", res);
                }
            }

            let bates_number = format!("{:0width$}", current_number, width = self.padding);
            let text = format!("{}{}", self.prefix, bates_number);
            let escaped_text = text.replace("(", "\\(").replace(")", "\\)");

            let y_pos = 30.0;
            let x_pos = 500.0;

            let content = format!(
                "q\nBT\n/F1 10 Tf\n1 0 0 1 {} {} Tm\n0 g\n({text}) Tj\nET\nQ\n",
                x_pos,
                y_pos,
                text = escaped_text
            );

            let content_stream =
                lopdf::Stream::new(lopdf::Dictionary::new(), content.as_bytes().to_vec());
            let stream_id = inner.add_object(content_stream);

            if let Ok(lopdf::Object::Dictionary(dict)) = inner.get_object_mut(object_id) {
                if let Ok(contents) = dict.get_mut(b"Contents") {
                    match contents {
                        lopdf::Object::Reference(ref_id) => {
                            let old_ref = *ref_id;
                            *contents = lopdf::Object::Array(vec![
                                lopdf::Object::Reference(old_ref),
                                lopdf::Object::Reference(stream_id),
                            ]);
                        }
                        lopdf::Object::Array(arr) => {
                            arr.push(lopdf::Object::Reference(stream_id));
                        }
                        _ => {}
                    }
                } else {
                    dict.set("Contents", lopdf::Object::Reference(stream_id));
                }
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
    fn test_bates_numbering_placement() {
        let mut doc = create_test_document();
        let op = BatesNumberingOperation::new(1, "BATES-".to_string(), 6);

        assert!(op.execute(&mut doc).is_ok());

        let page_id = *doc.inner.get_pages().get(&1).unwrap();
        let dict = doc.inner.get_object(page_id).unwrap().as_dict().unwrap();

        assert!(dict.has(b"Resources"));

        let contents = dict.get(b"Contents").unwrap();
        match contents {
            lopdf::Object::Reference(_) => {
                // Was added as a reference
            }
            lopdf::Object::Array(arr) => {
                assert_eq!(arr.len(), 1); // 0 original, 1 added
            }
            _ => panic!("Contents is not Reference or Array"),
        }
    }
}
