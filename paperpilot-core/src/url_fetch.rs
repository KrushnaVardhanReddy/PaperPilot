use crate::error::{OperationResult, PdfError};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::Write;
use std::net::IpAddr;
use std::path::PathBuf;
use std::time::Duration;

const MAX_DOWNLOAD_SIZE: u64 = 100 * 1024 * 1024; // 100 MB

pub fn is_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

pub fn resolve_url_to_bytes(url: &str) -> OperationResult<Vec<u8>> {
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

    let parsed_url = reqwest::Url::parse(url)
        .map_err(|_| PdfError::InvalidInput("Invalid URL format".to_string()))?;

    // SSRF Protection
    // Note: httptest uses 127.0.0.1 for local tests, so we need to bypass SSRF check during tests.
    #[cfg(not(test))]
    if let Some(host_str) = parsed_url.host_str() {
        if let Ok(ip) = host_str.parse::<IpAddr>()
            && is_private_ip(ip)
        {
            return Err(PdfError::InvalidInput(
                "SSRF blocked: private IP detected".to_string(),
            ));
        }
        if host_str == "localhost" {
            return Err(PdfError::InvalidInput(
                "SSRF blocked: localhost detected".to_string(),
            ));
        }
    }

    let mut response = client.get(parsed_url.clone()).send().map_err(|e| {
        PdfError::IoError(std::io::Error::other(format!("Failed to fetch URL: {}", e)))
    })?;

    if !response.status().is_success() {
        return Err(PdfError::InvalidInput(format!(
            "Failed to fetch remote PDF: HTTP {}",
            response.status()
        )));
    }

    if let Some(content_length) = response.content_length()
        && content_length > MAX_DOWNLOAD_SIZE
    {
        return Err(PdfError::InvalidInput(
            "File exceeds maximum download size of 100MB".to_string(),
        ));
    }

    let mut bytes = Vec::new();
    let mut chunk = vec![0; 8192];
    use std::io::Read;

    // Read first chunk for magic bytes check
    let bytes_read = response.read(&mut chunk).map_err(|e| {
        PdfError::IoError(std::io::Error::other(format!(
            "Failed to read stream: {}",
            e
        )))
    })?;

    if bytes_read < 4 || &chunk[0..4] != b"%PDF" {
        return Err(PdfError::InvalidFormat(
            "URL does not return a valid PDF document".to_string(),
        ));
    }

    bytes.extend_from_slice(&chunk[..bytes_read]);

    let mut total_bytes = bytes_read as u64;

    loop {
        let n = response.read(&mut chunk).map_err(|e| {
            PdfError::IoError(std::io::Error::other(format!(
                "Failed to read stream: {}",
                e
            )))
        })?;
        if n == 0 {
            break;
        }
        total_bytes += n as u64;
        if total_bytes > MAX_DOWNLOAD_SIZE {
            return Err(PdfError::InvalidInput(
                "File exceeds maximum download size of 100MB".to_string(),
            ));
        }
        bytes.extend_from_slice(&chunk[..n]);
    }

    Ok(bytes)
}

pub fn resolve_url_to_temp_file(url: &str) -> OperationResult<PathBuf> {
    let bytes = resolve_url_to_bytes(url)?;

    let temp_dir = std::env::temp_dir();
    let file_name = format!(
        "remote_pdf_{}.pdf",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let path = temp_dir.join(file_name);

    let mut file = File::create(&path).map_err(PdfError::IoError)?;

    file.write_all(&bytes).map_err(PdfError::IoError)?;

    Ok(path)
}

fn is_private_ip(ip: IpAddr) -> bool {
    if ip.is_loopback() || ip.is_unspecified() {
        return true;
    }
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            // 10.0.0.0/8 (Private)
            if octets[0] == 10 {
                return true;
            }
            // 172.16.0.0/12 (Private)
            if octets[0] == 172 && octets[1] >= 16 && octets[1] <= 31 {
                return true;
            }
            // 192.168.0.0/16 (Private)
            if octets[0] == 192 && octets[1] == 168 {
                return true;
            }
            // 169.254.0.0/16 (Link local / Cloud metadata)
            if octets[0] == 169 && octets[1] == 254 {
                return true;
            }
            // 100.64.0.0/10 (Shared address space / CGNAT)
            if octets[0] == 100 && octets[1] >= 64 && octets[1] <= 127 {
                return true;
            }

            false
        }
        IpAddr::V6(ipv6) => {
            let segments = ipv6.segments();
            // Unique local address (fc00::/7)
            if segments[0] & 0xfe00 == 0xfc00 {
                return true;
            }
            // Link local address (fe80::/10)
            if segments[0] & 0xffc0 == 0xfe80 {
                return true;
            }

            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httptest::{Expectation, Server, matchers::*, responders::*};

    #[test]
    fn test_url_fetch_valid_pdf() {
        let server = Server::run();
        let path = "/test.pdf";
        server.expect(
            Expectation::matching(request::method_path("GET", path))
                .respond_with(status_code(200).body(b"%PDF-1.4\n%...".to_vec())),
        );

        let url = server.url(path).to_string();
        let bytes = resolve_url_to_bytes(&url).unwrap();
        assert_eq!(&bytes[0..4], b"%PDF");
    }

    #[test]
    fn test_url_fetch_invalid_magic_bytes() {
        let server = Server::run();
        let path = "/test.txt";
        server.expect(
            Expectation::matching(request::method_path("GET", path))
                .respond_with(status_code(200).body(b"Hello world".to_vec())),
        );

        let url = server.url(path).to_string();
        let err = resolve_url_to_bytes(&url).unwrap_err();
        match err {
            PdfError::InvalidFormat(msg) => {
                assert_eq!(msg, "URL does not return a valid PDF document")
            }
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_ssrf_blocking() {
        assert!(is_private_ip("127.0.0.1".parse().unwrap()));
        assert!(is_private_ip("10.0.0.1".parse().unwrap()));
        assert!(is_private_ip("172.16.0.1".parse().unwrap()));
        assert!(is_private_ip("192.168.1.1".parse().unwrap()));
        assert!(is_private_ip("169.254.169.254".parse().unwrap()));
        assert!(is_private_ip("169.254.0.1".parse().unwrap()));
        assert!(is_private_ip("0.0.0.0".parse().unwrap()));
        assert!(is_private_ip("100.64.0.1".parse().unwrap()));
        assert!(is_private_ip("::1".parse().unwrap()));
        assert!(is_private_ip("::".parse().unwrap()));
        assert!(is_private_ip("fe80::1".parse().unwrap()));

        assert!(!is_private_ip("8.8.8.8".parse().unwrap()));
        assert!(!is_private_ip("1.1.1.1".parse().unwrap()));
    }
}
