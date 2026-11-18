# Week 09: Self-Hosted AI Stacks: Private Apache Tika Replacement for Open-WebUI & Ollama

**Theme:** Self-Hosted LLM Stacks, Edge Infrastructure & Lightweight Document Ingestion  
**Tools Featured:** PaperPilot Gateway (`/tika` and `/rmeta/text` wire emulation), `extract_meta`  
**Target Audience:** Open-WebUI Enthusiasts, Ollama Users, Homelab Operators, Private AI Architects & DevOps Engineers  
**Asset:** ![Week 09 Demo](../assets/showcase/week09-tika-wire-openwebui.gif)

---

## 📝 LinkedIn Post Copy

### Hook
Running **Open-WebUI** and **Ollama** on your local workstation or homelab cluster? 🤖

If you want Open-WebUI to parse PDFs and documents, the official docs tell you to spin up an **Apache Tika** container. 

The problem? Apache Tika is a 1.2GB Java behemoth that hungrily eats 1.8GB of RAM on idle, takes 15 seconds to boot, and introduces heavyweight JVM overhead to an otherwise lean homelab setup. 🛑

Here is how we replaced the entire Apache Tika container with a **19MB, 100% native Rust gateway** that speaks the exact same Tika wire protocol—slashing RAM usage by 98.5% and accelerating document ingestion by 13x. ⚡

---

### The Problem: The Heavyweight Tax of Java Tika
Self-hosted AI engineers love running quantized LLMs on modest hardware (Mac Studio, RTX 3060/4090 rigs, or home mini-PCs). Every gigabyte of VRAM and system memory counts.
1. **Memory Hog**: Apache Tika running in Docker typically consumes 1.5GB to 2.2GB of resident memory just idling.
2. **Slow Cold Starts**: Java Virtual Machine (JVM) initialization slows down automated document ingest pipelines.
3. **Bloated Images**: Pulling a 1.2GB+ image over residential or edge network connections wastes bandwidth and disk space.

---

### The Solution: PaperPilot Gateway Wire Emulation
Open-WebUI doesn't actually need Java. It only needs an HTTP service that answers standard Tika endpoints:
- `PUT /tika` (returns extracted plaintext or structured Markdown)
- `PUT /rmeta/text` (returns JSON metadata array)

The PaperPilot Gateway provides native wire compatibility for these exact endpoints:
1. **Drop-in Configuration**: In your Open-WebUI admin panel or `docker-compose.yml`, change:
   ```yaml
   TIKA_SERVER_URL: "http://paperpilot-gateway:9998"
   ```
2. **Zero Code Changes**: Open-WebUI talks to PaperPilot thinking it's talking to standard Tika.
3. **Rust Speed**: Ingests an 8.4MB financial PDF with dense tables in **18ms** instead of 234ms.

---

### Benchmark: Apache Tika vs. PaperPilot Gateway
| Metric | Official Apache Tika (JVM) | PaperPilot Gateway (Native) | Improvement |
| :--- | :---: | :---: | :---: |
| **Docker Image Size** | 1,220 MB | **19 MB** | **98.4% smaller** |
| **Idle RAM Footprint** | 1,850 MB | **28 MB** | **98.5% less RAM** |
| **Ingestion Latency (8.4MB PDF)** | 234 ms | **18 ms** | **13x faster** |
| **Runtime Dependency** | Java 17+ JVM | **None (Single Static Binary)** | **Zero overhead** |
| **Table Preservation** | Fragmented text lines | Structured Markdown Tables | Context-Aware RAG |

---

### Quickstart Docker Compose
```yaml
services:
  open-webui:
    image: ghcr.io/open-webui/open-webui:main
    ports:
      - "3000:8080"
    environment:
      - TIKA_SERVER_URL=http://paperpilot:9998
    depends_on:
      - paperpilot

  paperpilot:
    image: ghcr.io/usepaperpilot/paperpilot-gateway:latest
    ports:
      - "9998:9998"
    restart: unless-stopped
```

Drop an 8MB quarterly report into Open-WebUI, watch your local Ollama model answer table queries instantly, and check `docker stats` to see PaperPilot sipping just 28MB of RAM.

---

## 🏷️ Hashtags & Social Metadata
#OpenWebUI #Ollama #SelfHosted #Homelab #LocalAI #DevOps #RustLang #ApacheTika #Docker #PaperPilot #OpenSource #RAG
