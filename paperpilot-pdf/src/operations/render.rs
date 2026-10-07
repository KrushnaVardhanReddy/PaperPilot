use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use std::path::PathBuf;
use std::sync::Mutex;
use tempfile::NamedTempFile;

pub struct RenderOperation {
    pub page: Option<u32>,
    pub dpi: f32,
    pub output_path: Option<PathBuf>,
    pub rendered_images: Mutex<Vec<Vec<u8>>>,
}

impl RenderOperation {
    pub fn new() -> Self {
        Self {
            page: None,
            dpi: 150.0,
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

impl PdfOperation for RenderOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let doc = document
            .as_any_mut()
            .downcast_mut::<crate::document::LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Expected LopdfDocument".to_string()))?;

        let mut temp_file = NamedTempFile::new().map_err(PdfError::IoError)?;
        doc.inner
            .save_to(&mut temp_file)
            .map_err(|e| PdfError::ParseError(e.to_string()))?;

        let pdf_bytes = std::fs::read(temp_file.path()).map_err(PdfError::IoError)?;
        let pdf = hayro_syntax::Pdf::new(pdf_bytes)
            .map_err(|e| PdfError::ParseError(format!("{:?}", e)))?;

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

        let mut out_images = self
            .rendered_images
            .lock()
            .map_err(|_| PdfError::Other("Mutex lock poisoned".to_string()))?;

        for i in start_page..end_page {
            let page = pages
                .get(i)
                .ok_or_else(|| PdfError::ParseError("Failed to get page".to_string()))?;
            let cache = hayro::RenderCache::new();
            let render_settings = hayro::RenderSettings::default();
            // Setting scale based on DPI (72 DPI is scale 1.0)
            let mut pixmap_settings = hayro::PixmapSettings::default();
            let scale_factor = self.dpi / 72.0;
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
            let data = bytemuck::cast_slice(pixmap.data());

            let mut skia_pixmap = tiny_skia::Pixmap::new(width, height)
                .ok_or_else(|| PdfError::ParseError("Failed to allocate Pixmap".to_string()))?;
            skia_pixmap.data_mut().copy_from_slice(data);

            let png_bytes = skia_pixmap
                .encode_png()
                .map_err(|e| PdfError::ParseError(e.to_string()))?;

            if let Some(out_path) = &self.output_path {
                let current_out_path = if self.page.is_none() && total_pages > 1 {
                    let mut path = out_path.clone();
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("png");
                        path.set_file_name(format!("{}_{}.{}", stem, i + 1, ext));
                    }
                    path
                } else {
                    out_path.clone()
                };
                std::fs::write(&current_out_path, &png_bytes).map_err(PdfError::IoError)?;
            }

            out_images.push(png_bytes);
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
}
