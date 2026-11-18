# Specification: Flash Installation & One-Liner Distribution (Spec 169)

## Context & Objective
To make PaperPilot effortlessly accessible in 5 seconds without requiring users to have Rust (`cargo`), Node.js, or complex toolchains installed:
1. Provide a zero-dependency **Flash One-Liner Shell Installer** (`curl -fsSL https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.sh | bash`) for Linux and macOS.
2. Provide a **One-Liner PowerShell Installer** (`irm https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.ps1 | iex`) for Windows.
3. Update and verify package manager formulas (**Homebrew tap**, **Winget**, and **Flatpak**).
4. Update `ROADMAP.md` and `README.md` to showcase Flash Installation as the recommended zero-friction path.

## Components to Deliver
- `scripts/install.sh`: Enhanced, tested cross-platform bash/sh installer that automatically detects OS/arch, downloads release binary, verifies checksum, installs to `~/.local/bin` or `/usr/local/bin`, and tests execution.
- `scripts/install.ps1`: Windows PowerShell flash installer downloading Windows release zip and placing `paperpilot.exe` into user PATH.
- `packaging/homebrew/paperpilot.rb`: Updated formula ready for `homebrew-tap`.
- Documentation: Add Flash One-Liner instructions directly to `README.md` Quick Start.
