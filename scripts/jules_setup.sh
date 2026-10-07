#!/usr/bin/env bash
# PaperPilot Jules Cloud Environment Setup
# Lightweight, fast initialization (< 2s) for Jules cloud snapshots

echo "===> Registering required Rust targets"
rustup target add wasm32-unknown-unknown || true

echo "===> Verifying Preinstalled Developer Toolchains"
cargo --version
node --version || true
pnpm --version || true

echo "===> PaperPilot Jules environment ready"

