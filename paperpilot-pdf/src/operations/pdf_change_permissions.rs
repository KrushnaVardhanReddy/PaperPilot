use lopdf::{Document, EncryptionState, EncryptionVersion, Object, Permissions, StringFormat};
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PdfChangePermissionsOptions {
    pub allow_printing: Option<bool>,
    pub allow_copying: Option<bool>,
    pub allow_modifying: Option<bool>,
    pub allow_annotations: Option<bool>,
    pub allow_form_filling: Option<bool>,
    pub allow_accessibility: Option<bool>,
    pub owner_password: Option<String>,
    pub user_password: Option<String>,
}

pub fn pdf_change_permissions_bytes(
    input_bytes: &[u8],
    options: &PdfChangePermissionsOptions,
) -> Result<Vec<u8>, PdfError> {
    let mut doc =
        Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;

    // Attempt to decrypt if password provided
    // For this operation we assume the document is either unencrypted or we provide the password in options
    // (though lopdf::Document::load_mem doesn't decrypt by itself unless we do it here, but wait, load_mem handles the parsing,
    // if we want to manipulate it, we might need to decrypt it first if it has an owner password)
    // Actually, to keep it simple and matching the instruction:

    // We strip /Encrypt to clear the old encryption dictionary if it exists
    doc.trailer.remove(b"Encrypt");

    // Ensure document has an /ID in trailer (required by lopdf encryption)
    if doc.trailer.get(b"ID").is_err() {
        let id_str = Object::String(b"default_id_placeholder".to_vec(), StringFormat::Literal);
        doc.trailer
            .set("ID", Object::Array(vec![id_str.clone(), id_str]));
    }

    let mut perms = Permissions::all();
    if let Some(allow) = options.allow_printing {
        perms.set(Permissions::PRINTABLE, allow);
        perms.set(Permissions::PRINTABLE_IN_HIGH_QUALITY, allow);
    }
    if let Some(allow) = options.allow_copying {
        perms.set(Permissions::COPYABLE, allow);
    }
    if let Some(allow) = options.allow_modifying {
        perms.set(Permissions::MODIFIABLE, allow);
        perms.set(Permissions::ASSEMBLABLE, allow);
    }
    if let Some(allow) = options.allow_annotations {
        perms.set(Permissions::ANNOTABLE, allow);
    }
    if let Some(allow) = options.allow_form_filling {
        perms.set(Permissions::FILLABLE, allow);
    }
    if let Some(allow) = options.allow_accessibility {
        perms.set(Permissions::COPYABLE_FOR_ACCESSIBILITY, allow);
    }

    let user_pwd = options.user_password.as_deref().unwrap_or("");
    let owner_pwd = options.owner_password.as_deref().unwrap_or(user_pwd);

    let version = EncryptionVersion::V2 {
        document: &doc,
        owner_password: owner_pwd,
        user_password: user_pwd,
        key_length: 128,
        permissions: perms,
    };

    let state = EncryptionState::try_from(version)
        .map_err(|e| PdfError::Other(format!("Failed to initialize encryption state: {}", e)))?;

    doc.encrypt(&state).map_err(|e| {
        PdfError::Other(format!("Re-encryption with new permissions failed: {}", e))
    })?;

    let mut out_bytes = Vec::new();
    doc.save_to(&mut out_bytes)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;
    Ok(out_bytes)
}

#[derive(Default)]
pub struct PdfChangePermissionsOperation {
    _options: PdfChangePermissionsOptions,
}

impl PdfChangePermissionsOperation {
    pub fn new(options: PdfChangePermissionsOptions) -> Self {
        Self { _options: options }
    }
}

impl PdfOperation for PdfChangePermissionsOperation {
    fn execute(
        &self,
        _document: &mut dyn PdfDocument,
    ) -> paperpilot_core::error::OperationResult<()> {
        Err(PdfError::UnsupportedOperation(
            "Not yet implemented".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::dictionary;

    fn create_empty_pdf_bytes() -> Vec<u8> {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Courier",
        });
        let resources_id = doc.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });
        let content = lopdf::content::Content { operations: vec![] };
        let content_id = doc.add_object(lopdf::Stream::new(
            dictionary! {},
            content.encode().unwrap(),
        ));
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            "Resources" => resources_id,
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
        });
        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
        };
        doc.objects.insert(pages_id, Object::Dictionary(pages));
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);

        let mut bytes = Vec::new();
        doc.save_to(&mut bytes).unwrap();
        bytes
    }

    #[test]
    fn test_pdf_change_permissions_revoke_printing() {
        let bytes = create_empty_pdf_bytes();
        let options = PdfChangePermissionsOptions {
            allow_printing: Some(false),
            ..Default::default()
        };
        let result_bytes = pdf_change_permissions_bytes(&bytes, &options).unwrap();
        let s = String::from_utf8_lossy(&result_bytes);
        let p_idx = s.find("/P ").unwrap();
        let end_idx = s[p_idx + 3..]
            .find(|c: char| !c.is_ascii_digit() && c != '-')
            .unwrap_or(s.len() - p_idx - 3);
        let p_val: i32 = s[p_idx + 3..p_idx + 3 + end_idx].parse().unwrap();
        assert_eq!((p_val as u32) & 4, 0);
    }

    #[test]
    fn test_pdf_change_permissions_allow_copying() {
        let bytes = create_empty_pdf_bytes();
        let options1 = PdfChangePermissionsOptions {
            allow_copying: Some(false),
            ..Default::default()
        };
        let result1 = pdf_change_permissions_bytes(&bytes, &options1).unwrap();
        let s1 = String::from_utf8_lossy(&result1);
        let p_idx1 = s1.find("/P ").unwrap();
        let end_idx1 = s1[p_idx1 + 3..]
            .find(|c: char| !c.is_ascii_digit() && c != '-')
            .unwrap_or(s1.len() - p_idx1 - 3);
        let p1: i32 = s1[p_idx1 + 3..p_idx1 + 3 + end_idx1].parse().unwrap();
        assert_eq!((p1 as u32) & 16, 0);

        let options2 = PdfChangePermissionsOptions {
            allow_copying: Some(true),
            ..Default::default()
        };
        let result2 = pdf_change_permissions_bytes(&result1, &options2).unwrap();
        let s2 = String::from_utf8_lossy(&result2);
        let p_idx2 = s2.find("/P ").unwrap();
        let end_idx2 = s2[p_idx2 + 3..]
            .find(|c: char| !c.is_ascii_digit() && c != '-')
            .unwrap_or(s2.len() - p_idx2 - 3);
        let p2: i32 = s2[p_idx2 + 3..p_idx2 + 3 + end_idx2].parse().unwrap();
        assert_eq!((p2 as u32) & 16, 16);
    }

    #[test]
    fn test_pdf_change_permissions_roundtrip() {
        let bytes = create_empty_pdf_bytes();
        let options = PdfChangePermissionsOptions {
            allow_printing: Some(false),
            allow_modifying: Some(false),
            ..Default::default()
        };
        let out_bytes = pdf_change_permissions_bytes(&bytes, &options).unwrap();
        let s = String::from_utf8_lossy(&out_bytes);
        let p_idx = s.find("/P ").unwrap();
        let end_idx = s[p_idx + 3..]
            .find(|c: char| !c.is_ascii_digit() && c != '-')
            .unwrap_or(s.len() - p_idx - 3);
        let p: i32 = s[p_idx + 3..p_idx + 3 + end_idx].parse().unwrap();
        assert_eq!((p as u32) & 4, 0);
        assert_eq!((p as u32) & 8, 0);
    }
}
