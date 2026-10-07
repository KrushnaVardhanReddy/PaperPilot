use crate::entities::ExtractedEntities;
use crate::intent::Intent;
use crate::traits::NlpError;

/// Check if we have enough extracted entities to proceed with the intent.
/// Returns Ok(()) if sufficient, or Err(NlpError::MissingParameters) if not.
pub fn check_completeness(intent: &Intent, entities: &ExtractedEntities) -> Result<(), NlpError> {
    // Operations that require at least one input file:
    let needs_file = matches!(
        intent,
        Intent::Merge
            | Intent::Split
            | Intent::Rotate
            | Intent::Extract
            | Intent::Delete
            | Intent::Reorder
            | Intent::Encrypt
            | Intent::Decrypt
            | Intent::Compress
            | Intent::Watermark
            | Intent::Redact
            | Intent::ExtractText
            | Intent::ExtractImages
            | Intent::Validate
            | Intent::Repair
    );

    if needs_file && entities.files.is_empty() {
        return Err(NlpError::MissingParameters(format!(
            "Operation '{:?}' requires at least one PDF file path. Example: 'rotate my_file.pdf 90 degrees'",
            intent
        )));
    }

    // Rotation requires an angle
    if matches!(intent, Intent::Rotate) && entities.angles.is_empty() {
        return Err(NlpError::MissingParameters(
            "Rotate operation requires an angle (90, 180, or 270).".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::ExtractedEntities;
    use crate::intent::Intent;

    #[test]
    fn test_merge_with_files_ok() {
        let intent = Intent::Merge;
        let entities = ExtractedEntities {
            files: vec!["a.pdf".to_string(), "b.pdf".to_string()],
            ..Default::default()
        };
        assert!(check_completeness(&intent, &entities).is_ok());
    }

    #[test]
    fn test_merge_no_files_err() {
        let intent = Intent::Merge;
        let entities = ExtractedEntities {
            files: vec![],
            ..Default::default()
        };
        let res = check_completeness(&intent, &entities);
        assert!(res.is_err());
        if let Err(NlpError::MissingParameters(msg)) = res {
            assert!(msg.contains("requires at least one PDF file path"));
        } else {
            panic!("Expected MissingParameters error");
        }
    }

    #[test]
    fn test_rotate_no_angle_err() {
        let intent = Intent::Rotate;
        let entities = ExtractedEntities {
            files: vec!["a.pdf".to_string()],
            ..Default::default()
        };
        let res = check_completeness(&intent, &entities);
        assert!(res.is_err());
        if let Err(NlpError::MissingParameters(msg)) = res {
            assert!(msg.contains("requires an angle"));
        } else {
            panic!("Expected MissingParameters error");
        }
    }

    #[test]
    fn test_rotate_with_angle_ok() {
        let intent = Intent::Rotate;
        let entities = ExtractedEntities {
            files: vec!["a.pdf".to_string()],
            angles: vec![90],
            ..Default::default()
        };
        assert!(check_completeness(&intent, &entities).is_ok());
    }
}
