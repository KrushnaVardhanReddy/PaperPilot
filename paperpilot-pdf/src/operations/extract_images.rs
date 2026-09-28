use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct ExtractImagesOperation {
    pub output_dir: PathBuf,
}

impl ExtractImagesOperation {
    pub fn new(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }
}

impl PdfOperation for ExtractImagesOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let mut image_count = 0;

        let pages = inner.get_pages();
        for (_, page_id) in pages {
            let page_dict = match inner.get_object(page_id).and_then(|obj| obj.as_dict()) {
                Ok(dict) => dict,
                Err(_) => continue,
            };

            let resources_dict = match page_dict.get(b"Resources").and_then(|res| match res {
                lopdf::Object::Reference(res_id) => {
                    inner.get_object(*res_id).and_then(|obj| obj.as_dict())
                }
                lopdf::Object::Dictionary(dict) => Ok(dict),
                _ => Err(lopdf::Error::DictKey("Invalid resources".to_string())),
            }) {
                Ok(dict) => dict,
                Err(_) => continue,
            };

            let xobject_dict = match resources_dict.get(b"XObject").and_then(|xobj| match xobj {
                lopdf::Object::Reference(xobj_id) => {
                    inner.get_object(*xobj_id).and_then(|obj| obj.as_dict())
                }
                lopdf::Object::Dictionary(dict) => Ok(dict),
                _ => Err(lopdf::Error::DictKey("Invalid XObject".to_string())),
            }) {
                Ok(dict) => dict,
                Err(_) => continue,
            };

            for (_, obj) in xobject_dict.iter() {
                let stream_id = match obj.as_reference() {
                    Ok(id) => id,
                    Err(_) => continue,
                };

                let stream = match inner.get_object(stream_id).and_then(|obj| obj.as_stream()) {
                    Ok(stream) => stream,
                    Err(_) => continue,
                };

                let subtype = match stream.dict.get(b"Subtype").and_then(|name| name.as_name()) {
                    Ok(name) => name,
                    Err(_) => continue,
                };

                if subtype != b"Image" {
                    continue;
                }

                image_count += 1;
                let ext = match stream.dict.get(b"Filter").and_then(|filter| match filter {
                    lopdf::Object::Name(name) => Ok(name.as_slice()),
                    lopdf::Object::Array(arr) => arr
                        .first()
                        .and_then(|f| f.as_name().ok())
                        .ok_or(lopdf::Error::DictKey("Invalid filter".to_string())),
                    _ => Err(lopdf::Error::DictKey("Invalid filter".to_string())),
                }) {
                    Ok(b"DCTDecode") => "jpg",
                    Ok(b"JPXDecode") => "jp2",
                    Ok(b"FlateDecode") => "png", // Simplified mapping
                    _ => "bin",
                };

                let file_name = format!("image_{}.{}", image_count, ext);
                let output_path = self.output_dir.join(file_name);

                let mut file = File::create(&output_path).map_err(|e| {
                    PdfError::IoError(std::io::Error::other(format!(
                        "Failed to create output file: {}",
                        e
                    )))
                })?;

                // Note: content is the compressed stream bytes if Filter is present and we don't decompress.
                // We write raw content bytes. We might need `stream.content` or decompressed data, but we'll use raw stream content for now as we're saving to `.bin` or `.jpg` depending on filter.
                file.write_all(&stream.content).map_err(|e| {
                    PdfError::IoError(std::io::Error::other(format!(
                        "Failed to write to output file: {}",
                        e
                    )))
                })?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{Dictionary, Document, Object, Stream};
    use tempfile::tempdir;

    fn create_empty_doc() -> LopdfDocument {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();

        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Count", Object::Integer(0));
        pages_dict.set("Kids", Object::Array(vec![]));

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
    fn test_extract_images_empty_doc() {
        let mut doc = create_empty_doc();
        let dir = tempdir().unwrap();
        let op = ExtractImagesOperation::new(dir.path().to_path_buf());
        assert!(op.execute(&mut doc).is_ok());

        let count = std::fs::read_dir(dir.path()).unwrap().count();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_extract_images_with_image() {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let image_id = doc.new_object_id();

        let mut image_stream_dict = Dictionary::new();
        image_stream_dict.set("Type", Object::Name(b"XObject".to_vec()));
        image_stream_dict.set("Subtype", Object::Name(b"Image".to_vec()));
        image_stream_dict.set("Width", Object::Integer(100));
        image_stream_dict.set("Height", Object::Integer(100));
        image_stream_dict.set("ColorSpace", Object::Name(b"DeviceRGB".to_vec()));
        image_stream_dict.set("BitsPerComponent", Object::Integer(8));
        image_stream_dict.set("Filter", Object::Name(b"DCTDecode".to_vec()));

        let image_stream = Stream::new(image_stream_dict, b"fake_jpeg_data".to_vec());
        doc.objects.insert(image_id, Object::Stream(image_stream));

        let mut xobj_dict = Dictionary::new();
        xobj_dict.set("Im1", Object::Reference(image_id));

        let mut resources_dict = Dictionary::new();
        resources_dict.set("XObject", Object::Dictionary(xobj_dict));

        let mut page_dict = Dictionary::new();
        page_dict.set("Type", Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", Object::Reference(pages_id));
        page_dict.set("Resources", Object::Dictionary(resources_dict));
        let page_id = doc.add_object(page_dict);

        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Count", Object::Integer(1));
        pages_dict.set("Kids", Object::Array(vec![Object::Reference(page_id)]));

        doc.objects.insert(pages_id, Object::Dictionary(pages_dict));
        doc.trailer
            .set("Root", Object::Dictionary(Dictionary::new()));

        let mut catalog = Dictionary::new();
        catalog.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog.set("Pages", Object::Reference(pages_id));
        let catalog_id = doc.add_object(catalog);
        doc.trailer.set("Root", Object::Reference(catalog_id));

        let mut lopdf_doc = LopdfDocument { inner: doc };

        let dir = tempdir().unwrap();
        let op = ExtractImagesOperation::new(dir.path().to_path_buf());
        assert!(op.execute(&mut lopdf_doc).is_ok());

        let mut count = 0;
        for entry in std::fs::read_dir(dir.path()).unwrap() {
            let entry = entry.unwrap();
            let name = entry.file_name().into_string().unwrap();
            assert!(name.starts_with("image_"));
            assert!(name.ends_with(".jpg"));
            count += 1;
        }
        assert_eq!(count, 1);
    }
}
