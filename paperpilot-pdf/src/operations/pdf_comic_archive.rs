use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Write};
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

pub fn pdf_comic_archive_bytes(
    input_bytes: &[u8],
    options: &PdfComicArchiveOptions,
) -> Result<Vec<u8>, PdfError> {
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

    let mut zip_buffer = Cursor::new(Vec::new());
    {
        let mut zip = zip::ZipWriter::new(&mut zip_buffer);
        let zip_options = FileOptions::<()>::default()
            .compression_method(CompressionMethod::Stored)
            .unix_permissions(0o755);

        for (i, png_bytes) in rendered_images.iter().enumerate() {
            let filename = format!("page_{:03}.png", i + 1);
            zip.start_file(&filename, zip_options.clone())
                .map_err(|e| PdfError::Other(e.to_string()))?;
            zip.write_all(png_bytes).map_err(PdfError::IoError)?;
        }

        zip.finish().map_err(|e| PdfError::Other(e.to_string()))?;
    }

    Ok(zip_buffer.into_inner())
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
}
