use paperpilot_core::error::PdfError;
use paperpilot_pdf::operations::pii::{PiiScanOptions, scan_pii};
use std::path::PathBuf;

pub fn handle_pii_scan(
    input: &PathBuf,
    output: &Option<PathBuf>,
    scan_ssn: bool,
    scan_credit_cards: bool,
    scan_emails: bool,
    scan_phones: bool,
    scan_iban: bool,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;
    let options = PiiScanOptions {
        scan_ssn,
        scan_credit_cards,
        scan_emails,
        scan_phones,
        scan_iban,
    };

    let report = scan_pii(&input_bytes, &options)?;
    let json = serde_json::to_string_pretty(&report).unwrap_or_default();

    if let Some(out_path) = output {
        std::fs::write(out_path, json).map_err(PdfError::IoError)?;
        println!("PII Scan complete. Report saved to {:?}", out_path);
    } else {
        println!("{}", json);
    }

    Ok(())
}
