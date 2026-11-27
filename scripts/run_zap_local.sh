#!/usr/bin/env bash
set -euo pipefail

# Script to run OWASP ZAP Dynamic API Security Scan locally via Docker or Podman
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

DOCKER_CMD=""
if command -v podman &>/dev/null; then
  DOCKER_CMD="podman"
elif command -v docker &>/dev/null; then
  DOCKER_CMD="docker"
else
  echo "Error: Neither docker nor podman found in PATH." >&2
  exit 1
fi

echo "==> Using container engine: ${DOCKER_CMD}"
echo "==> Building PaperPilot Gateway..."
cd "${REPO_ROOT}"
cargo build -p paperpilot-gateway --release -j 4

echo "==> Launching PaperPilot Gateway on port 8080..."
PORT=8080 BIND=0.0.0.0 ./target/release/paperpilot-gateway &
GATEWAY_PID=$!

cleanup() {
  echo "==> Stopping PaperPilot Gateway (PID: ${GATEWAY_PID})..."
  kill "${GATEWAY_PID}" 2>/dev/null || true

  # Aggressive container cleanup to protect disk space (e.g. 14GB disk budget)
  if [ "${CLEANUP_IMAGE:-false}" = "true" ]; then
    echo "==> Removing ZAP container image to reclaim disk space..."
    ${DOCKER_CMD} rmi zaproxy/zap-stable 2>/dev/null || true
  fi
  ${DOCKER_CMD} container prune -f 2>/dev/null || true
  ${DOCKER_CMD} volume prune -f 2>/dev/null || true
}
trap cleanup EXIT

echo "==> Waiting for Gateway health check..."
for i in {1..30}; do
  if curl -s http://127.0.0.1:8080/health | grep -q "ok"; then
    echo "Gateway is up and healthy!"
    break
  fi
  sleep 1
done

echo "==> Running OWASP ZAP API scan against local OpenAPI spec..."
mkdir -p "${REPO_ROOT}/target/zap-reports"

${DOCKER_CMD} run --net=host --rm \
  -v "${REPO_ROOT}/.zap:/zap/wrk/:ro" \
  -v "${REPO_ROOT}/target/zap-reports:/zap/reports/:rw" \
  docker.io/zaproxy/zap-stable zap-api-scan.py \
  -t http://127.0.0.1:8080/api-docs/openapi.json \
  -f openapi \
  -c /zap/wrk/rules.tsv \
  -r /zap/reports/report_html.html \
  -J /zap/reports/report_json.json \
  -m /zap/reports/report_md.md || true

echo "==> ZAP Scan finished! Reports available in: ${REPO_ROOT}/target/zap-reports/"
