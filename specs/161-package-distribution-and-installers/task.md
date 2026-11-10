# Task 161A: Implement Universal Installers & Package Distribution Setup

## Checklist
- [ ] Create POSIX shell installer `scripts/install.sh` supporting macOS and Linux (with fallback to `~/.local/bin`)
- [ ] Create PowerShell installer `scripts/install.ps1` supporting Windows 10/11
- [ ] Create Homebrew formula template `packaging/homebrew/paperpilot.rb`
- [ ] Create Winget manifest template `packaging/winget/paperpilot.yaml`
- [ ] Add Release Action step to `.github/workflows/` that generates checksums and updates formula versions
- [ ] Document quick installation one-liners in `README.md` and documentation site
- [ ] Test `scripts/install.sh` in dry-run mode locally
