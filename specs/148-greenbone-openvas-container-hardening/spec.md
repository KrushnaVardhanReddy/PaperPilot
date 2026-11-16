# Spec 148: OpenVAS / Greenbone Community Container Hardening Suite

## Metadata
- **Spec Number**: 148
- **Title**: OpenVAS / Greenbone Community Container Hardening Suite
- **Phase**: Phase 8.14 / Infrastructure Hardening & Compliance
- **Status**: Ready for Implementation
- **Impacted Systems**: `docker/greenbone-scan/*`, `scripts/run_greenbone_scan.sh`
- **License / Cost**: 100% Free & Open-Source (GPL-2.0+ / Greenbone Community Edition)

---

## 1. Executive Summary & Purpose

Containerized deployments and on-premise enterprise installations require ongoing vulnerability assessments to guarantee that base OS packages, shared system libraries, exposed network ports, and TLS ciphers remain free of critical CVEs.

This specification establishes an automated **Turnkey OpenVAS / Greenbone Community Edition (GCE)** scanning harness:
1. **Containerized Architecture**: Uses official Greenbone Community Containers (`greenbone/community-containers`) requiring zero local host package installations.
2. **Automated Vulnerability Feed Sync**: Automatically synchronizes the latest free Greenbone Community Feed (100,000+ NVTs).
3. **Headless Execution**: A standalone shell script (`scripts/run_greenbone_scan.sh`) that spins up the target `paperpilot-gateway` container, connects via `gvm-cli`, initiates a "Full and Fast" audit, and polls until completion.
4. **Executive Compliance Reports**: Generates formal HTML and PDF vulnerability reports suitable for ISO 27001, SOC 2, and enterprise procurement reviews.

---

## 2. Directory Structure & Components

```text
docker/greenbone-scan/
├── docker-compose.yml       # Official Greenbone Community Edition multi-container stack
├── targets/                 # Target network definitions and port lists
└── reports/                 # Output directory for generated PDF/HTML audit reports

scripts/
└── run_greenbone_scan.sh    # Automated CLI orchestrator for headless scanning via gvm-cli
```

---

## 3. Verification & Exit Criteria
1. `docker-compose -f docker/greenbone-scan/docker-compose.yml config` passes validation.
2. Script successfully detects the target IP/port of `paperpilot-gateway`.
3. Headless scan completes and exports a valid audit summary report.
