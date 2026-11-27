#!/usr/bin/env bash
set -euo pipefail

# Script to run TruffleHog secrets scanner locally
# Checks git history and filesystem for leaked secrets, API keys, and credentials

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
BIN_DIR="${REPO_ROOT}/.tools/bin"
TRUFFLEHOG_BIN=""

echo "========================================================"
echo "  PaperPilot Secret & Credential Leak Scanner (TruffleHog)"
echo "========================================================"

# 1. Check if trufflehog is already installed in PATH or local project bin
if command -v trufflehog &>/dev/null; then
  TRUFFLEHOG_BIN="$(command -v trufflehog)"
elif [ -x "${BIN_DIR}/trufflehog" ]; then
  TRUFFLEHOG_BIN="${BIN_DIR}/trufflehog"
elif [ -x "${HOME}/.cargo/bin/trufflehog" ]; then
  TRUFFLEHOG_BIN="${HOME}/.cargo/bin/trufflehog"
elif command -v docker &>/dev/null && docker info &>/dev/null 2>&1; then
  TRUFFLEHOG_BIN="docker_container"
elif command -v podman &>/dev/null && podman info &>/dev/null 2>&1; then
  TRUFFLEHOG_BIN="podman_container"
fi

# 2. If not found, attempt to download standalone binary into .tools/bin
if [ -z "${TRUFFLEHOG_BIN}" ]; then
  echo "==> trufflehog not found. Downloading standalone binary into .tools/bin..."
  mkdir -p "${BIN_DIR}"
  ARCH="$(uname -m)"
  OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
  
  if [ "${ARCH}" = "x86_64" ]; then
    TRUFFLE_ARCH="amd64"
  elif [ "${ARCH}" = "aarch64" ] || [ "${ARCH}" = "arm64" ]; then
    TRUFFLE_ARCH="arm64"
  else
    TRUFFLE_ARCH="amd64"
  fi

  # TruffleHog latest release download
  TMP_TAR="$(mktemp /tmp/trufflehog.XXXXXX.tar.gz)"
  DOWNLOAD_URL="https://github.com/trufflesecurity/trufflehog/releases/latest/download/trufflehog_${OS}_${TRUFFLE_ARCH}.tar.gz"
  
  echo "==> Fetching TruffleHog from: ${DOWNLOAD_URL}"
  if curl -sSLf "${DOWNLOAD_URL}" -o "${TMP_TAR}" 2>/dev/null; then
    tar -xzf "${TMP_TAR}" -C "${BIN_DIR}" trufflehog 2>/dev/null || true
    rm -f "${TMP_TAR}"
    if [ -x "${BIN_DIR}/trufflehog" ]; then
      TRUFFLEHOG_BIN="${BIN_DIR}/trufflehog"
      echo "==> Installed TruffleHog to ${TRUFFLEHOG_BIN}"
    fi
  else
    rm -f "${TMP_TAR}"
    echo "Notice: Could not download binary directly (network/offline). Falling back to built-in git secret patterns scan."
  fi
fi

# 3. Execute Scan
cd "${REPO_ROOT}"

if [ "${TRUFFLEHOG_BIN}" = "docker_container" ]; then
  echo "==> Running TruffleHog scan via Docker..."
  docker run --rm -v "${REPO_ROOT}:/workdir" -w /workdir trufflesecurity/trufflehog:latest git file:///workdir --fail --only-verified=false
elif [ "${TRUFFLEHOG_BIN}" = "podman_container" ]; then
  echo "==> Running TruffleHog scan via Podman..."
  podman run --rm -v "${REPO_ROOT}:/workdir:z" -w /workdir trufflesecurity/trufflehog:latest git file:///workdir --fail --only-verified=false
elif [ -n "${TRUFFLEHOG_BIN}" ] && [ -x "${TRUFFLEHOG_BIN}" ]; then
  echo "==> Running TruffleHog scan using ${TRUFFLEHOG_BIN}..."
  "${TRUFFLEHOG_BIN}" git file://. --fail --no-update
else
  # Fast fallback: High-entropy secret scanner using git grep for critical pattern leakages
  echo "==> Running built-in secret & token regex verification across codebase..."
  LEAKS_FOUND=0
  
  PATTERNS=(
    "(sk_live_[0-9a-zA-Z]{24})"
    "(rk_live_[0-9a-zA-Z]{24})"
    "(ghp_[0-9a-zA-Z]{36})"
    "(github_pat_[0-9a-zA-Z_]{82})"
    "(xox[baprs]-[0-9a-zA-Z]{10,48})"
    "-----BEGIN (RSA|EC|OPENSSH|PRIVATE) KEY-----"
    "(AIza[0-9A-Za-z\\-_]{35})"
    "(sq0atp-[0-9A-Za-z\\-_]{22})"
  )

  for PATTERN in "${PATTERNS[@]}"; do
    MATCHES=$(git grep -E -I "${PATTERN}" -- ':!fixtures' ':!*.lock' ':!target' ':!test-corpus' 2>/dev/null || true)
    if [ -n "${MATCHES}" ]; then
      echo "⚠️  POTENTIAL SECRET LEAK DETECTED with pattern '${PATTERN}':"
      echo "${MATCHES}"
      LEAKS_FOUND=$((LEAKS_FOUND + 1))
    fi
  done

  if [ ${LEAKS_FOUND} -eq 0 ]; then
    echo "✅ No exposed production API keys, private keys, or tokens detected."
  else
    echo "❌ High-risk secret patterns discovered! Please review above."
    exit 1
  fi
fi

echo "========================================================"
echo "  Secret Scan Completed Successfully"
echo "========================================================"
