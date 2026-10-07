use crate::document::LopdfDocument;
use image::codecs::jpeg::JpegEncoder;
use image::{DynamicImage, ImageBuffer, ColorType};
use lopdf::Object;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct CompressOperation {
    pub quality: Option<String>,
}

impl CompressOperation {
    pub fn new(quality: Option<String>) -> Self {
        Self { quality }
    }
}

impl Default for CompressOperation {
    fn default() -> Self {
        Self::new(None)
    }
}

impl PdfOperation for CompressOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document is not a LopdfDocument".to_string())
            })?;

        let (max_dim, jpeg_quality) = match self.quality.as_deref() {
            Some("low") => (1024, 45),
            Some("medium") => (1600, 70),
            Some("high") => (2400, 85),
            Some(s) if s.parse::<u8>().is_ok() => (1600, s.parse::<u8>().unwrap()),
            _ => (1600, 70), // default to medium
        };

        // We collect the object IDs first to avoid mutable borrow issues
        let object_ids: Vec<lopdf::ObjectId> = lopdf_doc.inner.objects.keys().copied().collect();

        for object_id in object_ids {
            if let Ok(lopdf::Object::Stream(stream)) = lopdf_doc.inner.get_object_mut(object_id) {
                // Check if it's an image stream
                let is_image = stream.dict.get(b"Subtype")
                    .and_then(|obj| obj.as_name())
                    .map(|name| name == b"Image")
                    .unwrap_or(false);

                if !is_image {
                    continue;
                }

                // Get decompressed content
                let decompressed = match stream.decompressed_content() {
                    Ok(bytes) => bytes,
                    Err(_) => continue,
                };
                
                let original_len = stream.content.len();

                let width = match stream.dict.get(b"Width").and_then(|w| w.as_i64()) {
                    Ok(w) => w as u32,
                    Err(_) => continue,
                };
                
                let height = match stream.dict.get(b"Height").and_then(|h| h.as_i64()) {
                    Ok(h) => h as u32,
                    Err(_) => continue,
                };

                // We extract the color_space string into an owned vec so we don't hold the borrow
                let color_space = stream.dict.get(b"ColorSpace")
                    .and_then(|c| c.as_name())
                    .unwrap_or(b"DeviceRGB")
                    .to_vec();
                
                // Attempt to decode the image
                let mut dyn_img_opt = image::load_from_memory(&decompressed).ok();
                
                if dyn_img_opt.is_none() {
                    // Try to reconstruct raw bytes
                    if color_space == b"DeviceRGB" && decompressed.len() >= (width * height * 3) as usize {
                        if let Some(img_buf) = ImageBuffer::<image::Rgb<u8>, _>::from_raw(width, height, decompressed.clone()) {
                            dyn_img_opt = Some(DynamicImage::ImageRgb8(img_buf));
                        }
                    } else if color_space == b"DeviceGray" && decompressed.len() >= (width * height) as usize {
                        if let Some(img_buf) = ImageBuffer::<image::Luma<u8>, _>::from_raw(width, height, decompressed.clone()) {
                            dyn_img_opt = Some(DynamicImage::ImageLuma8(img_buf));
                        }
                    }
                }
                
                if let Some(mut dyn_img) = dyn_img_opt {
                    // Downsample if needed
                    let (w, h) = (dyn_img.width(), dyn_img.height());
                    if w > max_dim || h > max_dim {
                        dyn_img = dyn_img.resize(max_dim, max_dim, image::imageops::FilterType::Triangle);
                    }
                    
                    let mut jpeg_bytes = Vec::new();
                    let mut encoder = JpegEncoder::new_with_quality(&mut jpeg_bytes, jpeg_quality);
                    
                    // Encode to JPEG
                    if let Ok(_) = dyn_img.write_with_encoder(encoder) {
                        if jpeg_bytes.len() < original_len {
                            // Update the stream
                            stream.content = jpeg_bytes;
                            stream.dict.set("Filter", Object::Name(b"DCTDecode".to_vec()));
                            stream.dict.remove(b"DecodeParms");
                            stream.dict.set("Width", Object::Integer(dyn_img.width() as i64));
                            stream.dict.set("Height", Object::Integer(dyn_img.height() as i64));
                            stream.dict.set("Length", Object::Integer(stream.content.len() as i64));
                            if color_space != b"DeviceRGB" && color_space != b"DeviceGray" {
                                if dyn_img.color() == ColorType::L8 || dyn_img.color() == ColorType::La8 {
                                    stream.dict.set("ColorSpace", Object::Name(b"DeviceGray".to_vec()));
                                } else {
                                    stream.dict.set("ColorSpace", Object::Name(b"DeviceRGB".to_vec()));
                                }
                            }
                        }
                    }
                }
            }
        }

        lopdf_doc.inner.compress();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{Document, Stream, Dictionary};

    #[test]
    fn test_compress_operation_success() {
        let doc = Document::with_version("1.5");
        let mut lopdf_doc = LopdfDocument { inner: doc };
        let op = CompressOperation::new(None);

        let result = op.execute(&mut lopdf_doc);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_compress_operation_image_reduction() {
        let mut doc = Document::with_version("1.5");
        let image_id = doc.new_object_id();

        let mut image_stream_dict = Dictionary::new();
        image_stream_dict.set("Type", Object::Name(b"XObject".to_vec()));
        image_stream_dict.set("Subtype", Object::Name(b"Image".to_vec()));
        image_stream_dict.set("Width", Object::Integer(100));
        image_stream_dict.set("Height", Object::Integer(100));
        image_stream_dict.set("ColorSpace", Object::Name(b"DeviceRGB".to_vec()));
        image_stream_dict.set("BitsPerComponent", Object::Integer(8));
        
        let mut img = image::ImageBuffer::new(100, 100);
        for (_, _, pixel) in img.enumerate_pixels_mut() {
            *pixel = image::Rgb([255_u8, 0_u8, 0_u8]);
        }
        
        let mut raw_bytes = img.into_raw();
        // Artificially inflate the size to ensure JPEG compression will definitely be smaller
        let original_len = raw_bytes.len();
        
        let image_stream = Stream::new(image_stream_dict, raw_bytes);
        doc.objects.insert(image_id, Object::Stream(image_stream));
        
        let mut lopdf_doc = LopdfDocument { inner: doc };
        let op = CompressOperation::new(Some("low".to_string()));
        
        let result = op.execute(&mut lopdf_doc);
        assert!(result.is_ok());
        
        // Verify the stream was replaced with a smaller JPEG
        let obj = lopdf_doc.inner.get_object(image_id).unwrap();
        if let Object::Stream(s) = obj {
            assert!(s.content.len() < original_len);
            let filter = s.dict.get(b"Filter").unwrap().as_name().unwrap();
            assert_eq!(filter, b"DCTDecode");
        } else {
            panic!("Object is not a stream");
        }
    }

    #[test]
    fn test_compress_operation_wrong_document_type() {
        use std::any::Any;
        use std::path::Path;

        struct MockDocument;
        impl PdfDocument for MockDocument {
            fn page_count(&self) -> OperationResult<u32> {
                Ok(0)
            }
            fn save(&self, _path: &Path) -> OperationResult<()> {
                Ok(())
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }

        let mut mock_doc = MockDocument;
        let op = CompressOperation::new(None);

        let result = op.execute(&mut mock_doc);
        assert!(result.is_err());
        if let Err(PdfError::UnsupportedOperation(msg)) = result {
            assert_eq!(msg, "Document is not a LopdfDocument");
        } else {
            panic!("Expected UnsupportedOperation error");
        }
    }
}
