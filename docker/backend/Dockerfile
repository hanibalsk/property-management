# Multi-stage Dockerfile for Rust backend services (api-server, reality-server)
# Supports: linux/amd64, linux/arm64

# =============================================================================
# Stage 1: Chef - Prepare recipe for dependency caching
# =============================================================================
FROM --platform=$BUILDPLATFORM rust:1.75-slim-bookworm AS chef

RUN cargo install cargo-chef --locked
WORKDIR /app

# =============================================================================
# Stage 2: Planner - Create dependency recipe
# =============================================================================
FROM chef AS planner

COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/crates ./crates
COPY backend/servers ./servers

RUN cargo chef prepare --recipe-path recipe.json

# =============================================================================
# Stage 3: Builder - Build dependencies then application
# =============================================================================
FROM --platform=$BUILDPLATFORM rust:1.75-slim-bookworm AS builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM

# Install build dependencies and cross-compilation tools
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-chef
RUN cargo install cargo-chef --locked

WORKDIR /app

# Copy and build dependencies (cached layer)
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source code
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/crates ./crates
COPY backend/servers ./servers

# Build the applications
RUN cargo build --release -p api-server -p reality-server

# Strip debug symbols for smaller binaries
RUN strip target/release/api-server target/release/reality-server || true

# =============================================================================
# Stage 4: Runtime Base - Minimal production image base
# =============================================================================
FROM debian:bookworm-slim AS runtime-base

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r ppt && useradd -r -g ppt ppt

WORKDIR /app

# =============================================================================
# API Server - Production target
# =============================================================================
FROM runtime-base AS api-server

# Copy binary from builder
COPY --from=builder /app/target/release/api-server /app/api-server

# Set ownership
RUN chown -R ppt:ppt /app

# Switch to non-root user
USER ppt

# Expose service port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

CMD ["/app/api-server"]

# =============================================================================
# Reality Server - Production target
# =============================================================================
FROM runtime-base AS reality-server

# Copy binary from builder
COPY --from=builder /app/target/release/reality-server /app/reality-server

# Set ownership
RUN chown -R ppt:ppt /app

# Switch to non-root user
USER ppt

# Expose service port
EXPOSE 8081

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8081/health || exit 1

CMD ["/app/reality-server"]
