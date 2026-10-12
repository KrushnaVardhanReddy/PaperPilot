#![allow(clippy::collapsible_if)]
#![allow(clippy::redundant_closure)]
use crate::document::LopdfDocument;
use image;
use lopdf::dictionary;
use paperpilot_core::error::OperationResult;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use std::path::PathBuf;

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

fn find_model(file_name: &str) -> Option<PathBuf> {
    // 1. Explicit environment variable
    if let Ok(dir) = std::env::var("PAPERPILOT_OCR_MODELS_DIR") {
        let p = PathBuf::from(dir).join(file_name);
        if p.exists() {
            return Some(p);
        }
    }
    // 2. Local workspace models directory
    let local = PathBuf::from("models").join("ocr").join(file_name);
    if local.exists() {
        return Some(local);
    }
    // 3. User home / cache directory
    if let Ok(home) = std::env::var("HOME") {
        let user_path = PathBuf::from(&home)
            .join(".local")
            .join("share")
            .join("paperpilot")
            .join("models")
            .join("ocr")
            .join(file_name);
        if user_path.exists() {
            return Some(user_path);
        }
    }
    None
}

impl PdfOperation for OcrOperationImpl {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let mut executed_ocr = false;
        // Collect words with their vertical coordinate for later sorting and line-breaking
        let mut words_collected: Vec<(f32, f32, f32, String)> = Vec::new(); // (pdf_y, pdf_x, font_size, text)

        if let Some(lopdf_doc) = document.as_any_mut().downcast_mut::<LopdfDocument>() {
            let det_path = find_model("det.rten");
            let rec_path = find_model("rec.rten");
            let dict_path = find_model("ppocr_keys_v1.txt").or_else(|| find_model("dict.txt"));

            let mut ocr_engine = None;
            if let (Some(det), Some(rec), Some(dict)) = (det_path, rec_path, dict_path) {
                let config = rusto::config::InitializeConfig::ppv4(det, rec, dict);
                if let Ok(engine) = rusto::RustO::initialize(config) {
                    ocr_engine = Some(engine);
                } else {
                    eprintln!("Warning: Failed to initialize rusto-rs OCR engine.");
                }
            } else {
                eprintln!("Warning: OCR models not found. Falling back to embedded text extraction.");
            }

            if let Some(mut engine) = ocr_engine {
                let inner = &mut lopdf_doc.inner;

                // Add Helvetica font for text injection if needed
                let font_id = inner.add_object(lopdf::dictionary!(
                    "Type" => "Font",
                    "Subtype" => "Type1",
                    "BaseFont" => "Helvetica",
                ));

                let pages = inner.get_pages();
                for (_page_num, page_id) in pages {
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

                    let xobject_dict =
                        match resources_dict.get(b"XObject").and_then(|xobj| match xobj {
                            lopdf::Object::Reference(xobj_id) => {
                                inner.get_object(*xobj_id).and_then(|obj| obj.as_dict())
                            }
                            lopdf::Object::Dictionary(dict) => Ok(dict),
                            _ => Err(lopdf::Error::DictKey("Invalid XObject".to_string())),
                        }) {
                            Ok(dict) => dict,
                            Err(_) => continue,
                        };

                    let mut sorted_xobjects: Vec<(&Vec<u8>, &lopdf::Object)> = xobject_dict.iter().collect();
                    sorted_xobjects.sort_by(|a, b| {
                        let name_a = String::from_utf8_lossy(a.0);
                        let name_b = String::from_utf8_lossy(b.0);
                        // Extract trailing numbers for natural sorting (Image1, Image2, ... Image10)
                        let num_a = name_a.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap_or(0);
                        let num_b = name_b.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap_or(0);
                        num_a.cmp(&num_b)
                    });
                    let mut page_ocr_text = Vec::new();

                    for (_, obj) in sorted_xobjects {
                        let stream_id = match obj.as_reference() {
                            Ok(id) => id,
                            Err(_) => continue,
                        };

                        let stream =
                            match inner.get_object(stream_id).and_then(|obj| obj.as_stream()) {
                                Ok(stream) => stream,
                                Err(_) => continue,
                            };

                        let subtype =
                            match stream.dict.get(b"Subtype").and_then(|name| name.as_name()) {
                                Ok(name) => name,
                                Err(_) => continue,
                            };

                        if subtype != b"Image" {
                            continue;
                        }

                        if let Ok(dyn_img) = image::load_from_memory(&stream.content) {
                            let (orig_w, orig_h) = (dyn_img.width(), dyn_img.height());
                            if orig_w == 0 || orig_h < 15 {
                                continue;
                            }

                            // Quick luminance check to skip pure white/blank margin strips (e.g. Image1, Image26)
                            let luma = dyn_img.to_luma8();
                            let pixels = luma.as_raw();
                            let dark_count = pixels.iter().filter(|&&p| p < 200).count();
                            // If fewer than 0.2% of pixels are non-white, there is no text in this strip
                            if dark_count < (pixels.len() / 500) {
                                continue;
                            }

                            let resized = if orig_w > 1200 {
                                let new_h = (orig_h as f32 * 1200.0 / orig_w as f32).round() as u32;
                                dyn_img.resize_exact(1200, new_h.max(1), image::imageops::FilterType::Triangle)
                            } else {
                                dyn_img
                            };

                            let run_options = rusto::OcrRunOptions {
                                output: rusto::rusto_ocr::OutputGranularity::Words,
                                ..Default::default()
                            };

                            if let Ok(rusto::rusto_ocr::DetectTextResult::Structured(results)) = engine.detect_text(&rusto::rusto_ocr::ImageSource::Bytes(stream.content.clone()), &run_options) {
                                if !results.is_empty() {
                                    page_ocr_text.push((resized.width(), resized.height(), results.clone()));
                                    executed_ocr = true;
                                }
                            }
                        }
                    }

                    // If OCR recognized text on this page, inject transparent text layer across page
                    if !page_ocr_text.is_empty() {
                        // ExtGState with opacity 0.001 so text is selectable by cursor across page without covering image
                        let gs_id = inner.add_object(lopdf::dictionary!(
                            "Type" => "ExtGState",
                            "ca" => 0.001,
                            "CA" => 0.001,
                        ));

                        let mut text_ops = String::from("q\n/GS_OCR gs\n0 0 0 rg\nBT\n");

                        // Extract page dimensions to map coordinates
                        let mut page_w = 595.0; // A4 default
                        let mut page_h = 842.0;

                        if let Ok(page_dict_ref) = inner.get_object(page_id) {
                            if let Ok(dict) = page_dict_ref.as_dict() {
                                if let Ok(media_box) = dict.get(b"MediaBox") {
                                    if let Ok(arr) = media_box.as_array() {
                                        if arr.len() == 4 {
                                            if let (Ok(x0), Ok(y0), Ok(x1), Ok(y1)) = (
                                                arr[0].as_f32(),
                                                arr[1].as_f32(),
                                                arr[2].as_f32(),
                                                arr[3].as_f32(),
                                            ) {
                                                page_w = x1 - x0;
                                                page_h = y1 - y0;
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Assuming full page scan (x0=0, y0=0, w=page_w, h=page_h)
                        let strip_x0 = 0.0;
                        let strip_y0 = 0.0;
                        let strip_w = page_w;
                        let strip_h = page_h;

                        for (img_w, img_h, words) in &page_ocr_text {
                            let img_w = *img_w as f32;
                            let img_h = *img_h as f32;

                            for word in words {
                                let line = word.text.trim();
                                if line.is_empty() {
                                    continue;
                                }

                                let box_x = word.frame.left;
                                let box_y = word.frame.top;
                                let box_w = word.frame.width;
                                let box_h = word.frame.height;

                                let norm_x = box_x / img_w;
                                let norm_y = box_y / img_h;
                                let _norm_w = box_w / img_w;
                                let norm_h = box_h / img_h;

                                let pdf_x = strip_x0 + (norm_x * strip_w);
                                let pdf_y = strip_y0 + (strip_h - ((norm_y + norm_h) * strip_h));
                                let font_size = (norm_h * strip_h).max(6.0);

                                words_collected.push((pdf_y, pdf_x, font_size, line.to_string()));

                                let escaped = line
                                    .replace('\\', "\\\\")
                                    .replace('(', "\\(")
                                    .replace(')', "\\)");

                                text_ops.push_str(&format!("/F1 {:.2} Tf\n", font_size));
                                text_ops.push_str(&format!("1 0 0 1 {:.2} {:.2} Tm\n({}) Tj\n", pdf_x, pdf_y, escaped));
                            }
                        }
                        text_ops.push_str("ET\nQ\n");

                        let content_stream = lopdf::Stream::new(
                            lopdf::Dictionary::new(),
                            text_ops.into_bytes(),
                        );
                        let stream_obj_id = inner.add_object(content_stream);

                        if let Ok(lopdf::Object::Dictionary(page_d)) = inner.get_object_mut(page_id) {
                            // Ensure font & ExtGState reference in Resources
                            if let Ok(res_obj) = page_d.get_mut(b"Resources") {
                                if let Ok(res_dict) = res_obj.as_dict_mut() {
                                    let mut fonts = res_dict
                                        .get(b"Font")
                                        .and_then(|f| f.as_dict())
                                        .cloned()
                                        .unwrap_or_else(|_| lopdf::Dictionary::new());
                                    fonts.set("F1", lopdf::Object::Reference(font_id));
                                    res_dict.set("Font", lopdf::Object::Dictionary(fonts));

                                    let mut ext_gstates = res_dict
                                        .get(b"ExtGState")
                                        .and_then(|e| e.as_dict())
                                        .cloned()
                                        .unwrap_or_else(|_| lopdf::Dictionary::new());
                                    ext_gstates.set("GS_OCR", lopdf::Object::Reference(gs_id));
                                    res_dict.set("ExtGState", lopdf::Object::Dictionary(ext_gstates));
                                }
                            }

                            // Append stream to Contents
                            if let Ok(contents) = page_d.get_mut(b"Contents") {
                                match contents {
                                    lopdf::Object::Reference(ref_id) => {
                                        let old_ref = *ref_id;
                                        *contents = lopdf::Object::Array(vec![
                                            lopdf::Object::Reference(old_ref),
                                            lopdf::Object::Reference(stream_obj_id),
                                        ]);
                                    }
                                    lopdf::Object::Array(arr) => {
                                        arr.push(lopdf::Object::Reference(stream_obj_id));
                                    }
                                    _ => {
                                        *contents = lopdf::Object::Reference(stream_obj_id);
                                    }
                                }
                            } else {
                                page_d.set("Contents", lopdf::Object::Reference(stream_obj_id));
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
            if path.extension().and_then(|ext| ext.to_str()).map(|s| s.to_lowercase()) == Some("txt".to_string()) {
                // Sort words from top to bottom (descending y) and left to right (ascending x)
                words_collected.sort_by(|a, b| {
                    let y_cmp = b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal);
                    if y_cmp == std::cmp::Ordering::Equal {
                        a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)
                    } else {
                        y_cmp
                    }
                });

                let mut lines = Vec::new();
                let mut current_line = Vec::new();
                let mut current_y = None;

                for (y, _x, font_size, text) in words_collected {
                    if let Some(cy) = current_y {
                        // Delta y < 0.5 * font_size
                        #[allow(clippy::unnecessary_cast)]
                        if (cy as f32 - y as f32).abs() < 0.5 * font_size {
                            current_line.push(text);
                        } else {
                            lines.push(current_line.join(" "));
                            current_line = vec![text];
                            current_y = Some(y);
                        }
                    } else {
                        current_line.push(text);
                        current_y = Some(y);
                    }
                }
                if !current_line.is_empty() {
                    lines.push(current_line.join(" "));
                }

                let joined = lines.join("\n\n");
                std::fs::write(path, joined).map_err(|e| paperpilot_core::error::PdfError::IoError(e))?;
            } else {
                document.save(path)?;
            }
        }

        Ok(())
    }
}

pub type OcrOperation = OcrOperationImpl;
#[allow(non_upper_case_globals)]
pub const OcrOperation: OcrOperationImpl = OcrOperationImpl {
    output_path: None,
    language: None,
};

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

    #[test]
    fn test_e2e_full_ocr_generation() {
        let input_path = std::path::PathBuf::from("/home/krushna/Downloads/I-140 approval notice.pdf");
        if !input_path.exists() {
            println!("Input file does not exist: {:?}", input_path);
            return;
        }
        let out_path = std::path::PathBuf::from("/home/krushna/Downloads/I-140_approval_notice_ocr.pdf");
        let mut doc = LopdfDocument::load(&input_path).unwrap();
        let mut op = OcrOperationImpl::new();
        op.output_path = Some(out_path.clone());
        let res = op.execute(&mut doc);
        assert!(res.is_ok(), "OCR operation failed: {:?}", res);
        assert!(out_path.exists(), "Output PDF was not created!");
        println!(">>> Successfully generated: {:?}", out_path);
    }
}
