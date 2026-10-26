use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use lopdf::{Document, Object};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMergeAlternateOptions {
    pub step: usize,
    pub step_b: usize,
    pub reverse_a: bool,
    pub reverse_b: bool,
}

impl Default for PdfMergeAlternateOptions {
    fn default() -> Self {
        Self {
            step: 1,
            step_b: 1,
            reverse_a: false,
            reverse_b: false,
        }
    }
}

pub fn pdf_merge_alternate_bytes(
    a_bytes: &[u8],
    b_bytes: &[u8],
    options: &PdfMergeAlternateOptions,
) -> Result<Vec<u8>, PdfError> {
    let mut doc_a = Document::load_mem(a_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;
    let mut doc_b = Document::load_mem(b_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;

    doc_b.renumber_objects();

    let max_id_a = doc_a.max_id;
    let mut new_objects = BTreeMap::new();
    for (old_id, mut object) in doc_b.objects.clone() {
        let new_id = (old_id.0 + max_id_a, old_id.1);
        adjust_references(&mut object, max_id_a);
        new_objects.insert(new_id, object);
    }

    doc_a.objects.extend(new_objects);
    doc_a.max_id = doc_a.objects.keys().map(|k| k.0).max().unwrap_or(0);

    let mut pages_a: Vec<lopdf::ObjectId> = doc_a.get_pages().into_values().collect();
    let mut pages_b: Vec<lopdf::ObjectId> = doc_b
        .get_pages()
        .into_values()
        .map(|id| (id.0 + max_id_a, id.1))
        .collect();

    if options.reverse_a {
        pages_a.reverse();
    }
    if options.reverse_b {
        pages_b.reverse();
    }

    let mut collated_pages = Vec::new();
    let mut a_iter = pages_a.into_iter();
    let mut b_iter = pages_b.into_iter();

    loop {
        let mut took_any = false;
        for _ in 0..options.step {
            if let Some(p) = a_iter.next() {
                collated_pages.push(p);
                took_any = true;
            }
        }
        for _ in 0..options.step_b {
            if let Some(p) = b_iter.next() {
                collated_pages.push(p);
                took_any = true;
            }
        }
        if !took_any {
            break;
        }
    }

    let catalog_id = doc_a
        .trailer
        .get(b"Root")
        .and_then(|root| root.as_reference())
        .ok();

    let pages_id = match catalog_id {
        Some(cat_id) => {
            let catalog = doc_a.get_object(cat_id).unwrap();
            if let Object::Dictionary(dict) = catalog {
                if let Ok(pages) = dict.get(b"Pages") {
                    pages.as_reference().unwrap_or((doc_a.max_id + 1, 0))
                } else {
                    (doc_a.max_id + 1, 0)
                }
            } else {
                (doc_a.max_id + 1, 0)
            }
        }
        None => (doc_a.max_id + 1, 0),
    };

    if let Ok(Object::Dictionary(pages_dict)) = doc_a.get_object_mut(pages_id) {
        let kids_array = collated_pages.iter().map(|&id| Object::Reference(id)).collect();
        pages_dict.set("Kids", Object::Array(kids_array));
        pages_dict.set("Count", Object::Integer(collated_pages.len() as i64));
    }

    for page_id in collated_pages {
        if let Ok(Object::Dictionary(dict)) = doc_a.get_object_mut(page_id) {
            dict.set("Parent", Object::Reference(pages_id));
        }
    }

    let mut out_bytes = Vec::new();
    doc_a.save_to(&mut out_bytes).map_err(|e| PdfError::Other(e.to_string()))?;
    Ok(out_bytes)
}

fn adjust_references(object: &mut Object, offset: u32) {
    match object {
        Object::Reference(id) => {
            id.0 += offset;
        }
        Object::Array(arr) => {
            for obj in arr.iter_mut() {
                adjust_references(obj, offset);
            }
        }
        Object::Dictionary(dict) => {
            for (_, obj) in dict.iter_mut() {
                adjust_references(obj, offset);
            }
        }
        Object::Stream(stream) => {
            for (_, obj) in stream.dict.iter_mut() {
                adjust_references(obj, offset);
            }
        }
        _ => {}
    }
}

#[derive(Default)]
pub struct PdfMergeAlternateOperation {
    _options: PdfMergeAlternateOptions,
}

impl PdfMergeAlternateOperation {
    pub fn new(options: PdfMergeAlternateOptions) -> Self {
        Self { _options: options }
    }
}

impl PdfOperation for PdfMergeAlternateOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> paperpilot_core::error::OperationResult<()> {
        Err(PdfError::UnsupportedOperation("Not yet implemented for in-memory".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_pdf_merge_alternate_empty_document() {
        let options = PdfMergeAlternateOptions::default();
        let a_bytes = vec![];
        let b_bytes = vec![];
        let result = pdf_merge_alternate_bytes(&a_bytes, &b_bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_pdf_merge_alternate_with_real_fixture() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let fixture_path = PathBuf::from(manifest_dir).join("../tests/e2e_fixtures/real/sample_photo_doc.pdf");

        let a_bytes = fs::read(&fixture_path).expect("Failed to read fixture A");
        // For testing interleaving, we just use the same single page fixture for both.
        let b_bytes = fs::read(&fixture_path).expect("Failed to read fixture B");

        let options = PdfMergeAlternateOptions {
            step: 1,
            step_b: 1,
            reverse_a: false,
            reverse_b: false,
        };

        let result = pdf_merge_alternate_bytes(&a_bytes, &b_bytes, &options);
        assert!(result.is_ok(), "Merge alternate failed: {:?}", result.err());

        let out_bytes = result.unwrap();
        assert!(!out_bytes.is_empty(), "Output bytes are empty");

        // Verify it parses back as a valid document
        let doc = Document::load_mem(&out_bytes);
        assert!(doc.is_ok(), "Failed to parse the merged document");

        let parsed_doc = doc.unwrap();
        // Since we combined 1 page from A and 1 page from B (same single page doc), total pages should be 2.
        assert_eq!(parsed_doc.get_pages().len(), 2, "Expected exactly 2 pages in collated doc");
    }

    #[test]
    fn test_pdf_merge_alternate_reverse_and_step() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let fixture_path = PathBuf::from(manifest_dir).join("../tests/e2e_fixtures/real/sample_photo_doc.pdf");

        let a_bytes = fs::read(&fixture_path).unwrap();
        let b_bytes = fs::read(&fixture_path).unwrap();

        let options = PdfMergeAlternateOptions {
            step: 2, // will try to take 2 pages from A (but only 1 exists)
            step_b: 1,
            reverse_a: true,
            reverse_b: true,
        };

        let result = pdf_merge_alternate_bytes(&a_bytes, &b_bytes, &options).unwrap();
        let parsed_doc = Document::load_mem(&result).unwrap();

        assert_eq!(parsed_doc.get_pages().len(), 2);
    }
}
