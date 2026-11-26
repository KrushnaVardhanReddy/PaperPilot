use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Cursor, Write};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageOutputFormat {
    Png,
    Jpeg,
    Webp,
}

impl ImageOutputFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_ascii_lowercase().as_str() {
            "png" => Some(ImageOutputFormat::Png),
            "jpg" | "jpeg" => Some(ImageOutputFormat::Jpeg),
            "webp" => Some(ImageOutputFormat::Webp),
            _ => None,
        }
    }

    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            ImageOutputFormat::Png => "png",
            ImageOutputFormat::Jpeg => "jpg",
            ImageOutputFormat::Webp => "webp",
        }
    }
}

impl std::str::FromStr for ImageOutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "png" => Ok(ImageOutputFormat::Png),
            "jpg" | "jpeg" => Ok(ImageOutputFormat::Jpeg),
            "webp" => Ok(ImageOutputFormat::Webp),
            other => Err(format!("Unsupported image format: '{}'. Supported: png, jpeg, webp", other)),
        }
    }
}

pub struct RenderOperation {
    pub page: Option<u32>,
    pub dpi: f32,
    pub format: Option<ImageOutputFormat>,
    pub output_path: Option<PathBuf>,
    pub rendered_images: Mutex<Vec<Vec<u8>>>,
}

impl RenderOperation {
    pub fn new() -> Self {
        Self {
            page: None,
            dpi: 150.0,
            format: None,
            output_path: None,
            rendered_images: Mutex::new(Vec::new()),
        }
    }
}

impl Default for RenderOperation {
    fn default() -> Self {
        Self::new()
    }
}

fn render_single_page(
    page: &hayro_syntax::page::Page<'_>,
    dpi: f32,
    effective_format: ImageOutputFormat,
) -> OperationResult<Vec<u8>> {
    let cache = hayro::RenderCache::new();
    let render_settings = hayro::RenderSettings::default();
    // Setting scale based on DPI (72 DPI is scale 1.0)
    let mut pixmap_settings = hayro::PixmapSettings::default();
    let scale_factor = dpi / 72.0;
    pixmap_settings.x_scale = scale_factor;
    pixmap_settings.y_scale = scale_factor;

    let pixmap = hayro::render(
        page,
        &cache,
        &Default::default(),
        &render_settings,
        &pixmap_settings,
    );
    let width = pixmap.width() as u32;
    let height = pixmap.height() as u32;
    let data: &[u8] = bytemuck::cast_slice(pixmap.data());

    match effective_format {
        ImageOutputFormat::Png => {
            let mut skia_pixmap = tiny_skia::Pixmap::new(width, height)
                .ok_or_else(|| PdfError::ParseError("Failed to allocate Pixmap".to_string()))?;
            skia_pixmap.data_mut().copy_from_slice(data);
            skia_pixmap
                .encode_png()
                .map_err(|e| PdfError::ParseError(e.to_string()))
        }
        ImageOutputFormat::Jpeg => {
            let rgba_img = image::RgbaImage::from_raw(width, height, data.to_vec())
                .ok_or_else(|| PdfError::ParseError("Failed to construct RgbaImage".to_string()))?;
            let mut rgb_img = image::RgbImage::new(width, height);
            for (x, y, pixel) in rgba_img.enumerate_pixels() {
                let alpha = pixel[3] as f32 / 255.0;
                let r = ((pixel[0] as f32 * alpha) + (255.0 * (1.0 - alpha))).round() as u8;
                let g = ((pixel[1] as f32 * alpha) + (255.0 * (1.0 - alpha))).round() as u8;
                let b = ((pixel[2] as f32 * alpha) + (255.0 * (1.0 - alpha))).round() as u8;
                rgb_img.put_pixel(x, y, image::Rgb([r, g, b]));
            }

            let mut bytes = Vec::new();
            let mut cursor = Cursor::new(&mut bytes);
            rgb_img
                .write_to(&mut cursor, image::ImageFormat::Jpeg)
                .map_err(|e| PdfError::ParseError(format!("Failed to encode JPEG: {:?}", e)))?;
            Ok(bytes)
        }
        ImageOutputFormat::Webp => {
            let rgba_img = image::RgbaImage::from_raw(width, height, data.to_vec())
                .ok_or_else(|| PdfError::ParseError("Failed to construct RgbaImage".to_string()))?;
            let mut bytes = Vec::new();
            let mut cursor = Cursor::new(&mut bytes);
            rgba_img
                .write_to(&mut cursor, image::ImageFormat::WebP)
                .map_err(|e| PdfError::ParseError(format!("Failed to encode WebP: {:?}", e)))?;
            Ok(bytes)
        }
    }
}

impl PdfOperation for RenderOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let doc = document
            .as_any_mut()
            .downcast_mut::<crate::document::LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Expected LopdfDocument".to_string()))?;

        let lopdf_page_count = doc.inner.get_pages().len();
        let zero_copy_pdf = doc
            .shared_source()
            .filter(|src| !src.is_empty())
            .and_then(|src| hayro_syntax::Pdf::new(Arc::clone(src)).ok())
            .filter(|candidate| candidate.pages().len() == lopdf_page_count);

        let pdf = match zero_copy_pdf {
            Some(pdf) => pdf,
            None => {
                let mut pdf_bytes = Vec::new();
                doc.inner
                    .save_to(&mut pdf_bytes)
                    .map_err(|e| PdfError::ParseError(e.to_string()))?;
                hayro_syntax::Pdf::new(pdf_bytes)
                    .map_err(|e| PdfError::ParseError(format!("{:?}", e)))?
            }
        };

        let pages = pdf.pages();
        let total_pages = pages.len();

        if total_pages == 0 {
            return Ok(());
        }

        let start_page = if let Some(p) = self.page {
            if p == 0 || p as usize > total_pages {
                return Err(PdfError::ParseError("Page out of bounds".to_string()));
            }
            p as usize - 1
        } else {
            0
        };

        let end_page = if self.page.is_some() {
            start_page + 1
        } else {
            total_pages
        };

        let effective_format = self
            .format
            .or_else(|| self.output_path.as_deref().and_then(ImageOutputFormat::from_path))
            .unwrap_or(ImageOutputFormat::Png);

        let rendered_pages: Vec<Vec<u8>> = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                use rayon::prelude::*;
                (start_page..end_page)
                    .into_par_iter()
                    .map(|i| {
                        let page = pages
                            .get(i)
                            .ok_or_else(|| PdfError::ParseError("Failed to get page".to_string()))?;
                        render_single_page(page, self.dpi, effective_format)
                    })
                    .collect::<OperationResult<Vec<Vec<u8>>>>()?
            }
            #[cfg(target_arch = "wasm32")]
            {
                (start_page..end_page)
                    .map(|i| {
                        let page = pages
                            .get(i)
                            .ok_or_else(|| PdfError::ParseError("Failed to get page".to_string()))?;
                        render_single_page(page, self.dpi, effective_format)
                    })
                    .collect::<OperationResult<Vec<Vec<u8>>>>()?
            }
        };

        let mut out_images = self
            .rendered_images
            .lock()
            .map_err(|_| PdfError::Other("Mutex lock poisoned".to_string()))?;

        for (i, img_bytes) in (start_page..end_page).zip(rendered_pages) {
            if let Some(out_path) = &self.output_path {
                let current_out_path = if self.page.is_none() && total_pages > 1 {
                    let mut path = out_path.clone();
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        let ext = path
                            .extension()
                            .and_then(|s| s.to_str())
                            .unwrap_or(effective_format.file_extension());
                        path.set_file_name(format!("{}_{}.{}", stem, i + 1, ext));
                    }
                    path
                } else {
                    out_path.clone()
                };

                let mut file = File::create(&current_out_path).map_err(PdfError::IoError)?;
                file.write_all(&img_bytes).map_err(PdfError::IoError)?;
                file.sync_all().map_err(PdfError::IoError)?;
            }

            out_images.push(img_bytes);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Document as LopdfInnerDocument;
    use lopdf::dictionary;
    use std::path::PathBuf;
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
    fn test_render_operation() {
        let temp_dir = tempdir().unwrap();
        let doc_path = temp_dir.path().join("doc.pdf");
        create_test_pdf(&doc_path, "Hello World");

        let mut doc = LopdfDocument::load(&doc_path).unwrap();
        let op = RenderOperation::new();

        let result = op.execute(&mut doc);
        assert!(result.is_ok());

        let images = op.rendered_images.lock().unwrap();
        assert_eq!(images.len(), 1);

        let png_bytes = &images[0];
        assert!(png_bytes.len() > 4);
        assert_eq!(&png_bytes[0..4], &[0x89, 0x50, 0x4E, 0x47]);
    }

    #[test]
    fn test_render_operation_with_flush() {
        let temp_dir = tempdir().unwrap();
        let doc_path = temp_dir.path().join("doc.pdf");
        create_test_pdf(&doc_path, "Flush Test");

        let mut doc = LopdfDocument::load(&doc_path).unwrap();
        let out_path = temp_dir.path().join("out.png");

        let mut op = RenderOperation::new();
        op.output_path = Some(out_path.clone());

        let result = op.execute(&mut doc);
        assert!(result.is_ok());

        // Read the file from disk to ensure it was properly flushed and written
        let file_meta = std::fs::metadata(&out_path).unwrap();
        assert!(file_meta.len() > 0);

        let disk_bytes = std::fs::read(&out_path).unwrap();
        assert!(disk_bytes.len() > 4);
        assert_eq!(&disk_bytes[0..4], &[0x89, 0x50, 0x4E, 0x47]);
    }

    #[test]
    fn test_render_operation_jpeg() {
        let temp_dir = tempdir().unwrap();
        let doc_path = temp_dir.path().join("doc.pdf");
        create_test_pdf(&doc_path, "JPEG Test");

        let mut doc = LopdfDocument::load(&doc_path).unwrap();
        let out_path = temp_dir.path().join("out.jpg");

        let mut op = RenderOperation::new();
        op.output_path = Some(out_path.clone());

        let result = op.execute(&mut doc);
        assert!(result.is_ok());

        let disk_bytes = std::fs::read(&out_path).unwrap();
        assert!(disk_bytes.len() > 2);
        // JPEG magic bytes: 0xFF, 0xD8
        assert_eq!(&disk_bytes[0..2], &[0xFF, 0xD8]);
    }

    #[test]
    fn test_render_operation_webp() {
        let temp_dir = tempdir().unwrap();
        let doc_path = temp_dir.path().join("doc.pdf");
        create_test_pdf(&doc_path, "WebP Test");

        let mut doc = LopdfDocument::load(&doc_path).unwrap();
        let out_path = temp_dir.path().join("out.webp");

        let mut op = RenderOperation::new();
        op.output_path = Some(out_path.clone());

        let result = op.execute(&mut doc);
        assert!(result.is_ok());

        let disk_bytes = std::fs::read(&out_path).unwrap();
        assert!(disk_bytes.len() > 12);
        // RIFF....WEBP
        assert_eq!(&disk_bytes[0..4], b"RIFF");
        assert_eq!(&disk_bytes[8..12], b"WEBP");
    }

    #[test]
    fn test_render_operation_explicit_format() {
        let temp_dir = tempdir().unwrap();
        let doc_path = temp_dir.path().join("doc.pdf");
        create_test_pdf(&doc_path, "Explicit Format Test");

        let mut doc = LopdfDocument::load(&doc_path).unwrap();
        let mut op = RenderOperation::new();
        op.format = Some(ImageOutputFormat::Jpeg);

        let result = op.execute(&mut doc);
        assert!(result.is_ok());

        let images = op.rendered_images.lock().unwrap();
        assert_eq!(images.len(), 1);
        let bytes = &images[0];
        assert_eq!(&bytes[0..2], &[0xFF, 0xD8]);
    }
}
