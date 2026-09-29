#!/usr/bin/env bash
# Build portable distribution artifacts for all platforms
# Run this on the target platform (Linux for AppImage, macOS for dmg, Windows for NSIS)

set -e

echo "🚀 Building PaperPilot distribution artifacts..."

cd "$(dirname "$0")"

# Install JS dependencies
npm install

# Build the frontend
npm run build

# Build the Tauri app with bundler
npm run tauri build

echo ""
echo "✅ Build complete! Artifacts available in:"
echo "   src-tauri/target/release/bundle/"
