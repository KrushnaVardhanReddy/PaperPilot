#[path = "../../paperpilot-pdf/tests/support/workflow_fixture.rs"]
mod fixture;
use fixture::{PageKind::Text, pdf};
use std::{fs, process::Command};
#[test]
fn cli_named_workflow_returns_final_path_and_protects_existing_output() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("source.pdf");
    fs::write(&input, pdf(&[Text])).unwrap();
    let recipe = dir.path().join("rotate.ppflow");
    fs::write(&recipe, "name='Rotate'\n[[stage]]\nid='turn'\nop='rotate'").unwrap();
    let out = dir.path().join("out");
    let invoke = || {
        Command::new(env!("CARGO_BIN_EXE_paperpilot"))
            .arg("run")
            .arg(&recipe)
            .arg(&input)
            .arg("--output")
            .arg(&out)
            .args(["--name-template", "{input_stem}-{index:04}.pdf", "--json"])
            .output()
            .unwrap()
    };
    let r = invoke();
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    let value: serde_json::Value = serde_json::from_slice(&r.stdout).unwrap();
    assert_eq!(
        value["workflow"]["output"],
        serde_json::json!(out.join("source-0001.pdf"))
    );
    let saved = fs::read(out.join("source-0001.pdf")).unwrap();
    assert!(!invoke().status.success());
    assert_eq!(fs::read(out.join("source-0001.pdf")).unwrap(), saved);
    let r = Command::new(env!("CARGO_BIN_EXE_paperpilot"))
        .args(["workflow", "run", "--input"])
        .arg(&input)
        .arg("--output")
        .arg(dir.path().join("sub"))
        .arg("--recipe")
        .arg(&recipe)
        .args(["--name-template", "{input_name}"])
        .output()
        .unwrap();
    assert!(r.status.success());
    assert!(lopdf::Document::load(dir.path().join("sub/source.pdf")).is_ok());
}
#[test]
fn cli_once_supports_dynamic_watcher_names() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("original.pdf"), pdf(&[Text])).unwrap();
    let r = Command::new(env!("CARGO_BIN_EXE_paperpilot"))
        .arg("watch")
        .arg("--directory")
        .arg(dir.path())
        .args([
            "--recipe",
            "preset-legal-discovery",
            "--once",
            "--settle-delay-ms",
            "50",
            "--name-template",
            "{input_stem}-{index:04}.pdf",
        ])
        .output()
        .unwrap();
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    let doc = lopdf::Document::load(dir.path().join("processed/original-0001.pdf")).unwrap();
    assert!(doc.extract_text(&[1]).unwrap().contains("Original invoice"));
}
