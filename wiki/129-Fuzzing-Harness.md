# Automated Malformed PDF Fuzzing & Security Regression Harness

## Overview
As part of PaperPilot's zero-cloud, enterprise-ready security foundation, the PDF engine offers a strict **Zero-Panic, Zero-Hang Guarantee**. No input—no matter how damaged, crafted, or adversarial—may cause an `unwrap()` panic, stack overflow, or memory exhaustion. All errors must be gracefully returned.

This documentation outlines the fuzzer harness used to verify these assertions.

## Test Location
The fuzzing harness tests are located in:
`paperpilot-pdf/tests/malformed_regression.rs`

## Attack Vectors Covered
The test suite covers the following common adversarial and malformed payloads:
1.  **Corrupted Headers and Random Bytes**: Empty byte slices `b""`, incomplete headers, missing objects, and non-printable binary garbage are used to test that the core parser fails cleanly rather than panicking.
2.  **Circular References (Infinite Loops)**: PDFs containing `/Pages` trees that reference themselves as `/Parent` or `/Kids`. Core tree-traversing operations are executed to ensure they don't get stuck in an infinite recursion or cause stack overflows.
3.  **Stream Length Bombs**: PDFs crafted with a declared stream `/Length` of 1 GB, while only providing a few bytes of payload. The parser's behavior is verified to ensure it avoids bounded memory allocation vulnerabilities and `OOM` panics.
4.  **Deeply Nested Structures**: Dictionaries nested to a depth of 200+. Tested to ensure stack exhaustion (stack overflow) does not crash the server.
5.  **Corrupted Cross-Reference Tables**: Malformed object offsets such as `-100` are sent to ensure the xref parsing module returns a correct `Err(PdfError::ParseError)` safely.

## Running the Harness
To verify the suite, run the following command:
```bash
cargo test -p paperpilot-pdf --test malformed_regression -j 2
```

## Maintenance & Extension
When adding new operations or changing core layout/parsing behavior in `lopdf` or the transformation engine, ensure this test file passes to uphold the security posture.
