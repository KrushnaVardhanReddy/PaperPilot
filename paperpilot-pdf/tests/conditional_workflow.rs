#[path = "support/workflow_fixture.rs"]
mod fixture;
use fixture::{PageKind::*, pdf};
use lopdf::Document;
use paperpilot_core::workflow::{WorkflowStageStatus::*, parse_workflow};
use paperpilot_pdf::operations::workflow::{run_workflow_bytes, run_workflow_bytes_detailed};

fn recipe(stages: &str) -> paperpilot_core::workflow::SavedWorkflow {
    parse_workflow(&format!("name = 'Conditional test'\n{stages}")).unwrap()
}
fn rotation(bytes: &[u8]) -> i64 {
    let d = Document::load_mem(bytes).unwrap();
    d.get_dictionary(*d.get_pages().values().next().unwrap())
        .unwrap()
        .get(b"Rotate")
        .and_then(|v| v.as_i64())
        .unwrap_or(0)
}

#[test]
fn page_count_boundary_and_real_pdf_output() {
    let wf = recipe(
        "[[stage]]\nid='large'\nop='rotate'\ncondition={if_page_count_gt=2}\n[stage.params]\nangle=180",
    );
    for (count, expected, status) in [(2, 0, Skipped), (3, 180, Completed)] {
        let (out, r) = run_workflow_bytes(&pdf(&vec![Text; count]), &wf).unwrap();
        assert_eq!(Document::load_mem(&out).unwrap().get_pages().len(), count);
        assert_eq!(rotation(&out), expected);
        assert_eq!(r.stages[0].status, status);
        assert!(
            r.stages[0]
                .reason
                .as_ref()
                .unwrap()
                .contains(&format!("page_count={count}"))
        );
    }
}

#[test]
fn detects_images_nested_forms_mixed_documents_and_blank_pages() {
    let wf = recipe("[[stage]]\nid='scan'\nop='rotate'\ncondition={if_scanned=true}");
    for (kinds, scanned) in [
        (vec![Text], false),
        (vec![Blank], false),
        (vec![UnusedImage], false),
        (vec![Image], true),
        (vec![NestedImage], true),
        (vec![Text, Image], true),
    ] {
        let (out, r) = run_workflow_bytes(&pdf(&kinds), &wf).unwrap();
        assert_eq!(rotation(&out), if scanned { 90 } else { 0 });
        assert_eq!(r.steps_completed, usize::from(scanned));
    }
}

#[test]
fn complementary_conditions_and_comparators() {
    let wf = recipe(
        "[[stage]]\nid='text'\nop='rotate'\ncondition={if_scanned=false, comparator='LessOrEqual',value=1}\n[[stage]]\nid='large'\nop='rotate'\ncondition={comparator='Greater',value=1}\n[stage.params]\nangle=180",
    );
    assert_eq!(
        rotation(&run_workflow_bytes(&pdf(&[Text]), &wf).unwrap().0),
        90
    );
    assert_eq!(
        rotation(&run_workflow_bytes(&pdf(&[Text, Text]), &wf).unwrap().0),
        180
    );
    assert_eq!(
        rotation(&run_workflow_bytes(&pdf(&[Image]), &wf).unwrap().0),
        0
    );
}

#[test]
fn conditions_use_each_steps_actual_input_after_prior_changes() {
    let wf = recipe(
        "[[stage]]\nid='number'\nop='page_numbers'\n[[stage]]\nid='scan'\nop='rotate'\ncondition={if_scanned=true}",
    );
    let (out, report) = run_workflow_bytes(&pdf(&[Image]), &wf).unwrap();
    assert_eq!(report.steps_completed, 1);
    assert_eq!(report.steps_skipped, 1);
    assert_eq!(rotation(&out), 0);
    assert!(
        Document::load_mem(&out)
            .unwrap()
            .extract_text(&[1])
            .unwrap()
            .contains("Page 1")
    );
}

#[test]
fn dependency_selects_artifact_and_topological_order_is_stable() {
    let wf = recipe(
        "[[stage]]\nid='root'\nop='rotate'\n[stage.params]\nangle=90\n[[stage]]\nid='later'\nop='rotate'\ndepends_on=['branch']\n[stage.params]\nangle=90\n[[stage]]\nid='branch'\nop='rotate'\ndepends_on=['root']\n[stage.params]\nangle=180\n[[stage]]\nid='output'\nop='rotate'\ndepends_on=['root']\n[stage.params]\nangle=90",
    );
    let (out, report) = run_workflow_bytes(&pdf(&[Text]), &wf).unwrap();
    // Final declared stage uses root's 90-degree artifact, not branch's output.
    assert_eq!(rotation(&out), 180);
    assert_eq!(report.steps_completed, 4);
}

#[test]
fn archive_original_restores_exact_bytes_and_stops_remaining_stages() {
    let wf = recipe(
        "[[stage]]\nid='modify'\nop='rotate'\n[[stage]]\nid='gate'\nop='filter_page_count'\ncondition={if_page_count_gt=50}\non_no_match='archive_original'\n[[stage]]\nid='never'\nop='watermark'",
    );
    let original = pdf(&[Text]);
    let (out, report) = run_workflow_bytes(&original, &wf).unwrap();
    assert_eq!(out, original);
    assert!(report.archived_original);
    assert_eq!(report.stages[2].status, NotRun);
    assert_eq!(report.steps_completed, 1);
    assert_eq!(report.steps_skipped, 1);
}

#[test]
fn unmatched_error_includes_failed_and_not_run_stage_reports() {
    let wf = recipe(
        "[[stage]]\nid='first'\nop='rotate'\n[[stage]]\nid='gate'\nop='rotate'\ncondition={if_page_count_gt=50}\non_no_match='error'\n[[stage]]\nid='never'\nop='compress'",
    );
    let failure = run_workflow_bytes_detailed(&pdf(&[Text]), &wf).unwrap_err();
    assert_eq!(failure.report.stages[0].status, Completed);
    assert_eq!(failure.report.stages[1].status, Failed);
    assert_eq!(failure.report.stages[2].status, NotRun);
    assert_eq!(failure.report.output_bytes_len, 0);
    assert!(failure.to_string().contains("gate"));
}

#[test]
fn unknown_redaction_and_invalid_parameters_never_silently_succeed() {
    for stage in [
        "op='does_not_exist'",
        "op='redact'",
        "op='encrypt'",
        "op='rotate'\n[stage.params]\nangle=45",
        "op='rotate'\n[stage.params]\nangle=-90",
        "op='compress'\n[stage.params]\nquality=300",
        "op='watermark'\n[stage.params]\nignored=true",
    ] {
        let wf = recipe(&format!(
            "[[stage]]\nid='bad'\ncondition={{if_page_count_gt=100}}\n{stage}"
        ));
        assert!(run_workflow_bytes(&pdf(&[Text]), &wf).is_err(), "{stage}");
    }
}

#[test]
fn encrypt_requires_password_and_really_encrypts_output() {
    let wf =
        recipe("[[stage]]\nid='encrypt'\nop='encrypt'\n[stage.params]\npassword='local-test-pass'");
    let (out, r) = run_workflow_bytes(&pdf(&[Text]), &wf).unwrap();
    let mut d = Document::load_mem(&out).unwrap();
    assert!(d.is_encrypted());
    assert!(d.decrypt("incorrect").is_err());
    let d =
        Document::load_mem_with_options(&out, lopdf::LoadOptions::with_password("local-test-pass"))
            .unwrap();
    assert_eq!(d.get_pages().len(), 1);
    assert!(d.extract_text(&[1]).unwrap().contains("Original invoice"));
    assert!(
        !serde_json::to_string(&r)
            .unwrap()
            .contains("local-test-pass")
    );
}

#[test]
fn rejects_malformed_and_encrypted_input_even_if_conditions_skip_everything() {
    let wf = recipe("[[stage]]\nid='skip'\nop='rotate'\ncondition={if_page_count_gt=100}");
    assert!(run_workflow_bytes(b"invalid input", &wf).is_err());
    let enc = recipe("[[stage]]\nid='e'\nop='encrypt'\n[stage.params]\npassword='secret'");
    let (encrypted, _) = run_workflow_bytes(&pdf(&[Text]), &enc).unwrap();
    assert!(run_workflow_bytes(&encrypted, &wf).is_err());
}

#[test]
fn encryption_must_be_terminal_and_legacy_presets_remain_usable() {
    let wf = recipe(
        "[[stage]]\nid='e'\nop='encrypt'\n[stage.params]\npassword='secret'\n[[stage]]\nid='next'\nop='rotate'",
    );
    assert!(run_workflow_bytes(&pdf(&[Text]), &wf).is_err());
    let forward_consumer = recipe(
        "[[stage]]\nid='root'\nop='rotate'\n[[stage]]\nid='consumer'\nop='rotate'\ndepends_on=['encrypted']\n[[stage]]\nid='encrypted'\nop='encrypt'\ndepends_on=['root']\n[stage.params]\npassword='secret'",
    );
    assert!(run_workflow_bytes(&pdf(&[Text]), &forward_consumer).is_err());
    for name in ["legal-discovery", "clean-scan"] {
        let wf = paperpilot_core::workflow::get_preset_by_id(name).unwrap();
        let (out, report) = run_workflow_bytes(&pdf(&[Text]), &wf).unwrap();
        assert_eq!(report.steps_completed, 3);
        assert_eq!(Document::load_mem(&out).unwrap().get_pages().len(), 1);
    }
}

#[test]
fn cyclic_form_content_fails_inspection_without_recursing_forever() {
    let mut doc = Document::load_mem(&pdf(&[NestedImage])).unwrap();
    let id = *doc
        .objects
        .iter()
        .find(|(_, object)| {
            object.as_stream().ok().is_some_and(|s| {
                s.dict
                    .get(b"Subtype")
                    .and_then(|v| v.as_name())
                    .is_ok_and(|n| n == b"Form")
            })
        })
        .unwrap()
        .0;
    let stream = doc.get_object_mut(id).unwrap().as_stream_mut().unwrap();
    stream.content = b"q /Self Do Q".to_vec();
    stream.dict.set("Length", stream.content.len() as i64);
    stream.dict.set(
        "Resources",
        lopdf::dictionary! { "XObject" => lopdf::dictionary! { "Self" => id } },
    );
    let mut source = Vec::new();
    doc.save_to(&mut source).unwrap();
    let wf = recipe("[[stage]]\nid='scan'\nop='rotate'\ncondition={if_scanned=true}");
    assert!(run_workflow_bytes(&source, &wf).is_err());
}
