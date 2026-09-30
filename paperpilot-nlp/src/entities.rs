use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_FILE: Regex = Regex::new(r#"(?i)(?:"([^"]+\.pdf)")|(?:'([^']+\.pdf)')|([a-zA-Z0-9_\-\./\\]+\.pdf)"#).unwrap();
    static ref RE_PAGES: Regex = Regex::new(r#"(?i)pages?\s+([\d,\-\s]+)|\b(\d+\s*-\s*\d+)\b"#).unwrap();
    static ref RE_ANGLE: Regex = Regex::new(r#"(?i)(?:rotate\s+)?(-?(?:90|180|270))\b(?:\s*degrees?)?"#).unwrap();
    static ref RE_PASSWORD: Regex = Regex::new(r#"(?i)(?:password is|pw)\s+(?:["']([^"']+)["']|([^\s]+))"#).unwrap();
}

#[derive(Debug, Default, PartialEq)]
pub struct ExtractedEntities {
    pub files: Vec<String>,
    pub page_ranges: Vec<String>,
    pub passwords: Vec<String>,
    pub angles: Vec<i32>,
}

pub fn extract_entities(text: &str) -> ExtractedEntities {
    let mut entities = ExtractedEntities::default();

    for cap in RE_FILE.captures_iter(text) {
        if let Some(m) = cap.get(1).or_else(|| cap.get(2)).or_else(|| cap.get(3)) {
            entities.files.push(m.as_str().to_string());
        }
    }

    for cap in RE_PAGES.captures_iter(text) {
        if let Some(m) = cap.get(1).or_else(|| cap.get(2)) {
            entities.page_ranges.push(m.as_str().trim().to_string());
        }
    }

    for cap in RE_PASSWORD.captures_iter(text) {
        if let Some(m) = cap.get(1).or_else(|| cap.get(2)) {
            entities.passwords.push(m.as_str().to_string());
        }
    }

    for cap in RE_ANGLE.captures_iter(text) {
        if let Some(m) = cap.get(1) {
            #[allow(clippy::collapsible_if)]
            if let Ok(angle) = m.as_str().parse::<i32>() {
                entities.angles.push(angle);
            }
        }
    }

    entities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_files() {
        let text = "merge a.pdf and b.pdf and \"c.pdf\" and 'd.pdf'";
        let entities = extract_entities(text);
        assert_eq!(entities.files, vec!["a.pdf", "b.pdf", "c.pdf", "d.pdf"]);

        let text = "path is /var/tmp/bar.pdf";
        let entities = extract_entities(text);
        assert_eq!(entities.files, vec!["/var/tmp/bar.pdf"]);
    }

    #[test]
    fn test_extract_page_ranges() {
        let text = "extract pages 1-5 from doc.pdf";
        let entities = extract_entities(text);
        assert_eq!(entities.page_ranges, vec!["1-5"]);

        let text = "extract page 3 and pages 2,4,6";
        let entities = extract_entities(text);
        assert_eq!(entities.page_ranges, vec!["3", "2,4,6"]);
    }

    #[test]
    fn test_extract_angles() {
        let text = "rotate 90 degrees";
        let entities = extract_entities(text);
        assert_eq!(entities.angles, vec![90]);

        let text = "rotate 180";
        let entities = extract_entities(text);
        assert_eq!(entities.angles, vec![180]);

        let text = "rotate -90";
        let entities = extract_entities(text);
        assert_eq!(entities.angles, vec![-90]);

        let text = "rotate 270 degrees";
        let entities = extract_entities(text);
        assert_eq!(entities.angles, vec![270]);
    }

    #[test]
    fn test_extract_passwords() {
        let text = "password is mysecret";
        let entities = extract_entities(text);
        assert_eq!(entities.passwords, vec!["mysecret"]);

        let text = "pw \"secret words\"";
        let entities = extract_entities(text);
        assert_eq!(entities.passwords, vec!["secret words"]);

        let text = "password is 'foo'";
        let entities = extract_entities(text);
        assert_eq!(entities.passwords, vec!["foo"]);
    }

    #[test]
    fn test_extract_mixed() {
        let text = "decrypt doc.pdf pw secret and extract pages 1-3 then rotate 90 degrees and merge with other.pdf";
        let entities = extract_entities(text);
        assert_eq!(entities.files, vec!["doc.pdf", "other.pdf"]);
        assert_eq!(entities.passwords, vec!["secret"]);
        assert_eq!(entities.page_ranges, vec!["1-3"]);
        assert_eq!(entities.angles, vec![90]);
    }
}
