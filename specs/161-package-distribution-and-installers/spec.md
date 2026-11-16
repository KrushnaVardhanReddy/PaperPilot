# Spec 161: Package Distribution, Homebrew Formula, & Install Scripts

## 1. Executive Summary & Problem Statement
Currently, end users and AI agent developers install PaperPilot either by compiling from source via `cargo install --path .` or manually downloading archives from GitHub Releases.
To drive frictionless adoption across macOS, Linux, and Windows:
- macOS & Linux developers expect `brew install paperpilot` (or a tap: `brew install krushnavardhanreddy/tap/paperpilot`).
- Windows developers expect Winget / Scoop recipes.
- Shell-first users expect a verified one-liner installer: `curl -fsSL https://usepaperpilot.com/install.sh | sh`.

Spec 161 establishes official distribution channels and automated formula updates via GitHub Actions.

---

## 2. Deliverables & Specifications

### 2.1 Universal Install Script (`install.sh` / `install.ps1`)
Location: `scripts/install.sh` and `scripts/install.ps1`
- Auto-detects OS (`Darwin`, `Linux`, `Windows_NT`) and architecture (`x86_64`, `aarch64`).
- Queries the GitHub Releases API for the latest version tag.
- Downloads the matching release archive (`paperpilot-cli-<os>-<arch>.tar.gz` or `.zip`).
- Verifies SHA-256 checksum against `checksums.txt`.
- Installs binary to `/usr/local/bin` (or `~/.local/bin` / `%USERPROFILE%\.paperpilot\bin`).
- Verifies installation by running `paperpilot --version`.

### 2.2 Homebrew Formula (`Formula/paperpilot.rb`)
Location: `packaging/homebrew/paperpilot.rb`
```ruby
class Paperpilot < Formula
  desc "Privacy-first, pure-Rust multi-surface PDF & document intelligence toolkit"
  homepage "https://usepaperpilot.com"
  version "0.6.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/KrushnaVardhanReddy/PaperPilot/releases/download/v#{version}/paperpilot-cli-darwin-aarch64.tar.gz"
      sha256 "..."
    else
      url "https://github.com/KrushnaVardhanReddy/PaperPilot/releases/download/v#{version}/paperpilot-cli-darwin-x86_64.tar.gz"
      sha256 "..."
    end
  end

  on_linux do
    url "https://github.com/KrushnaVardhanReddy/PaperPilot/releases/download/v#{version}/paperpilot-cli-linux-x86_64.tar.gz"
    sha256 "..."
  end

  def install
    bin.install "paperpilot"
  end

  test do
    system "#{bin}/paperpilot", "--version"
  end
end
```

### 2.3 Automated Release Pipeline Update
Update `.github/workflows/release.yml` to automatically calculate SHA-256 sums and commit updated formula recipes to the Homebrew tap repository upon tagged release creation.
