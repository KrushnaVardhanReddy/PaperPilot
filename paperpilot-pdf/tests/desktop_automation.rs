#[path = "support/workflow_fixture.rs"]
mod fixture;
use fixture::{PageKind::*, pdf};
use paperpilot_core::traits::PdfOperation;
use paperpilot_pdf::operations::document_inspection::inspect_document_bytes;
use paperpilot_pdf::operations::watch::{WatchConfig, WatchEvent, WatchStatus, WatcherManager};
use std::path::{Path, PathBuf};
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant};

fn recipe() -> String {
    "name='Desktop QA'\n[[stage]]\nid='rotate'\nop='rotate'\n[stage.params]\nangle=90".into()
}
fn config(dir: &Path, output: Option<PathBuf>) -> WatchConfig {
    WatchConfig {
        directory: dir.into(),
        recipe: recipe(),
        output_dir: output,
        settle_delay_ms: 50,
        move_original: None,
        error_dir: None,
        once: false,
    }
}
fn next_event(rx: &mpsc::Receiver<WatchEvent>, kind: &str) -> WatchEvent {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let event = rx
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
            .expect("Real filesystem event should arrive before deadline");
        if event.kind == kind {
            return event;
        }
    }
}

#[test]
fn actual_watcher_processes_existing_and_arriving_pdfs_and_stops_cleanly() {
    let root = tempfile::tempdir().unwrap();
    let source = pdf(&[Text]);
    std::fs::write(root.path().join("initial.pdf"), &source).unwrap();
    let manager = Arc::new(WatcherManager::default());
    let (tx, rx) = mpsc::channel();
    let started = manager
        .start(
            "qa".into(),
            config(root.path(), None),
            Arc::new(move |e| {
                let _ = tx.send(e);
            }),
        )
        .unwrap();
    assert_eq!(started.status, WatchStatus::Watching);
    let first = next_event(&rx, "file_processed");
    let output = first.output.as_ref().unwrap();
    let doc = lopdf::Document::load(output).unwrap();
    assert_eq!(doc.get_pages().len(), 1);
    assert_eq!(
        doc.get_dictionary(*doc.get_pages().values().next().unwrap())
            .unwrap()
            .get(b"Rotate")
            .unwrap()
            .as_i64()
            .unwrap(),
        90
    );
    assert!(doc.extract_text(&[1]).unwrap().contains("Original invoice"));
    let receipt: serde_json::Value =
        serde_json::from_slice(&std::fs::read(first.receipt.as_ref().unwrap()).unwrap()).unwrap();
    assert_eq!(receipt["success"], true);
    assert_eq!(receipt["report"]["steps_completed"], 1);
    assert_eq!(receipt["sha256"].as_str().unwrap().len(), 64);
    assert_eq!(
        std::fs::read(root.path().join("initial.pdf")).unwrap(),
        source
    );
    std::fs::write(root.path().join("arrived.pdf"), pdf(&[Text, Text])).unwrap();
    let second = next_event(&rx, "file_processed");
    assert_eq!(second.watcher.succeeded, 2);
    let stopped = manager.stop("qa").unwrap();
    assert_eq!(stopped.status, WatchStatus::Stopped);
    std::fs::write(root.path().join("after-stop.pdf"), pdf(&[Text])).unwrap();
    std::thread::sleep(Duration::from_millis(200));
    assert!(!stopped.output_dir.join("after-stop.pdf").exists());
    assert_eq!(manager.list()[0].succeeded, 2);
    if let Ok(path) = std::env::var("PAPERPILOT_DESKTOP_EVIDENCE_DIR") {
        let path = PathBuf::from(path);
        std::fs::create_dir_all(&path).unwrap();
        let events = serde_json::json!([first, second, {"kind":"stopped","watcher":stopped}]);
        std::fs::write(
            path.join("watch-events.json"),
            serde_json::to_vec_pretty(&events).unwrap(),
        )
        .unwrap();
    }
}

#[test]
fn rejects_bad_configuration_and_duplicate_watchers_before_claiming_success() {
    let root = tempfile::tempdir().unwrap();
    let manager = WatcherManager::default();
    let mut bad = config(root.path(), None);
    bad.recipe = "missing-recipe".into();
    assert!(manager.start("bad".into(), bad, Arc::new(|_| {})).is_err());
    assert!(manager.list().is_empty());
    assert!(
        manager
            .start(
                "same".into(),
                config(root.path(), Some(root.path().into())),
                Arc::new(|_| {})
            )
            .is_err()
    );
    manager
        .start("real".into(), config(root.path(), None), Arc::new(|_| {}))
        .unwrap();
    assert!(
        manager
            .start(
                "duplicate".into(),
                config(root.path(), None),
                Arc::new(|_| {})
            )
            .is_err()
    );
    assert!(
        manager
            .start("real".into(), config(root.path(), None), Arc::new(|_| {}))
            .is_err()
    );
    manager.shutdown();
    assert!(manager.list().is_empty());
}

#[test]
fn output_write_failure_is_reported_and_does_not_stop_later_files() {
    let root = tempfile::tempdir().unwrap();
    let out = root.path().join("outputs");
    let manager = WatcherManager::default();
    let (tx, rx) = mpsc::channel();
    manager
        .start(
            "failure".into(),
            config(root.path(), Some(out.clone())),
            Arc::new(move |e| {
                let _ = tx.send(e);
            }),
        )
        .unwrap();
    // Replace only this test's empty output directory with a regular file: a real
    // filesystem failure, not an injected or mocked operation result.
    std::fs::remove_dir(&out).unwrap();
    std::fs::write(&out, b"blocked output location").unwrap();
    let source = pdf(&[Text]);
    std::fs::write(root.path().join("blocked.pdf"), &source).unwrap();
    let failure = next_event(&rx, "error");
    assert_eq!(failure.watcher.succeeded, 0);
    assert_eq!(failure.watcher.failed, 1);
    assert!(failure.error.is_some());
    let audit: serde_json::Value =
        serde_json::from_slice(&std::fs::read(failure.receipt.unwrap()).unwrap()).unwrap();
    assert_eq!(audit["success"], false);
    assert_eq!(
        std::fs::read(root.path().join("blocked.pdf")).unwrap(),
        source
    );
    std::fs::remove_file(&out).unwrap();
    std::fs::create_dir(&out).unwrap();
    std::fs::write(root.path().join("valid.pdf"), pdf(&[Text])).unwrap();
    let success = next_event(&rx, "file_processed");
    assert_eq!(success.watcher.succeeded, 1);
    assert_eq!(success.watcher.failed, 1);
    manager.stop("failure").unwrap();
}

#[test]
fn outputs_are_not_overwritten_or_recursively_reprocessed() {
    let root = tempfile::tempdir().unwrap();
    let out = root.path().join("processed");
    std::fs::create_dir(&out).unwrap();
    let existing = pdf(&[Image]);
    std::fs::write(out.join("input.pdf"), &existing).unwrap();
    let manager = WatcherManager::default();
    let (tx, rx) = mpsc::channel();
    manager
        .start(
            "collision".into(),
            config(root.path(), None),
            Arc::new(move |e| {
                let _ = tx.send(e);
            }),
        )
        .unwrap();
    std::fs::write(root.path().join("input.pdf"), pdf(&[Text])).unwrap();
    let event = next_event(&rx, "file_processed");
    assert_ne!(event.output.unwrap(), out.join("input.pdf"));
    assert_eq!(std::fs::read(out.join("input.pdf")).unwrap(), existing);
    std::thread::sleep(Duration::from_millis(250));
    assert_eq!(manager.list()[0].succeeded, 1);
    manager.stop("collision").unwrap();
}

#[test]
fn concurrent_stop_calls_and_restart_have_true_lifecycle_states() {
    let root = tempfile::tempdir().unwrap();
    let manager = Arc::new(WatcherManager::default());
    let first = manager
        .start(
            "lifecycle".into(),
            config(root.path(), None),
            Arc::new(|_| {}),
        )
        .unwrap();
    let threads: Vec<_> = (0..2)
        .map(|_| {
            let manager = manager.clone();
            std::thread::spawn(move || manager.stop("lifecycle").unwrap())
        })
        .collect();
    for thread in threads {
        assert_eq!(thread.join().unwrap().status, WatchStatus::Stopped);
    }
    let second = manager
        .start(
            "lifecycle".into(),
            config(root.path(), None),
            Arc::new(|_| {}),
        )
        .unwrap();
    assert!(second.run_id > first.run_id);
    manager.shutdown();
}

#[test]
fn document_facts_are_real_and_unsigned_signature_fields_stay_unsigned() {
    use paperpilot_pdf::document::LopdfDocument;
    use paperpilot_pdf::operations::form::CreateFormFieldOperation;
    use paperpilot_pdf::operations::signature::SignatureOperation;
    let mut form = LopdfDocument::from_bytes(&pdf(&[Text])).unwrap();
    CreateFormFieldOperation::new("customer".into(), "text".into(), 1, [50., 100., 200., 125.])
        .execute(&mut form)
        .unwrap();
    let mut form_bytes = Vec::new();
    form.inner.save_to(&mut form_bytes).unwrap();
    let mut placeholder = LopdfDocument::from_bytes(&pdf(&[Text])).unwrap();
    SignatureOperation::new().execute(&mut placeholder).unwrap();
    let mut unsigned = Vec::new();
    placeholder.inner.save_to(&mut unsigned).unwrap();
    let signed = std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../apps/desktop/tests/fixtures/automation/signed.pdf"),
    )
    .unwrap();
    let fixtures = [
        ("text.pdf", pdf(&[Text])),
        ("scan.pdf", pdf(&[Image])),
        ("multi.pdf", pdf(&[Text, Text])),
        ("form.pdf", form_bytes),
        ("unsigned.pdf", unsigned),
        ("signed.pdf", signed),
    ];
    let mut facts = serde_json::Map::new();
    let destination = std::env::var("PAPERPILOT_DESKTOP_EVIDENCE_DIR")
        .ok()
        .map(PathBuf::from);
    for (name, bytes) in fixtures {
        let f = inspect_document_bytes(&bytes).unwrap();
        assert_eq!(f.is_scanned, Some(name == "scan.pdf"));
        assert_eq!(f.is_signed, Some(name == "signed.pdf"));
        assert_eq!(
            f.has_form_fields,
            Some(name == "form.pdf" || name == "signed.pdf")
        );
        assert_eq!(f.page_count, Some(if name == "multi.pdf" { 2 } else { 1 }));
        if let Some(path) = &destination {
            std::fs::create_dir_all(path).unwrap();
            std::fs::write(path.join(name), &bytes).unwrap();
        }
        facts.insert(name.into(), serde_json::to_value(f).unwrap());
    }
    if let Some(path) = destination {
        std::fs::write(
            path.join("document-facts.json"),
            serde_json::to_vec_pretty(&facts).unwrap(),
        )
        .unwrap();
    }
}

#[test]
fn encrypted_and_malformed_documents_never_acquire_invented_metadata() {
    use paperpilot_pdf::document::LopdfDocument;
    use paperpilot_pdf::operations::encrypt::EncryptOperation;
    let source = pdf(&[Text]);
    let mut document = LopdfDocument::from_bytes(&source).unwrap();
    let mut encrypt = EncryptOperation::new();
    encrypt.user_password = Some("local-qa-password".into());
    encrypt.execute(&mut document).unwrap();
    let mut bytes = Vec::new();
    document.inner.save_to(&mut bytes).unwrap();
    let facts = inspect_document_bytes(&bytes).unwrap();
    assert!(facts.is_encrypted);
    assert_eq!(facts.page_count, None);
    assert_eq!(facts.is_scanned, None);
    assert_eq!(facts.has_form_fields, None);
    assert_eq!(facts.is_signed, None);
    assert!(inspect_document_bytes(&source[..32]).is_err());
}

#[test]
fn signature_byte_ranges_must_describe_nonempty_signed_segments() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../apps/desktop/tests/fixtures/automation/signed.pdf");
    let mut doc = lopdf::Document::load(path).unwrap();
    for object in doc.objects.values_mut() {
        if let Ok(dict) = object.as_dict_mut() {
            if dict.has(b"ByteRange") {
                dict.set(
                    "ByteRange",
                    lopdf::Object::Array(vec![0.into(), 0.into(), 0.into(), 0.into()]),
                );
            }
        }
    }
    let mut bytes = Vec::new();
    doc.save_to(&mut bytes).unwrap();
    assert_eq!(
        inspect_document_bytes(&bytes).unwrap().is_signed,
        Some(false)
    );
}
