use std::path::Path;
use std::process::Command;

fn main() {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("tests/fixtures/simple.pdf");

    if !fixture_path.exists() {
        // Attempt to run make-fixtures
        let status = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "make-fixtures",
                "--manifest-path",
                &format!(
                    "{}/tools/make-fixtures/Cargo.toml",
                    env!("CARGO_MANIFEST_DIR").replace("/paperpilot-pdf", "")
                ),
            ])
            .status();
        if let Err(e) = status {
            println!("cargo:warning=Could not auto-generate fixtures: {e}");
        }
    }

    println!("cargo:rerun-if-changed=tests/fixtures/");
}
