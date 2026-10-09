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
}
