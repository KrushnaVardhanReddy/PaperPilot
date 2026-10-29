#[path = "support/workflow_fixture.rs"]
mod fixture;
use chrono::{TimeZone, Utc};
use fixture::{PageKind::Text, pdf};
use paperpilot_pdf::operations::{
    output_naming::OutputNameTemplate,
    receipts,
    watch::{WatchConfig, WatchStatus, WatcherManager},
};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
#[test]
fn tokens_use_explicit_utc_date_hash_index_and_safe_input_parts() {
    let template =
        OutputNameTemplate::parse("{input_stem}-{date:%Y%m%d}-{hash:8}-{index:04}.{input_ext}")
            .unwrap();
    let name = template
        .render(
            Path::new("报价:a.pdf"),
            &"ab".repeat(32),
            Utc.with_ymd_and_hms(2026, 10, 29, 23, 59, 58).unwrap(),
            12,
        )
        .unwrap();
    assert_eq!(name, "报价_a-20261029-abababab-0012.pdf");
    assert_eq!(
        OutputNameTemplate::parse("{input_name}")
            .unwrap()
            .render(Path::new("a.pdf"), &"cd".repeat(32), Utc::now(), 1)
            .unwrap(),
        "a.pdf"
    );
    for pattern in [
        "",
        "../{input_name}",
        "C:\\{input_name}",
        "NUL.pdf",
        "{unknown}.pdf",
        "{hash:3}.pdf",
        "{hash:65}.pdf",
        "{index:999}.pdf",
        "{date:%Q}.pdf",
        "{date:%#z}.pdf",
        "{date:%n}.pdf",
        "{date:%Y/%m}.pdf",
        "{input_stem",
        "x}.pdf",
        "out.txt",
        "out.pdf ",
    ] {
        assert!(OutputNameTemplate::parse(pattern).is_err(), "{pattern}");
    }
}
#[test]
fn recipe_hash_matches_actual_input_and_collisions_persist_failure() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("invoice.pdf");
    let source = pdf(&[Text]);
    fs::write(&input, &source).unwrap();
    let wf = paperpilot_core::workflow::parse_workflow(
        "name='Name test'\n[[stage]]\nid='mark'\nop='watermark'\nparams={text='NAMED'}",
    )
    .unwrap();
    let out = dir.path().join("out");
    let pattern = "{input_stem}-{hash:8}-{index:04}.pdf";
    let run = receipts::run_to_path_with_naming(&input, &out, &wf, Some(pattern)).unwrap();
    run.outcome().unwrap();
    let target = out.join(format!("invoice-{}-0001.pdf", &digest(&source)[..8]));
    assert_eq!(run.receipt.output.as_deref(), Some(target.as_path()));
    assert_eq!(
        run.receipt.input_sha256.as_deref(),
        Some(digest(&source).as_str())
    );
    let doc = lopdf::Document::load(&target).unwrap();
    let text = doc.extract_text(&[1]).unwrap();
    assert!(text.contains("NAMED") && text.contains("Original invoice"));
    let saved = fs::read(&target).unwrap();
    let failure = receipts::run_to_path_with_naming(&input, &out, &wf, Some(pattern)).unwrap();
    assert!(failure.outcome().is_err());
    assert!(failure.receipt.output.is_none());
    assert!(failure.receipt_path.exists());
    assert_eq!(fs::read(&target).unwrap(), saved);
    assert_eq!(fs::read(&input).unwrap(), source);
    assert_eq!(
        receipts::list(&out.join(".paperpilot-receipts"), 50)
            .unwrap()
            .len(),
        2
    );
    assert!(
        receipts::run_to_path_with_naming(
            &input,
            &dir.path().join("bad"),
            &wf,
            Some("../escape.pdf")
        )
        .is_err()
    );
    assert!(!dir.path().join("bad").exists());
}

#[test]
fn metadata_names_decode_real_pdf_info_and_reject_missing_fields() {
    let dir = tempfile::tempdir().unwrap();
    let mut doc = lopdf::Document::load_mem(&pdf(&[Text])).unwrap();
    let mut utf16 = vec![0xfe, 0xff];
    for value in "发票:Q4/Acme".encode_utf16() {
        utf16.extend_from_slice(&value.to_be_bytes());
    }
    let info = doc.add_object(lopdf::dictionary! {
        "Title" => lopdf::Object::String(utf16, lopdf::StringFormat::Hexadecimal),
        "Author" => lopdf::Object::string_literal("Ada")
    });
    doc.trailer.set("Info", info);
    let input = dir.path().join("input.pdf");
    doc.save(&input).unwrap();
    let wf = paperpilot_core::workflow::get_preset_by_id("preset-legal-discovery").unwrap();
    let out = dir.path().join("out");
    let run = receipts::run_to_path_with_naming(
        &input,
        &out,
        &wf,
        Some("{metadata.title}-{metadata.author}.pdf"),
    )
    .unwrap();
    run.outcome().unwrap();
    let target = out.join("发票_Q4_Acme-Ada.pdf");
    assert_eq!(run.receipt.output.as_deref(), Some(target.as_path()));
    assert!(
        lopdf::Document::load(&target)
            .unwrap()
            .extract_text(&[1])
            .unwrap()
            .contains("Original invoice")
    );
    let missing = dir.path().join("missing");
    assert!(
        receipts::run_to_path_with_naming(&input, &missing, &wf, Some("{metadata.subject}.pdf"))
            .is_err()
    );
    assert!(!missing.exists());
    assert!(input.exists());
    assert!(OutputNameTemplate::parse("{metadata.unknown}.pdf").is_err());
}
fn config(root: &Path) -> WatchConfig {
    WatchConfig {
        directory: root.into(),
        recipe: "preset-legal-discovery".into(),
        output_dir: None,
        settle_delay_ms: 50,
        move_original: None,
        error_dir: None,
        once: true,
    }
}
fn finished(manager: &WatcherManager) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while manager.list()[0].status != WatchStatus::Stopped {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(20));
    }
}
#[test]
fn watcher_names_actual_inputs_and_keeps_original_archive_names() {
    let dir = tempfile::tempdir().unwrap();
    let source = pdf(&[Text]);
    fs::write(dir.path().join("a.pdf"), &source).unwrap();
    let manager = WatcherManager::default();
    let mut cfg = config(dir.path());
    cfg.move_original = Some(dir.path().join("archive"));
    manager
        .start_with_naming(
            "named".into(),
            cfg,
            Some("{input_stem}-{hash:8}-{index:04}.pdf".into()),
            Arc::new(|_| {}),
        )
        .unwrap();
    finished(&manager);
    let state = manager.list().remove(0);
    assert_eq!(state.succeeded, 1);
    let path = state.last_output.unwrap();
    assert_eq!(
        path.file_name().unwrap().to_str().unwrap(),
        format!("a-{}-0001.pdf", &digest(&source)[..8])
    );
    let record: serde_json::Value =
        serde_json::from_slice(&fs::read(state.last_receipt.unwrap()).unwrap()).unwrap();
    assert_eq!(record["sha256"], digest(&source));
    assert_eq!(record["output"], serde_json::json!(path));
    assert_eq!(fs::read(dir.path().join("archive/a.pdf")).unwrap(), source);
    assert!(!dir.path().join("a.pdf").exists());
}
#[test]
fn watcher_collisions_use_suffix_and_invalid_templates_start_no_worker() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("a.pdf"), pdf(&[Text])).unwrap();
    fs::write(dir.path().join("b.pdf"), pdf(&[Text, Text])).unwrap();
    let manager = WatcherManager::default();
    assert!(
        manager
            .start_with_naming(
                "bad".into(),
                config(dir.path()),
                Some("../out.pdf".into()),
                Arc::new(|_| {})
            )
            .is_err()
    );
    assert!(manager.list().is_empty());
    assert!(!dir.path().join("processed").exists());
    manager
        .start_with_naming(
            "ok".into(),
            config(dir.path()),
            Some("fixed.pdf".into()),
            Arc::new(|_| {}),
        )
        .unwrap();
    finished(&manager);
    assert_eq!(manager.list()[0].succeeded, 2);
    for name in ["fixed.pdf", "fixed-1.pdf"] {
        let path = dir.path().join("processed").join(name);
        assert!(
            lopdf::Document::load(&path)
                .unwrap()
                .extract_text(&[1])
                .unwrap()
                .contains("Original invoice")
        );
        let receipt: serde_json::Value = serde_json::from_slice(
            &fs::read(path.with_file_name(format!("{name}.receipt.json"))).unwrap(),
        )
        .unwrap();
        assert_eq!(receipt["output"], serde_json::json!(path.canonicalize().unwrap()));
    }
}

#[test]
fn missing_metadata_fails_one_watch_file_and_later_files_continue() {
    let dir = tempfile::tempdir().unwrap();
    let source = pdf(&[Text]);
    fs::write(dir.path().join("missing.pdf"), &source).unwrap();
    let mut doc = lopdf::Document::load_mem(&source).unwrap();
    let info =
        doc.add_object(lopdf::dictionary! { "Title" => lopdf::Object::string_literal("Present") });
    doc.trailer.set("Info", info);
    doc.save(dir.path().join("present.pdf")).unwrap();
    let manager = WatcherManager::default();
    manager
        .start_with_naming(
            "metadata".into(),
            config(dir.path()),
            Some("{metadata.title}.pdf".into()),
            Arc::new(|_| {}),
        )
        .unwrap();
    finished(&manager);
    let state = manager.list().remove(0);
    assert_eq!(state.succeeded, 1);
    assert_eq!(state.failed, 1);
    let failure: serde_json::Value = serde_json::from_slice(
        &fs::read(dir.path().join(".error/missing.pdf.receipt.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(failure["success"], false);
    assert!(failure["error"].as_str().unwrap().contains("metadata"));
    assert_eq!(fs::read(dir.path().join("missing.pdf")).unwrap(), source);
    assert!(lopdf::Document::load(dir.path().join("processed/Present.pdf")).is_ok());
}
