use crate::document::LopdfDocument;
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PdfDedupStreamsOptions {
    // Add specific options here if needed later
}

pub fn pdf_dedup_streams_bytes(
    input_bytes: &[u8],
    options: &PdfDedupStreamsOptions,
) -> Result<Vec<u8>, PdfError> {
    let mut doc = LopdfDocument::from_inner(
        lopdf::Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?,
    );
    let op = PdfDedupStreamsOperation::new(options.clone());
    op.execute(&mut doc)?;

    let mut out_bytes = Vec::new();
    doc.inner
        .save_to(&mut out_bytes)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;
    Ok(out_bytes)
}

#[derive(Default)]
pub struct PdfDedupStreamsOperation {
    _options: PdfDedupStreamsOptions,
}

impl PdfDedupStreamsOperation {
    pub fn new(options: PdfDedupStreamsOptions) -> Self {
        Self { _options: options }
    }
}

impl PdfOperation for PdfDedupStreamsOperation {
    fn execute(
        &self,
        document: &mut dyn PdfDocument,
    ) -> paperpilot_core::error::OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document is not a LopdfDocument".to_string())
            })?;

        // 1. Map hash -> first encountered ObjectId
        let mut stream_hashes: HashMap<Vec<u8>, lopdf::ObjectId> = HashMap::new();
        // 2. Map duplicate ObjectId -> canonical ObjectId
        let mut duplicates: HashMap<lopdf::ObjectId, lopdf::ObjectId> = HashMap::new();

        let object_ids: Vec<lopdf::ObjectId> = lopdf_doc.inner.objects.keys().copied().collect();

        // Pass 1: Identify duplicates
        for object_id in object_ids {
            if let Ok(lopdf::Object::Stream(stream)) = lopdf_doc.inner.get_object(object_id) {
                // Determine decompressed content for hashing
                let content = match stream.decompressed_content() {
                    Ok(bytes) => bytes,
                    Err(_) => stream.content.clone(), // Fallback to compressed bytes if failure
                };

                let mut hasher = Sha256::new();
                hasher.update(&content);
                // Also hash important dict keys like Width, Height, BitsPerComponent, ColorSpace
                // to ensure we don't merge an image and a font with identical random binary data (unlikely but safe)
                if let Ok(w) = stream.dict.get(b"Width") {
                    hasher.update(format!("{:?}", w).as_bytes());
                }
                if let Ok(h) = stream.dict.get(b"Height") {
                    hasher.update(format!("{:?}", h).as_bytes());
                }
                if let Ok(bpc) = stream.dict.get(b"BitsPerComponent") {
                    hasher.update(format!("{:?}", bpc).as_bytes());
                }
                if let Ok(cs) = stream.dict.get(b"ColorSpace") {
                    hasher.update(format!("{:?}", cs).as_bytes());
                }
                let hash = hasher.finalize().to_vec();

                let mut found_duplicate = false;

                // Track current canonical id
                if let Some(&canonical_id) = stream_hashes.get(&hash) {
                    // Collision found, verify byte-by-byte for 100% safety
                    if let Ok(lopdf::Object::Stream(canonical_stream)) =
                        lopdf_doc.inner.get_object(canonical_id)
                    {
                        let canonical_content = match canonical_stream.decompressed_content() {
                            Ok(bytes) => bytes,
                            Err(_) => canonical_stream.content.clone(),
                        };

                        if content == canonical_content {
                            found_duplicate = true;
                            // Map the dropped one to the kept one
                            duplicates.insert(object_id, canonical_id);
                        }
                    }
                }

                // Track the very first one as canonical, subsequent ones map to it
                if !found_duplicate {
                    stream_hashes.insert(hash, object_id);
                }
            }
        }

        // Pass 2: Rewrite references
        if !duplicates.is_empty() {
            fn rewrite_references(
                obj: &mut lopdf::Object,
                duplicates: &HashMap<lopdf::ObjectId, lopdf::ObjectId>,
            ) {
                match obj {
                    lopdf::Object::Reference(id) => {
                        if let Some(&canonical_id) = duplicates.get(id) {
                            *id = canonical_id;
                        }
                    }
                    lopdf::Object::Array(arr) => {
                        for item in arr {
                            rewrite_references(item, duplicates);
                        }
                    }
                    lopdf::Object::Dictionary(dict) => {
                        // Note: iterating and modifying dict.values mutably is standard
                        for (_, value) in dict.iter_mut() {
                            rewrite_references(value, duplicates);
                        }
                    }
                    lopdf::Object::Stream(stream) => {
                        for (_, value) in stream.dict.iter_mut() {
                            rewrite_references(value, duplicates);
                        }
                    }
                    _ => {}
                }
            }

            for (_, obj) in lopdf_doc.inner.objects.iter_mut() {
                rewrite_references(obj, &duplicates);
            }

            // Also rewrite the trailer
            for (_, value) in lopdf_doc.inner.trailer.iter_mut() {
                rewrite_references(value, &duplicates);
            }
        }

        // Must rebuild object tree correctly
        lopdf_doc.inner.prune_objects();

        // Ensure catalog correctly tracks pages in case `prune_objects` is removing too much
        // Wait, `prune_objects` will traverse from Root to find reachable objects.
        // We shouldn't need any further cleanup.
        // But `delete_zero_length_streams` is unrelated. Let's explicitly remove our duplicates.
        for (duplicate_id, _) in duplicates.clone() {
            lopdf_doc.inner.objects.remove(&duplicate_id);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{Dictionary, Document, Object, Stream};

    #[test]
    fn test_pdf_dedup_streams_invalid_bytes() {
        let options = PdfDedupStreamsOptions::default();
        let bytes = vec![0, 1, 2, 3];
        let result = pdf_dedup_streams_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_pdf_dedup_streams_success() {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let page1_id = doc.new_object_id();
        let page2_id = doc.new_object_id();
        let img1_id = doc.new_object_id();
        let img2_id = doc.new_object_id();

        // Identical stream content
        let content = b"identical_image_data_12345".to_vec();

        let mut img1_dict = Dictionary::new();
        img1_dict.set("Type", Object::Name(b"XObject".to_vec()));
        img1_dict.set("Subtype", Object::Name(b"Image".to_vec()));
        let stream1 = Stream::new(img1_dict.clone(), content.clone());
        doc.objects.insert(img1_id, Object::Stream(stream1));

        let stream2 = Stream::new(img1_dict, content);
        doc.objects.insert(img2_id, Object::Stream(stream2));

        let mut res1 = Dictionary::new();
        let mut xo1 = Dictionary::new();
        xo1.set("Im1", Object::Reference(img1_id));
        res1.set("XObject", Object::Dictionary(xo1));

        let mut page1 = Dictionary::new();
        page1.set("Type", Object::Name(b"Page".to_vec()));
        page1.set("Parent", Object::Reference(pages_id));
        page1.set("Resources", Object::Dictionary(res1));
        doc.objects.insert(page1_id, Object::Dictionary(page1));

        let mut res2 = Dictionary::new();
        let mut xo2 = Dictionary::new();
        xo2.set("Im2", Object::Reference(img2_id));
        res2.set("XObject", Object::Dictionary(xo2));

        let mut page2 = Dictionary::new();
        page2.set("Type", Object::Name(b"Page".to_vec()));
        page2.set("Parent", Object::Reference(pages_id));
        page2.set("Resources", Object::Dictionary(res2));
        doc.objects.insert(page2_id, Object::Dictionary(page2));

        let mut pages = Dictionary::new();
        pages.set("Type", Object::Name(b"Pages".to_vec()));
        pages.set("Count", Object::Integer(2));
        pages.set(
            "Kids",
            Object::Array(vec![
                Object::Reference(page1_id),
                Object::Reference(page2_id),
            ]),
        );
        doc.objects.insert(pages_id, Object::Dictionary(pages));

        let mut catalog = Dictionary::new();
        catalog.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog.set("Pages", Object::Reference(pages_id));
        let catalog_id = doc.add_object(catalog);
        doc.trailer.set("Root", Object::Reference(catalog_id));

        let mut out_bytes = Vec::new();
        doc.save_to(&mut out_bytes).unwrap();

        let options = PdfDedupStreamsOptions::default();
        let deduped_bytes = pdf_dedup_streams_bytes(&out_bytes, &options).unwrap();

        let deduped_doc = Document::load_mem(&deduped_bytes).unwrap();

        // Verify references were actually rewritten correctly
        let page1 = deduped_doc.get_object(page1_id).unwrap().as_dict().unwrap();
        let res1 = page1.get(b"Resources").unwrap().as_dict().unwrap();
        let xo1 = res1.get(b"XObject").unwrap().as_dict().unwrap();
        let im1_ref = xo1.get(b"Im1").unwrap().as_reference().unwrap();

        let page2 = deduped_doc.get_object(page2_id).unwrap().as_dict().unwrap();
        let res2 = page2.get(b"Resources").unwrap().as_dict().unwrap();
        let xo2 = res2.get(b"XObject").unwrap().as_dict().unwrap();
        let im2_ref = xo2.get(b"Im2").unwrap().as_reference().unwrap();

        assert_eq!(
            im1_ref, im2_ref,
            "References should point to the same stream"
        );

        // Wait, saving generates a cross-reference stream which counts as a Stream Object!
        // XRef streams are added when saving. So we might see 1 original stream + 1 XRef stream = 2 streams.
        // Let's filter out XRef streams.
        let mut _data_streams = 0;
        for (_, obj) in deduped_doc.objects.iter() {
            if let Object::Stream(s) = obj {
                let is_xref = s
                    .dict
                    .get(b"Type")
                    .and_then(|t| t.as_name())
                    .map_or(false, |name| name == b"XRef");
                if !is_xref {
                    _data_streams += 1;
                }
            }
        }

        // Before dedup there were 2 data streams, now there should be 1 data stream
        assert_eq!(_data_streams, 1, "Duplicate streams should be merged");
    }
}
