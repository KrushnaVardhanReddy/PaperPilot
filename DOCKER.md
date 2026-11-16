# 🐳 PaperPilot Docker Quickstart Guide

This Docker container allows anyone (developers, testers, teammates, or friends) to run and test **all PaperPilot interfaces** on **any operating system** (macOS, Windows, Linux) without needing Rust, Node.js, or local compilers installed.

---

## 🚀 1-Click Quickstart

### Method 1: Docker Compose (Recommended)

From the project root:
```bash
docker compose up -d
```

### Method 2: Standard Docker Run
```bash
# Build the image
docker build -t paperpilot .

# Run the container
docker run -d \
  -p 7823:7823 \
  -v $(pwd)/data:/data \
  --name paperpilot \
  paperpilot
```

---

## 🧪 Testing All Interfaces via Docker

Once the container is running, all interfaces are immediately accessible:

### 1. 🌐 Interactive Swagger UI & REST API
Open in any web browser:
👉 **[http://localhost:7823/swagger-ui](http://localhost:7823/swagger-ui)**

- Test all 50+ operations directly in the browser (e.g. `merge`, `split`, `compress`, `ocr`, `excel-to-json`).
- Upload real files and download transformed results with zero configuration.

### 2. 💻 CLI Commands (Inside Container)
Anyone can test the CLI without installing Rust on their host machine:

```bash
# View CLI help and all available commands
docker exec -it paperpilot paperpilot --help

# Test merging documents inside /data
docker exec -it paperpilot paperpilot merge -i /data/doc1.pdf -i /data/doc2.pdf -o /data/merged.pdf

# Test plain text extraction
docker exec -it paperpilot paperpilot text -i /data/doc1.pdf -o /data/output.txt

# Test Excel to JSON conversion
docker exec -it paperpilot paperpilot office excel-to-json -i /data/sheet.xlsx -o /data/sheet.json
```

*(Any files placed in your local `./data/` folder on your host machine are instantly visible inside the container at `/data/`!)*

### 3. 🤖 Model Context Protocol (MCP) Server for AI Agents
Connect Cursor, Claude Desktop, or local AI agent tools to the container:

- **Via Server-Sent Events (SSE):**
  - Gateway endpoint: `http://localhost:7823/mcp/sse`
- **Via stdio:**
  - Command: `docker exec -i paperpilot paperpilot-mcp`

### 4. 🩺 Health Check
Check container status:
```bash
curl http://localhost:7823/health
```
Response:
```json
{"status":"ok","version":"0.1.0","tools_count":51}
```

### 5. 💾 Data Persistence & Retention Lifecycle

The Docker container runs with an embedded **libSQL / Turso** database engine (`paperpilot.db`) storing async jobs, audit records, and semantic vector chunks:

- **Where data is stored**:
  - Mounted at `/data` (points to `./data` on your host machine).
  - The SQLite database is created at `/data/paperpilot.db`.
- **Automatic Archiving & Cleanup**:
  - The database records can be pruned using retention policies:
    ```bash
    # Clear completed/failed background job history
    docker exec -it paperpilot paperpilot storage clear-jobs
    ```
  - Document vector chunks can be deleted by document hash:
    ```bash
    # Remove indexed document chunks from vector store
    docker exec -it paperpilot paperpilot storage delete-chunks --hash <DOC_SHA256>
    ```
  - To completely wipe local history and start clean, simply remove `./data/paperpilot.db` or restart with an empty volume.

---

## ☁️ Continuous S3 / Cloudflare R2 Disaster Recovery (Litestream)

For production or VPS self-hosting, PaperPilot provides an automated, zero-maintenance backup sidecar using **[Litestream](https://litestream.io/)** (`docker-compose.backup.yml`). It streams database transactions in real-time to your private S3, Cloudflare R2, or MinIO bucket with sub-second RPO.

### 1. Configure Cloud Storage Credentials (`.env`)
Create or edit your `.env` file:
```env
# AWS S3, Cloudflare R2, MinIO, or Backblaze B2 credentials
LITESTREAM_BUCKET=my-paperpilot-backup-bucket
LITESTREAM_PATH=paperpilot-db
LITESTREAM_ACCESS_KEY_ID=your_access_key
LITESTREAM_SECRET_ACCESS_KEY=your_secret_key

# Optional: Custom endpoint (required for Cloudflare R2 or MinIO)
# LITESTREAM_ENDPOINT=https://<account_id>.r2.cloudflarestorage.com
# LITESTREAM_REGION=auto
```

### 2. Start with Continuous Replication
```bash
docker compose -f docker-compose.yml -f docker-compose.backup.yml up -d
```
Litestream immediately connects and streams all database commits to your bucket.

### 3. Disaster Recovery (Point-in-Time Restore)
If your server crashes or volume unmounts, restore the entire database in milliseconds:
```bash
# Restore latest backup to ./data/paperpilot.db
docker run --rm \
  -v $(pwd)/data:/data \
  --env-file .env \
  litestream/litestream:0.3.13 \
  restore -config /etc/litestream.yml /data/paperpilot.db

# Or restore to a specific point-in-time timestamp:
# docker run --rm -v $(pwd)/data:/data --env-file .env litestream/litestream:0.3.13 restore -timestamp 2026-11-06T12:00:00Z /data/paperpilot.db
```

---

## 🛑 Stopping the Container
```bash
docker compose down
# OR
docker stop paperpilot && docker rm paperpilot
```
