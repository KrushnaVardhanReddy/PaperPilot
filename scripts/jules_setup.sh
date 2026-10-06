#!/usr/bin/env bash
set -euo pipefail

echo "===> Installing Core System Build Tools & Python Prereqs"
sudo apt-get update -y
sudo apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    build-essential \
    curl \
    python3 \
    python3-pip

# Ensure Python requirements for test_tri_interface_e2e.py are present
pip3 install --quiet --upgrade requests psutil || true

echo "===> Installing wasm-pack & wasm32 target"
if ! command -v wasm-pack &> /dev/null; then
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh || true
fi
rustup target add wasm32-unknown-unknown || true

echo "===> Pre-building Rust Core, PDF Engine & Gateway"
cargo check -p paperpilot-pdf -p paperpilot-cli -p paperpilot-mcp -p paperpilot-gateway

echo "===> Environment setup and dependency pre-caching complete!"
