# Task 148: Implement OpenVAS / Greenbone Community Container Hardening Suite

## Overview
Implement Spec 148 by providing a turnkey Docker Compose configuration (`docker/greenbone-scan/docker-compose.yml`) and automation script (`scripts/run_greenbone_scan.sh`) to execute headless OpenVAS vulnerability scans against PaperPilot Gateway containers and export audit reports.

---

## Subtasks & Checklist

- [ ] **Subtask 1: Docker Compose Stack for Greenbone Community Containers**
  - [ ] Create `docker/greenbone-scan/docker-compose.yml` leveraging official Greenbone images (`gsa`, `gvmd`, `ospd-openvas`, `notus-scanner`).
  - [ ] Configure network bridge connecting to `paperpilot-gateway`.

- [ ] **Subtask 2: Headless Scan Orchestrator (`scripts/run_greenbone_scan.sh`)**
  - [ ] Implement target host IP/port detection.
  - [ ] Authenticate with `gvmd` via `gvm-cli`.
  - [ ] Create target, configure "Full and Fast" scan task, and start audit.
  - [ ] Poll progress until 100% complete and export PDF/HTML reports to `reports/`.

- [ ] **Subtask 3: Documentation & Makefile Integration**
  - [ ] Add `make scan-greenbone` target to `Makefile`.
  - [ ] Document usage instructions in `docker/greenbone-scan/README.md`.
