#![cfg(feature = "pdf")]
#[path = "../../paperpilot-pdf/tests/support/workflow_fixture.rs"]
mod fixture;
use assert_cmd::Command;
use fixture::{PageKind::*, pdf};
use lopdf::Document;
use serde_json::{Value, json};
use std::fs;
use tempfile::tempdir;

fn cli(args: &[&str], success: bool) -> Value {
    let output = Command::new(assert_cmd::cargo::cargo_bin!("paperpilot"))
        .args(args)
        .output()
        .unwrap();
    assert_eq!(
        output.status.success(),
        success,
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let data: Value =
        serde_json::from_slice(&output.stdout).expect("exactly one JSON response on stdout");
    assert_eq!(data["success"], success);
    data
}

#[test]
fn run_toml_and_both_legacy_cli_forms_produce_real_pdfs() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("input.pdf");
    let recipe = dir.path().join("invoice.ppflow");
    fs::write(&input, pdf(&[Text, Text, Text])).unwrap();
    fs::write(&recipe, "name='Invoice'\nversion='1'\n[[stage]]\nid='large'\nop='rotate'\ncondition={if_page_count_gt=2}\n[stage.params]\nangle=180").unwrap();
    let output = dir.path().join("result.pdf");
    let data = cli(
        &[
            "run",
            recipe.to_str().unwrap(),
            input.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            "--json",
        ],
        true,
    );
    assert_eq!(data["workflow"]["steps_completed"], 1);
    let doc = Document::load(&output).unwrap();
    assert_eq!(doc.get_pages().len(), 3);
    assert_eq!(
        doc.get_dictionary(*doc.get_pages().values().next().unwrap())
            .unwrap()
            .get(b"Rotate")
            .unwrap()
            .as_i64()
            .unwrap(),
        180
    );
    let legacy = json!({"id":"old","name":"Old JSON","steps":[{"id":"r","operationId":"rotate","params":{"angle":90}}],"updatedAt":"2025-01-01"});
    fs::write(&recipe, legacy.to_string()).unwrap();
    for sub in [true, false] {
        let mut args = vec!["workflow"];
        if sub {
            args.push("run");
        }
        args.extend([
            "--recipe",
            recipe.to_str().unwrap(),
            "--input",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--json",
        ]);
        assert_eq!(cli(&args, true)["workflow"]["steps_completed"], 1);
    }
}

#[test]
fn errors_preserve_existing_output_and_include_stage_statuses() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("in.pdf");
    let recipe = dir.path().join("error.ppflow");
    let output = dir.path().join("out.pdf");
    fs::write(&input, pdf(&[Text])).unwrap();
    let previous = pdf(&[Image]);
    fs::write(&output, &previous).unwrap();
    fs::write(&recipe, "name='Failure'\n[[stage]]\nid='first'\nop='rotate'\n[[stage]]\nid='gate'\nop='rotate'\ncondition={if_page_count_gt=50}\non_no_match='error'\n[[stage]]\nid='never'\nop='compress'").unwrap();
    let data = cli(
        &[
            "run",
            recipe.to_str().unwrap(),
            input.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            "--json",
        ],
        false,
    );
    assert_eq!(fs::read(&output).unwrap(), previous);
    assert_eq!(data["workflow"]["stages"][0]["status"], "completed");
    assert_eq!(data["workflow"]["stages"][1]["status"], "failed");
    assert_eq!(data["workflow"]["stages"][2]["status"], "not_run");
    assert_eq!(data["workflow"]["output_bytes_len"], 0);
}

#[test]
fn archive_original_writes_exact_source_and_stops_workflow() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("in.pdf");
    let recipe = dir.path().join("archive.ppflow");
    let output = dir.path().join("out.pdf");
    let original = pdf(&[Text]);
    fs::write(&input, &original).unwrap();
    fs::write(&recipe, "name='Archive'\n[[stage]]\nid='change'\nop='rotate'\n[[stage]]\nid='gate'\nop='filter_page_count'\ncondition={if_page_count_gt=50}\non_no_match='archive_original'\n[[stage]]\nid='never'\nop='compress'").unwrap();
    let data = cli(
        &[
            "run",
            recipe.to_str().unwrap(),
            input.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            "--json",
        ],
        true,
    );
    assert_eq!(fs::read(output).unwrap(), original);
    assert_eq!(data["workflow"]["archived_original"], true);
    assert_eq!(data["workflow"]["stages"][2]["status"], "not_run");
}

#[test]
fn rejects_unsupported_ops_and_bad_schema_without_publishing_files() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("in.pdf");
    let recipe = dir.path().join("bad.ppflow");
    let output = dir.path().join("out.pdf");
    fs::write(&input, pdf(&[Text])).unwrap();
    for stage in [
        "op='typo'\ncondition={if_scanned=true}",
        "op='redact'",
        "op='encrypt'",
        "op='rotate'\nretry=3",
        "op='rotate'\ncondition={if_scaned=true}",
        "op='rotate'\ndepends_on=['missing']",
    ] {
        fs::write(&recipe, format!("name='Bad'\n[[stage]]\nid='bad'\n{stage}")).unwrap();
        cli(
            &[
                "run",
                recipe.to_str().unwrap(),
                input.to_str().unwrap(),
                "-o",
                output.to_str().unwrap(),
                "--json",
            ],
            false,
        );
        assert!(!output.exists());
    }
}

#[test]
fn json_inspect_and_list_are_single_responses_and_hide_parameters() {
    assert!(cli(&["workflow", "list", "--json"], true)["workflow"].is_array());
    let dir = tempdir().unwrap();
    let recipe = dir.path().join("secret.ppflow");
    fs::write(&recipe, "name='Secret'\n[[stage]]\nid='encrypt'\nop='encrypt'\n[stage.params]\npassword='should-never-be-printed'").unwrap();
    let response = cli(
        &[
            "workflow",
            "inspect",
            "--recipe",
            recipe.to_str().unwrap(),
            "--json",
        ],
        true,
    );
    assert!(!response.to_string().contains("should-never-be-printed"));
    assert_eq!(
        response["workflow"]["steps"][0]["parameter_names"][0],
        "password"
    );
}

#[test]
fn same_input_output_and_ambiguous_source_are_rejected() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("in.pdf");
    let recipe = dir.path().join("test.ppflow");
    let original = pdf(&[Text]);
    fs::write(&input, &original).unwrap();
    fs::write(&recipe, "name='Test'\n[[stage]]\nid='r'\nop='rotate'").unwrap();
    cli(
        &[
            "run",
            recipe.to_str().unwrap(),
            input.to_str().unwrap(),
            "-o",
            input.to_str().unwrap(),
            "--json",
        ],
        false,
    );
    assert_eq!(fs::read(&input).unwrap(), original);
    cli(
        &[
            "workflow",
            "run",
            "--input",
            input.to_str().unwrap(),
            "--output",
            input.to_str().unwrap(),
            "--preset",
            "clean-scan",
            "--recipe",
            recipe.to_str().unwrap(),
            "--json",
        ],
        false,
    );
}

#[test]
fn text_input_skips_ocr_without_loading_models() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("in.pdf");
    let recipe = dir.path().join("ocr.ppflow");
    let output = dir.path().join("out.pdf");
    fs::write(&input, pdf(&[Text])).unwrap();
    fs::write(&recipe, "name='OCR if needed'\n[[stage]]\nid='ocr'\nop='ocr'\ncondition={if_scanned=true}\n[[stage]]\nid='finish'\nop='rotate'").unwrap();
    let data = cli(
        &[
            "run",
            recipe.to_str().unwrap(),
            input.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            "--json",
        ],
        true,
    );
    assert_eq!(data["workflow"]["stages"][0]["status"], "skipped");
    assert_eq!(data["workflow"]["steps_completed"], 1);
}
