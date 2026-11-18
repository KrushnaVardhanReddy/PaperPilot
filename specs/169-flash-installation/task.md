# Tasks: Flash Installation & One-Liner Distribution (Spec 169)

- [ ] 1. Enhance and verify `scripts/install.sh` <!-- id: 1 -->
  - Test dry-run and release resolution for Linux/macOS.
  - Support fallback to precompiled binaries in GitHub releases.
  - Set execute permissions and verify PATH advice.

- [ ] 2. Create Windows PowerShell Flash Installer `scripts/install.ps1` <!-- id: 2 -->
  - Detect Windows architecture (x64 / arm64).
  - Download Windows release zip from GitHub Releases.
  - Extract `paperpilot.exe` to `$HOME\.paperpilot\bin` and add to user PATH if not present.

- [ ] 3. Update Homebrew formula `packaging/homebrew/paperpilot.rb` <!-- id: 3 -->
  - Update version to current release target (`0.7.0`).
  - Configure multi-arch URLs (macOS Apple Silicon, macOS Intel, Linux x64).

- [ ] 4. Add Flash Installation to `README.md` <!-- id: 4 -->
  - Put flash install one-liners at the top of the Quick Start section for instant user adoption.

- [ ] 5. Run tests, graphify, and commit <!-- id: 5 -->
  - Test scripts locally with `DRY_RUN=1`.
  - Run `graphify-rs build`.
  - Commit and push to `feature/flash-installation`.
