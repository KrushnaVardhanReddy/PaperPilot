use std::fs;
use std::io::Write;

pub fn save_pii_report(output_path: &str, report_json: &str) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(output_path)?;
    file.write_all(report_json.as_bytes())?;
    file.flush()?;
    Ok(())
}
