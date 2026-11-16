use lopdf::{Dictionary, Document, Object, Stream, dictionary};
use paperpilot_core::traits::PdfOperation;
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::flatten::FlattenOperation;
use paperpilot_pdf::operations::form_fill::{FormFillTemplate, read_fields};
use std::collections::HashMap;

fn template() -> Document {
    let mut doc = Document::with_version("1.5");
    let pages = doc.new_object_id();
    let font = doc.add_object(
        dictionary! { "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica" },
    );
    let content = doc.add_object(Stream::new(
        Dictionary::new(),
        b"BT /TemplateFont 14 Tf 50 760 Td (Original template text) Tj ET".to_vec(),
    ));
    let page = doc.add_object(dictionary! { "Type" => "Page", "Parent" => pages, "MediaBox" => vec![0.into(),0.into(),612.into(),792.into()], "Contents" => content });
    // Resources inherited from a direct dictionary on the parent page tree.
    doc.set_object(pages, dictionary! { "Type" => "Pages", "Count" => 1, "Kids" => vec![page.into()], "Resources" => dictionary! { "Font" => dictionary! { "TemplateFont" => font } } });
    let parent = doc.new_object_id();
    let name = doc.add_object(dictionary! { "Type" => "Annot", "Subtype" => "Widget", "Parent" => parent,
        "T" => lopdf::text_string("姓名"), "Rect" => vec![50.into(),680.into(),400.into(),710.into()], "V" => lopdf::text_string("初始值") });
    doc.set_object(parent, dictionary! { "T" => lopdf::text_string("customer"), "FT" => "Tx", "Kids" => vec![name.into()] });
    let blank = doc.add_object(Stream::new(dictionary! { "Type" => "XObject", "Subtype" => "Form", "BBox" => vec![0.into(),0.into(),20.into(),20.into()] }, b"q Q".to_vec()));
    let checkbox = doc.add_object(dictionary! { "Type" => "Annot", "Subtype" => "Widget", "T" => lopdf::text_string("agree"), "FT" => "Btn",
        "Rect" => vec![50.into(),630.into(),70.into(),650.into()], "AP" => dictionary! { "N" => dictionary! { "Off" => blank, "Accepted" => blank } }, "V" => Object::Name(b"Off".to_vec()) });
    let note = doc.add_object(dictionary! { "Type" => "Annot", "Subtype" => "Text", "Rect" => vec![20.into(),20.into(),40.into(),40.into()], "Contents" => Object::string_literal("Retain note") });
    doc.get_object_mut(page)
        .unwrap()
        .as_dict_mut()
        .unwrap()
        .set("Annots", vec![name.into(), checkbox.into(), note.into()]);
    let acroform = doc.add_object(
        dictionary! { "Fields" => vec![parent.into(), checkbox.into()], "NeedAppearances" => true },
    );
    let root = doc
        .add_object(dictionary! { "Type" => "Catalog", "Pages" => pages, "AcroForm" => acroform });
    doc.trailer.set("Root", root);
    doc
}

fn bytes(mut doc: Document) -> Vec<u8> {
    let mut out = Vec::new();
    doc.save_to(&mut out).unwrap();
    out
}
fn values() -> HashMap<String, String> {
    HashMap::from([
        ("customer.姓名".into(), "张三".into()),
        ("agree".into(), "true".into()),
    ])
}

#[test]
fn hierarchical_unicode_fields_and_checkbox_export_states_round_trip() {
    let source = bytes(template());
    let form = FormFillTemplate::from_bytes(&source).unwrap();
    assert_eq!(form.field_names(), vec!["customer.姓名", "agree"]);
    let filled = Document::load_mem(&form.fill(&values(), false).unwrap()).unwrap();
    let fields = read_fields(&filled).unwrap();
    assert_eq!(fields["customer.姓名"], "张三");
    assert_eq!(fields["agree"], "Accepted");
    let acroform = filled
        .get_object(
            filled
                .catalog()
                .unwrap()
                .get(b"AcroForm")
                .unwrap()
                .as_reference()
                .unwrap(),
        )
        .unwrap()
        .as_dict()
        .unwrap();
    assert!(!acroform.get(b"NeedAppearances").unwrap().as_bool().unwrap());
    assert_eq!(
        read_fields(&Document::load_mem(&source).unwrap()).unwrap()["customer.姓名"],
        "初始值"
    );
}

#[test]
fn flatten_preserves_inherited_resources_and_non_form_annotations() {
    let form = FormFillTemplate::from_bytes(&bytes(template())).unwrap();
    let flat = Document::load_mem(&form.fill(&values(), true).unwrap()).unwrap();
    assert!(!flat.catalog().unwrap().has(b"AcroForm"));
    let page = flat
        .get_dictionary(*flat.get_pages().get(&1).unwrap())
        .unwrap();
    let resources = page.get(b"Resources").unwrap().as_dict().unwrap();
    assert!(
        resources
            .get(b"Font")
            .unwrap()
            .as_dict()
            .unwrap()
            .has(b"TemplateFont")
    );
    assert_eq!(
        resources.get(b"XObject").unwrap().as_dict().unwrap().len(),
        2
    );
    assert_eq!(page.get(b"Annots").unwrap().as_array().unwrap().len(), 1);
}

#[test]
fn standalone_flatten_builds_missing_appearances_and_supports_inline_acroform() {
    let mut doc = template();
    let root = doc.trailer.get(b"Root").unwrap().as_reference().unwrap();
    let af = doc
        .get_dictionary(
            doc.catalog()
                .unwrap()
                .get(b"AcroForm")
                .unwrap()
                .as_reference()
                .unwrap(),
        )
        .unwrap()
        .clone();
    doc.get_object_mut(root)
        .unwrap()
        .as_dict_mut()
        .unwrap()
        .set("AcroForm", af);
    let mut document = LopdfDocument::from_inner(doc);
    FlattenOperation::new().execute(&mut document).unwrap();
    let reopened = Document::load_mem(&bytes(document.inner)).unwrap();
    assert!(!reopened.catalog().unwrap().has(b"AcroForm"));
    let page = reopened
        .get_dictionary(*reopened.get_pages().get(&1).unwrap())
        .unwrap();
    assert_eq!(page.get(b"Annots").unwrap().as_array().unwrap().len(), 1);
}

#[test]
fn missing_unknown_and_unsupported_values_fail_without_touching_template() {
    let form = FormFillTemplate::from_bytes(&bytes(template())).unwrap();
    let mut data = values();
    data.remove("agree");
    assert!(form.fill(&data, true).is_err());
    let mut data = values();
    data.insert("unknown".into(), "x".into());
    assert!(form.fill(&data, false).is_err());
    let mut data = values();
    data.insert("customer.姓名".into(), "😀".into());
    assert!(form.fill(&data, true).is_err());
    let mut data = values();
    data.insert("agree".into(), "invalid-export-state".into());
    assert!(form.fill(&data, true).is_err());
    assert!(form.fill(&values(), false).is_ok());
}

#[test]
fn cyclic_field_tree_is_rejected() {
    let mut doc = template();
    let af = doc
        .catalog()
        .unwrap()
        .get(b"AcroForm")
        .unwrap()
        .as_reference()
        .unwrap();
    let parent = doc
        .get_dictionary(af)
        .unwrap()
        .get(b"Fields")
        .unwrap()
        .as_array()
        .unwrap()[0]
        .as_reference()
        .unwrap();
    doc.get_object_mut(parent)
        .unwrap()
        .as_dict_mut()
        .unwrap()
        .set("Kids", vec![Object::Reference(parent)]);
    assert!(FormFillTemplate::from_bytes(&bytes(doc)).is_err());
}

#[test]
fn hidden_widget_is_removed_without_drawing_its_value() {
    let mut doc = template();
    let page = *doc.get_pages().get(&1).unwrap();
    let name = doc
        .get_dictionary(page)
        .unwrap()
        .get(b"Annots")
        .unwrap()
        .as_array()
        .unwrap()[0]
        .as_reference()
        .unwrap();
    doc.get_object_mut(name)
        .unwrap()
        .as_dict_mut()
        .unwrap()
        .set("F", 2);
    let form = FormFillTemplate::from_bytes(&bytes(doc)).unwrap();
    let flat = Document::load_mem(&form.fill(&values(), true).unwrap()).unwrap();
    let page = flat.get_dictionary(page).unwrap();
    assert_eq!(
        page.get(b"Resources")
            .unwrap()
            .as_dict()
            .unwrap()
            .get(b"XObject")
            .unwrap()
            .as_dict()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(page.get(b"Annots").unwrap().as_array().unwrap().len(), 1);
}
