# Mobile Setup Guide

This document outlines the prerequisites and steps to run PaperPilot on Android and iOS using Tauri 2.

## Architectural Note

The mobile builds of PaperPilot exclude `paperpilot-gateway` (Axum HTTP REST server) and `paperpilot-mcp` (Model Context Protocol JSON-RPC server) to reduce binary size by 10-20MB, prevent battery drain, and comply with mobile OS network restrictions. This is handled via conditional compilation feature gating in Cargo (`--no-default-features`).

Native PDF operations are dispatched directly through `process_mobile_buffer` in `mobile_bridge.rs`.

## Prerequisites

### Android

1. Install **Android Studio**.
2. Install the **Android SDK (version 34)** and **NDK (version 26+)**.
3. Install Rust cross-compilation targets:
   ```bash
   rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
   ```
4. Make sure your `ANDROID_HOME` and `NDK_HOME` environment variables are properly configured.

### iOS

1. Install **macOS**.
2. Install **Xcode 15+**.
3. Install **CocoaPods** (`sudo gem install cocoapods`).
4. Install Rust cross-compilation targets:
   ```bash
   rustup target add aarch64-apple-ios aarch64-apple-ios-sim
   ```

## Initialization & Testing

To initialize the mobile platform and generate native scaffoldings:

```bash
cd apps/desktop
pnpm tauri android init
pnpm tauri ios init
```

To run the application on an emulator or connected device:

```bash
cd apps/desktop
pnpm tauri android dev
pnpm tauri ios dev
```

The Tauri CLI will automatically handle compiling the rust core and bridging with the mobile frontend.
