#![cfg(feature = "pdf")]

use assert_cmd::Command;
use lopdf::{Dictionary, Document, Object, Stream, dictionary};
use paperpilot_core::traits::PdfOperation;
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::form::CreateFormFieldOperation;
use paperpilot_pdf::operations::form_fill::read_fields;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn template(path: &Path) {
    let mut doc = Document::with_version("1.5");
    let pages = doc.new_object_id();
    let content = doc.add_object(Stream::new(
        Dictionary::new(),
        b"q 0.9 G 40 600 400 180 re S Q".to_vec(),
    ));
    let note = doc.add_object(dictionary! { "Type" => "Annot", "Subtype" => "Text", "Rect" => vec![10.into(), 10.into(), 30.into(), 30.into()], "Contents" => Object::string_literal("Keep this annotation") });
    let page = doc.add_object(dictionary! {
        "Type" => "Page", "Parent" => pages, "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Contents" => content, "Annots" => vec![note.into()],
    });
    doc.set_object(
        pages,
        dictionary! { "Type" => "Pages", "Kids" => vec![page.into()], "Count" => 1 },
    );
    let root = doc.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages });
    doc.trailer.set("Root", root);
    let mut doc = LopdfDocument::from_inner(doc);
    for (name, kind, rect) in [
        ("name", "text", [60.0, 720.0, 420.0, 748.0]),
        ("email", "text", [60.0, 680.0, 420.0, 708.0]),
        ("subscribe", "checkbox", [60.0, 640.0, 80.0, 660.0]),
    ] {
        CreateFormFieldOperation::new(name.into(), kind.into(), 1, rect)
            .execute(&mut doc)
            .unwrap();
    }
    doc.inner.save(path).unwrap();
}

fn invoke(template: &Path, data: &Path, out: &Path, extra: &[&str], success: bool) -> Value {
    let mut command = Command::cargo_bin("paperpilot").unwrap();
    command
        .arg("fill-batch")
        .arg("--template")
        .arg(template)
        .arg("--data")
        .arg(data)
        .arg("--output-dir")
        .arg(out)
        .arg("--json")
        .args(extra);
    if !extra.contains(&"--date") {
        command.arg("--date").arg("2026-10-28");
    }
    let assertion = command.assert();
    let assertion = if success {
        assertion.success()
    } else {
        assertion.failure()
    };
    serde_json::from_slice(&assertion.get_output().stdout).unwrap()
}

fn check_flattened(path: &Path) {
    let doc = Document::load(path).unwrap();
    assert_eq!(doc.get_pages().len(), 1);
    assert!(!doc.catalog().unwrap().has(b"AcroForm"));
    let page = doc
        .get_dictionary(*doc.get_pages().get(&1).unwrap())
        .unwrap();
    let annotations = page.get(b"Annots").unwrap().as_array().unwrap();
    assert_eq!(annotations.len(), 1); // Preserve the unrelated note annotation.
    let resources = page.get(b"Resources").unwrap().as_dict().unwrap();
    let xobjects = resources.get(b"XObject").unwrap().as_dict().unwrap();
    assert_eq!(xobjects.len(), 3);
    for (_, reference) in xobjects {
        assert!(
            doc.get_object(reference.as_reference().unwrap())
                .unwrap()
                .as_stream()
                .is_ok()
        );
    }
}

#[test]
fn csv_quotes_unicode_empty_values_and_dynamic_names() {
    let dir = tempdir().unwrap();
    let form = dir.path().join("form.pdf");
    template(&form);
    let original = fs::read(&form).unwrap();
    let data = dir.path().join("records.csv");
    fs::write(
        &data,
        "\u{feff}name,email,subscribe\r\n\"Doe, Jane\",jane@example.test,true\r\n李雷,,false\r\n",
    )
    .unwrap();
    let out = dir.path().join("out");
    let result = invoke(
        &form,
        &data,
        &out,
        &[
            "--name-template",
            "{row.name}_{date}_{index:04}.pdf",
            "--keep-editable",
        ],
        true,
    );
    assert_eq!(result["batch"]["succeeded"], 2);
    let first = Document::load(out.join("Doe, Jane_2026-10-28_0001.pdf")).unwrap();
    let values = read_fields(&first).unwrap();
    assert_eq!(values["name"], "Doe, Jane");
    assert_eq!(values["subscribe"], "Yes");
    let second = Document::load(out.join("李雷_2026-10-28_0002.pdf")).unwrap();
    let values = read_fields(&second).unwrap();
    assert_eq!(values["name"], "李雷");
    assert_eq!(values["email"], "");
    assert_eq!(values["subscribe"], "Off");
    assert_eq!(fs::read(&form).unwrap(), original);
    let receipt: Value =
        serde_json::from_slice(&fs::read(out.join("execution_receipt.json")).unwrap()).unwrap();
    assert_eq!(receipt, result["batch"]);
    assert_eq!(receipt["template_sha256"].as_str().unwrap().len(), 64);
    assert_eq!(
        receipt["results"][0]["output_sha256"]
            .as_str()
            .unwrap()
            .len(),
        64
    );
}

#[test]
fn json_scalars_mapping_and_default_flattening() {
    let dir = tempdir().unwrap();
    let form = dir.path().join("form.pdf");
    template(&form);
    let data = dir.path().join("records.json");
    fs::write(&data, r#"[{"customer":"张三","email":null,"subscribe":true},{"customer":12345,"email":"info@example.test","subscribe":false}]"#).unwrap();
    let mapping = dir.path().join("mapping.json");
    fs::write(&mapping, r#"{"name":"customer"}"#).unwrap();
    let out = dir.path().join("out");
    let result = invoke(
        &form,
        &data,
        &out,
        &["--field-map", mapping.to_str().unwrap()],
        true,
    );
    assert_eq!(result["operation"], "FillBatch");
    assert_eq!(result["batch"]["failed"], 0);
    assert_eq!(
        result["batch"]["operations_applied"],
        json!(["pdf_fill_form", "pdf_flatten"])
    );
    check_flattened(&out.join("0001.pdf"));
    check_flattened(&out.join("0002.pdf"));
}

#[test]
fn quoted_multiline_csv_values_fill_multiline_fields() {
    let dir = tempdir().unwrap();
    let form = dir.path().join("form.pdf");
    template(&form);
    let mut doc = Document::load(&form).unwrap();
    let af = doc
        .catalog()
        .unwrap()
        .get(b"AcroForm")
        .unwrap()
        .as_reference()
        .unwrap();
    let name = doc
        .get_dictionary(af)
        .unwrap()
        .get(b"Fields")
        .unwrap()
        .as_array()
        .unwrap()[0]
        .as_reference()
        .unwrap();
    doc.get_object_mut(name)
        .unwrap()
        .as_dict_mut()
        .unwrap()
        .set("Ff", 1 << 12);
    doc.save(&form).unwrap();
    let data = dir.path().join("records.csv");
    fs::write(
        &data,
        "name,email,subscribe\n\"First line\nSecond line\",first@example.test,true\nAlice,,false\n",
    )
    .unwrap();
    let out = dir.path().join("out");
    let result = invoke(&form, &data, &out, &["--keep-editable"], true);
    assert_eq!(result["batch"]["succeeded"], 2);
    let filled = Document::load(out.join("0001.pdf")).unwrap();
    assert_eq!(
        read_fields(&filled).unwrap()["name"],
        "First line\nSecond line"
    );
}

#[test]
fn invalid_rows_continue_and_return_nonzero_with_receipt() {
    let dir = tempdir().unwrap();
    let form = dir.path().join("form.pdf");
    template(&form);
    let data = dir.path().join("records.json");
    fs::write(&data, r#"[{"name":"Alice","email":"a@example.test","subscribe":true},{"name":"Missing"},null,{"name":{},"email":"a","subscribe":false},{"name":"Bob","email":"b@example.test","subscribe":false}]"#).unwrap();
    let out = dir.path().join("out");
    let result = invoke(&form, &data, &out, &[], false);
    assert_eq!(result["success"], false);
    assert_eq!(result["batch"]["total"], 5);
    assert_eq!(result["batch"]["succeeded"], 2);
    assert_eq!(result["batch"]["failed"], 3);
    assert!(out.join("0001.pdf").exists());
    assert!(out.join("0005.pdf").exists());
    for index in 2..=4 {
        assert!(!out.join(format!("{index:04}.pdf")).exists());
    }
    assert!(
        result["batch"]["results"][1]["error"]
            .as_str()
            .unwrap()
            .contains("Missing column")
    );
    assert_eq!(fs::read_dir(&out).unwrap().count(), 3); // No partial temporary PDFs.
}

#[test]
fn duplicate_names_and_existing_outputs_are_never_overwritten() {
    let dir = tempdir().unwrap();
    let form = dir.path().join("form.pdf");
    template(&form);
    let data = dir.path().join("records.csv");
    fs::write(
        &data,
        "name,email,subscribe\nAlice,a,true\nAlice,b,false\nExisting,e,false\nBob,b,false\n",
    )
    .unwrap();
    let out = dir.path().join("out");
    fs::create_dir(&out).unwrap();
    fs::write(out.join("Existing.pdf"), b"pre-existing user file").unwrap();
    let result = invoke(
        &form,
        &data,
        &out,
        &["--name-template", "{row.name}.pdf"],
        false,
    );
    assert_eq!(result["batch"]["succeeded"], 2);
    assert_eq!(result["batch"]["failed"], 2);
    assert_eq!(
        fs::read(out.join("Existing.pdf")).unwrap(),
        b"pre-existing user file"
    );
    let receipt_before = fs::read(out.join("execution_receipt.json")).unwrap();
    let pdf_before = fs::read(out.join("Alice.pdf")).unwrap();
    let rerun = invoke(&form, &data, &out, &[], false);
    assert!(rerun.get("batch").is_none());
    assert_eq!(
        fs::read(out.join("execution_receipt.json")).unwrap(),
        receipt_before
    );
    assert_eq!(fs::read(out.join("Alice.pdf")).unwrap(), pdf_before);
}

#[test]
fn malformed_csv_row_does_not_discard_later_records() {
    let dir = tempdir().unwrap();
    let form = dir.path().join("form.pdf");
    template(&form);
    let data = dir.path().join("records.csv");
    fs::write(
        &data,
        "name,email,subscribe\nAlice,a,true\nMissing,only-two\nBob,b,false\n",
    )
    .unwrap();
    let out = dir.path().join("out");
    let result = invoke(&form, &data, &out, &[], false);
    assert_eq!(result["batch"]["succeeded"], 2);
    assert_eq!(result["batch"]["failed"], 1);
    assert!(out.join("0003.pdf").exists());
}

#[test]
fn filename_substitutions_cannot_escape_output_directory() {
    let dir = tempdir().unwrap();
    let form = dir.path().join("form.pdf");
    template(&form);
    let data = dir.path().join("records.json");
    fs::write(&data, r#"[{"name":"../escape","email":"a","subscribe":false},{"name":"CON","email":"b","subscribe":true}]"#).unwrap();
    let out = dir.path().join("out");
    let result = invoke(
        &form,
        &data,
        &out,
        &["--name-template", "{row.name}.pdf"],
        false,
    );
    assert_eq!(result["batch"]["succeeded"], 1);
    assert_eq!(result["batch"]["failed"], 1);
    assert!(out.join(".._escape.pdf").is_file());
    assert!(!dir.path().join("escape.pdf").exists());
}

#[test]
fn invalid_configuration_fails_before_creating_outputs() {
    let dir = tempdir().unwrap();
    let form = dir.path().join("form.pdf");
    template(&form);
    let data = dir.path().join("records.csv");
    fs::write(&data, "name,email,subscribe\nAlice,a,true\n").unwrap();
    for (index, extra) in [
        vec!["--name-template", "../{index}.pdf"],
        vec!["--name-template", "{unknown}.pdf"],
        vec!["--name-template", "{index:99999}.pdf"],
        vec!["--date", "not-a-date"],
    ]
    .iter()
    .enumerate()
    {
        let out = dir.path().join(format!("out{index}"));
        invoke(&form, &data, &out, extra, false);
        assert!(!out.exists());
    }
}

#[test]
fn duplicate_csv_headers_and_invalid_json_are_global_errors() {
    let dir = tempdir().unwrap();
    let form = dir.path().join("form.pdf");
    template(&form);
    for (index, (name, data)) in [
        ("duplicate.csv", "name,name\nAlice,Bob\n"),
        ("wrong.json", "{}"),
        ("empty.json", "[]"),
        ("broken.json", "[{"),
        ("unknown.txt", "name,email\na,b"),
    ]
    .iter()
    .enumerate()
    {
        let input = dir.path().join(name);
        fs::write(&input, data).unwrap();
        let out = dir.path().join(format!("out{index}"));
        invoke(&form, &input, &out, &[], false);
        assert!(!out.exists());
    }
}

#[test]
fn thousand_records_produce_thousand_readable_flattened_pdfs() {
    let dir = tempdir().unwrap();
    let form = dir.path().join("form.pdf");
    template(&form);
    let data = dir.path().join("records.csv");
    let mut csv = String::from("name,email,subscribe\n");
    for index in 1..=1000 {
        csv += &format!("Customer{index},person{index}@example.test,true\n");
    }
    fs::write(&data, csv).unwrap();
    let out = dir.path().join("out");
    let result = invoke(&form, &data, &out, &[], true);
    assert_eq!(result["batch"]["succeeded"], 1000);
    assert_eq!(result["batch"]["failed"], 0);
    assert_eq!(fs::read_dir(&out).unwrap().count(), 1001);
    for index in 1..=1000 {
        check_flattened(&out.join(format!("{index:04}.pdf")));
    }
}
