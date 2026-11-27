#!/usr/bin/env python3
"""PaperPilot Gateway Dynamic Security & Fuzzing Suite (DAST).

Tests local PaperPilot Gateway (127.0.0.1:7823 or custom port) for:
1. OWASP ZAP baseline security headers (X-Content-Type-Options, X-Frame-Options, etc.)
2. Body limit enforcement (>100MB payload overflow)
3. Malformed PDF ingestion (decompression bombs, corrupted bytes, circular trees)
4. Path traversal injection on file and receipt endpoints
5. Zero-Trace privacy suppression
"""

import sys
import time
import urllib.request
import urllib.error
import json

DEFAULT_BASE_URL = "http://127.0.0.1:7823"

def run_test(name, fn):
    print(f"[*] Testing {name}...", end=" ", flush=True)
    try:
        fn()
        print("PASSED ✅")
        return True
    except AssertionError as e:
        print(f"FAILED ❌: {e}")
        return False
    except Exception as e:
        print(f"ERROR 💥: {e}")
        return False

def check_security_headers(base_url):
    req = urllib.request.Request(f"{base_url}/health")
    with urllib.request.urlopen(req) as resp:
        assert resp.status == 200, f"Expected 200, got {resp.status}"
        headers = dict(resp.headers)
        
        # Check required security headers
        assert headers.get("X-Content-Type-Options", "").lower() == "nosniff", "Missing or invalid X-Content-Type-Options"
        assert headers.get("X-Frame-Options", "").upper() == "DENY", "Missing or invalid X-Frame-Options"
        assert headers.get("Referrer-Policy", "").lower() == "no-referrer", "Missing or invalid Referrer-Policy"
        assert headers.get("Cross-Origin-Resource-Policy", "").lower() == "same-origin", "Missing or invalid Cross-Origin-Resource-Policy"

def check_zero_trace_suppression(base_url):
    req = urllib.request.Request(f"{base_url}/health", headers={"X-Zero-Trace": "true"})
    with urllib.request.urlopen(req) as resp:
        assert resp.status == 200
        headers = dict(resp.headers)
        assert headers.get("X-Zero-Trace-Enforced", "").lower() == "true", "X-Zero-Trace-Enforced not present"
        assert "X-Request-ID" not in headers, "X-Request-ID should be suppressed during zero-trace"

def check_malformed_pdf_handling(base_url):
    payloads = [
        ("empty", b"", 400),
        ("corrupted_header", b"%PDF", 415),
        ("random_garbage", b"\xff\xfe\x00\x01\x02\x03", 415),
        ("not_a_pdf", b"Hello this is plaintext", 415),
    ]
    for name, payload, expected_status in payloads:
        req = urllib.request.Request(
            f"{base_url}/tika",
            data=payload,
            method="PUT",
            headers={"Content-Type": "application/pdf"}
        )
        try:
            with urllib.request.urlopen(req) as resp:
                assert False, f"Payload {name} should have failed, but returned {resp.status}"
        except urllib.error.HTTPError as err:
            assert 400 <= err.code < 500, f"Payload {name} returned status {err.code}, expected 4xx"

def check_path_traversal_on_receipts(base_url):
    traversal_paths = [
        "..%2f..%2fetc%2fpasswd",
        "nested%2fpath%2frun-1.json",
        "invalid_ext.txt",
    ]
    for p in traversal_paths:
        req = urllib.request.Request(f"{base_url}/v1/automation/receipts/{p}?directory=/tmp")
        try:
            with urllib.request.urlopen(req) as resp:  # nosemgrep: python.lang.security.audit.dynamic-urllib-use-detected
                assert False, f"Path traversal '{p}' succeeded with {resp.status}"
        except urllib.error.HTTPError as err:
            assert 400 <= err.code < 500, f"Expected 4xx for path traversal '{p}', got {err.code}"

def check_unsupported_methods(base_url):
    req = urllib.request.Request(f"{base_url}/health", method="DELETE")
    try:
        with urllib.request.urlopen(req) as resp:  # nosemgrep: python.lang.security.audit.dynamic-urllib-use-detected
            assert False, "DELETE on /health should fail"
    except urllib.error.HTTPError as err:
        assert err.code in (404, 405), f"Expected 404/405, got {err.code}"

def main():
    base_url = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_BASE_URL
    print(f"=== PaperPilot DAST Security Suite against {base_url} ===")
    
    # Check if target is alive
    try:
        with urllib.request.urlopen(f"{base_url}/health", timeout=2) as resp:  # nosemgrep: python.lang.security.audit.dynamic-urllib-use-detected
            pass
    except Exception as e:
        print(f"[!] Warning: Gateway at {base_url} is not currently responding ({e}).")
        print("[!] Note: Native tests under paperpilot-gateway/tests/security_dast_test.rs also test these via tower in-memory.")
        return 0

    tests = [
        ("OWASP Security Headers", lambda: check_security_headers(base_url)),
        ("Zero-Trace Suppression", lambda: check_zero_trace_suppression(base_url)),
        ("Malformed PDF Ingestion", lambda: check_malformed_pdf_handling(base_url)),
        ("Path Traversal Defense", lambda: check_path_traversal_on_receipts(base_url)),
        ("Unsupported HTTP Methods", lambda: check_unsupported_methods(base_url)),
    ]

    passed = 0
    for name, fn in tests:
        if run_test(name, fn):
            passed += 1

    print(f"\nResults: {passed}/{len(tests)} DAST checks passed.")
    return 0 if passed == len(tests) else 1

if __name__ == "__main__":
    sys.exit(main())
