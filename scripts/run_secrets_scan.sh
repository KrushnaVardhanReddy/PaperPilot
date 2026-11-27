#!/usr/bin/env bash
set -euo pipefail

# Script to run TruffleHog secrets scanner locally
# Checks git repository and codebase for leaked secrets, API keys, and credentials

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
BIN_DIR="${REPO_ROOT}/.tools/bin"
TRUFFLEHOG_BIN=""

echo "========================================================"
echo "  PaperPilot Secret & Credential Leak Scanner (TruffleHog)"
echo "========================================================"

# Enforce: only run on main branch (can be bypassed with FORCE_SCAN=true)
CURRENT_BRANCH="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")"
if [ "${CURRENT_BRANCH}" != "main" ] && [ "${FORCE_SCAN:-false}" != "true" ]; then
  echo "Notice: Current branch is '${CURRENT_BRANCH}'. Secret scan runs only on 'main' branch."
  echo "To force scan on this branch, run: FORCE_SCAN=true make trufflehog-scan"
  echo "========================================================"
  exit 0
fi

# 1. Prefer native standalone binary in .tools/bin, PATH, or ~/.cargo/bin
if [ -x "${BIN_DIR}/trufflehog" ]; then
  TRUFFLEHOG_BIN="${BIN_DIR}/trufflehog"
elif command -v trufflehog &>/dev/null; then
  TRUFFLEHOG_BIN="$(command -v trufflehog)"
elif [ -x "${HOME}/.cargo/bin/trufflehog" ]; then
  TRUFFLEHOG_BIN="${HOME}/.cargo/bin/trufflehog"
fi

# 2. If binary not present, automatically download standalone binary into .tools/bin
if [ -z "${TRUFFLEHOG_BIN}" ]; then
  echo "==> trufflehog binary not found. Installing into .tools/bin..."
  mkdir -p "${BIN_DIR}"
  ARCH="$(uname -m)"
  
  if [ "${ARCH}" = "x86_64" ]; then
    TRUFFLE_ARCH="amd64"
  elif [ "${ARCH}" = "aarch64" ] || [ "${ARCH}" = "arm64" ]; then
    TRUFFLE_ARCH="arm64"
  else
    TRUFFLE_ARCH="amd64"
  fi

  TMP_TAR="$(mktemp /tmp/trufflehog.XXXXXX.tar.gz)"
  TAG="v3.100.1"
  VERSION="3.100.1"
  DOWNLOAD_URL="https://github.com/trufflesecurity/trufflehog/releases/download/${TAG}/trufflehog_${VERSION}_linux_${TRUFFLE_ARCH}.tar.gz"

  echo "==> Fetching TruffleHog (${TAG}) from GitHub releases..."
  if curl -sSLf "${DOWNLOAD_URL}" -o "${TMP_TAR}" 2>/dev/null; then
    tar -xzf "${TMP_TAR}" -C "${BIN_DIR}" trufflehog 2>/dev/null || true
    rm -f "${TMP_TAR}"
    if [ -x "${BIN_DIR}/trufflehog" ]; then
      TRUFFLEHOG_BIN="${BIN_DIR}/trufflehog"
      echo "==> Successfully installed TruffleHog to ${TRUFFLEHOG_BIN}"
    fi
  else
    rm -f "${TMP_TAR}"
  fi
fi

cd "${REPO_ROOT}"

# 3. Execute Scan
if [ -n "${TRUFFLEHOG_BIN}" ] && [ -x "${TRUFFLEHOG_BIN}" ]; then
  echo "==> Running TruffleHog git scan (${TRUFFLEHOG_BIN})..."
  # Note: --only-verified ensures verified actionable secret leaks fail the build,
  # while ignoring false-positive package checksums in flatpak/lockfiles.
  "${TRUFFLEHOG_BIN}" git file://. --since-commit HEAD~20 --fail --no-update --only-verified
else
  # Offline pattern fallback
  echo "==> Running built-in secret pattern verification across codebase..."
  LEAKS_FOUND=0
  PATTERNS=(
    "(sk_live_[0-9a-zA-Z]{24})"
    "(rk_live_[0-9a-zA-Z]{24})"
    "(ghp_[0-9a-zA-Z]{36})"
    "(github_pat_[0-9a-zA-Z_]{82})"
    "(xox[baprs]-[0-9a-zA-Z]{10,48})"
    "-----BEGIN (RSA|EC|OPENSSH|PRIVATE) KEY-----"
    "(AIza[0-9A-Za-z\\-_]{35})"
  )

  for PATTERN in "${PATTERNS[@]}"; do
    MATCHES=$(git grep -E -I "${PATTERN}" -- ':!fixtures' ':!*.lock' ':!target' ':!test-corpus' 2>/dev/null || true)
    if [ -n "${MATCHES}" ]; then
      echo "⚠️  POTENTIAL SECRET LEAK: ${MATCHES}"
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
