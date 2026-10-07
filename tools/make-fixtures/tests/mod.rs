use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_make_fixtures_runs_successfully() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let status = Command::new("cargo")
        .args(["run", "--bin", "make-fixtures"])
        .current_dir(&manifest_dir)
        .status()
        .expect("Failed to execute make-fixtures");
    assert!(status.success());
}
