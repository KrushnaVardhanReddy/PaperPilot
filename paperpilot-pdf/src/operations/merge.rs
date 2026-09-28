use crate::document::LopdfDocument;
use lopdf::{Document, Object};
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use std::collections::BTreeMap;
use std::path::PathBuf;

pub struct MergeOperation {
    pub paths: Vec<PathBuf>,
}

impl MergeOperation {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self { paths }
    }
}

impl PdfOperation for MergeOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be a LopdfDocument".to_string())
            })?;

        for path in &self.paths {
            let mut other_doc =
                Document::load(path).map_err(|e| PdfError::ParseError(e.to_string()))?;

            other_doc.renumber_objects();

            let max_id = lopdf_doc.inner.max_id;

            let mut new_objects = BTreeMap::new();
            for (old_id, mut object) in other_doc.objects.clone() {
                let new_id = (old_id.0 + max_id, old_id.1);

                Self::adjust_references(&mut object, max_id);
                new_objects.insert(new_id, object);
            }

            lopdf_doc.inner.objects.extend(new_objects);
            lopdf_doc.inner.max_id = lopdf_doc
                .inner
                .objects
                .keys()
                .map(|k| k.0)
                .max()
                .unwrap_or(0);

            let pages = other_doc.get_pages();

            let catalog_id = lopdf_doc
                .inner
                .trailer
                .get(b"Root")
                .and_then(|root| root.as_reference())
                .ok();
            let pages_id = match catalog_id {
                Some(catalog_id) => {
                    let catalog = lopdf_doc.inner.get_object(catalog_id).unwrap();
                    if let Object::Dictionary(dict) = catalog {
                        if let Ok(pages) = dict.get(b"Pages") {
                            pages
                                .as_reference()
                                .unwrap_or((lopdf_doc.inner.max_id + 1, 0))
                        } else {
                            (lopdf_doc.inner.max_id + 1, 0)
                        }
                    } else {
                        (lopdf_doc.inner.max_id + 1, 0)
                    }
                }
                None => (lopdf_doc.inner.max_id + 1, 0),
            };

            if !lopdf_doc.inner.objects.contains_key(&pages_id) {
                lopdf_doc.inner.max_id += 1;
                lopdf_doc.inner.objects.insert(
                    pages_id,
                    Object::Dictionary(lopdf::Dictionary::from_iter(vec![
                        (b"Type".to_vec(), Object::Name(b"Pages".to_vec())),
                        (b"Count".to_vec(), Object::Integer(0)),
                        (b"Kids".to_vec(), Object::Array(vec![])),
                    ])),
                );

                if let Some(catalog_id) = catalog_id {
                    if let Ok(Object::Dictionary(dict)) = lopdf_doc.inner.get_object_mut(catalog_id)
                    {
                        dict.set("Pages", Object::Reference(pages_id));
                    }
                } else {
                    let new_catalog_id = (lopdf_doc.inner.max_id + 1, 0);
                    lopdf_doc.inner.max_id += 1;
                    lopdf_doc.inner.objects.insert(
                        new_catalog_id,
                        Object::Dictionary(lopdf::Dictionary::from_iter(vec![
                            (b"Type".to_vec(), Object::Name(b"Catalog".to_vec())),
                            (b"Pages".to_vec(), Object::Reference(pages_id)),
                        ])),
                    );
                    lopdf_doc
                        .inner
                        .trailer
                        .set("Root", Object::Reference(new_catalog_id));
                }
            }

            for (_, page_id) in pages {
                let adjusted_page_id = (page_id.0 + max_id, page_id.1);

                let pages_obj = lopdf_doc.inner.get_object_mut(pages_id).unwrap();
                if let Object::Dictionary(dict) = pages_obj {
                    if let Ok(Object::Array(kids)) = dict.get_mut(b"Kids") {
                        kids.push(Object::Reference(adjusted_page_id));
                    }
                    if let Ok(Object::Integer(count)) = dict.get_mut(b"Count") {
                        *count += 1;
                    }
                }

                if let Ok(Object::Dictionary(dict)) =
                    lopdf_doc.inner.get_object_mut(adjusted_page_id)
                {
                    dict.set("Parent", Object::Reference(pages_id));
                }
            }
        }

        Ok(())
    }
}

impl MergeOperation {
    fn adjust_references(object: &mut Object, offset: u32) {
        match object {
            Object::Reference(id) => {
                id.0 += offset;
            }
            Object::Array(arr) => {
                for obj in arr.iter_mut() {
                    Self::adjust_references(obj, offset);
                }
            }
            Object::Dictionary(dict) => {
                for (_, obj) in dict.iter_mut() {
                    Self::adjust_references(obj, offset);
                }
            }
            Object::Stream(stream) => {
                for (_, obj) in stream.dict.iter_mut() {
                    Self::adjust_references(obj, offset);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Dictionary;
    use tempfile::tempdir;

    #[test]
    fn test_merge_operation() {
        let dir = tempdir().unwrap();
        let path1 = dir.path().join("doc1.pdf");
        let path2 = dir.path().join("doc2.pdf");

        let mut d1 = Document::with_version("1.5");
        let pages_id1 = d1.new_object_id();
        d1.objects.insert(
            pages_id1,
            Object::Dictionary(Dictionary::from_iter(vec![
                (b"Type".to_vec(), Object::Name(b"Pages".to_vec())),
                (b"Count".to_vec(), Object::Integer(0)),
                (b"Kids".to_vec(), Object::Array(vec![])),
            ])),
        );
        let root_id1 = d1.new_object_id();
        d1.objects.insert(
            root_id1,
            Object::Dictionary(Dictionary::from_iter(vec![
                (b"Type".to_vec(), Object::Name(b"Catalog".to_vec())),
                (b"Pages".to_vec(), Object::Reference(pages_id1)),
            ])),
        );
        d1.trailer.set("Root", Object::Reference(root_id1));
        d1.save(&path1).unwrap();

        let mut d2 = Document::with_version("1.5");
        let pages_id2 = d2.new_object_id();
        d2.objects.insert(
            pages_id2,
            Object::Dictionary(Dictionary::from_iter(vec![
                (b"Type".to_vec(), Object::Name(b"Pages".to_vec())),
                (b"Count".to_vec(), Object::Integer(0)),
                (b"Kids".to_vec(), Object::Array(vec![])),
            ])),
        );
        let root_id2 = d2.new_object_id();
        d2.objects.insert(
            root_id2,
            Object::Dictionary(Dictionary::from_iter(vec![
                (b"Type".to_vec(), Object::Name(b"Catalog".to_vec())),
                (b"Pages".to_vec(), Object::Reference(pages_id2)),
            ])),
        );
        d2.trailer.set("Root", Object::Reference(root_id2));
        d2.save(&path2).unwrap();

        let mut main_doc = LopdfDocument::new();

        let merge_op = MergeOperation::new(vec![path1.clone(), path2.clone()]);

        let result = merge_op.execute(&mut main_doc);
        assert!(result.is_ok());
    }
}
