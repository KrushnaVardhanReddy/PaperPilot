# Spec 173: Automated Secret and Credential Leak Auditing (TruffleHog Integration)

## Metadata
- **Spec Number**: 173
- **Title**: Automated Secret and Credential Leak Auditing (TruffleHog Integration)
- **Phase**: Security & Supply Chain Hardening
- **Status**: ✅ Completed
- **Impacted Systems**:
  - `scripts/run_secrets_scan.sh`: Local automated scanner script
  - `Makefile`: Targets `trufflehog-scan` and `secret-scan-local`
  - `.github/workflows/security.yml`: CI secret audit workflow

---

## 1. Executive Summary & Problem Statement

Modern enterprise document platforms handle private keys, S3 storage credentials, API keys for LLMs, and cryptographic signing certificates. Unintentional commits of high-entropy credentials or private tokens into git history present critical security risks.

**Spec 173** introduces automated, verified secrets detection using **TruffleHog OSS** across both local developer tooling and GitHub Actions CI pipelines:
1. **Zero-Setup Developer Tooling**: A self-contained script (`scripts/run_secrets_scan.sh`) that checks for local `trufflehog`, downloads the official release binary on demand if missing, and executes targeted scans.
2. **Verified Credentials Filtering (`--only-verified`)**: Eliminates noise and false positives by verifying cryptographic secrets against target providers before raising security alerts.
3. **Branch-Scoped CI Enforcement**: Prevents accidental scan failures on ephemeral branches while strictly gating commits and pull requests against the protected `main` branch.

---

## 2. Technical Architecture & Workflows

```
[ Developer Terminal / CI Runner ]
                │
                ├──► 1. Trigger: `make trufflehog-scan` or GitHub Actions
                │
                ├──► 2. Binary Auto-Detection & Provisioning
                │      • Check PATH for `trufflehog`
                │      • If absent: download official native binary to `.bin/trufflehog`
                │
                ├──► 3. Branch Verification
                │      • Detect active branch (git rev-parse)
                │      • On feature branches: log informational notice and pass cleanly
                │      • On `main` or pull requests targeting `main`: enforce strict audit
                │
                └──► 4. Secret Verification
                       • Run `trufflehog git file://. --only-verified --fail`
                       • Exit with code 0 on clean pass; non-zero on confirmed secret leak
```

---

## 3. Tooling & Interface Definitions

### 3.1 Makefile Targets
```makefile
trufflehog-scan:
	@./scripts/run_secrets_scan.sh

secret-scan-local: trufflehog-scan
```

### 3.2 CI Pipeline (`.github/workflows/security.yml`)
```yaml
  trufflehog-scan:
    name: TruffleHog Secret Leak Audit
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main' || github.base_ref == 'main'
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - name: Run TruffleHog OSS Scan
        uses: trufflesecurity/trufflehog@main
        with:
          path: ./
          base: ${{ github.event.repository.default_branch }}
          head: HEAD
          extra_args: --only-verified
```

---

## 4. Verification & Validation
1. `make trufflehog-scan` runs locally and passes cleanly with 0 verified secrets.
2. Verified secrets flag `--only-verified` filters out test hashes and mock IDs in test fixtures.
3. Feature branch workflow skips cleanly without breaking local developer velocity.
