use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs;
use std::io::{Cursor, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::CompressionMethod;
use crate::document::LopdfDocument;
use crate::operations::render::RenderOperation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfComicArchiveOptions {
    pub dpi: f32,
}

impl Default for PdfComicArchiveOptions {
    fn default() -> Self {
        Self { dpi: 150.0 }
    }
}

pub fn natural_cmp(a: &str, b: &str) -> Ordering {
    let mut a_chars = a.chars().peekable();
    let mut b_chars = b.chars().peekable();

    loop {
        match (a_chars.peek(), b_chars.peek()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(&ca), Some(&cb)) => {
                if ca.is_ascii_digit() && cb.is_ascii_digit() {
                    let mut a_num_str = String::new();
                    while let Some(&c) = a_chars.peek() {
                        if c.is_ascii_digit() {
                            a_num_str.push(c);
                            a_chars.next();
                        } else {
                            break;
                        }
                    }

                    let mut b_num_str = String::new();
                    while let Some(&c) = b_chars.peek() {
                        if c.is_ascii_digit() {
                            b_num_str.push(c);
                            b_chars.next();
                        } else {
                            break;
                        }
                    }

                    let a_num = a_num_str.parse::<u128>().ok();
                    let b_num = b_num_str.parse::<u128>().ok();

                    match (a_num, b_num) {
                        (Some(an), Some(bn)) => {
                            if an != bn {
                                return an.cmp(&bn);
                            }
                            if a_num_str.len() != b_num_str.len() {
                                return a_num_str.len().cmp(&b_num_str.len());
                            }
                        }
                        _ => {
                            let cmp = a_num_str.cmp(&b_num_str);
                            if cmp != Ordering::Equal {
                                return cmp;
                            }
                        }
                    }
                } else {
                    let ca_lower = ca.to_ascii_lowercase();
                    let cb_lower = cb.to_ascii_lowercase();
                    if ca_lower != cb_lower {
                        return ca_lower.cmp(&cb_lower);
                    }
                    if ca != cb {
                        return ca.cmp(&cb);
                    }
                    a_chars.next();
                    b_chars.next();
                }
            }
        }
    }
}

pub fn images_to_comic_archive_bytes(
    images: &[(String, Vec<u8>)],
) -> Result<Vec<u8>, PdfError> {
    if images.is_empty() {
        return Err(PdfError::ParseError("No images found to pack into comic archive".to_string()));
    }

    let mut sorted_images = images.to_vec();
    sorted_images.sort_by(|a, b| natural_cmp(&a.0, &b.0));

    let mut zip_buffer = Cursor::new(Vec::new());
    {
        let mut zip = zip::ZipWriter::new(&mut zip_buffer);
        let zip_options = FileOptions::<()>::default()
            .compression_method(CompressionMethod::Stored)
            .unix_permissions(0o755);

        for (filename, img_bytes) in sorted_images.iter() {
            zip.start_file(filename, zip_options.clone())
                .map_err(|e| PdfError::Other(e.to_string()))?;
            zip.write_all(img_bytes).map_err(PdfError::IoError)?;
        }

        zip.finish().map_err(|e| PdfError::Other(e.to_string()))?;
    }

    Ok(zip_buffer.into_inner())
}

pub fn is_image_path(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        matches!(
            ext.to_ascii_lowercase().as_str(),
            "png" | "jpg" | "jpeg" | "webp"
        )
    } else {
        false
    }
}

pub fn pdf_comic_archive_path(
    input_path: &Path,
    output_path: &Path,
    options: &PdfComicArchiveOptions,
) -> Result<(), PdfError> {
    if input_path.is_dir() {
        let mut image_files = Vec::new();
        let entries = fs::read_dir(input_path).map_err(PdfError::IoError)?;

        for entry in entries {
            let entry = entry.map_err(PdfError::IoError)?;
            let path = entry.path();
            if path.is_file() && is_image_path(&path) {
                let filename = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("image.png")
                    .to_string();
                let bytes = fs::read(&path).map_err(PdfError::IoError)?;
                image_files.push((filename, bytes));
            }
        }

        if image_files.is_empty() {
            return Err(PdfError::ParseError(format!(
                "No supported images (.png, .jpg, .jpeg, .webp) found in directory: {:?}",
                input_path
            )));
        }

        let cbz_bytes = images_to_comic_archive_bytes(&image_files)?;
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).map_err(PdfError::IoError)?;
        }
        fs::write(output_path, cbz_bytes).map_err(PdfError::IoError)?;
        return Ok(());
    }

    if is_image_path(input_path) {
        let parent_dir = input_path.parent();
        let mut image_files = Vec::new();

        if let Some(parent) = parent_dir {
            if let Ok(entries) = fs::read_dir(parent) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && is_image_path(&path) {
                        let filename = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("image.png")
                            .to_string();
                        if let Ok(bytes) = fs::read(&path) {
                            image_files.push((filename, bytes));
                        }
                    }
                }
            }
        }

        if image_files.is_empty() {
            let filename = input_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("image.png")
                .to_string();
            let bytes = fs::read(input_path).map_err(PdfError::IoError)?;
            image_files.push((filename, bytes));
        }

        let cbz_bytes = images_to_comic_archive_bytes(&image_files)?;
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).map_err(PdfError::IoError)?;
        }
        fs::write(output_path, cbz_bytes).map_err(PdfError::IoError)?;
        return Ok(());
    }

    let input_bytes = fs::read(input_path).map_err(PdfError::IoError)?;
    let cbz_bytes = pdf_comic_archive_bytes(&input_bytes, options)?;
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(PdfError::IoError)?;
    }
    fs::write(output_path, cbz_bytes).map_err(PdfError::IoError)?;
    Ok(())
}

pub fn pdf_comic_archive_bytes(
    input_bytes: &[u8],
    options: &PdfComicArchiveOptions,
) -> Result<Vec<u8>, PdfError> {
    if input_bytes.starts_with(b"%PDF") {
        let mut doc = LopdfDocument::from_bytes(input_bytes)
            .map_err(|e| PdfError::ParseError(e.to_string()))?;

        let render_op = RenderOperation {
            dpi: options.dpi,
            ..Default::default()
        };

        render_op.execute(&mut doc)?;

        let rendered_images = render_op.rendered_images.lock()
            .map_err(|_| PdfError::Other("Mutex lock poisoned".to_string()))?;

        if rendered_images.is_empty() {
            return Err(PdfError::ParseError("Document has no pages".to_string()));
        }

        let mut images = Vec::new();
        for (i, png_bytes) in rendered_images.iter().enumerate() {
            let filename = format!("page_{:03}.png", i + 1);
            images.push((filename, png_bytes.clone()));
        }

        images_to_comic_archive_bytes(&images)
    } else {
        // Attempt to parse as PDF, if fails return ParseError
        match LopdfDocument::from_bytes(input_bytes) {
            Ok(mut doc) => {
                let render_op = RenderOperation {
                    dpi: options.dpi,
                    ..Default::default()
                };

                render_op.execute(&mut doc)?;

                let rendered_images = render_op.rendered_images.lock()
                    .map_err(|_| PdfError::Other("Mutex lock poisoned".to_string()))?;

                if rendered_images.is_empty() {
                    return Err(PdfError::ParseError("Document has no pages".to_string()));
                }

                let mut images = Vec::new();
                for (i, png_bytes) in rendered_images.iter().enumerate() {
                    let filename = format!("page_{:03}.png", i + 1);
                    images.push((filename, png_bytes.clone()));
                }

                images_to_comic_archive_bytes(&images)
            }
            Err(e) => {
                // If it's a single loose image file bytes (PNG, JPEG, WEBP)
                let is_png = input_bytes.starts_with(b"\x89PNG\r\n\x1a\n");
                let is_jpg = input_bytes.starts_with(b"\xFF\xD8\xFF");
                let is_webp = input_bytes.len() > 12 && &input_bytes[0..4] == b"RIFF" && &input_bytes[8..12] == b"WEBP";

                if is_png || is_jpg || is_webp {
                    let ext = if is_png {
                        "png"
                    } else if is_jpg {
                        "jpg"
                    } else {
                        "webp"
                    };
                    let images = vec![(format!("page_001.{}", ext), input_bytes.to_vec())];
                    images_to_comic_archive_bytes(&images)
                } else {
                    Err(PdfError::ParseError(e.to_string()))
                }
            }
        }
    }
}

#[derive(Default)]
pub struct PdfComicArchiveOperation {
    options: PdfComicArchiveOptions,
}

impl PdfComicArchiveOperation {
    pub fn new(options: PdfComicArchiveOptions) -> Self {
        Self { options }
    }
}

impl PdfOperation for PdfComicArchiveOperation {
    fn execute(
        &self,
        document: &mut dyn PdfDocument,
    ) -> paperpilot_core::error::OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Expected LopdfDocument".to_string()))?;

        let bytes = if let Some(orig) = &lopdf_doc.original_bytes {
            orig.clone()
        } else {
            let mut buf = Vec::new();
            lopdf_doc
                .inner
                .save_to(&mut buf)
                .map_err(|e| PdfError::Other(e.to_string()))?;
            buf
        };

        let cbz_bytes = pdf_comic_archive_bytes(&bytes, &self.options)?;
        lopdf_doc.original_bytes = Some(cbz_bytes);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::Document as LopdfInnerDocument;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use lopdf::dictionary;

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
            "MediaBox" => vec![0.into(), 0.into(), 200.into(), 200.into()],
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
    fn test_pdf_comic_archive_empty_document() {
        let options = PdfComicArchiveOptions::default();
        let bytes = vec![];
        let result = pdf_comic_archive_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_pdf_comic_archive_valid_document() {
        let temp_dir = tempdir().unwrap();
        let doc_path = temp_dir.path().join("doc.pdf");
        create_test_pdf(&doc_path, "Comic Page");

        let bytes = std::fs::read(&doc_path).unwrap();
        let options = PdfComicArchiveOptions::default();
        let result = pdf_comic_archive_bytes(&bytes, &options);
        assert!(result.is_ok());

        let cbz_bytes = result.unwrap();
        assert!(cbz_bytes.len() > 4);
        assert_eq!(&cbz_bytes[0..4], &[0x50, 0x4B, 0x03, 0x04]);

        let cursor = std::io::Cursor::new(cbz_bytes);
        let mut archive = zip::ZipArchive::new(cursor).unwrap();
        assert_eq!(archive.len(), 1);

        let mut file = archive.by_index(0).unwrap();
        assert_eq!(file.name(), "page_001.png");

        let mut png_bytes = Vec::new();
        std::io::Read::read_to_end(&mut file, &mut png_bytes).unwrap();
        assert!(png_bytes.len() > 4);
        assert_eq!(&png_bytes[0..4], &[0x89, 0x50, 0x4E, 0x47]);
    }

    #[test]
    fn test_pdf_comic_archive_directory_and_natural_sort() {
        let temp_dir = tempdir().unwrap();
        let img1 = temp_dir.path().join("photo_scan_1.png");
        let img2 = temp_dir.path().join("photo_scan_2.png");
        let img10 = temp_dir.path().join("photo_scan_10.png");

        let fake_png = b"\x89PNG\r\n\x1a\nfakeimagecontent";
        fs::write(&img1, fake_png).unwrap();
        fs::write(&img2, fake_png).unwrap();
        fs::write(&img10, fake_png).unwrap();

        let out_cbz = temp_dir.path().join("out.cbz");
        let options = PdfComicArchiveOptions::default();

        let res = pdf_comic_archive_path(temp_dir.path(), &out_cbz, &options);
        assert!(res.is_ok());

        let cbz_bytes = fs::read(&out_cbz).unwrap();
        let cursor = std::io::Cursor::new(cbz_bytes);
        let mut archive = zip::ZipArchive::new(cursor).unwrap();
        assert_eq!(archive.len(), 3);

        assert_eq!(archive.by_index(0).unwrap().name(), "photo_scan_1.png");
        assert_eq!(archive.by_index(1).unwrap().name(), "photo_scan_2.png");
        assert_eq!(archive.by_index(2).unwrap().name(), "photo_scan_10.png");
    }

    #[test]
    fn test_images_to_comic_archive_bytes() {
        let fake_png = b"\x89PNG\r\n\x1a\nfakeimagecontent".to_vec();
        let fake_jpg = b"\xFF\xD8\xFFfakejpg".to_vec();

        let images = vec![
            ("img2.jpg".to_string(), fake_jpg),
            ("img1.png".to_string(), fake_png),
        ];

        let cbz_bytes = images_to_comic_archive_bytes(&images).unwrap();
        let cursor = std::io::Cursor::new(cbz_bytes);
        let mut archive = zip::ZipArchive::new(cursor).unwrap();
        assert_eq!(archive.len(), 2);
        assert_eq!(archive.by_index(0).unwrap().name(), "img1.png");
        assert_eq!(archive.by_index(1).unwrap().name(), "img2.jpg");
    }
}
