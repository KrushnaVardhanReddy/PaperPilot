# ==============================================================================
# Multi-Stage Dockerfile for PaperPilot All-Interface Container
# Includes:
#   1. Gateway REST API + Swagger UI (:7823)
#   2. MCP Server (SSE on :7823 / stdio via `docker exec -i ... paperpilot-mcp`)
#   3. CLI Command Suite (`docker exec ... paperpilot <subcommand>`)
#   4. Web Client UI (WASM-powered Svelte 5 frontend on :7823 or :3000)
# ==============================================================================

# --- Stage 1: Build Rust Binaries ---
FROM docker.io/library/rust:latest AS rust-builder
WORKDIR /app

# Install system dependencies needed for compiling font / vector crates
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy cargo manifest files
COPY Cargo.toml Cargo.lock ./
COPY paperpilot-core/Cargo.toml ./paperpilot-core/
COPY paperpilot-pdf/Cargo.toml ./paperpilot-pdf/
COPY paperpilot-nlp/Cargo.toml ./paperpilot-nlp/
COPY paperpilot-ai/Cargo.toml ./paperpilot-ai/
COPY paperpilot-storage/Cargo.toml ./paperpilot-storage/
COPY paperpilot-mcp/Cargo.toml ./paperpilot-mcp/
COPY paperpilot-gateway/Cargo.toml ./paperpilot-gateway/
COPY paperpilot-cli/Cargo.toml ./paperpilot-cli/
COPY paperpilot-wasm/Cargo.toml ./paperpilot-wasm/
COPY tools/ ./tools/
COPY apps/desktop/src-tauri/Cargo.toml ./apps/desktop/src-tauri/

# Copy source trees
COPY paperpilot-core ./paperpilot-core
COPY paperpilot-pdf ./paperpilot-pdf
COPY paperpilot-nlp ./paperpilot-nlp
COPY paperpilot-ai ./paperpilot-ai
COPY paperpilot-storage ./paperpilot-storage
COPY paperpilot-mcp ./paperpilot-mcp
COPY paperpilot-gateway ./paperpilot-gateway
COPY paperpilot-cli ./paperpilot-cli
COPY paperpilot-wasm ./paperpilot-wasm
COPY apps/desktop/src-tauri ./apps/desktop/src-tauri

# Build optimized release binaries for the server, CLI, and MCP
RUN cargo build --release \
    -p paperpilot-gateway \
    -p paperpilot-cli \
    -p paperpilot-mcp \
    -j 6

# --- Stage 2: Final Minimal Runtime Image ---
FROM docker.io/library/debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy compiled binaries from builder
COPY --from=rust-builder /app/target/release/paperpilot-gateway /usr/local/bin/paperpilot-gateway
COPY --from=rust-builder /app/target/release/paperpilot-cli /usr/local/bin/paperpilot
COPY --from=rust-builder /app/target/release/paperpilot-mcp /usr/local/bin/paperpilot-mcp

# Create shared data folder for mounting local host PDFs and models
RUN mkdir -p /data /models
WORKDIR /data

# Create non-root user and set permissions
RUN groupadd -r paperpilot && useradd -r -g paperpilot paperpilot && \
    chown -R paperpilot:paperpilot /data /models /app /usr/local/bin/paperpilot*

# Default environment configuration
ENV PORT=7823
ENV BIND=0.0.0.0
ENV RUST_LOG=info
ENV PAPERPILOT_DATA_DIR=/data
ENV PAPERPILOT_OCR_MODELS_DIR=/models

# Switch to non-root user
USER paperpilot

# Expose Gateway REST API & MCP Server port
EXPOSE 7823

# Default startup command runs the unified Gateway Server
ENTRYPOINT ["/usr/local/bin/paperpilot-gateway"]
