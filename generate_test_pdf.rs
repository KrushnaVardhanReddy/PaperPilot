use lopdf::{Document, Dictionary, Object, Stream, content::Content, content::Operation};

fn main() {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let mut page_ids = vec![];
    let font_id = doc.add_object(lopdf::dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    });

    let resources_id = doc.add_object(lopdf::dictionary! {
        "Font" => lopdf::dictionary! { "F1" => font_id },
    });

    let content = Content {
        operations: vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 24.into()]),
            Operation::new("Td", vec![100.into(), 100.into()]),
            Operation::new("Tj", vec![Object::String(b"Test Page".to_vec(), lopdf::StringFormat::Literal)]),
            Operation::new("ET", vec![]),
        ],
    };

    let content_id = doc.add_object(Stream::new(lopdf::dictionary! {}, content.encode().unwrap()));
    let page_id = doc.add_object(lopdf::dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    });
    page_ids.push(Object::Reference(page_id));

    let pages_dict = lopdf::dictionary! {
        "Type" => "Pages",
        "Count" => 1,
        "Kids" => page_ids,
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

    let catalog_id = doc.add_object(lopdf::dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });

    doc.trailer.set("Root", catalog_id);

    doc.save("tests/e2e_fixtures/real/single_page.pdf").unwrap();
}
