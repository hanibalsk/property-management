# Backend - CLAUDE.md

> **Parent:** See root `CLAUDE.md` for namespace and architecture.

## Overview

Rust backend with Axum framework. Contains two servers sharing common crates.

## Servers

| Server | Port | Purpose |
|--------|------|---------|
| api-server | 8080 | Property Management API |
| reality-server | 8081 | Reality Portal public API |

## Quick Commands

```bash
# Build all
cargo build

# Build release
cargo build --release

# Run api-server
cargo run -p api-server

# Run reality-server
cargo run -p reality-server

# Run tests
cargo test --workspace

# Format
cargo fmt --all

# Lint
cargo clippy --workspace -- -D warnings

# Check
cargo check --workspace
```

## Workspace Structure

```
backend/
├── Cargo.toml           # Workspace root
├── crates/              # Shared libraries (see crates/CLAUDE.md)
│   ├── common/
│   ├── api-core/
│   ├── db/
│   └── integrations/
└── servers/             # Backend servers (see servers/CLAUDE.md)
    ├── api-server/
    └── reality-server/
```

## Dependencies

Key workspace dependencies:
- `axum` - Web framework
- `tokio` - Async runtime
- `sqlx` - Database
- `utoipa` - OpenAPI generation
- `serde` - Serialization
- `tracing` - Logging

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `DATABASE_URL` | Yes | PostgreSQL connection string |
| `JWT_SECRET` | Yes | Secret key for JWT signing (min 32 chars) |
| `RUST_LOG` | No | Log level (default: info) |

```bash
# Required
DATABASE_URL=postgres://user:pass@localhost:5432/ppt
JWT_SECRET=your-secure-random-secret-key-min-32-chars

# Optional
RUST_LOG=debug
```

> **Security:** `JWT_SECRET` has no fallback. Server will fail to authenticate requests if not set.
