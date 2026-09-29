# Building PaperPilot Distribution Packages

## Prerequisites
- Rust (stable) + Cargo
- Node.js 20+
- Platform-specific requirements:
  - **Linux (AppImage + .deb):** `libwebkit2gtk-4.1-dev`, `libssl-dev`, `librsvg2-dev`
  - **macOS (.dmg):** Xcode Command Line Tools
  - **Windows (.exe/.msi):** Visual Studio Build Tools

## Build

```bash
cd apps/desktop
chmod +x build-portable.sh
./build-portable.sh
```

## Output Locations

| Platform | Target | Output path |
|---|---|---|
| Linux | AppImage | `src-tauri/target/release/bundle/appimage/` |
| Linux | .deb | `src-tauri/target/release/bundle/deb/` |
| macOS | .dmg | `src-tauri/target/release/bundle/dmg/` |
| Windows | NSIS installer | `src-tauri/target/release/bundle/nsis/` |
| Windows | MSI | `src-tauri/target/release/bundle/msi/` |

## CI/CD (GitHub Actions)

Add a matrix build job that runs on `ubuntu-latest`, `macos-latest`, and `windows-latest` to produce all artifacts in one pipeline.
