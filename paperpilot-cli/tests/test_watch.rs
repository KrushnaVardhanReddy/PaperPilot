#[path = "../../paperpilot-pdf/tests/support/workflow_fixture.rs"]
mod fixture;
use fixture::{PageKind::Text, pdf};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

fn command(directory: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_paperpilot"));
    command
        .arg("watch")
        .arg("--directory")
        .arg(directory)
        .args([
            "--recipe",
            "preset-legal-discovery",
            "--settle-delay-ms",
            "50",
        ]);
    command
}
struct Daemon(Child);
impl Drop for Daemon {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
fn cli_once_writes_real_pdf_and_receipt_without_touching_the_source() {
    let root = tempfile::tempdir().unwrap();
    let source = pdf(&[Text, Text]);
    std::fs::write(root.path().join("input.pdf"), &source).unwrap();
    let result = command(root.path()).arg("--once").output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let doc = lopdf::Document::load(root.path().join("processed/input.pdf")).unwrap();
    assert_eq!(doc.get_pages().len(), 2);
    assert!(
        doc.extract_text(&[1, 2])
            .unwrap()
            .contains("Original invoice")
    );
    assert_eq!(
        std::fs::read(root.path().join("input.pdf")).unwrap(),
        source
    );
    let receipt: serde_json::Value = serde_json::from_slice(
        &std::fs::read(root.path().join("processed/input.pdf.receipt.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(receipt["success"], true);
    assert_eq!(receipt["report"]["steps_completed"], 3);
}

#[test]
fn daemon_processes_a_genuine_pdf_arriving_after_startup() {
    let root = tempfile::tempdir().unwrap();
    let _daemon = Daemon(
        command(root.path())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let deadline = Instant::now() + Duration::from_secs(10);
    while !root.path().join("processed").exists() {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(50));
    }
    std::fs::write(root.path().join("arriving.pdf"), pdf(&[Text])).unwrap();
    while !root
        .path()
        .join("processed/arriving.pdf.receipt.json")
        .exists()
    {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(50));
    }
    let doc = lopdf::Document::load(root.path().join("processed/arriving.pdf")).unwrap();
    assert!(doc.extract_text(&[1]).unwrap().contains("Original invoice"));
}

#[test]
fn cli_once_continues_after_a_truncated_pdf_and_returns_a_failure_exit_code() {
    let root = tempfile::tempdir().unwrap();
    let source = pdf(&[Text]);
    std::fs::write(root.path().join("bad.pdf"), &source[..32]).unwrap();
    std::fs::write(root.path().join("good.pdf"), &source).unwrap();
    let result = command(root.path()).arg("--once").output().unwrap();
    assert!(!result.status.success());
    assert!(root.path().join(".error/bad.pdf.receipt.json").exists());
    assert_eq!(
        std::fs::read(root.path().join("bad.pdf")).unwrap(),
        source[..32]
    );
    assert_eq!(
        lopdf::Document::load(root.path().join("processed/good.pdf"))
            .unwrap()
            .get_pages()
            .len(),
        1
    );
}

#[test]
fn opt_in_webhook_keeps_the_existing_completion_contract_for_real_pdf_work() {
    use std::io::{BufRead, BufReader, Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    listener.set_nonblocking(true).unwrap();
    let receiver = std::thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut stream = loop {
            match listener.accept() {
                Ok((stream, _)) => break stream,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(
                        Instant::now() < deadline,
                        "Expected the opt-in local webhook"
                    );
                    std::thread::sleep(Duration::from_millis(20));
                }
                Err(e) => panic!("Webhook listener failed: {e}"),
            }
        };
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut size = None;
        loop {
            let mut line = String::new();
            assert!(reader.read_line(&mut line).unwrap() > 0);
            if line == "\r\n" {
                break;
            }
            if let Some((key, value)) = line.split_once(':') {
                if key.eq_ignore_ascii_case("content-length") {
                    size = Some(value.trim().parse::<usize>().unwrap());
                }
            }
        }
        let size = size.unwrap();
        assert!(size < 1024 * 1024);
        let mut payload = vec![0; size];
        reader.read_exact(&mut payload).unwrap();
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .unwrap();
        serde_json::from_slice::<serde_json::Value>(&payload).unwrap()
    });
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("input.pdf"), pdf(&[Text])).unwrap();
    let result = command(root.path())
        .args([
            "--once",
            "--webhook",
            &format!("http://{address}/completion"),
        ])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let payload = receiver.join().unwrap();
    assert_eq!(payload["success"], true);
    assert_eq!(payload["operation"], "recipe");
    assert!(payload["file"].as_str().unwrap().ends_with("input.pdf"));
    assert!(payload["error"].is_null());
    assert!(Path::new(payload["output"].as_str().unwrap()).is_file());
    assert!(Path::new(payload["receipt"].as_str().unwrap()).is_file());
}
