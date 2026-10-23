use lopdf::Document;
use paperpilot_core::error::PdfError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiScanOptions {
    #[serde(default = "default_true")]
    pub scan_ssn: bool,
    #[serde(default = "default_true")]
    pub scan_credit_cards: bool,
    #[serde(default = "default_true")]
    pub scan_emails: bool,
    #[serde(default = "default_true")]
    pub scan_phones: bool,
    #[serde(default = "default_true")]
    pub scan_iban: bool,
}

fn default_true() -> bool {
    true
}

impl Default for PiiScanOptions {
    fn default() -> Self {
        Self {
            scan_ssn: true,
            scan_credit_cards: true,
            scan_emails: true,
            scan_phones: true,
            scan_iban: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PiiFinding {
    pub category: String,
    pub value_masked: String,
    pub page: u32,
    pub confidence: f32,
    pub context_snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiScanReport {
    pub total_findings: usize,
    pub findings_by_category: HashMap<String, usize>,
    pub findings: Vec<PiiFinding>,
    pub execution_time_ms: f64,
}

fn validate_luhn(number: &str) -> bool {
    let digits: Vec<u32> = number.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() < 13 || digits.len() > 19 {
        return false;
    }
    let mut sum = 0;
    let mut alternate = false;
    for &digit in digits.iter().rev() {
        if alternate {
            let mut d = digit * 2;
            if d > 9 {
                d -= 9;
            }
            sum += d;
        } else {
            sum += digit;
        }
        alternate = !alternate;
    }
    sum % 10 == 0
}

fn extract_context(text: &str, start: usize, end: usize, ctx_len: usize) -> String {
    let s = if start > ctx_len {
        let mut idx = start - ctx_len;
        while idx > 0 && !text.is_char_boundary(idx) {
            idx -= 1;
        }
        idx
    } else {
        0
    };
    let e = if end + ctx_len < text.len() {
        let mut idx = end + ctx_len;
        while idx < text.len() && !text.is_char_boundary(idx) {
            idx += 1;
        }
        idx
    } else {
        text.len()
    };
    text[s..e].replace('\n', " ")
}

pub fn scan_pii(input_bytes: &[u8], options: &PiiScanOptions) -> Result<PiiScanReport, PdfError> {
    let start = std::time::Instant::now();
    let doc = Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;

    let mut findings = Vec::new();
    let mut findings_by_category: HashMap<String, usize> = HashMap::new();

    let ssn_re = regex::Regex::new(r"\b(\d{3})-(\d{2})-(\d{4})\b").unwrap();
    let cc_re = regex::Regex::new(r"\b(?:\d[ -]*?){13,19}\b").unwrap();
    let email_re =
        regex::Regex::new(r"\b([a-zA-Z0-9._%+-]+)@([a-zA-Z0-9.-]+\.[a-zA-Z]{2,})\b").unwrap();
    let phone_re =
        regex::Regex::new(r"\b(\+?1?[-.\s]?)?(\()?\d{3}(\))?[-.\s]?\d{3}[-.\s]?\d{4}\b").unwrap();
    let iban_re = regex::Regex::new(r"\b[A-Z]{2}[0-9]{2}(?:[ ]?[a-zA-Z0-9]){11,28}\b").unwrap();

    let pages = doc.get_pages();
    let mut sorted_pages: Vec<u32> = pages.keys().copied().collect();
    sorted_pages.sort_unstable();

    for page_num in sorted_pages {
        let page_text = doc.extract_text(&[page_num]).unwrap_or_default();
        if page_text.trim().is_empty() {
            continue;
        }

        if options.scan_ssn {
            for cap in ssn_re.captures_iter(&page_text) {
                let m = cap.get(0).unwrap();
                let last4 = cap.get(3).unwrap().as_str();
                findings.push(PiiFinding {
                    category: "ssn".to_string(),
                    value_masked: format!("XXX-XX-{}", last4),
                    page: page_num,
                    confidence: 0.99,
                    context_snippet: extract_context(&page_text, m.start(), m.end(), 20),
                });
                *findings_by_category.entry("ssn".to_string()).or_insert(0) += 1;
            }
        }

        if options.scan_credit_cards {
            for m in cc_re.find_iter(&page_text) {
                let s = m.as_str();
                let clean_s: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
                if validate_luhn(&clean_s) {
                    let last4 = if clean_s.len() >= 4 {
                        &clean_s[clean_s.len() - 4..]
                    } else {
                        &clean_s
                    };
                    findings.push(PiiFinding {
                        category: "credit_card".to_string(),
                        value_masked: format!("**** **** **** {}", last4),
                        page: page_num,
                        confidence: 0.95,
                        context_snippet: extract_context(&page_text, m.start(), m.end(), 20),
                    });
                    *findings_by_category
                        .entry("credit_card".to_string())
                        .or_insert(0) += 1;
                }
            }
        }

        if options.scan_emails {
            for cap in email_re.captures_iter(&page_text) {
                let m = cap.get(0).unwrap();
                let user = cap.get(1).unwrap().as_str();
                let domain = cap.get(2).unwrap().as_str();
                let masked_user = if user.len() > 1 {
                    format!("{}***{}", &user[..1], &user[user.len() - 1..])
                } else {
                    "***".to_string()
                };
                findings.push(PiiFinding {
                    category: "email".to_string(),
                    value_masked: format!("{}@{}", masked_user, domain),
                    page: page_num,
                    confidence: 0.98,
                    context_snippet: extract_context(&page_text, m.start(), m.end(), 20),
                });
                *findings_by_category.entry("email".to_string()).or_insert(0) += 1;
            }
        }

        if options.scan_phones {
            for m in phone_re.find_iter(&page_text) {
                let s = m.as_str();
                if ssn_re.is_match(s) {
                    continue;
                }

                let clean_s: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
                if clean_s.len() >= 10 {
                    let last4 = &clean_s[clean_s.len() - 4..];
                    findings.push(PiiFinding {
                        category: "phone".to_string(),
                        value_masked: format!("(***) ***-{}", last4),
                        page: page_num,
                        confidence: 0.85,
                        context_snippet: extract_context(&page_text, m.start(), m.end(), 20),
                    });
                    *findings_by_category.entry("phone".to_string()).or_insert(0) += 1;
                }
            }
        }

        if options.scan_iban {
            for m in iban_re.find_iter(&page_text) {
                let s = m.as_str();
                let len = s.len();
                let masked = if len >= 8 {
                    let country = &s[..2];
                    let last4 = &s[len - 4..];
                    format!("{}** **** **** **** **{}", country, last4)
                } else {
                    "****".to_string()
                };
                findings.push(PiiFinding {
                    category: "iban".to_string(),
                    value_masked: masked,
                    page: page_num,
                    confidence: 0.80,
                    context_snippet: extract_context(&page_text, m.start(), m.end(), 20),
                });
                *findings_by_category.entry("iban".to_string()).or_insert(0) += 1;
            }
        }
    }

    let report = PiiScanReport {
        total_findings: findings.len(),
        findings_by_category,
        findings,
        execution_time_ms: start.elapsed().as_secs_f64() * 1000.0,
    };

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_pdf_with_text(text: &str) -> Vec<u8> {
        use lopdf::content::{Content, Operation};
        use lopdf::{Document, Object, Stream, StringFormat, dictionary};

        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        });
        let resources_id = doc.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });
        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F1".into(), 12.into()]),
                Operation::new("Td", vec![100.into(), 200.into()]),
                Operation::new(
                    "Tj",
                    vec![Object::String(
                        text.as_bytes().to_vec(),
                        StringFormat::Literal,
                    )],
                ),
                Operation::new("ET", vec![]),
            ],
        };
        let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            "Resources" => resources_id,
        });
        doc.objects.insert(
            pages_id,
            dictionary! {
                "Type" => "Pages",
                "Count" => 1,
                "Kids" => vec![page_id.into()],
            }
            .into(),
        );
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);

        let mut out = Vec::new();
        doc.save_to(&mut out).unwrap();
        out
    }

    #[test]
    fn test_scan_pii_no_findings() {
        let pdf = create_test_pdf_with_text("This is a clean document without any sensitive data.");
        let options = PiiScanOptions::default();
        let report = scan_pii(&pdf, &options).unwrap();
        assert_eq!(report.total_findings, 0);
    }

    #[test]
    fn test_scan_pii_ssn() {
        let pdf = create_test_pdf_with_text("My SSN is 123-45-6789.");
        let options = PiiScanOptions::default();
        let report = scan_pii(&pdf, &options).unwrap();
        assert_eq!(report.total_findings, 1);
        assert_eq!(report.findings[0].category, "ssn");
        assert_eq!(report.findings[0].value_masked, "XXX-XX-6789");
    }

    #[test]
    fn test_scan_pii_credit_card() {
        let pdf = create_test_pdf_with_text("Please charge card 4242-4242-4242-4242 tomorrow.");
        let options = PiiScanOptions::default();
        let report = scan_pii(&pdf, &options).unwrap();
        assert_eq!(report.total_findings, 1);
        assert_eq!(report.findings[0].category, "credit_card");
        assert_eq!(report.findings[0].value_masked, "**** **** **** 4242");
    }

    #[test]
    fn test_scan_pii_email() {
        let pdf = create_test_pdf_with_text("Contact us at test.user@example.com for more info.");
        let options = PiiScanOptions::default();
        let report = scan_pii(&pdf, &options).unwrap();
        assert_eq!(report.total_findings, 1);
        assert_eq!(report.findings[0].category, "email");
        assert_eq!(report.findings[0].value_masked, "t***r@example.com");
    }

    #[test]
    fn test_scan_pii_invalid_buffer() {
        let bad_pdf = vec![0, 1, 2, 3];
        let options = PiiScanOptions::default();
        let res = scan_pii(&bad_pdf, &options);
        assert!(res.is_err());
    }
}
