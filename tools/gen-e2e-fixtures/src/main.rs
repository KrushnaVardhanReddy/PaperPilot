use lopdf::{
    content::{Content, Operation},
    dictionary, Document, Object, Stream,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn get_workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn create_pdf(pages_text: &[&str]) -> Document {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    });

    let mut kids = Vec::new();
    for text in pages_text {
        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F1".into(), 14.into()]),
                Operation::new("Td", vec![50.into(), 700.into()]),
                Operation::new("Tj", vec![Object::string_literal(*text)]),
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
        kids.push(page_id.into());
    }

    let count = kids.len() as i32;
    doc.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => kids,
            "Count" => count,
        }),
    );

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);
    doc
}

fn create_image_pdf() -> Document {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();

    let image_stream = Stream::new(
        dictionary! {
            "Type" => "XObject",
            "Subtype" => "Image",
            "Width" => 10,
            "Height" => 10,
            "ColorSpace" => "DeviceRGB",
            "BitsPerComponent" => 8,
        },
        vec![255; 10 * 10 * 3],
    ); // dummy white image
    let image_id = doc.add_object(image_stream);

    let mut kids = Vec::new();

    for _ in 0..3 {
        let content = Content {
            operations: vec![
                Operation::new("q", vec![]),
                Operation::new(
                    "cm",
                    vec![
                        100.into(),
                        0.into(),
                        0.into(),
                        100.into(),
                        0.into(),
                        0.into(),
                    ],
                ),
                Operation::new("Do", vec!["Im1".into()]),
                Operation::new("Q", vec![]),
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
                "XObject" => dictionary! {
                    "Im1" => image_id,
                }
            }
        });
        kids.push(page_id.into());
    }

    doc.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => kids.clone(),
            "Count" => 3,
        }),
    );

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);
    doc
}

fn create_encrypted_pdf(output_path: &Path) {
    let encrypted_pdf_hex = "255044462d312e340a25e2e3cfd30a312030206f626a0a3c3c202f54797065202f436174616c6f67202f5061676573203220302052203e3e0a656e646f626a0a322030206f626a0a3c3c202f54797065202f5061676573202f4b696473205b203320302052205d202f436f756e742031203e3e0a656e646f626a0a332030206f626a0a3c3c202f54797065202f50616765202f506172656e74203220302052202f4d65646961426f78205b203020302036313220373932205d202f5265736f7572636573203c3c202f466f6e74203c3c202f4631203420302052203e3e203e3e202f436f6e74656e7473203520302052203e3e0a656e646f626a0a342030206f626a0a3c3c202f54797065202f466f6e74202f53756274797065202f5479706531202f42617365466f6e74202f48656c766574696361203e3e0a656e646f626a0a352030206f626a0a3c3c202f4c656e677468203531202f46696c746572202f466c6174654465636f6465203e3e0a73747265616d0a789c0b115428e17208315208762956c84c2d2e4d5548cecf2bc94c4bc50200880309990a656e6473747265616d0a656e646f626a0a362030206f626a0a3c3c202f46696c746572202f5374616e64617264202f562031202f522032202f4f203c6239333333333633643761376536313636616439396264383164393661666666623631623939333363333139363964343e202f55203c38663435646162623337346261363236306533376264333038616238386364373e202f50202d34203e3e0a656e646f626a0a787265660a3020370a303030303030303030302036353533352066200a30303030303030303135203030303030206e200a30303030303030303734203030303030206e200a30303030303030313333203030303030206e200a30303030303030323737203030303030206e200a30303030303030333635203030303030206e200a30303030303030343736203030303030206e200a747261696c65720a3c3c202f53697a652037202f526f6f74203120302052202f456e6372797074203620302052202f4944205b3c37336666666133643534346430383765353866386562306138656138376562343e3c37336666666133643534346430383765353866386562306138656138376562343e5d203e3e0a7374617274787265660a3637370a2525454f460a";
    let decoded = (0..encrypted_pdf_hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&encrypted_pdf_hex[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    fs::write(output_path, decoded).unwrap();
}

fn main() {
    let workspace = get_workspace_root();
    let fixtures_dir = workspace.join("tests").join("e2e_fixtures");
    fs::create_dir_all(&fixtures_dir).unwrap();

    let mut single = create_pdf(&["Single page text"]);
    single.save(fixtures_dir.join("single_page.pdf")).unwrap();

    let mut multi = create_pdf(&["Page 1", "Page 2", "Page 3", "Page 4", "Page 5"]);
    multi.save(fixtures_dir.join("multi_page.pdf")).unwrap();

    let pages: Vec<String> = (1..=20).map(|i| format!("Page {}", i)).collect();
    let page_refs: Vec<&str> = pages.iter().map(|s| s.as_str()).collect();
    let mut large = create_pdf(&page_refs);
    large.save(fixtures_dir.join("large_doc.pdf")).unwrap();

    let mut image_doc = create_image_pdf();
    image_doc.save(fixtures_dir.join("image_doc.pdf")).unwrap();

    create_encrypted_pdf(&fixtures_dir.join("encrypted.pdf"));

    // Generate form fixtures using paperpilot-cli
    let cli_path = workspace
        .join("target")
        .join("debug")
        .join("paperpilot-cli");
    let cli_path_str = if cli_path.exists() {
        cli_path.to_str().unwrap()
    } else {
        "cargo"
    };

    let mut add_text_cmd = Command::new(cli_path_str);
    if !cli_path.exists() {
        add_text_cmd.args(["run", "-p", "paperpilot-cli", "--"]);
    }
    add_text_cmd
        .args([
            "form",
            "add-field",
            fixtures_dir.join("single_page.pdf").to_str().unwrap(),
            "--name",
            "TestText",
            "--type",
            "text",
            "--page",
            "1",
            "--rect",
            "100,600,300,650",
            "--output",
            fixtures_dir.join("form.pdf").to_str().unwrap(),
        ])
        .status()
        .unwrap();

    let mut add_cb_cmd = Command::new(cli_path_str);
    if !cli_path.exists() {
        add_cb_cmd.args(["run", "-p", "paperpilot-cli", "--"]);
    }
    add_cb_cmd
        .args([
            "form",
            "add-field",
            fixtures_dir.join("form.pdf").to_str().unwrap(),
            "--name",
            "TestCheckbox",
            "--type",
            "checkbox",
            "--page",
            "1",
            "--rect",
            "100,500,120,520",
            "--output",
            fixtures_dir.join("form_temp.pdf").to_str().unwrap(),
        ])
        .status()
        .unwrap();
    fs::rename(
        fixtures_dir.join("form_temp.pdf"),
        fixtures_dir.join("form.pdf"),
    )
    .unwrap();

    let data_json = fixtures_dir.join("form_data.json");
    fs::write(
        &data_json,
        r#"{"TestText": "New Value", "TestCheckbox": "Off"}"#,
    )
    .unwrap();

    let mut fill_cmd = Command::new(cli_path_str);
    if !cli_path.exists() {
        fill_cmd.args(["run", "-p", "paperpilot-cli", "--"]);
    }
    fill_cmd
        .args([
            "form",
            "fill",
            fixtures_dir.join("form.pdf").to_str().unwrap(),
            "--data",
            data_json.to_str().unwrap(),
            "--output",
            fixtures_dir.join("form_filled.pdf").to_str().unwrap(),
        ])
        .status()
        .unwrap();

    println!("Fixtures generated.");
}
