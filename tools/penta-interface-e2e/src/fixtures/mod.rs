use anyhow::Result;
use image::{ImageBuffer, RgbImage};
use lopdf::{
    content::{Content, Operation},
    dictionary, Document, Object, Stream,
};
use std::path::PathBuf;

pub struct FixtureManager {
    pub dir: PathBuf,
}

impl FixtureManager {
    pub fn new() -> Self {
        let dir = PathBuf::from("tests/e2e_fixtures/real");
        std::fs::create_dir_all(&dir).ok();
        Self { dir }
    }

    pub fn ensure_fixtures(&self) -> Result<()> {
        self.create_single_page("single_page.pdf", "SINGLE_PAGE_SAMPLE")?;
        self.create_single_page("merge_a.pdf", "MERGE_PAGE_AAA")?;
        self.create_single_page("merge_b.pdf", "MERGE_PAGE_BBB")?;
        self.create_single_page("merge_c.pdf", "MERGE_PAGE_CCC")?;
        self.create_multi_page("multi_page.pdf", 5)?;
        self.create_multi_page("large_doc.pdf", 10)?;
        self.create_text_file("test.csv", "ColA,ColB,ColC\n10,20,30\n40,50,60\n")?;
        self.create_text_file(
            "test.html",
            "<html><body><h1>HTML Test</h1><p>Sample</p></body></html>",
        )?;
        self.create_text_file("test.md", "# MD Test\n\nSample markdown paragraph.")?;
        self.create_png("test.png")?;
        self.create_encrypted_pdf("encrypted.pdf", "testpass")?;
        self.create_excel_file("test.xlsx")?;
        self.create_multimodal_pdf("multimodal.pdf")?;
        self.create_multimodal_pdf("minimal.pdf")?;
        Ok(())
    }

    fn create_multimodal_pdf(&self, filename: &str) -> Result<()> {
        let path = self.dir.join(filename);
        if path.exists() {
            return Ok(());
        }

        let mut doc = lopdf::Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let image_id = doc.new_object_id();

        // Minimal JPEG-like data
        let fake_jpeg_data = b"fake_jpeg_data".to_vec();

        let mut image_stream_dict = lopdf::Dictionary::new();
        image_stream_dict.set("Type", lopdf::Object::Name(b"XObject".to_vec()));
        image_stream_dict.set("Subtype", lopdf::Object::Name(b"Image".to_vec()));
        image_stream_dict.set("Width", lopdf::Object::Integer(100));
        image_stream_dict.set("Height", lopdf::Object::Integer(100));
        image_stream_dict.set("ColorSpace", lopdf::Object::Name(b"DeviceRGB".to_vec()));
        image_stream_dict.set("BitsPerComponent", lopdf::Object::Integer(8));
        image_stream_dict.set("Filter", lopdf::Object::Name(b"DCTDecode".to_vec()));

        let image_stream = lopdf::Stream::new(image_stream_dict, fake_jpeg_data);
        doc.objects
            .insert(image_id, lopdf::Object::Stream(image_stream));

        let mut xobj_dict = lopdf::Dictionary::new();
        xobj_dict.set("Im1", lopdf::Object::Reference(image_id));

        let mut resources_dict = lopdf::Dictionary::new();
        resources_dict.set("XObject", lopdf::Object::Dictionary(xobj_dict));

        let mut page_dict = lopdf::Dictionary::new();
        page_dict.set("Type", lopdf::Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", lopdf::Object::Reference(pages_id));
        page_dict.set("Resources", lopdf::Object::Dictionary(resources_dict));

        let content = "BT /F1 12 Tf 100 100 Td (Multimodal Page Text) Tj ET";
        let content_stream =
            lopdf::Stream::new(lopdf::Dictionary::new(), content.as_bytes().to_vec());
        let content_id = doc.add_object(lopdf::Object::Stream(content_stream));
        page_dict.set("Contents", lopdf::Object::Reference(content_id));

        let page_id = doc.add_object(page_dict);

        let mut pages_dict = lopdf::Dictionary::new();
        pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
        pages_dict.set("Count", lopdf::Object::Integer(1));
        pages_dict.set(
            "Kids",
            lopdf::Object::Array(vec![lopdf::Object::Reference(page_id)]),
        );

        doc.objects
            .insert(pages_id, lopdf::Object::Dictionary(pages_dict));
        doc.trailer
            .set("Root", lopdf::Object::Dictionary(lopdf::Dictionary::new()));

        let mut catalog = lopdf::Dictionary::new();
        catalog.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
        catalog.set("Pages", lopdf::Object::Reference(pages_id));
        let catalog_id = doc.add_object(catalog);
        doc.trailer
            .set("Root", lopdf::Object::Reference(catalog_id));

        doc.save(path)?;
        Ok(())
    }

    fn create_encrypted_pdf(&self, filename: &str, pass: &str) -> Result<()> {
        let path = self.dir.join(filename);
        let src = self.dir.join("single_page.pdf");

        let needs_creation = if !path.exists() {
            true
        } else {
            match lopdf::Document::load(&path) {
                Ok(doc) => !doc.trailer.has(b"Encrypt"),
                Err(_) => true,
            }
        };

        if needs_creation {
            use paperpilot_core::traits::{PdfDocument, PdfOperation};
            use paperpilot_pdf::document::LopdfDocument;
            use paperpilot_pdf::operations::encrypt::EncryptOperation;

            let mut doc = LopdfDocument::load(&src)
                .map_err(|e| anyhow::anyhow!("Failed to load source PDF: {}", e))?;

            let mut op = EncryptOperation::new();
            op.user_password = Some(pass.to_string());
            op.owner_password = Some(pass.to_string());

            op.execute(&mut doc)
                .map_err(|e| anyhow::anyhow!("Failed to encrypt PDF: {}", e))?;

            doc.save(&path)
                .map_err(|e| anyhow::anyhow!("Failed to save encrypted PDF: {}", e))?;
        }
        Ok(())
    }

    fn create_png(&self, filename: &str) -> Result<()> {
        let path = self.dir.join(filename);
        if !path.exists() {
            let mut img: RgbImage = ImageBuffer::new(256, 256);
            for (x, y, pixel) in img.enumerate_pixels_mut() {
                let r = (0.3 * x as f32) as u8;
                let b = (0.3 * y as f32) as u8;
                *pixel = image::Rgb([r, 0, b]);
            }
            img.save(path)?;
        }
        Ok(())
    }

    fn create_single_page(&self, filename: &str, text: &str) -> Result<()> {
        let path = self.dir.join(filename);
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        });

        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F1".into(), 24.into()]),
                Operation::new("Td", vec![50.into(), 700.into()]),
                Operation::new("Tj", vec![Object::string_literal(text)]),
                Operation::new("ET", vec![]),
            ],
        };
        let content_bytes = content.encode().unwrap();
        let content_id = doc.add_object(Stream::new(dictionary! {}, content_bytes));

        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
            "Contents" => content_id,
            "Resources" => dictionary! {
                "Font" => dictionary! {
                    "F1" => font_id,
                }
            }
        });

        doc.objects.insert(
            pages_id,
            Object::Dictionary(dictionary! {
                "Type" => "Pages",
                "Kids" => vec![page_id.into()],
                "Count" => 1,
            }),
        );

        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);
        doc.save(path)?;
        Ok(())
    }

    fn create_multi_page(&self, filename: &str, num_pages: usize) -> Result<()> {
        let path = self.dir.join(filename);
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        });

        let mut page_ids = Vec::new();

        for i in 1..=num_pages {
            let text = format!("PAGE_TEXT_P{}", i);
            let content = Content {
                operations: vec![
                    Operation::new("BT", vec![]),
                    Operation::new("Tf", vec!["F1".into(), 24.into()]),
                    Operation::new("Td", vec![50.into(), 700.into()]),
                    Operation::new("Tj", vec![Object::string_literal(text)]),
                    Operation::new("ET", vec![]),
                ],
            };
            let content_bytes = content.encode().unwrap();
            let content_id = doc.add_object(Stream::new(dictionary! {}, content_bytes));

            let page_id = doc.add_object(dictionary! {
                "Type" => "Page",
                "Parent" => pages_id,
                "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
                "Contents" => content_id,
                "Resources" => dictionary! {
                    "Font" => dictionary! {
                        "F1" => font_id,
                    }
                }
            });
            page_ids.push(page_id.into());
        }

        doc.objects.insert(
            pages_id,
            Object::Dictionary(dictionary! {
                "Type" => "Pages",
                "Kids" => page_ids,
                "Count" => num_pages as i64,
            }),
        );

        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);
        doc.save(path)?;
        Ok(())
    }

    fn create_text_file(&self, filename: &str, content: &str) -> Result<()> {
        let path = self.dir.join(filename);
        if !path.exists() {
            std::fs::write(path, content)?;
        }
        Ok(())
    }

    fn create_excel_file(&self, filename: &str) -> Result<()> {
        let path = self.dir.join(filename);
        if !path.exists() {
            let b64 = "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";
            use base64::{engine::general_purpose, Engine as _};
            let bytes = general_purpose::STANDARD.decode(b64).unwrap();
            std::fs::write(path, bytes)?;
        }
        Ok(())
    }
}
