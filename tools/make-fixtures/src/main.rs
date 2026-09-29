use lopdf::{Document, Object, Stream, dictionary, content::{Content, Operation}};
use std::fs;
use std::path::{Path, PathBuf};

fn get_workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn create_simple_pdf(text: &str) -> Document {
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
            Operation::new("Tf", vec!["F1".into(), 14.into()]),
            Operation::new("Td", vec![50.into(), 700.into()]),
            Operation::new("Tj", vec![Object::string_literal(text)]),
            Operation::new("ET", vec![]),
        ],
    };
    let content_bytes = content.encode().unwrap();
    let content_id = doc.add_object(Stream::new(dictionary!{}, content_bytes));

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

    doc.objects.insert(pages_id, Object::Dictionary(dictionary! {
        "Type" => "Pages",
        "Kids" => vec![page_id.into()],
        "Count" => 1,
    }));

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);
    doc
}

fn create_multi_page_pdf(pages_text: &[&str]) -> Document {
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
        let content_id = doc.add_object(Stream::new(dictionary!{}, content_bytes));

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
    doc.objects.insert(pages_id, Object::Dictionary(dictionary! {
        "Type" => "Pages",
        "Kids" => kids,
        "Count" => count,
    }));

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);
    doc
}

fn create_form_fields_pdf() -> Document {
    // Basic form fields PDF
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
            Operation::new("Tf", vec!["F1".into(), 14.into()]),
            Operation::new("Td", vec![50.into(), 700.into()]),
            Operation::new("Tj", vec![Object::string_literal("Form Document")]),
            Operation::new("ET", vec![]),
        ],
    };
    let content_bytes = content.encode().unwrap();
    let content_id = doc.add_object(Stream::new(dictionary!{}, content_bytes));

    let field_id = doc.add_object(dictionary! {
        "Type" => "Annot",
        "Subtype" => "Widget",
        "FT" => "Tx",
        "T" => Object::string_literal("TextField1"),
        "V" => Object::string_literal("Value1"),
        "Rect" => vec![100.into(), 100.into(), 200.into(), 120.into()],
    });

    let checkbox_id = doc.add_object(dictionary! {
        "Type" => "Annot",
        "Subtype" => "Widget",
        "FT" => "Btn",
        "T" => Object::string_literal("Checkbox1"),
        "V" => Object::Name(b"Yes".to_vec()),
        "Rect" => vec![100.into(), 150.into(), 120.into(), 170.into()],
    });

    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Contents" => content_id,
        "Resources" => dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            }
        },
        "Annots" => vec![field_id.into(), checkbox_id.into()]
    });

    doc.objects.insert(pages_id, Object::Dictionary(dictionary! {
        "Type" => "Pages",
        "Kids" => vec![page_id.into()],
        "Count" => 1,
    }));

    let acroform_id = doc.add_object(dictionary! {
        "Fields" => vec![field_id.into(), checkbox_id.into()]
    });

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
        "AcroForm" => acroform_id,
    });
    doc.trailer.set("Root", catalog_id);
    doc
}

fn create_encrypted_pdf(output_path: &Path) {
    // Using a known-good minimum encrypted PDF hex dump for password "test123"
    // that lopdf can successfully load and decrypt, instead of trying to manually construct the encryption
    // dictionary, since lopdf does not support generating encrypted documents.
    let encrypted_pdf_hex = "255044462d312e340a25e2e3cfd30a312030206f626a0a3c3c202f54797065202f436174616c6f67202f5061676573203220302052203e3e0a656e646f626a0a322030206f626a0a3c3c202f54797065202f5061676573202f4b696473205b203320302052205d202f436f756e742031203e3e0a656e646f626a0a332030206f626a0a3c3c202f54797065202f50616765202f506172656e74203220302052202f4d65646961426f78205b203020302036313220373932205d202f5265736f7572636573203c3c202f466f6e74203c3c202f4631203420302052203e3e203e3e202f436f6e74656e7473203520302052203e3e0a656e646f626a0a342030206f626a0a3c3c202f54797065202f466f6e74202f53756274797065202f5479706531202f42617365466f6e74202f48656c766574696361203e3e0a656e646f626a0a352030206f626a0a3c3c202f4c656e677468203531202f46696c746572202f466c6174654465636f6465203e3e0a73747265616d0a789c0b115428e17208315208762956c84c2d2e4d5548cecf2bc94c4bc50200880309990a656e6473747265616d0a656e646f626a0a362030206f626a0a3c3c202f46696c746572202f5374616e64617264202f562031202f522032202f4f203c6239333333333633643761376536313636616439396264383164393661666666623631623939333363333139363964343e202f55203c38663435646162623337346261363236306533376264333038616238386364373e202f50202d34203e3e0a656e646f626a0a787265660a3020370a303030303030303030302036353533352066200a30303030303030303135203030303030206e200a30303030303030303734203030303030206e200a30303030303030313333203030303030206e200a30303030303030323737203030303030206e200a30303030303030333635203030303030206e200a30303030303030343736203030303030206e200a747261696c65720a3c3c202f53697a652037202f526f6f74203120302052202f456e6372797074203620302052202f4944205b3c37336666666133643534346430383765353866386562306138656138376562343e3c37336666666133643534346430383765353866386562306138656138376562343e5d203e3e0a7374617274787265660a3637370a2525454f460a";
    let decoded = (0..encrypted_pdf_hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&encrypted_pdf_hex[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    fs::write(output_path, decoded).unwrap();
}

fn create_scanned_pdf() -> Document {
    // Just create a PDF with an image dictionary (empty stream is fine for a stub scanned PDF)
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();

    let image_stream = Stream::new(dictionary! {
        "Type" => "XObject",
        "Subtype" => "Image",
        "Width" => 100,
        "Height" => 100,
        "ColorSpace" => "DeviceRGB",
        "BitsPerComponent" => 8,
    }, vec![255; 100 * 100 * 3]); // dummy white image
    let image_id = doc.add_object(image_stream);

    let content = Content {
        operations: vec![
            Operation::new("q", vec![]),
            Operation::new("cm", vec![100.into(), 0.into(), 0.into(), 100.into(), 0.into(), 0.into()]),
            Operation::new("Do", vec!["Im1".into()]),
            Operation::new("Q", vec![]),
        ],
    };
    let content_bytes = content.encode().unwrap();
    let content_id = doc.add_object(Stream::new(dictionary!{}, content_bytes));

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

    doc.objects.insert(pages_id, Object::Dictionary(dictionary! {
        "Type" => "Pages",
        "Kids" => vec![page_id.into()],
        "Count" => 1,
    }));

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);
    doc
}

fn create_large_text_pdf() -> Document {
    let lorem_ipsum = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(10);
    let pages = vec![lorem_ipsum.as_str(); 10];
    create_multi_page_pdf(&pages)
}

fn main() {
    let workspace = get_workspace_root();

    let fixtures_dir = workspace.join("tests").join("fixtures");
    fs::create_dir_all(&fixtures_dir).unwrap();

    let paperpilot_fixtures_dir = workspace.join("paperpilot-pdf").join("tests").join("fixtures");
    fs::create_dir_all(&paperpilot_fixtures_dir).unwrap();

    // A. tests/fixtures/simple.pdf
    let mut simple_pdf = create_simple_pdf("Dummy text. Lorem ipsum dolor sit amet");
    simple_pdf.save(fixtures_dir.join("simple.pdf")).unwrap();

    // B. tests/fixtures/multi_page.pdf
    let mut multi_page_pdf = create_multi_page_pdf(&["Page 1", "Page 2", "Page 3"]);
    multi_page_pdf.save(fixtures_dir.join("multi_page.pdf")).unwrap();

    // C. tests/fixtures/encrypted.pdf
    create_encrypted_pdf(&fixtures_dir.join("encrypted.pdf"));

    // D. tests/fixtures/malformed.pdf
    fs::write(fixtures_dir.join("malformed.pdf"), b"%PDF-1.4\n1 0 obj\n<<CORRUPT GARBAGE HERE>>\nxref\n%%EOF").unwrap();

    // E. tests/fixtures/scanned.pdf
    let mut scanned_pdf = create_scanned_pdf();
    scanned_pdf.save(fixtures_dir.join("scanned.pdf")).unwrap();

    // F. tests/fixtures/form_fields.pdf
    let mut form_fields_pdf = create_form_fields_pdf();
    form_fields_pdf.save(fixtures_dir.join("form_fields.pdf")).unwrap();

    // G. tests/fixtures/large_text.pdf
    let mut large_text_pdf = create_large_text_pdf();
    large_text_pdf.save(fixtures_dir.join("large_text.pdf")).unwrap();

    // H. paperpilot-pdf/tests/fixtures/sample.pdf
    let mut sample_pdf = create_simple_pdf("Sample PDF content");
    sample_pdf.save(paperpilot_fixtures_dir.join("sample.pdf")).unwrap();
}
