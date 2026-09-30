use paperpilot_nlp::intent::Intent;
use paperpilot_nlp::resolver::OfflineNlpResolver;
use paperpilot_nlp::traits::{NlpError, NlpResolver};

struct TestCase {
    query: &'static str,
    expected_intent: Intent,
    expected_file_count: Option<usize>,
    expected_page_ranges: Option<Vec<&'static str>>,
    expected_angles: Option<Vec<i32>>,
    expected_passwords: Option<Vec<&'static str>>,
}

impl TestCase {
    fn new(query: &'static str, expected_intent: Intent) -> Self {
        Self {
            query,
            expected_intent,
            expected_file_count: None,
            expected_page_ranges: None,
            expected_angles: None,
            expected_passwords: None,
        }
    }

    fn files(mut self, count: usize) -> Self {
        self.expected_file_count = Some(count);
        self
    }

    fn pages(mut self, ranges: Vec<&'static str>) -> Self {
        self.expected_page_ranges = Some(ranges);
        self
    }

    fn angles(mut self, angles: Vec<i32>) -> Self {
        self.expected_angles = Some(angles);
        self
    }

    fn passwords(mut self, passwords: Vec<&'static str>) -> Self {
        self.expected_passwords = Some(passwords);
        self
    }
}

#[test]
fn test_offline_resolver_accuracy() {
    let cases = vec![
        TestCase::new("Please merge document A.pdf and B.pdf into output.pdf", Intent::Merge).files(3),
        TestCase::new("Extract pages 1-5 from my_file.pdf", Intent::Extract).pages(vec!["1-5"]).files(1),
        TestCase::new("Rotate this scanned.pdf by 90 degrees", Intent::Rotate).angles(vec![90]).files(1),
        TestCase::new("Compress financial_report.pdf", Intent::Compress).files(1),
        TestCase::new("encrypt secret.pdf. password is '12345'", Intent::Encrypt).passwords(vec!["12345"]).files(1),
        TestCase::new("Remove pages 2,4,6 from draft.pdf", Intent::Delete).pages(vec!["2,4,6"]).files(1),
        TestCase::new("convert images image.png to PDF", Intent::ImagesToPdf).files(1),
        TestCase::new("split large_book.pdf", Intent::Split).files(1),
        TestCase::new("Add a watermark to confidential.pdf", Intent::Watermark).files(1),
        TestCase::new("Redact sensitive info from contract.pdf", Intent::Redact).files(1),
        TestCase::new("crop margins from photo.pdf", Intent::Crop).files(1),
        TestCase::new("unlock protected.pdf", Intent::Decrypt).files(1),
        TestCase::new("get photos out of vacation.pdf", Intent::ExtractImages).files(1),
        TestCase::new("grab text from article.pdf", Intent::ExtractText).files(1),
        TestCase::new("fill out form application.pdf", Intent::FormFill).files(1),
        TestCase::new("read fields from survey.pdf", Intent::FormRead).files(1),
        TestCase::new("flatten annotations in review.pdf", Intent::Flatten).files(1),
        TestCase::new("calculate hash of download.pdf", Intent::Hash).files(1),
        TestCase::new("add page numbers to thesis.pdf", Intent::HeaderFooter).files(1),
        TestCase::new("optimize for web brochure.pdf", Intent::Linearize).files(1),
        TestCase::new("view document info of report.pdf", Intent::Metadata).files(1),
        TestCase::new("ocr scanned_receipt.pdf", Intent::Ocr).files(1),
        TestCase::new("convert to pdf/a archive.pdf", Intent::PdfA).files(1),
        TestCase::new("render presentation.pdf to image", Intent::Render).files(1),
        TestCase::new("reorder pages in messy.pdf", Intent::Reorder).files(1),
        TestCase::new("repair broken.pdf", Intent::Repair).files(1),
        TestCase::new("find 'revenue' in financials.pdf", Intent::Search).files(1),
        TestCase::new("add digital signature to contract.pdf", Intent::Sign).files(1),
        TestCase::new("verify pdf conformance for strict.pdf", Intent::Validate).files(1),
        TestCase::new("detect type of unknown.pdf", Intent::Classify).files(1),
    ];

    let resolver = OfflineNlpResolver::new();
    let mut failed = vec![];

    for case in cases {
        let result = resolver.resolve(case.query);
        match result {
            Ok(plan) => {
                let mut case_failed = false;
                if plan.intent != case.expected_intent {
                    failed.push(format!("Query: '{}'\nExpected Intent: {:?}, Got: {:?}", case.query, case.expected_intent, plan.intent));
                    case_failed = true;
                }

                if !case_failed {
                    if let Some(expected_count) = case.expected_file_count {
                        let actual_count = plan.input_files.len() + plan.output_file.as_ref().map_or(0, |_| 1);
                        if actual_count != expected_count {
                            failed.push(format!("Query: '{}'\nExpected file count: {}, Got: {} (inputs: {:?}, output: {:?})", case.query, expected_count, actual_count, plan.input_files, plan.output_file));
                        }
                    }
                    if let Some(expected_pages) = case.expected_page_ranges {
                        if plan.page_ranges != expected_pages {
                            failed.push(format!("Query: '{}'\nExpected pages: {:?}, Got: {:?}", case.query, expected_pages, plan.page_ranges));
                        }
                    }
                    if let Some(expected_angles) = case.expected_angles {
                        if plan.angles != expected_angles {
                            failed.push(format!("Query: '{}'\nExpected angles: {:?}, Got: {:?}", case.query, expected_angles, plan.angles));
                        }
                    }
                    if let Some(expected_passwords) = case.expected_passwords {
                        if plan.passwords != expected_passwords {
                            failed.push(format!("Query: '{}'\nExpected passwords: {:?}, Got: {:?}", case.query, expected_passwords, plan.passwords));
                        }
                    }
                }
            }
            Err(e) => {
                failed.push(format!("Query: '{}'\nFailed with error: {:?}", case.query, e));
            }
        }
    }

    if !failed.is_empty() {
        panic!("{} tests failed:\n\n{}", failed.len(), failed.join("\n\n"));
    }
}

#[test]
fn test_offline_resolver_failures() {
    let resolver = OfflineNlpResolver::new();

    // Ambiguous Intent
    let res1 = resolver.resolve("make the font bigger");
    assert!(matches!(res1, Err(NlpError::AmbiguousIntent(_))), "Expected AmbiguousIntent, got {:?}", res1);

    // Missing Parameters
    let res2 = resolver.resolve("rotate");
    assert!(matches!(res2, Err(NlpError::MissingParameters(_))), "Expected MissingParameters, got {:?}", res2);

    let res3 = resolver.resolve("merge");
    assert!(matches!(res3, Err(NlpError::MissingParameters(_))), "Expected MissingParameters, got {:?}", res3);

    let res4 = resolver.resolve("extract");
    assert!(matches!(res4, Err(NlpError::MissingParameters(_))), "Expected MissingParameters, got {:?}", res4);
}
