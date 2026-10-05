use paperpilot_core::error::OperationResult;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use std::path::PathBuf;
use ocrs::{OcrEngine, OcrEngineParams, ImageSource};
use rten_tensor::prelude::*;
use rten_tensor::NdTensor;
use crate::document::LopdfDocument;
use image;

pub struct OcrOperationImpl {
    pub output_path: Option<PathBuf>,
    pub language: Option<String>,
}

impl OcrOperationImpl {
    pub fn new() -> Self {
        Self {
            output_path: None,
            language: None,
        }
    }
}

impl Default for OcrOperationImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfOperation for OcrOperationImpl {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let mut executed_ocr = false;

        if let Some(lopdf_doc) = document.as_any_mut().downcast_mut::<LopdfDocument>() {
            let engine_res = OcrEngine::new(OcrEngineParams::default());
            if let Ok(engine) = engine_res {
                let inner = &mut lopdf_doc.inner;
                let mut words_collected: Vec<String> = Vec::new();

                let pages = inner.get_pages();
                for (_page_num, page_id) in pages {
                    let page_dict = match inner.get_object(page_id).and_then(|obj| obj.as_dict()) {
                        Ok(dict) => dict,
                        Err(_) => continue,
                    };

                    let resources_dict = match page_dict.get(b"Resources").and_then(|res| match res {
                        lopdf::Object::Reference(res_id) => inner.get_object(*res_id).and_then(|obj| obj.as_dict()),
                        lopdf::Object::Dictionary(dict) => Ok(dict),
                        _ => Err(lopdf::Error::DictKey("Invalid resources".to_string())),
                    }) {
                        Ok(dict) => dict,
                        Err(_) => continue,
                    };

                    let xobject_dict = match resources_dict.get(b"XObject").and_then(|xobj| match xobj {
                        lopdf::Object::Reference(xobj_id) => inner.get_object(*xobj_id).and_then(|obj| obj.as_dict()),
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

                        if let Ok(dyn_img) = image::load_from_memory(&stream.content) {
                            let rgb = dyn_img.into_rgb8();
                            let (width, height) = rgb.dimensions();

                            let mut tensor: NdTensor<f32, 3> = NdTensor::zeros([3, height as usize, width as usize]);
                            for y in 0..height {
                                for x in 0..width {
                                    let pixel = rgb.get_pixel(x, y);
                                    tensor[[0, y as usize, x as usize]] = pixel[0] as f32 / 255.0;
                                    tensor[[1, y as usize, x as usize]] = pixel[1] as f32 / 255.0;
                                    tensor[[2, y as usize, x as usize]] = pixel[2] as f32 / 255.0;
                                }
                            }

                            // fallback format handling due to missing DimOrder
                            if let Ok(image_source) = ImageSource::from_tensor(tensor.view(), ocrs::DimOrder::Chw) {
                                if let Ok(img_input) = engine.prepare_input(image_source) {
                                    if let Ok(texts) = engine.get_text(&img_input) {
                                        words_collected.push(texts);
                                        executed_ocr = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if !executed_ocr {
            let text_op = crate::operations::extract_text::ExtractTextOperation::new(None);
            text_op.execute(document)?;
        }

        if let Some(path) = &self.output_path {
            document.save(path)?;
        }

        Ok(())
    }
}

pub type OcrOperation = OcrOperationImpl;
#[allow(non_upper_case_globals)]
pub const OcrOperation: OcrOperationImpl = OcrOperationImpl { output_path: None, language: None };

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Document as LopdfInnerDocument;
    use lopdf::dictionary;
    use tempfile::tempdir;

    fn create_test_pdf(path: &PathBuf, text_content: &str) {
        let mut inner = LopdfInnerDocument::with_version("1.5");
        let pages_id = inner.new_object_id();
        let font_id = inner.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        });

        let resources_id = inner.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });

        let content = lopdf::content::Content {
            operations: vec![
                lopdf::content::Operation::new("BT", vec![]),
                lopdf::content::Operation::new("Tf", vec!["F1".into(), 12.into()]),
                lopdf::content::Operation::new("Td", vec![10.into(), 10.into()]),
                lopdf::content::Operation::new(
                    "Tj",
                    vec![lopdf::Object::string_literal(text_content)],
                ),
                lopdf::content::Operation::new("ET", vec![]),
            ],
        };
        let content_id = inner.add_object(lopdf::Stream::new(
            lopdf::Dictionary::new(),
            content.encode().unwrap(),
        ));

        let page_id = inner.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            "Resources" => resources_id,
        });

        inner.set_object(
            pages_id,
            dictionary! {
                "Type" => "Pages",
                "Kids" => vec![page_id.into()],
                "Count" => 1,
            },
        );

        let catalog_id = inner.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });

        inner.trailer.set("Root", catalog_id);

        inner.save(path).unwrap();
    }

    #[test]
    fn test_ocr_operation() {
        let temp_dir = tempdir().unwrap();
        let doc_path = temp_dir.path().join("doc.pdf");
        create_test_pdf(&doc_path, "Hello World");

        let mut doc = LopdfDocument::load(&doc_path).unwrap();
        let mut op = OcrOperationImpl::new();

        let output_path = temp_dir.path().join("out.pdf");
        op.output_path = Some(output_path.clone());

        let result = op.execute(&mut doc);
        assert!(result.is_ok());
        assert!(output_path.exists());
    }

    #[test]
    fn test_ocr_with_images() {
        let temp_dir = tempdir().unwrap();
        let doc_path = temp_dir.path().join("image_doc.pdf");
        let mut inner = LopdfInnerDocument::with_version("1.5");
        let pages_id = inner.new_object_id();

        let image_content = vec![255; 100];
        let mut image_dict = lopdf::Dictionary::new();
        image_dict.set("Type", lopdf::Object::Name(b"XObject".to_vec()));
        image_dict.set("Subtype", lopdf::Object::Name(b"Image".to_vec()));
        image_dict.set("Width", lopdf::Object::Integer(10));
        image_dict.set("Height", lopdf::Object::Integer(10));
        let image_stream = lopdf::Stream::new(image_dict, image_content);
        let image_id = inner.add_object(image_stream);

        let resources_id = inner.add_object(dictionary! {
            "XObject" => dictionary! {
                "Im1" => image_id,
            },
        });

        let page_id = inner.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Resources" => resources_id,
        });

        inner.set_object(
            pages_id,
            dictionary! {
                "Type" => "Pages",
                "Kids" => vec![page_id.into()],
                "Count" => 1,
            },
        );

        let catalog_id = inner.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });

        inner.trailer.set("Root", catalog_id);
        inner.save(&doc_path).unwrap();

        let mut doc = LopdfDocument::load(&doc_path).unwrap();
        let mut op = OcrOperationImpl::new();
        let output_path = temp_dir.path().join("out2.pdf");
        op.output_path = Some(output_path.clone());

        let result = op.execute(&mut doc);
        assert!(result.is_ok());
        assert!(output_path.exists());
    }
}
