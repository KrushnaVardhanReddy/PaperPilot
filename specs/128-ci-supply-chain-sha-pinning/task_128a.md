# Task 128A: CI Supply Chain Hardening & Action SHA Pinning

## 1. Overview
Remediates the CI supply chain vulnerabilities flagged by `devy` on Hacker News. All third-party GitHub Actions across `.github/workflows/` must be pinned to immutable 40-character commit hashes with inline version comments, unverified installer scripts replaced, and `.github/dependabot.yml` configured to track GitHub Action updates while preserving SHA pinning.

---

## 2. Target Files & Detailed Changes

### File 1: `.github/workflows/security.yml`
Replace mutable action version tags with immutable 40-character commit SHAs:
- `actions/checkout@v4` -> `actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2`
- `github/codeql-action/upload-sarif@v3` -> `github/codeql-action/upload-sarif@48ab28a6f5dab2a7254580218764ddb44ecddb90 # v3.28.1`
- `dtolnay/rust-toolchain@stable` -> `dtolnay/rust-toolchain@a54c7fe9363c9033d27f38488693fb106733fe4c # stable`

### File 2: `.github/workflows/rust.yml`
Replace mutable action version tags:
- `actions/checkout@v4` -> `actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2`
- `dtolnay/rust-toolchain@stable` -> `dtolnay/rust-toolchain@a54c7fe9363c9033d27f38488693fb106733fe4c # stable`
- `swatinem/rust-cache@v2` -> `swatinem/rust-cache@f0deed1e0edfc6a9be95417271a02b6009b1bed8 # v2.7.7`
- `actions/upload-artifact@v4` -> `actions/upload-artifact@4cec3d8aa04e39d1a68397de0c4cd6fb99ba841c # v4.6.1`

### File 3: `.github/workflows/release.yml`
Replace mutable action version tags:
- `actions/checkout@v4` -> `actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2`
- `dtolnay/rust-toolchain@stable` -> `dtolnay/rust-toolchain@a54c7fe9363c9033d27f38488693fb106733fe4c # stable`
- `swatinem/rust-cache@v2` -> `swatinem/rust-cache@f0deed1e0edfc6a9be95417271a02b6009b1bed8 # v2.7.7`
- `actions/upload-artifact@v4` -> `actions/upload-artifact@4cec3d8aa04e39d1a68397de0c4cd6fb99ba841c # v4.6.1`
- `actions/download-artifact@v4` -> `actions/download-artifact@cc203385981b70ca67e1cc392babf9cc229d5806 # v4.1.9`
- `softprops/action-gh-release@v2` -> `softprops/action-gh-release@c062e08bd532815e2082a85e87e3ef29c3e6d191 # v2.0.8`
- `actions/setup-node@v4` -> `actions/setup-node@1e60f620b9541d16bece96c5465dc8ee9832be0b # v4.0.3`
- `pnpm/action-setup@v4` -> `pnpm/action-setup@a3252b78c470c02df07e9d79298aa2cff490e5da # v4.0.0`
- `tauri-apps/tauri-action@v0` -> `tauri-apps/tauri-action@b7f08b3a726beea8655099308e9ca85b37651dd0 # v0.5.17`

**Safe Installer Fix**:
- Line 178: Replace raw `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh` with:
  ```bash
  which wasm-pack || cargo install wasm-pack --locked
  ```

### File 4: `.github/dependabot.yml`
Create `.github/dependabot.yml` with:
```yaml
version: 2
updates:
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
    commit-message:
      prefix: "ci"
      include: "scope"
```

---

## 3. Verification Commands
1. Verify YAML syntax across workflows:
   ```bash
   python3 -c "import yaml; [yaml.safe_load(open(f)) for f in ['.github/workflows/security.yml', '.github/workflows/rust.yml', '.github/workflows/release.yml', '.github/dependabot.yml']]"
   ```
2. Check git diff ensuring no build targets or runtime logic were accidentally modified.
