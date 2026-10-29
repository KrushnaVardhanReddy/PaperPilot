#![allow(dead_code)]
use lopdf::{Dictionary, Document, Object, Stream, dictionary};

#[derive(Clone, Copy)]
pub enum PageKind {
    Text,
    Image,
    Blank,
    NestedImage,
    UnusedImage,
}

/// Complete PDFs with a valid catalog, page tree, fonts and genuine image data.
pub fn pdf(kinds: &[PageKind]) -> Vec<u8> {
    let mut doc = Document::with_version("1.5");
    let pages = doc.new_object_id();
    let font = doc.add_object(dictionary! { "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica", "Encoding" => "WinAnsiEncoding" });
    let image = doc.add_object(Stream::new(
        dictionary! {
            "Type" => "XObject", "Subtype" => "Image", "Width" => 2, "Height" => 2,
            "BitsPerComponent" => 8, "ColorSpace" => "DeviceGray"
        },
        vec![0, 255, 255, 0],
    ));
    let form = doc.add_object(Stream::new(dictionary! {
        "Type" => "XObject", "Subtype" => "Form", "BBox" => vec![0.into(), 0.into(), 1.into(), 1.into()],
        "Resources" => dictionary! { "XObject" => dictionary! { "Im" => image } }
    }, b"q /Im Do Q".to_vec()));
    let resources = doc.add_object(dictionary! { "Font" => dictionary! { "F1" => font }, "XObject" => dictionary! { "Im" => image, "Fm" => form } });
    let mut kids = Vec::<Object>::new();
    for kind in kinds {
        let content = match kind {
            PageKind::Text => b"BT /F1 18 Tf 60 720 Td (Original invoice 123) Tj ET".to_vec(),
            PageKind::Image => b"q 200 0 0 200 60 400 cm /Im Do Q".to_vec(),
            PageKind::NestedImage => b"q 200 0 0 200 60 400 cm /Fm Do Q".to_vec(),
            PageKind::Blank | PageKind::UnusedImage => b"q Q".to_vec(),
        };
        let stream = doc.add_object(Stream::new(Dictionary::new(), content));
        let mut page = dictionary! { "Type" => "Page", "Parent" => pages,
        "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()], "Contents" => stream };
        // Nested images deliberately exercise inherited page resources.
        if !matches!(kind, PageKind::NestedImage) {
            page.set("Resources", resources);
        }
        kids.push(doc.add_object(page).into());
    }
    doc.set_object(pages, dictionary! { "Type" => "Pages", "Count" => kinds.len() as i64, "Kids" => kids, "Resources" => resources });
    let catalog = doc.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages });
    doc.trailer.set("Root", catalog);
    let mut bytes = Vec::new();
    doc.save_to(&mut bytes).unwrap();
    bytes
}
