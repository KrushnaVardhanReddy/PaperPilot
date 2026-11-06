# Spec 155: Self-Hosted S3/R2 Disaster Recovery & Replication via Litestream

## 1. Background & Context
For open-source, self-hosted deployments of PaperPilot (running via Docker or Docker Compose), users store asynchronous job receipts, batch progress, audit logs, and recipe presets inside SQLite / libSQL located in `/data/paperpilot.db`.

In production or VPS self-hosting environments, users need an automated, zero-maintenance backup solution so that:
1. If the VPS crashes or the Docker volume unmounts, zero data is lost.
2. Changes to the database are streamed in real time (sub-second RPO) directly to their private S3, Cloudflare R2, MinIO, or Google Cloud Storage bucket.
3. Upon container restart or disaster recovery, the database is restored from S3 in milliseconds before the application starts up.

---

## 2. Architecture & Design

### 2.1 The Litestream Sidecar / Replica Pattern
Instead of adding heavy S3 dependencies to the Rust core, Litestream runs as a dedicated lightweight replication container alongside the `paperpilot` container, sharing the `/data` volume.

```
                    ┌────────────────────────────────────────┐
                    │          Docker Compose Network        │
                    │                                        │
                    │  ┌─────────────────┐                   │
                    │  │   paperpilot    │                   │
                    │  │  (REST Gateway) │                   │
                    │  └────────┬────────┘                   │
                    │           │ writes WAL                 │
                    │           ▼                            │
                    │    /data/paperpilot.db                 │
                    │           ▲                            │
                    │           │ streams WAL                │
                    │  ┌────────┴────────┐      S3 / R2      │
                    │  │   litestream    │───────────────────┼──► AWS S3 / Cloudflare R2
                    │  │    (sidecar)    │ (sub-second sync) │    (Disaster Recovery)
                    │  └─────────────────┘                   │
                    └────────────────────────────────────────┘
```

### 2.2 Key Configuration Files
1. `docker/litestream.yml`: Configuration defining replica targets, sync intervals, and credentials.
2. `docker-compose.backup.yml`: An override compose file that spins up `litestream` alongside `paperpilot`.
3. `scripts/litestream-restore.sh`: One-click script to restore a database from an S3 backup timestamp.

---

## 3. Deliverables
- `docker/litestream.yml` template with support for AWS S3, Cloudflare R2, MinIO, and Backblaze B2.
- `docker-compose.backup.yml` plug-and-play compose file.
- Documentation in `DOCKER.md` showing step-by-step backup and point-in-time recovery instructions.
- Verification test validating litestream configuration syntax.
