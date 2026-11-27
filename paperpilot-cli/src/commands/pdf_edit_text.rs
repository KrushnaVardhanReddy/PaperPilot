use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::operations::pdf_edit_text::{PdfEditTextOperation, PdfEditTextOptions};
use std::path::PathBuf;

pub fn handle_pdf_edit_text_command(
    input: &PathBuf,
    output: &PathBuf,
    find: &str,
    replace: &str,
    page: Option<u32>,
    occurrence: Option<u32>,
    match_case: Option<bool>,
) -> OperationResult<()> {
    let mut doc = super::load_document(input)?;
    let options = PdfEditTextOptions {
        find: find.to_string(),
        replace: replace.to_string(),
        page,
        match_case,
        occurrence,
    };

    PdfEditTextOperation::new(options).execute(&mut doc)?;

    if let Some(parent) = output.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).map_err(PdfError::IoError)?;
    }

    doc.save(output)?;
    println!("Edited PDF text saved to {}", output.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::content::{Content, Operation};
    use lopdf::{Dictionary, Document, Object, Stream, dictionary};

    #[test]
    fn test_cli_handle_pdf_edit_text_command_roundtrip() {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        });
        let resources_id = doc.add_object(dictionary! {
            "Font" => dictionary! { "F1" => font_id },
        });

        let p1 = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), Object::Real(12.0)]),
                Operation::new("Tj", vec![Object::string_literal("CLI Typoo page 1")]),
                Operation::new("ET", vec![]),
            ],
        };
        let p2 = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), Object::Real(12.0)]),
                Operation::new(
                    "Tj",
                    vec![Object::string_literal("CLI Typoo one and Typoo two page 2")],
                ),
                Operation::new("ET", vec![]),
            ],
        };
        let c1_id = doc.add_object(Stream::new(Dictionary::new(), p1.encode().unwrap()));
        let c2_id = doc.add_object(Stream::new(Dictionary::new(), p2.encode().unwrap()));
        let p1_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Resources" => resources_id,
            "Contents" => c1_id,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        });
        let p2_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Resources" => resources_id,
            "Contents" => c2_id,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        });
        doc.objects.insert(
            pages_id,
            Object::Dictionary(dictionary! {
                "Type" => "Pages",
                "Kids" => vec![p1_id.into(), p2_id.into()],
                "Count" => 2,
            }),
        );
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);

        let dir = tempfile::tempdir().unwrap();
        let in_path = dir.path().join("cli_in.pdf");
        let out_path = dir.path().join("nested/cli_out.pdf");
        doc.save(&in_path).unwrap();

        handle_pdf_edit_text_command(
            &in_path,
            &out_path,
            "Typoo",
            "Typo",
            Some(2),
            Some(1),
            Some(true),
        )
        .unwrap();

        let edited = Document::load(&out_path).unwrap();
        assert!(edited.extract_text(&[1]).unwrap().contains("CLI Typoo page 1"));
        assert!(
            edited
                .extract_text(&[2])
                .unwrap()
                .contains("CLI Typo one and Typoo two page 2")
        );
    }
}
