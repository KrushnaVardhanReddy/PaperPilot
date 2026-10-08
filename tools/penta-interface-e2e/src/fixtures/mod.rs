use std::path::PathBuf;
use lopdf::{
    content::{Content, Operation},
    dictionary, Document, Object, Stream,
};
use anyhow::Result;
use image::{ImageBuffer, RgbImage};

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
        self.create_single_page("merge_a.pdf", "MERGE_PAGE_AAA")?;
        self.create_single_page("merge_b.pdf", "MERGE_PAGE_BBB")?;
        self.create_single_page("merge_c.pdf", "MERGE_PAGE_CCC")?;
        self.create_multi_page("multi_page.pdf", 5)?;
        self.create_text_file("test.csv", "ColA,ColB,ColC\n10,20,30\n40,50,60\n")?;
        self.create_text_file("test.html", "<html><body><h1>HTML Test</h1><p>Sample</p></body></html>")?;
        self.create_text_file("test.md", "# MD Test\n\nSample markdown paragraph.")?;
        self.create_png("test.png")?;
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
