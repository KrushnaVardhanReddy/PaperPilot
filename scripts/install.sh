#!/bin/sh

set -e

REPO="KrushnaVardhanReddy/PaperPilot"
GITHUB_API="https://api.github.com/repos/${REPO}/releases/latest"
GITHUB_DL="https://github.com/${REPO}/releases/download"

echo "Detecting operating system..."
OS=$(uname -s)
ARCH=$(uname -m)

TARGET=""
case "${OS}" in
    Linux)
        case "${ARCH}" in
            x86_64) TARGET="paperpilot-x86_64-unknown-linux-gnu" ;;
            aarch64|arm64) TARGET="paperpilot-aarch64-unknown-linux-gnu" ;;
            *) echo "Unsupported architecture: ${ARCH}"; exit 1 ;;
        esac
        ;;
    Darwin)
        case "${ARCH}" in
            x86_64) TARGET="paperpilot-x86_64-apple-darwin" ;;
            aarch64|arm64) TARGET="paperpilot-aarch64-apple-darwin" ;;
            *) echo "Unsupported architecture: ${ARCH}"; exit 1 ;;
        esac
        ;;
    *)
        echo "Unsupported OS: ${OS}"
        exit 1
        ;;
esac

echo "Target architecture identified as: ${TARGET}"

if [ -z "${VERSION}" ]; then
    echo "Fetching latest release version..."
    VERSION=$(curl -sSL "${GITHUB_API}" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
fi

if [ -z "${VERSION}" ]; then
    echo "Warning: Rate limited or unable to fetch from GitHub API. Defaulting to stable tag v0.6.1"
    VERSION="v0.6.1"
fi

echo "Installing version: ${VERSION}"

FILENAME="${TARGET}.tar.gz"
DOWNLOAD_URL="${GITHUB_DL}/${VERSION}/${FILENAME}"
CHECKSUM_URL="${DOWNLOAD_URL}.sha256"

TMP_DIR=$(mktemp -d)

if [ -n "${DRY_RUN}" ]; then
    echo "DRY_RUN mode enabled. Would have downloaded from ${DOWNLOAD_URL}"
    echo "Would have verified checksum from ${CHECKSUM_URL}"
    echo "Would have installed to /usr/local/bin or ~/.local/bin"
    rm -rf "${TMP_DIR}"
    exit 0
fi

cd "${TMP_DIR}" || exit 1

echo "Downloading ${FILENAME}..."
if ! curl -fL -sS -o "${FILENAME}" "${DOWNLOAD_URL}"; then
    echo "Failed to download ${DOWNLOAD_URL}"
    cd /
    rm -rf "${TMP_DIR}"
    exit 1
fi

echo "Downloading checksum file..."
if ! curl -fL -sS -o "${FILENAME}.sha256" "${CHECKSUM_URL}"; then
    echo "Warning: Checksum file not found, skipping checksum verification."
else
    echo "Verifying checksum..."
    EXPECTED_SUM=$(cat "${FILENAME}.sha256" | awk '{print $1}')
    if command -v sha256sum >/dev/null 2>&1; then
        ACTUAL_SUM=$(sha256sum "${FILENAME}" | awk '{print $1}')
    elif command -v shasum >/dev/null 2>&1; then
        ACTUAL_SUM=$(shasum -a 256 "${FILENAME}" | awk '{print $1}')
    else
        echo "Warning: No sha256sum or shasum found to verify checksum."
        ACTUAL_SUM=${EXPECTED_SUM}
    fi

    if [ -n "${EXPECTED_SUM}" ] && [ "${EXPECTED_SUM}" != "${ACTUAL_SUM}" ]; then
        echo "Checksum verification failed!"
        echo "Expected: ${EXPECTED_SUM}"
        echo "Actual:   ${ACTUAL_SUM}"
        cd /
        rm -rf "${TMP_DIR}"
        exit 1
    fi
    echo "Checksum OK."
fi

echo "Extracting ${FILENAME}..."
tar -xzf "${FILENAME}"

INSTALL_DIR="/usr/local/bin"
if [ "$(id -u)" -ne 0 ] && [ ! -w "${INSTALL_DIR}" ]; then
    INSTALL_DIR="${HOME}/.local/bin"
    mkdir -p "${INSTALL_DIR}"
fi

echo "Installing to ${INSTALL_DIR}..."
if [ -w "${INSTALL_DIR}" ]; then
    mv paperpilot "${INSTALL_DIR}/"
    chmod +x "${INSTALL_DIR}/paperpilot"
else
    sudo mv paperpilot "${INSTALL_DIR}/"
    sudo chmod +x "${INSTALL_DIR}/paperpilot"
fi

cd /
rm -rf "${TMP_DIR}"

echo "PaperPilot ${VERSION} has been successfully installed to ${INSTALL_DIR}/paperpilot"
echo "You can run it by typing: paperpilot"
if [ "${INSTALL_DIR}" = "${HOME}/.local/bin" ]; then
    echo "Make sure ${HOME}/.local/bin is in your PATH."
fi
