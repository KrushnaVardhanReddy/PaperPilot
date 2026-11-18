#!/bin/sh
# PaperPilot Flash Installer for Linux & macOS
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.sh | bash
#
# Component selection:
#   --desktop   Install Desktop GUI App (default when GUI/interactive)
#   --cli       Install standalone CLI binary only
#   --all       Install Desktop App + CLI + MCP Server
#
# Pin a specific version (leading 'v' is optional and normalized):
#   curl -fsSL https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.sh | VERSION=0.7.0 bash
#   curl -fsSL https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.sh | bash -s -- --all 0.7.0
#   bash scripts/install.sh --cli 0.7.0
#
# Environment variables:
#   VERSION / PAPERPILOT_VERSION      Release version or tag to install (default: latest)
#   COMPONENT / PAPERPILOT_COMPONENT  Component to install: desktop, cli, or all
#   INSTALL_DIR                       Override binary installation directory
#   DRY_RUN=1                         Print actions without downloading or modifying disk
#   NONINTERACTIVE=1                  Skip interactive component prompt

set -eu

REPO="KrushnaVardhanReddy/PaperPilot"
GITHUB_RELEASES_API="https://api.github.com/repos/${REPO}/releases"
GITHUB_BASE="${GITHUB_BASE:-https://github.com/${REPO}}"
GITHUB_DL="${GITHUB_DL:-${GITHUB_BASE}/releases/download}"
GITHUB_RAW="${GITHUB_RAW:-https://raw.githubusercontent.com/${REPO}}"
FALLBACK_VERSION="v0.7.0"
TMP_DIR=""
MOUNT_DIR=""

usage() {
    cat <<'EOF'
PaperPilot Flash Installer (Linux & macOS)

Usage:
  install.sh [OPTIONS] [VERSION]

Arguments:
  VERSION                    Version or tag to install (e.g. 0.7.0, v0.7.0, latest)

Component Options:
  --desktop                  Install Desktop GUI App (Default on GUI / interactive)
  --cli                      Install standalone 'paperpilot' CLI binary only
  --all                      Install Desktop App + CLI + MCP Server
  --component COMPONENT      Select component explicitly: desktop, cli, or all

General Options:
  -v, --version VERSION      Specify version or tag to install
  -n, --dry-run              Simulate installation without downloading or modifying files
  -y, --yes                  Skip interactive prompt and use default component
  -h, --help                 Show this help message and exit

Environment Variables:
  VERSION / PAPERPILOT_VERSION       Version or tag to install (overridden by argument)
  COMPONENT / PAPERPILOT_COMPONENT   Component to install: desktop, cli, or all
  INSTALL_DIR                        Custom binary directory (default: ~/.local/bin, or /usr/local/bin as root)
  DRY_RUN=1                          Simulate installation without downloading or modifying files
  NONINTERACTIVE=1                   Skip interactive prompt and use default component
EOF
}

cleanup() {
    if [ -n "${MOUNT_DIR}" ] && [ -d "${MOUNT_DIR}" ]; then
        hdiutil detach "${MOUNT_DIR}" -quiet 2>/dev/null || hdiutil detach "${MOUNT_DIR}" -force -quiet 2>/dev/null || true
    fi
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

normalize_component() {
    val=$(printf '%s' "$1" | tr '[:upper:]' '[:lower:]' | tr -d '[:space:]')
    case "${val}" in
        desktop|gui)
            printf '%s' "desktop"
            ;;
        cli)
            printf '%s' "cli"
            ;;
        all|all-in-one|suite)
            printf '%s' "all"
            ;;
        *)
            return 1
            ;;
    esac
}

set_component_arg() {
    new_comp="$1"
    if [ -n "${COMPONENT_ARG}" ] && [ "${COMPONENT_ARG}" != "${new_comp}" ]; then
        echo "Error: Conflicting component options specified ('${COMPONENT_ARG}' and '${new_comp}')." >&2
        exit 1
    fi
    COMPONENT_ARG="${new_comp}"
}

first_word() {
    printf '%s' "$1" | awk '{print $1}'
}

download_file() {
    url="$1"
    dest="$2"
    if command -v curl >/dev/null 2>&1; then
        if ! curl -fsSL -o "${dest}" "${url}"; then
            rm -f "${dest}"
            return 1
        fi
    elif command -v wget >/dev/null 2>&1; then
        if ! wget -qO "${dest}" "${url}"; then
            rm -f "${dest}"
            return 1
        fi
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

verify_checksum() {
    asset_url="$1"
    local_file="$2"
    checksum_url="${asset_url}.sha256"
    checksum_file="${local_file}.sha256"

    echo "Checking SHA-256 checksum..."
    if download_file "${checksum_url}" "${checksum_file}" 2>/dev/null && [ -s "${checksum_file}" ]; then
        expected_sum=$(awk '{print $1}' "${checksum_file}" | head -n 1 | tr '[:upper:]' '[:lower:]')
        actual_sum=$(compute_sha256 "${local_file}")
        if [ -z "${actual_sum}" ]; then
            echo "Warning: No SHA-256 tool (sha256sum, shasum, openssl) found; skipping checksum verification."
        elif [ -n "${expected_sum}" ] && [ "${expected_sum}" != "${actual_sum}" ]; then
            echo "Error: Checksum verification failed!" >&2
            echo "  Expected: ${expected_sum}" >&2
            echo "  Actual:   ${actual_sum}" >&2
            exit 1
        else
            echo "Checksum verified."
        fi
    else
        echo "Warning: Checksum file not found at ${checksum_url}; skipping checksum verification."
    fi
}

download_first_candidate() {
    candidates_list="$1"
    DL_FILE=""
    DL_URL=""
    for candidate in ${candidates_list}; do
        candidate_url="${GITHUB_DL}/${VERSION}/${candidate}"
        echo "Downloading ${candidate}..."
        if download_file "${candidate_url}" "${TMP_DIR}/${candidate}" 2>/dev/null; then
            DL_FILE="${candidate}"
            DL_URL="${candidate_url}"
            verify_checksum "${DL_URL}" "${TMP_DIR}/${DL_FILE}"
            return 0
        fi
    done
    return 1
}

install_file_with_mode() {
    src_file="$1"
    dest_dir="$2"
    dest_name="$3"
    mode="$4"
    dest_path="${dest_dir}/${dest_name}"

    if mkdir -p "${dest_dir}" 2>/dev/null && [ -w "${dest_dir}" ]; then
        cp "${src_file}" "${dest_path}"
        chmod "${mode}" "${dest_path}"
    elif command -v sudo >/dev/null 2>&1; then
        sudo mkdir -p "${dest_dir}"
        sudo cp "${src_file}" "${dest_path}"
        sudo chmod "${mode}" "${dest_path}"
    else
        echo "Error: Cannot write to ${dest_dir} and 'sudo' is unavailable." >&2
        exit 1
    fi
}

install_binary() {
    src_bin="$1"
    dest_name="$2"
    dest_path="${INSTALL_DIR}/${dest_name}"

    echo "Installing ${dest_name} to ${INSTALL_DIR}..."
    install_file_with_mode "${src_bin}" "${INSTALL_DIR}" "${dest_name}" "0755"

    if [ "${OS}" = "Darwin" ] && command -v xattr >/dev/null 2>&1; then
        xattr -d com.apple.quarantine "${dest_path}" 2>/dev/null || true
    fi
}

create_symlink_in_install_dir() {
    target_name="$1"
    link_name="$2"
    link_path="${INSTALL_DIR}/${link_name}"

    if [ -w "${INSTALL_DIR}" ]; then
        ln -sf "${target_name}" "${link_path}" 2>/dev/null || true
    elif command -v sudo >/dev/null 2>&1; then
        sudo ln -sf "${target_name}" "${link_path}" 2>/dev/null || true
    fi
}

install_linux_desktop_icon() {
    icon_tmp="${TMP_DIR}/paperpilot-icon.png"
    appimage_path="$1"

    if download_file "${GITHUB_RAW}/${VERSION}/apps/desktop/src-tauri/icons/icon.png" "${icon_tmp}" 2>/dev/null && [ -s "${icon_tmp}" ]; then
        install_file_with_mode "${icon_tmp}" "${ICON_DIR}" "paperpilot.png" "0644"
        return 0
    fi

    if download_file "${GITHUB_RAW}/main/apps/desktop/src-tauri/icons/icon.png" "${icon_tmp}" 2>/dev/null && [ -s "${icon_tmp}" ]; then
        install_file_with_mode "${icon_tmp}" "${ICON_DIR}" "paperpilot.png" "0644"
        return 0
    fi

    # Fallback: try extracting embedded icon from AppImage without requiring FUSE
    if [ -n "${appimage_path}" ] && [ -x "${appimage_path}" ]; then
        icon_extract_dir="${TMP_DIR}/appimage_icon"
        mkdir -p "${icon_extract_dir}"
        if (cd "${icon_extract_dir}" && APPIMAGE_EXTRACT_AND_RUN=1 "${appimage_path}" --appimage-extract "*.png" >/dev/null 2>&1); then
            found_icon=$(find "${icon_extract_dir}/squashfs-root" -maxdepth 4 -type f -name "*.png" | head -n 1 || true)
            if [ -n "${found_icon}" ] && [ -s "${found_icon}" ]; then
                install_file_with_mode "${found_icon}" "${ICON_DIR}" "paperpilot.png" "0644"
                return 0
            fi
        fi
    fi

    echo "Warning: Could not download desktop icon; desktop entry will reference 'paperpilot' theme icon."
    return 1
}

write_linux_desktop_entry() {
    exec_bin="$1"
    has_icon="$2"
    entry_tmp="${TMP_DIR}/paperpilot.desktop"

    case "${exec_bin}" in
        *" "*) desktop_exec="\"${exec_bin}\" %U" ;;
        *) desktop_exec="${exec_bin} %U" ;;
    esac

    if [ "${has_icon}" -eq 1 ] && [ -f "${ICON_FILE}" ]; then
        icon_value="${ICON_FILE}"
    else
        icon_value="paperpilot"
    fi

    cat > "${entry_tmp}" <<EOF
[Desktop Entry]
Name=PaperPilot
Comment=Privacy-first, local PDF and document manipulation toolkit
Exec=${desktop_exec}
Icon=${icon_value}
Terminal=false
Type=Application
Categories=Office;Utility;Publishing;
MimeType=application/pdf;application/epub+zip;image/png;image/jpeg;
StartupWMClass=PaperPilot
Keywords=PDF;Editor;Merge;Split;Compress;OCR;Sign;Forms;
EOF

    echo "Creating desktop launcher at ${DESKTOP_ENTRY_FILE}..."
    install_file_with_mode "${entry_tmp}" "${APPLICATIONS_DIR}" "paperpilot.desktop" "0644"

    if command -v update-desktop-database >/dev/null 2>&1; then
        update-desktop-database "${APPLICATIONS_DIR}" >/dev/null 2>&1 || true
    fi
    if command -v gtk-update-icon-cache >/dev/null 2>&1; then
        gtk-update-icon-cache -f -t "${XDG_DATA_TARGET}/icons/hicolor" >/dev/null 2>&1 || true
    fi
}

install_desktop_linux() {
    if ! download_first_candidate "${DESKTOP_CANDIDATES}"; then
        echo "Error: Failed to download PaperPilot Desktop package for ${RUST_TARGET} (${VERSION})." >&2
        echo "Checked base URL: ${GITHUB_DL}/${VERSION}/" >&2
        exit 1
    fi

    downloaded_path="${TMP_DIR}/${DL_FILE}"
    case "${DL_FILE}" in
        *.AppImage)
            install_binary "${downloaded_path}" "paperpilot-desktop"
            # Allow 'paperpilot ui' to resolve the AppImage when CLI is installed alongside it
            create_symlink_in_install_dir "paperpilot-desktop" "PaperPilot.AppImage"

            has_icon=0
            if install_linux_desktop_icon "${DESKTOP_BIN}"; then
                has_icon=1
            fi
            write_linux_desktop_entry "${DESKTOP_BIN}" "${has_icon}"

            if [ ! -x "${DESKTOP_BIN}" ]; then
                echo "Error: Installed desktop binary '${DESKTOP_BIN}' is not executable." >&2
                exit 1
            fi
            echo "Verified Desktop AppImage at ${DESKTOP_BIN}"
            echo "Verified Desktop Entry at ${DESKTOP_ENTRY_FILE}"

            if command -v ldconfig >/dev/null 2>&1; then
                if ! ldconfig -p 2>/dev/null | grep -q "libfuse\.so\.2"; then
                    echo "Note: AppImages may require FUSE2 on Ubuntu 22.04+/Debian (install with: sudo apt install libfuse2), or launch with APPIMAGE_EXTRACT_AND_RUN=1."
                fi
            fi
            ;;
        *.deb)
            if command -v dpkg >/dev/null 2>&1 && { [ "$(id -u)" -eq 0 ] || command -v sudo >/dev/null 2>&1; }; then
                echo "Installing Debian package ${DL_FILE}..."
                if [ "$(id -u)" -eq 0 ]; then
                    dpkg -i "${downloaded_path}" || { apt-get update && apt-get install -f -y; }
                else
                    sudo dpkg -i "${downloaded_path}" || { sudo apt-get update && sudo apt-get install -f -y; }
                fi
                echo "PaperPilot Desktop package (${DL_FILE}) installed via dpkg."
            else
                echo "Extracting ${DL_FILE} to ${INSTALL_DIR}..."
                deb_extract="${TMP_DIR}/deb_extract"
                mkdir -p "${deb_extract}"
                (cd "${deb_extract}" && ar x "${downloaded_path}" && tar -xf data.tar.*)
                deb_bin=$(find "${deb_extract}" -type f \( -name "PaperPilot" -o -name "paperpilot-desktop" -o -name "desktop" \) -path "*/bin/*" | head -n 1 || true)
                if [ -z "${deb_bin}" ]; then
                    echo "Error: Desktop executable not found inside ${DL_FILE}." >&2
                    exit 1
                fi
                install_binary "${deb_bin}" "paperpilot-desktop"
                create_symlink_in_install_dir "paperpilot-desktop" "PaperPilot.AppImage"
                has_icon=0
                if install_linux_desktop_icon ""; then
                    has_icon=1
                fi
                write_linux_desktop_entry "${DESKTOP_BIN}" "${has_icon}"
            fi
            ;;
        *)
            echo "Error: Unsupported Linux desktop package format '${DL_FILE}'." >&2
            exit 1
            ;;
    esac
}

install_desktop_macos() {
    if ! download_first_candidate "${DESKTOP_CANDIDATES}"; then
        echo "Error: Failed to download PaperPilot Desktop DMG for ${RUST_TARGET} (${VERSION})." >&2
        echo "Checked base URL: ${GITHUB_DL}/${VERSION}/" >&2
        exit 1
    fi

    dmg_path="${TMP_DIR}/${DL_FILE}"
    MOUNT_DIR="${TMP_DIR}/dmg_mount"
    mkdir -p "${MOUNT_DIR}"

    echo "Mounting ${DL_FILE}..."
    hdiutil attach "${dmg_path}" -mountpoint "${MOUNT_DIR}" -nobrowse -readonly -quiet

    app_bundle="${MOUNT_DIR}/PaperPilot.app"
    if [ ! -d "${app_bundle}" ]; then
        app_bundle=$(find "${MOUNT_DIR}" -maxdepth 2 -type d -name "*.app" | head -n 1 || true)
    fi

    if [ -z "${app_bundle}" ] || [ ! -d "${app_bundle}" ]; then
        echo "Error: PaperPilot.app not found inside mounted image ${DL_FILE}." >&2
        exit 1
    fi

    echo "Installing PaperPilot.app to ${MAC_APP_DIR}..."
    if mkdir -p "${MAC_APP_DIR}" 2>/dev/null && [ -w "${MAC_APP_DIR}" ]; then
        rm -rf "${MAC_APP_DEST}"
        if command -v ditto >/dev/null 2>&1; then
            ditto "${app_bundle}" "${MAC_APP_DEST}"
        else
            cp -R "${app_bundle}" "${MAC_APP_DEST}"
        fi
        if command -v xattr >/dev/null 2>&1; then
            xattr -rd com.apple.quarantine "${MAC_APP_DEST}" 2>/dev/null || true
        fi
    elif command -v sudo >/dev/null 2>&1; then
        sudo mkdir -p "${MAC_APP_DIR}"
        sudo rm -rf "${MAC_APP_DEST}"
        if command -v ditto >/dev/null 2>&1; then
            sudo ditto "${app_bundle}" "${MAC_APP_DEST}"
        else
            sudo cp -R "${app_bundle}" "${MAC_APP_DEST}"
        fi
        if command -v xattr >/dev/null 2>&1; then
            sudo xattr -rd com.apple.quarantine "${MAC_APP_DEST}" 2>/dev/null || true
        fi
    else
        echo "Error: Cannot write to ${MAC_APP_DIR} and 'sudo' is unavailable." >&2
        exit 1
    fi

    echo "Unmounting ${DL_FILE}..."
    hdiutil detach "${MOUNT_DIR}" -quiet 2>/dev/null || hdiutil detach "${MOUNT_DIR}" -force -quiet 2>/dev/null || true
    MOUNT_DIR=""

    if [ ! -d "${MAC_APP_DEST}" ]; then
        echo "Error: Verification failed — ${MAC_APP_DEST} was not found after installation." >&2
        exit 1
    fi
    echo "Verified: ${MAC_APP_DEST}"
}

install_desktop() {
    case "${OS}" in
        Linux)
            install_desktop_linux
            ;;
        Darwin)
            install_desktop_macos
            ;;
    esac
}

install_archive_binary() {
    candidates_list="$1"
    bin_primary="$2"
    bin_alt="$3"
    dest_name="$4"
    optional="$5"

    extract_dir="${TMP_DIR}/extract_${dest_name}"
    # Reuse already-extracted all-in-one archive if CLI fell back to paperpilot-all-*
    if [ -d "${TMP_DIR}/extract_paperpilot" ]; then
        cached_bin=$(find "${TMP_DIR}/extract_paperpilot" -maxdepth 3 -type f \( -name "${bin_primary}" -o -name "${bin_alt}" \) | head -n 1 || true)
        if [ -n "${cached_bin}" ]; then
            install_binary "${cached_bin}" "${dest_name}"
            echo "Validating ${dest_name} installation..."
            version_out=$("${INSTALL_DIR}/${dest_name}" --version)
            echo "Verified: ${version_out}"
            return 0
        fi
    fi

    if ! download_first_candidate "${candidates_list}"; then
        if [ "${optional}" = "1" ]; then
            echo "Warning: Could not download ${dest_name} archive for ${RUST_TARGET} (${VERSION}); skipping."
            return 0
        fi
        echo "Error: Failed to download a precompiled release asset for ${RUST_TARGET} (${VERSION})." >&2
        echo "Checked base URL: ${GITHUB_DL}/${VERSION}/" >&2
        echo "You can also build from source with: cargo install --git ${GITHUB_BASE} paperpilot-cli" >&2
        exit 1
    fi

    mkdir -p "${extract_dir}"
    echo "Extracting ${DL_FILE}..."
    tar -xzf "${TMP_DIR}/${DL_FILE}" -C "${extract_dir}"

    found_bin=$(find "${extract_dir}" -maxdepth 3 -type f \( -name "${bin_primary}" -o -name "${bin_alt}" \) | head -n 1 || true)
    if [ -z "${found_bin}" ]; then
        if [ "${optional}" = "1" ]; then
            echo "Warning: '${dest_name}' executable not found in ${DL_FILE}; skipping."
            return 0
        fi
        echo "Error: '${dest_name}' executable not found in extracted archive." >&2
        exit 1
    fi

    install_binary "${found_bin}" "${dest_name}"

    echo "Validating ${dest_name} installation..."
    version_out=$("${INSTALL_DIR}/${dest_name}" --version)
    echo "Verified: ${version_out}"
}

install_cli() {
    install_archive_binary "${CLI_CANDIDATES}" "paperpilot" "paperpilot-cli" "paperpilot" "0"
}

install_mcp() {
    install_archive_binary "${MCP_CANDIDATES}" "paperpilot-mcp" "paperpilot-mcp" "paperpilot-mcp" "1"
}

RAW_VERSION=""
COMPONENT_ARG=""
FORCE_NONINTERACTIVE="${PAPERPILOT_NONINTERACTIVE:-${NONINTERACTIVE:-0}}"
if [ -z "${FORCE_NONINTERACTIVE}" ]; then
    FORCE_NONINTERACTIVE="0"
fi

while [ "$#" -gt 0 ]; do
    case "$1" in
        -h|--help)
            usage
            exit 0
            ;;
        --desktop)
            set_component_arg "desktop"
            shift
            ;;
        --cli)
            set_component_arg "cli"
            shift
            ;;
        --all)
            set_component_arg "all"
            shift
            ;;
        --component)
            if [ "$#" -lt 2 ]; then
                echo "Error: Option '$1' requires a component argument (desktop, cli, or all)." >&2
                exit 1
            fi
            if ! norm_comp=$(normalize_component "$2"); then
                echo "Error: Invalid component '$2'. Expected desktop, cli, or all." >&2
                exit 1
            fi
            set_component_arg "${norm_comp}"
            shift 2
            ;;
        --component=*)
            raw_comp="${1#--component=}"
            if ! norm_comp=$(normalize_component "${raw_comp}"); then
                echo "Error: Invalid component '${raw_comp}'. Expected desktop, cli, or all." >&2
                exit 1
            fi
            set_component_arg "${norm_comp}"
            shift
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
        -n|--dry-run)
            DRY_RUN=1
            shift
            ;;
        -y|--yes)
            FORCE_NONINTERACTIVE=1
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

SELECTED_COMPONENT="${COMPONENT_ARG}"
if [ -z "${SELECTED_COMPONENT}" ]; then
    ENV_COMPONENT="${PAPERPILOT_COMPONENT:-${COMPONENT:-}}"
    if [ -n "${ENV_COMPONENT}" ]; then
        if ! SELECTED_COMPONENT=$(normalize_component "${ENV_COMPONENT}"); then
            echo "Error: Invalid component '${ENV_COMPONENT}'. Expected desktop, cli, or all." >&2
            exit 1
        fi
    fi
fi

echo "Detecting operating system and architecture..."
OS=$(uname -s)
ARCH=$(uname -m)

RUST_TARGET=""
LINUX_SHORT_ARCH=""
MAC_SHORT_ARCH=""
APPIMAGE_ARCH=""

case "${OS}" in
    Linux)
        case "${ARCH}" in
            x86_64|amd64)
                RUST_TARGET="x86_64-unknown-linux-gnu"
                LINUX_SHORT_ARCH="x86_64"
                APPIMAGE_ARCH="amd64"
                ;;
            aarch64|arm64)
                RUST_TARGET="aarch64-unknown-linux-gnu"
                LINUX_SHORT_ARCH="aarch64"
                APPIMAGE_ARCH="aarch64"
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
                MAC_SHORT_ARCH="aarch64"
                ;;
            x86_64)
                RUST_TARGET="x86_64-apple-darwin"
                MAC_SHORT_ARCH="x86_64"
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

if [ -z "${SELECTED_COMPONENT}" ]; then
    if [ -t 0 ] && [ -t 1 ] && [ "${FORCE_NONINTERACTIVE}" = "0" ] && [ -z "${CI:-}" ]; then
        echo "Select PaperPilot components to install:"
        echo "  1) Desktop App (Visual GUI workspace + PDF Viewer) [Default]"
        echo "  2) CLI only (Headless terminal & scripting engine)"
        echo "  3) All-in-One (CLI + Desktop App + MCP Server)"
        while true; do
            printf 'Choice [1/2/3] (default: 1): '
            if ! IFS= read -r CHOICE; then
                CHOICE="1"
            fi
            CHOICE=$(printf '%s' "${CHOICE}" | tr -d '[:space:]')
            case "${CHOICE}" in
                ""|1)
                    SELECTED_COMPONENT="desktop"
                    break
                    ;;
                2)
                    SELECTED_COMPONENT="cli"
                    break
                    ;;
                3)
                    SELECTED_COMPONENT="all"
                    break
                    ;;
                *)
                    echo "Invalid choice '${CHOICE}'. Please enter 1, 2, or 3." >&2
                    ;;
            esac
        done
    else
        if [ "${OS}" = "Linux" ] && [ -z "${DISPLAY:-}" ] && [ -z "${WAYLAND_DISPLAY:-}" ]; then
            SELECTED_COMPONENT="cli"
            echo "Headless Linux environment detected (DISPLAY/WAYLAND_DISPLAY unset); defaulting to CLI."
        else
            SELECTED_COMPONENT="desktop"
            echo "Non-interactive environment detected; defaulting to Desktop App."
        fi
    fi
fi

echo "Selected component: ${SELECTED_COMPONENT}"

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

CLEAN_VERSION="${VERSION#v}"
CLEAN_VERSION="${CLEAN_VERSION#V}"
echo "Installing PaperPilot version: ${VERSION}"

if [ -z "${INSTALL_DIR:-}" ]; then
    if [ "$(id -u)" -eq 0 ]; then
        INSTALL_DIR="/usr/local/bin"
    else
        INSTALL_DIR="${HOME}/.local/bin"
    fi
fi

if [ "$(id -u)" -eq 0 ]; then
    XDG_DATA_TARGET="/usr/local/share"
else
    XDG_DATA_TARGET="${XDG_DATA_HOME:-${HOME}/.local/share}"
fi

APPLICATIONS_DIR="${XDG_DATA_TARGET}/applications"
ICON_DIR="${XDG_DATA_TARGET}/icons/hicolor/512x512/apps"
ICON_FILE="${ICON_DIR}/paperpilot.png"
DESKTOP_BIN="${INSTALL_DIR}/paperpilot-desktop"
DESKTOP_ENTRY_FILE="${APPLICATIONS_DIR}/paperpilot.desktop"
MAC_APP_DIR="${MAC_APP_DIR:-/Applications}"
MAC_APP_DEST="${MAC_APP_DIR}/PaperPilot.app"

DESKTOP_CANDIDATES=""
CLI_CANDIDATES=""
MCP_CANDIDATES=""

case "${OS}" in
    Linux)
        if [ "${APPIMAGE_ARCH}" = "amd64" ]; then
            DESKTOP_CANDIDATES="PaperPilot_${CLEAN_VERSION}_amd64.AppImage PaperPilot_${CLEAN_VERSION}_x86_64.AppImage PaperPilot.AppImage PaperPilot_${CLEAN_VERSION}_amd64.deb"
        else
            DESKTOP_CANDIDATES="PaperPilot_${CLEAN_VERSION}_aarch64.AppImage PaperPilot_${CLEAN_VERSION}_arm64.AppImage PaperPilot_${CLEAN_VERSION}_arm64.deb PaperPilot_${CLEAN_VERSION}_aarch64.deb"
        fi
        CLI_CANDIDATES="paperpilot-cli-ubuntu-22.04-${RUST_TARGET}.tar.gz paperpilot-cli-ubuntu-latest-${RUST_TARGET}.tar.gz paperpilot-${RUST_TARGET}.tar.gz paperpilot-cli-linux-${LINUX_SHORT_ARCH}.tar.gz paperpilot-all-ubuntu-22.04-${RUST_TARGET}.tar.gz paperpilot-all-ubuntu-latest-${RUST_TARGET}.tar.gz"
        MCP_CANDIDATES="paperpilot-mcp-ubuntu-22.04-${RUST_TARGET}.tar.gz paperpilot-mcp-ubuntu-latest-${RUST_TARGET}.tar.gz paperpilot-mcp-linux-${LINUX_SHORT_ARCH}.tar.gz paperpilot-all-ubuntu-22.04-${RUST_TARGET}.tar.gz paperpilot-all-ubuntu-latest-${RUST_TARGET}.tar.gz"
        ;;
    Darwin)
        if [ "${MAC_SHORT_ARCH}" = "aarch64" ]; then
            DESKTOP_CANDIDATES="PaperPilot_${CLEAN_VERSION}_aarch64.dmg PaperPilot_${CLEAN_VERSION}_universal.dmg"
        else
            DESKTOP_CANDIDATES="PaperPilot_${CLEAN_VERSION}_x86_64.dmg PaperPilot_${CLEAN_VERSION}_x64.dmg PaperPilot_${CLEAN_VERSION}_universal.dmg"
        fi
        CLI_CANDIDATES="paperpilot-cli-macos-latest-${RUST_TARGET}.tar.gz paperpilot-${RUST_TARGET}.tar.gz paperpilot-cli-darwin-${MAC_SHORT_ARCH}.tar.gz paperpilot-all-macos-latest-${RUST_TARGET}.tar.gz"
        MCP_CANDIDATES="paperpilot-mcp-macos-latest-${RUST_TARGET}.tar.gz paperpilot-mcp-darwin-${MAC_SHORT_ARCH}.tar.gz paperpilot-all-macos-latest-${RUST_TARGET}.tar.gz"
        ;;
esac

PRIMARY_DESKTOP_FILENAME=$(first_word "${DESKTOP_CANDIDATES}")
PRIMARY_DESKTOP_URL="${GITHUB_DL}/${VERSION}/${PRIMARY_DESKTOP_FILENAME}"
PRIMARY_CLI_FILENAME=$(first_word "${CLI_CANDIDATES}")
PRIMARY_CLI_URL="${GITHUB_DL}/${VERSION}/${PRIMARY_CLI_FILENAME}"
PRIMARY_MCP_FILENAME=$(first_word "${MCP_CANDIDATES}")
PRIMARY_MCP_URL="${GITHUB_DL}/${VERSION}/${PRIMARY_MCP_FILENAME}"

if [ -n "${DRY_RUN:-}" ] && [ "${DRY_RUN}" != "0" ]; then
    echo "DRY_RUN mode enabled."
    echo "  Component:    ${SELECTED_COMPONENT}"
    echo "  Target:       ${RUST_TARGET}"
    echo "  Version:      ${VERSION}"
    if [ "${SELECTED_COMPONENT}" = "desktop" ] || [ "${SELECTED_COMPONENT}" = "all" ]; then
        echo "  Desktop URL:      ${PRIMARY_DESKTOP_URL}"
        echo "  Desktop Checksum: ${PRIMARY_DESKTOP_URL}.sha256"
        if [ "${OS}" = "Linux" ]; then
            echo "  Desktop path:     ${DESKTOP_BIN}"
            echo "  Desktop entry:    ${DESKTOP_ENTRY_FILE}"
            echo "  Desktop icon:     ${ICON_FILE}"
        else
            echo "  Desktop app:      ${MAC_APP_DEST}"
        fi
    fi
    if [ "${SELECTED_COMPONENT}" = "cli" ] || [ "${SELECTED_COMPONENT}" = "all" ]; then
        echo "  CLI URL:          ${PRIMARY_CLI_URL}"
        echo "  CLI Checksum:     ${PRIMARY_CLI_URL}.sha256"
        echo "  CLI path:         ${INSTALL_DIR}/paperpilot"
        echo "  CLI verification: ${INSTALL_DIR}/paperpilot --version"
    fi
    if [ "${SELECTED_COMPONENT}" = "all" ]; then
        echo "  MCP URL:          ${PRIMARY_MCP_URL}"
        echo "  MCP Checksum:     ${PRIMARY_MCP_URL}.sha256"
        echo "  MCP path:         ${INSTALL_DIR}/paperpilot-mcp"
        echo "  MCP verification: ${INSTALL_DIR}/paperpilot-mcp --version"
    fi
    exit 0
fi

TMP_DIR=$(mktemp -d)

case "${SELECTED_COMPONENT}" in
    desktop)
        install_desktop
        ;;
    cli)
        install_cli
        ;;
    all)
        install_desktop
        install_cli
        install_mcp
        ;;
esac

echo ""
echo "PaperPilot ${VERSION} (${SELECTED_COMPONENT}) installation complete!"
if [ "${SELECTED_COMPONENT}" = "desktop" ] || [ "${SELECTED_COMPONENT}" = "all" ]; then
    if [ "${OS}" = "Linux" ]; then
        echo "  Desktop App:   ${DESKTOP_BIN}"
        echo "  Launcher:      ${DESKTOP_ENTRY_FILE} (launch from your Application Menu)"
    else
        echo "  Desktop App:   ${MAC_APP_DEST}"
    fi
fi

if [ "${SELECTED_COMPONENT}" = "cli" ] || [ "${SELECTED_COMPONENT}" = "all" ]; then
    echo "  CLI Binary:    ${INSTALL_DIR}/paperpilot"
fi
if [ "${SELECTED_COMPONENT}" = "all" ] && [ -x "${INSTALL_DIR}/paperpilot-mcp" ]; then
    echo "  MCP Server:    ${INSTALL_DIR}/paperpilot-mcp"
fi

if [ "${OS}" = "Linux" ] || [ "${SELECTED_COMPONENT}" != "desktop" ]; then
    case ":${PATH}:" in
        *":${INSTALL_DIR}:"*)
            if [ "${SELECTED_COMPONENT}" = "cli" ] || [ "${SELECTED_COMPONENT}" = "all" ]; then
                echo "Run 'paperpilot --help' to get started."
            fi
            ;;
        *)
            echo "Note: ${INSTALL_DIR} is not currently in your PATH."
            echo "Add it to your shell profile (~/.bashrc, ~/.zshrc, or ~/.profile):"
            echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
            ;;
    esac
fi
