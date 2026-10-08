use std::path::{Path, PathBuf};
use lopdf::{Document, Object, Dictionary};
use anyhow::Result;

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
        Ok(())
    }

    fn create_single_page(&self, filename: &str, text: &str) -> Result<()> {
        let path = self.dir.join(filename);
        if path.exists() { return Ok(()); }
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let content_id = doc.add_object(Object::Stream(lopdf::Stream::new(
            Dictionary::new(),
            format!("BT /F1 24 Tf 100 700 Td ({}) Tj ET", text).into_bytes(),
        )));
        let page_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", "Page".into()),
            ("Parent", pages_id.into()),
            ("Contents", content_id.into()),
        ]));
        let pages = Dictionary::from_iter(vec![
            ("Type", "Pages".into()),
            ("Count", 1.into()),
            ("Kids", vec![page_id.into()].into()),
        ]);
        doc.objects.insert(pages_id, Object::Dictionary(pages));
        let catalog_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", "Catalog".into()),
            ("Pages", pages_id.into()),
        ]));
        doc.trailer.set("Root", catalog_id);
        doc.save(path)?;
        Ok(())
    }

    fn create_multi_page(&self, filename: &str, num_pages: usize) -> Result<()> {
        let path = self.dir.join(filename);
        if path.exists() { return Ok(()); }
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let mut page_ids = Vec::new();

        for i in 1..=num_pages {
            let content_id = doc.add_object(Object::Stream(lopdf::Stream::new(
                Dictionary::new(),
                format!("BT /F1 24 Tf 100 700 Td (PAGE_TEXT_P{}) Tj ET", i).into_bytes(),
            )));
            let page_id = doc.add_object(Dictionary::from_iter(vec![
                ("Type", "Page".into()),
                ("Parent", pages_id.into()),
                ("Contents", content_id.into()),
            ]));
            page_ids.push(page_id.into());
        }

        let pages = Dictionary::from_iter(vec![
            ("Type", "Pages".into()),
            ("Count", (num_pages as i64).into()),
            ("Kids", page_ids.into()),
        ]);
        doc.objects.insert(pages_id, Object::Dictionary(pages));
        let catalog_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", "Catalog".into()),
            ("Pages", pages_id.into()),
        ]));
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
