#!/bin/sh
# PaperPilot Flash Installer for Linux & macOS
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.sh | bash
#
# Pin a specific version (leading 'v' is optional and normalized):
#   curl -fsSL https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.sh | VERSION=0.7.0 bash
#   curl -fsSL https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.sh | bash -s -- 0.7.0
#   bash scripts/install.sh 0.7.0
#   bash scripts/install.sh v0.7.0
#
# Environment variables:
#   VERSION / PAPERPILOT_VERSION  Release version or tag to install (default: latest)
#   INSTALL_DIR                   Override binary installation directory
#   DRY_RUN=1                     Print actions without downloading or modifying disk

set -eu

REPO="KrushnaVardhanReddy/PaperPilot"
GITHUB_RELEASES_API="https://api.github.com/repos/${REPO}/releases"
GITHUB_BASE="${GITHUB_BASE:-https://github.com/${REPO}}"
GITHUB_DL="${GITHUB_DL:-${GITHUB_BASE}/releases/download}"
FALLBACK_VERSION="v0.7.0"
TMP_DIR=""

usage() {
    cat <<'EOF'
PaperPilot Flash Installer (Linux & macOS)

Usage:
  install.sh [OPTIONS] [VERSION]

Arguments:
  VERSION                    Version or tag to install (e.g. 0.7.0, v0.7.0, latest)

Options:
  -v, --version VERSION      Specify version or tag to install
  -h, --help                 Show this help message and exit

Environment Variables:
  VERSION                    Version or tag to install (overridden by argument)
  PAPERPILOT_VERSION         Version or tag to install (overridden by argument)
  INSTALL_DIR                Custom installation directory (default: ~/.local/bin, or /usr/local/bin as root)
  DRY_RUN=1                  Simulate installation without downloading or modifying files
EOF
}

cleanup() {
    if [ -n "${TMP_DIR}" ] && [ -d "${TMP_DIR}" ]; then
        rm -rf "${TMP_DIR}"
    fi
}

trap cleanup EXIT INT TERM

normalize_version() {
    val="$1"
    if [ -z "${val}" ] || [ "${val}" = "latest" ]; then
        printf '%s' ""
        return 0
    fi
    val="${val#v}"
    val="${val#V}"
    printf 'v%s' "${val}"
}

download_file() {
    url="$1"
    dest="$2"
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL -o "${dest}" "${url}"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO "${dest}" "${url}"
    else
        echo "Error: Neither 'curl' nor 'wget' is available to download files." >&2
        return 1
    fi
}

compute_sha256() {
    file_path="$1"
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "${file_path}" | awk '{print $1}' | tr '[:upper:]' '[:lower:]'
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 "${file_path}" | awk '{print $1}' | tr '[:upper:]' '[:lower:]'
    elif command -v openssl >/dev/null 2>&1; then
        openssl dgst -sha256 "${file_path}" | awk '{print $NF}' | tr '[:upper:]' '[:lower:]'
    else
        printf '%s' ""
    fi
}

RAW_VERSION=""
while [ "$#" -gt 0 ]; do
    case "$1" in
        -h|--help)
            usage
            exit 0
            ;;
        -v|--version)
            if [ "$#" -lt 2 ]; then
                echo "Error: Option '$1' requires a version argument." >&2
                exit 1
            fi
            RAW_VERSION="$2"
            shift 2
            ;;
        --version=*)
            RAW_VERSION="${1#--version=}"
            shift
            ;;
        -*)
            echo "Error: Unknown option '$1'. Run with --help for usage." >&2
            exit 1
            ;;
        *)
            if [ -z "${RAW_VERSION}" ]; then
                RAW_VERSION="$1"
            else
                echo "Error: Unexpected argument '$1'. Run with --help for usage." >&2
                exit 1
            fi
            shift
            ;;
    esac
done

if [ -z "${RAW_VERSION}" ]; then
    RAW_VERSION="${PAPERPILOT_VERSION:-${VERSION:-}}"
fi

VERSION=$(normalize_version "${RAW_VERSION}")
if [ -n "${VERSION}" ]; then
    case "${VERSION}" in
        *[!A-Za-z0-9._+-]*)
            echo "Error: Invalid version string '${RAW_VERSION}'." >&2
            exit 1
            ;;
    esac
fi

echo "Detecting operating system and architecture..."
OS=$(uname -s)
ARCH=$(uname -m)

RUST_TARGET=""
CANDIDATES=""
case "${OS}" in
    Linux)
        case "${ARCH}" in
            x86_64|amd64)
                RUST_TARGET="x86_64-unknown-linux-gnu"
                CANDIDATES="paperpilot-cli-ubuntu-22.04-${RUST_TARGET}.tar.gz paperpilot-cli-ubuntu-latest-${RUST_TARGET}.tar.gz paperpilot-${RUST_TARGET}.tar.gz paperpilot-cli-linux-x86_64.tar.gz paperpilot-all-ubuntu-22.04-${RUST_TARGET}.tar.gz paperpilot-all-ubuntu-latest-${RUST_TARGET}.tar.gz"
                ;;
            aarch64|arm64)
                RUST_TARGET="aarch64-unknown-linux-gnu"
                CANDIDATES="paperpilot-cli-ubuntu-22.04-${RUST_TARGET}.tar.gz paperpilot-cli-ubuntu-latest-${RUST_TARGET}.tar.gz paperpilot-${RUST_TARGET}.tar.gz paperpilot-cli-linux-aarch64.tar.gz paperpilot-all-ubuntu-22.04-${RUST_TARGET}.tar.gz paperpilot-all-ubuntu-latest-${RUST_TARGET}.tar.gz"
                ;;
            *)
                echo "Unsupported Linux architecture: ${ARCH}" >&2
                exit 1
                ;;
        esac
        ;;
    Darwin)
        case "${ARCH}" in
            aarch64|arm64)
                RUST_TARGET="aarch64-apple-darwin"
                CANDIDATES="paperpilot-cli-macos-latest-${RUST_TARGET}.tar.gz paperpilot-${RUST_TARGET}.tar.gz paperpilot-cli-darwin-aarch64.tar.gz paperpilot-all-macos-latest-${RUST_TARGET}.tar.gz"
                ;;
            x86_64)
                RUST_TARGET="x86_64-apple-darwin"
                CANDIDATES="paperpilot-cli-macos-latest-${RUST_TARGET}.tar.gz paperpilot-${RUST_TARGET}.tar.gz paperpilot-cli-darwin-x86_64.tar.gz paperpilot-all-macos-latest-${RUST_TARGET}.tar.gz"
                ;;
            *)
                echo "Unsupported macOS architecture: ${ARCH}" >&2
                exit 1
                ;;
        esac
        ;;
    MINGW*|MSYS*|CYGWIN*)
        echo "Windows shell detected (${OS}). Please use the PowerShell installer instead:" >&2
        echo "  irm https://raw.githubusercontent.com/${REPO}/main/scripts/install.ps1 | iex" >&2
        exit 1
        ;;
    *)
        echo "Unsupported operating system: ${OS}" >&2
        exit 1
        ;;
esac

echo "Detected platform: ${OS} (${ARCH}) -> ${RUST_TARGET}"

if [ -z "${VERSION}" ]; then
    echo "Fetching latest release version..."
    if command -v curl >/dev/null 2>&1; then
        VERSION=$(curl -fsSL --connect-timeout 5 --max-time 10 "${GITHUB_RELEASES_API}" 2>/dev/null \
            | grep '"tag_name":' | head -n 1 | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' || true)
        if [ -z "${VERSION}" ]; then
            REDIRECT_URL=$(curl -fsSLI --connect-timeout 5 --max-time 10 -o /dev/null -w '%{url_effective}' "${GITHUB_BASE}/releases/latest" 2>/dev/null || true)
            case "${REDIRECT_URL}" in
                */releases/tag/*)
                    VERSION="${REDIRECT_URL##*/releases/tag/}"
                    ;;
            esac
        fi
    elif command -v wget >/dev/null 2>&1; then
        VERSION=$(wget -qO- --timeout=10 "${GITHUB_RELEASES_API}" 2>/dev/null \
            | grep '"tag_name":' | head -n 1 | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' || true)
    fi

    if [ -z "${VERSION}" ]; then
        echo "Warning: Rate limited or unable to reach GitHub API. Defaulting to stable tag ${FALLBACK_VERSION}"
        VERSION="${FALLBACK_VERSION}"
    fi
fi

echo "Installing PaperPilot version: ${VERSION}"

if [ -z "${INSTALL_DIR:-}" ]; then
    if [ "$(id -u)" -eq 0 ]; then
        INSTALL_DIR="/usr/local/bin"
    else
        INSTALL_DIR="${HOME}/.local/bin"
    fi
fi

PRIMARY_FILENAME=$(printf '%s' "${CANDIDATES}" | awk '{print $1}')
PRIMARY_URL="${GITHUB_DL}/${VERSION}/${PRIMARY_FILENAME}"
PRIMARY_CHECKSUM_URL="${PRIMARY_URL}.sha256"

if [ -n "${DRY_RUN:-}" ] && [ "${DRY_RUN}" != "0" ]; then
    echo "DRY_RUN mode enabled."
    echo "  Target:       ${RUST_TARGET}"
    echo "  Version:      ${VERSION}"
    echo "  Primary URL:  ${PRIMARY_URL}"
    echo "  Checksum URL: ${PRIMARY_CHECKSUM_URL}"
    echo "  Install path: ${INSTALL_DIR}/paperpilot"
    echo "  Verification: ${INSTALL_DIR}/paperpilot --version"
    exit 0
fi

TMP_DIR=$(mktemp -d)

SELECTED_FILE=""
SELECTED_URL=""
for CANDIDATE in ${CANDIDATES}; do
    CANDIDATE_URL="${GITHUB_DL}/${VERSION}/${CANDIDATE}"
    echo "Downloading ${CANDIDATE}..."
    if download_file "${CANDIDATE_URL}" "${TMP_DIR}/${CANDIDATE}" 2>/dev/null; then
        SELECTED_FILE="${CANDIDATE}"
        SELECTED_URL="${CANDIDATE_URL}"
        break
    fi
done

if [ -z "${SELECTED_FILE}" ]; then
    echo "Error: Failed to download a precompiled release asset for ${RUST_TARGET} (${VERSION})." >&2
    echo "Checked base URL: ${GITHUB_DL}/${VERSION}/" >&2
    echo "You can also build from source with: cargo install --git ${GITHUB_BASE} paperpilot-cli" >&2
    exit 1
fi

CHECKSUM_URL="${SELECTED_URL}.sha256"
CHECKSUM_FILE="${TMP_DIR}/${SELECTED_FILE}.sha256"
echo "Checking SHA-256 checksum..."
if download_file "${CHECKSUM_URL}" "${CHECKSUM_FILE}" 2>/dev/null && [ -s "${CHECKSUM_FILE}" ]; then
    EXPECTED_SUM=$(awk '{print $1}' "${CHECKSUM_FILE}" | head -n 1 | tr '[:upper:]' '[:lower:]')
    ACTUAL_SUM=$(compute_sha256 "${TMP_DIR}/${SELECTED_FILE}")
    if [ -z "${ACTUAL_SUM}" ]; then
        echo "Warning: No SHA-256 tool (sha256sum, shasum, openssl) found; skipping checksum verification."
    elif [ -n "${EXPECTED_SUM}" ] && [ "${EXPECTED_SUM}" != "${ACTUAL_SUM}" ]; then
        echo "Error: Checksum verification failed!" >&2
        echo "  Expected: ${EXPECTED_SUM}" >&2
        echo "  Actual:   ${ACTUAL_SUM}" >&2
        exit 1
    else
        echo "Checksum verified."
    fi
else
    echo "Warning: Checksum file not found at ${CHECKSUM_URL}; skipping checksum verification."
fi

EXTRACT_DIR="${TMP_DIR}/extract"
mkdir -p "${EXTRACT_DIR}"
echo "Extracting ${SELECTED_FILE}..."
tar -xzf "${TMP_DIR}/${SELECTED_FILE}" -C "${EXTRACT_DIR}"

CLI_BIN=$(find "${EXTRACT_DIR}" -maxdepth 3 -type f \( -name "paperpilot" -o -name "paperpilot-cli" \) | head -n 1 || true)
if [ -z "${CLI_BIN}" ]; then
    echo "Error: 'paperpilot' executable not found in extracted archive." >&2
    exit 1
fi

echo "Installing to ${INSTALL_DIR}..."
TARGET_BIN="${INSTALL_DIR}/paperpilot"
if mkdir -p "${INSTALL_DIR}" 2>/dev/null && [ -w "${INSTALL_DIR}" ]; then
    cp "${CLI_BIN}" "${TARGET_BIN}"
    chmod 0755 "${TARGET_BIN}"
elif command -v sudo >/dev/null 2>&1; then
    sudo mkdir -p "${INSTALL_DIR}"
    sudo cp "${CLI_BIN}" "${TARGET_BIN}"
    sudo chmod 0755 "${TARGET_BIN}"
else
    echo "Error: Cannot write to ${INSTALL_DIR} and 'sudo' is unavailable." >&2
    exit 1
fi

if [ "${OS}" = "Darwin" ] && command -v xattr >/dev/null 2>&1; then
    xattr -d com.apple.quarantine "${TARGET_BIN}" 2>/dev/null || true
fi

echo "Validating installation..."
VERSION_OUT=$("${TARGET_BIN}" --version)
echo "Verified: ${VERSION_OUT}"

echo "PaperPilot ${VERSION} has been successfully installed to ${TARGET_BIN}"
case ":${PATH}:" in
    *":${INSTALL_DIR}:"*)
        echo "Run 'paperpilot --help' to get started."
        ;;
    *)
        echo "Note: ${INSTALL_DIR} is not currently in your PATH."
        echo "Add it to your shell profile (~/.bashrc, ~/.zshrc, or ~/.profile):"
        echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
        ;;
esac
