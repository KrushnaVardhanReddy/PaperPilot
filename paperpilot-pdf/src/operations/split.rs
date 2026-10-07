use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use std::path::PathBuf;

pub struct SplitOperation {
    pub output_dir: PathBuf,
    pub split_points: Vec<u32>, // pages to split after (1-based index)
}

impl SplitOperation {
    pub fn new(output_dir: PathBuf, split_points: Vec<u32>) -> Self {
        Self {
            output_dir,
            split_points,
        }
    }
}

impl PdfOperation for SplitOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        std::fs::create_dir_all(&self.output_dir).map_err(|e| PdfError::IoError(e))?;

        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be a LopdfDocument".to_string())
            })?;

        let mut start_page = 1;
        let mut doc_index = 1;

        let total_pages = lopdf_doc.page_count()?;
        let mut split_pts = self.split_points.clone();
        split_pts.sort();
        split_pts.push(total_pages);

        for end_page in split_pts {
            if end_page < start_page {
                continue;
            }
            if start_page > total_pages {
                break;
            }

            let mut part_doc = lopdf_doc.inner.clone();
            let mut pages_to_delete = Vec::new();

            for p in 1..=total_pages {
                if p < start_page || p > end_page {
                    pages_to_delete.push(p);
                }
            }

            if !pages_to_delete.is_empty() {
                part_doc.delete_pages(&pages_to_delete);
            }

            let output_path = self.output_dir.join(format!("split_{}.pdf", doc_index));
            part_doc
                .save(&output_path)
                .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

            start_page = end_page + 1;
            doc_index += 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{Dictionary, Object};
    use tempfile::tempdir;

    #[test]
    fn test_split_operation() {
        let dir = tempdir().unwrap();

        let mut main_doc = LopdfDocument::new();
        let pages_id = main_doc.inner.new_object_id();
        main_doc.inner.objects.insert(
            pages_id,
            Object::Dictionary(Dictionary::from_iter(vec![
                (b"Type".to_vec(), Object::Name(b"Pages".to_vec())),
                (b"Count".to_vec(), Object::Integer(0)),
                (b"Kids".to_vec(), Object::Array(vec![])),
            ])),
        );
        let root_id = main_doc.inner.new_object_id();
        main_doc.inner.objects.insert(
            root_id,
            Object::Dictionary(Dictionary::from_iter(vec![
                (b"Type".to_vec(), Object::Name(b"Catalog".to_vec())),
                (b"Pages".to_vec(), Object::Reference(pages_id)),
            ])),
        );
        main_doc
            .inner
            .trailer
            .set("Root", Object::Reference(root_id));

        for _ in 1..=3 {
            let page_id = main_doc.inner.new_object_id();
            main_doc.inner.objects.insert(
                page_id,
                Object::Dictionary(Dictionary::from_iter(vec![
                    (b"Type".to_vec(), Object::Name(b"Page".to_vec())),
                    (b"Parent".to_vec(), Object::Reference(pages_id)),
                ])),
            );
            if let Object::Dictionary(dict) = main_doc.inner.get_object_mut(pages_id).unwrap() {
                if let Ok(Object::Array(kids)) = dict.get_mut(b"Kids") {
                    kids.push(Object::Reference(page_id));
                }
                if let Ok(Object::Integer(count)) = dict.get_mut(b"Count") {
                    *count += 1;
                }
            }
        }

        let split_op = SplitOperation::new(dir.path().to_path_buf(), vec![1, 2]);
        let result = split_op.execute(&mut main_doc);
        assert!(result.is_ok());

        assert!(dir.path().join("split_1.pdf").exists());
        assert!(dir.path().join("split_2.pdf").exists());
        assert!(dir.path().join("split_3.pdf").exists());
    }
}
