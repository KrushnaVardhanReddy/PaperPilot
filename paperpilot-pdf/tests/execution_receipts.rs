#[path = "support/workflow_fixture.rs"]
mod fixture;
use fixture::{PageKind::*, pdf};
use paperpilot_core::workflow::parse_workflow;
use paperpilot_pdf::operations::receipts;
use std::fs;

#[test]
fn terminal_receipts_survive_reopening_and_match_real_pdf_artifacts() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("result.pdf");
    let source = pdf(&[Text, Text]);
    fs::write(&input, &source).unwrap();
    let workflow = parse_workflow(
        "name='Rotate'\n[[stage]]\nid='rotate'\nop='rotate'\n[stage.params]\nangle=90",
    )
    .unwrap();
    let run = receipts::run_to_path(&input, &output, &workflow).unwrap();
    run.outcome().unwrap();
    let document = lopdf::Document::load(&output).unwrap();
    assert_eq!(document.get_pages().len(), 2);
    assert!(
        document
            .extract_text(&[1])
            .unwrap()
            .contains("Original invoice")
    );
    let folder = run.receipt_path.parent().unwrap().to_path_buf();
    let id = run.receipt_path.file_name().unwrap().to_str().unwrap();
    let entry = receipts::get(&folder, id).unwrap();
    assert_eq!(entry.job_id.as_deref(), Some(run.receipt.job_id.as_str()));
    assert!(entry.success);
    assert_eq!(entry.input_sha256.as_ref().unwrap().len(), 64);
    assert_eq!(entry.output_sha256.as_ref().unwrap().len(), 64);
    drop(run);
    assert_eq!(receipts::list(&folder, 50).unwrap().len(), 1);
    assert_eq!(fs::read(input).unwrap(), source);
}
#[test]
fn failed_stage_and_output_write_failure_are_durable_without_publishing_partial_pdf() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("in.pdf");
    let output = dir.path().join("out.pdf");
    let previous = pdf(&[Image]);
    fs::write(&input, pdf(&[Text])).unwrap();
    fs::write(&output, &previous).unwrap();
    let workflow=parse_workflow("name='Fail'\n[[stage]]\nid='gate'\nop='rotate'\ncondition={if_page_count_gt=50}\non_no_match='error'").unwrap();
    let run = receipts::run_to_path(&input, &output, &workflow).unwrap();
    assert!(run.outcome().is_err());
    let entry = receipts::get(
        run.receipt_path.parent().unwrap(),
        run.receipt_path.file_name().unwrap().to_str().unwrap(),
    )
    .unwrap();
    assert_eq!(entry.failed_stage.as_deref(), Some("gate"));
    assert!(entry.output_sha256.is_none());
    assert_eq!(fs::read(&output).unwrap(), previous);
    let good = parse_workflow("name='Good'\n[[stage]]\nid='r'\nop='rotate'").unwrap();
    let target = dir.path().join("directory.pdf");
    fs::create_dir(&target).unwrap();
    let run = receipts::run_to_path(&input, &target, &good).unwrap();
    assert!(!run.receipt.success);
    assert!(run.receipt.report.unwrap().error.is_some());
    assert!(run.receipt.output.is_none());
}
#[test]
fn history_rejects_traversal_and_corruption_and_does_not_store_passwords() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("in.pdf");
    let output = dir.path().join("out.pdf");
    fs::write(&input, pdf(&[Text])).unwrap();
    let wf=parse_workflow("name='Private'\n[[stage]]\nid='enc'\nop='encrypt'\n[stage.params]\npassword='do-not-store-password'").unwrap();
    let run = receipts::run_to_path(&input, &output, &wf).unwrap();
    run.outcome().unwrap();
    let bytes = fs::read(&run.receipt_path).unwrap();
    assert!(!String::from_utf8_lossy(&bytes).contains("do-not-store-password"));
    assert!(lopdf::Document::load(output).unwrap().is_encrypted());
    let folder = run.receipt_path.parent().unwrap();
    assert!(receipts::get(folder, "../outside.json").is_err());
    assert!(receipts::list(folder, 0).is_err());
    fs::write(&run.receipt_path, &bytes[..32]).unwrap();
    assert!(receipts::list(folder, 50).is_err());
}

#[test]
fn genuine_watcher_receipts_are_queryable_without_rewriting_them() {
    use paperpilot_pdf::operations::watch::{WatchConfig, WatchStatus, WatcherManager};
    use std::{
        sync::Arc,
        time::{Duration, Instant},
    };
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("invoice.pdf"), pdf(&[Text])).unwrap();
    let manager = WatcherManager::default();
    manager
        .start(
            "history".into(),
            WatchConfig {
                directory: dir.path().into(),
                recipe: "preset-legal-discovery".into(),
                output_dir: None,
                settle_delay_ms: 50,
                move_original: None,
                error_dir: None,
                once: true,
            },
            Arc::new(|_| {}),
        )
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while manager.list()[0].status != WatchStatus::Stopped {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(20));
    }
    let history = receipts::list(&dir.path().join("processed"), 50).unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].source_schema, "watcher/legacy");
    assert!(history[0].success);
    assert_eq!(history[0].input_sha256.as_ref().unwrap().len(), 64);
}
