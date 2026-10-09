use lopdf::content::{Content, Operation};
use lopdf::{dictionary, Document, Object, Stream};
use std::fs;

use std::path::Path;

fn create_text_page(
    doc: &mut Document,
    text: &str,
    _page_number: u32,
) -> (lopdf::ObjectId, lopdf::ObjectId) {
    let font_id = doc.add_object(dictionary!(
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    ));

    let resources_id = doc.add_object(dictionary!(
        "Font" => dictionary!(
            "F1" => font_id,
        ),
    ));

    let content = Content {
        operations: vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 12.into()]),
            Operation::new("Td", vec![50.into(), 700.into()]),
            Operation::new("Tj", vec![Object::string_literal(text)]),
            Operation::new("ET", vec![]),
        ],
    };

    let content_stream = Stream::new(dictionary!(), content.encode().unwrap());
    let content_id = doc.add_object(content_stream);

    let page_id = doc.add_object(dictionary!(
        "Type" => "Page",
        "MediaBox" => vec![0.into(), 0.into(), 600.into(), 800.into()],
        "Contents" => content_id,
        "Resources" => resources_id,
    ));

    (page_id, content_id)
}

fn generate_doc1() -> Document {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();

    let mut page_ids = vec![];
    for i in 1..=4 {
        let text = if i == 1 {
            "Asset Purchase Agreement. PII included: SSN: 987-65-4321, Routing: 021000021"
        } else {
            "Standard clause text."
        };
        let (page_id, _) = create_text_page(&mut doc, text, i as u32);
        if let Object::Dictionary(ref mut dict) = doc.get_object_mut(page_id).unwrap() {
            dict.set("Parent", pages_id);
        }
        page_ids.push(page_id);
    }

    doc.objects.insert(
        pages_id,
        Object::Dictionary(dictionary!(
            "Type" => "Pages",
            "Kids" => page_ids.iter().map(|&id| Object::Reference(id)).collect::<Vec<_>>(),
            "Count" => page_ids.len() as u32,
        )),
    );

    let catalog_id = doc.add_object(dictionary!(
        "Type" => "Catalog",
        "Pages" => pages_id,
    ));

    doc.trailer.set("Root", catalog_id);
    doc.compress();
    doc
}

fn generate_doc2() -> Document {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();

    let mut page_ids = vec![];
    for i in 1..=3 {
        let text = format!("Disclosure Schedule - Page {}", i);
        let (page_id, _) = create_text_page(&mut doc, &text, i as u32);
        if let Object::Dictionary(ref mut dict) = doc.get_object_mut(page_id).unwrap() {
            dict.set("Parent", pages_id);
            if i == 2 {
                dict.set("Rotate", 90);
            }
        }
        page_ids.push(page_id);
    }

    doc.objects.insert(
        pages_id,
        Object::Dictionary(dictionary!(
            "Type" => "Pages",
            "Kids" => page_ids.iter().map(|&id| Object::Reference(id)).collect::<Vec<_>>(),
            "Count" => page_ids.len() as u32,
        )),
    );

    let catalog_id = doc.add_object(dictionary!(
        "Type" => "Catalog",
        "Pages" => pages_id,
    ));

    doc.trailer.set("Root", catalog_id);
    doc.compress();
    doc
}

fn generate_doc3() -> Document {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();

    let mut page_ids = vec![];
    for i in 1..=2 {
        let text = format!("Financial Exhibit - Confidential Table {}", i);
        let (page_id, _) = create_text_page(&mut doc, &text, i as u32);
        if let Object::Dictionary(ref mut dict) = doc.get_object_mut(page_id).unwrap() {
            dict.set("Parent", pages_id);
        }
        page_ids.push(page_id);
    }

    doc.objects.insert(
        pages_id,
        Object::Dictionary(dictionary!(
            "Type" => "Pages",
            "Kids" => page_ids.iter().map(|&id| Object::Reference(id)).collect::<Vec<_>>(),
            "Count" => page_ids.len() as u32,
        )),
    );

    let catalog_id = doc.add_object(dictionary!(
        "Type" => "Catalog",
        "Pages" => pages_id,
    ));

    doc.trailer.set("Root", catalog_id);
    doc.compress();
    doc
}

pub fn generate_fixtures(output_dir: &str) -> std::io::Result<()> {
    fs::create_dir_all(output_dir)?;

    let mut doc1 = generate_doc1();
    doc1.save(Path::new(output_dir).join("01_asset_purchase_agreement.pdf"))
        .unwrap();

    let mut doc2 = generate_doc2();
    doc2.save(Path::new(output_dir).join("02_disclosure_schedules_tilted.pdf"))
        .unwrap();

    let mut doc3 = generate_doc3();
    doc3.save(Path::new(output_dir).join("03_unredacted_financial_exhibit.pdf"))
        .unwrap();

    Ok(())
}

fn main() {
    let output_dir = "../../demo/legal_discovery/input";
    let args: Vec<String> = std::env::args().collect();
    let dir = if args.len() > 1 { &args[1] } else { output_dir };

    if let Err(e) = generate_fixtures(dir) {
        eprintln!("Failed to generate fixtures: {}", e);
        std::process::exit(1);
    }
    println!("Fixtures generated successfully at {}", dir);
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[test]
    fn test_generate_fixtures() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_str().unwrap();

        assert!(generate_fixtures(path).is_ok());

        assert!(Path::new(path)
            .join("01_asset_purchase_agreement.pdf")
            .exists());
        assert!(Path::new(path)
            .join("02_disclosure_schedules_tilted.pdf")
            .exists());
        assert!(Path::new(path)
            .join("03_unredacted_financial_exhibit.pdf")
            .exists());
    }

    #[test]
    fn test_doc2_rotated_page() {
        let doc = generate_doc2();
        let mut rotated_page_found = false;

        for (_, object) in doc.objects.iter() {
            if let Object::Dictionary(dict) = object {
                let is_page =
                    matches!(dict.get(b"Type"), Ok(Object::Name(name)) if name == b"Page");
                let is_rotated = matches!(dict.get(b"Rotate"), Ok(Object::Integer(90)));
                if is_page && is_rotated {
                    rotated_page_found = true;
                }
            }
        }
        assert!(
            rotated_page_found,
            "Document 2 should have a page rotated 90 degrees"
        );
    }
}
