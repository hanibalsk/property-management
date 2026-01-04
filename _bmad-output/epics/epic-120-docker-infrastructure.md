# Epic 120: Docker Infrastructure for Local Development and Synology Deployment

## Epic Summary

Implement comprehensive Docker containerization for the Property Management System (PPT) that supports both local development workflows and production deployment to Synology NAS with multi-architecture builds.

## Business Value

- **Developer Onboarding**: Reduce setup time from hours to minutes with `docker compose up`
- **Environment Parity**: Eliminate "works on my machine" issues with consistent containers
- **Synology Deployment**: Enable self-hosted deployment on Synology NAS (ARM64/x86_64)
- **Resource Efficiency**: Optimize container sizes for NAS deployment constraints
- **CI/CD Ready**: Provide foundation for automated builds and deployments

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    Docker Compose Stack                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ api-server  │  │reality-server│  │    ppt-web (nginx)     │  │
│  │  Port 8080  │  │  Port 8081   │  │      Port 80/443       │  │
│  └──────┬──────┘  └──────┬───────┘  └───────────────────────┘  │
│         │                │                                       │
│         └───────┬────────┘                                       │
│                 ▼                                                │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                     PostgreSQL 16                           ││
│  │                      Port 5432                              ││
│  └─────────────────────────────────────────────────────────────┘│
│                              │                                   │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                        Redis                                ││
│  │                      Port 6379                              ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                  │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │              MinIO (S3-compatible storage)                  ││
│  │                Port 9000 (API) / 9001 (Console)             ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Target Platforms

| Platform | Architecture | Notes |
|----------|--------------|-------|
| Local Development | darwin-arm64, linux-amd64 | macOS (Apple Silicon), Linux x86_64 |
| Synology DS920+ | linux-amd64 | Intel Celeron J4125 |
| Synology DS223 | linux-arm64 | Realtek RTD1619B |

## Acceptance Criteria

1. **Local Development**
   - [ ] Single command startup: `docker compose up`
   - [ ] Hot-reloading for Rust backend (cargo-watch)
   - [ ] Hot-reloading for frontend (Vite HMR)
   - [ ] Database migrations run automatically
   - [ ] Persistent volumes for data

2. **Production Build**
   - [ ] Multi-stage Dockerfiles (build + runtime)
   - [ ] Optimized image sizes (< 100MB for Rust services)
   - [ ] Multi-architecture builds (amd64, arm64)
   - [ ] Health checks for all services

3. **Synology Deployment**
   - [ ] Synology Container Manager compatible
   - [ ] Docker Compose v2 compatible
   - [ ] Network configuration for NAS environment
   - [ ] Volume mounts for Synology storage

4. **Security**
   - [ ] Non-root containers
   - [ ] Secrets management via environment/files
   - [ ] Network isolation between services

## Stories

### Story 120.1: Base Dockerfiles for Rust Services

**As a** developer
**I want** optimized Dockerfiles for api-server and reality-server
**So that** I can build and run the backend services in containers

**Acceptance Criteria:**
- Multi-stage build (builder + runtime)
- Uses rust:1.75-slim as builder
- Uses debian:bookworm-slim as runtime
- Static linking with musl for smaller images
- Health check endpoint integration
- Non-root user execution

**Technical Notes:**
- Use cargo-chef for layer caching
- Target size: < 50MB per service
- Support both amd64 and arm64

### Story 120.2: Docker Compose for Local Development

**As a** developer
**I want** a docker-compose.dev.yml for local development
**So that** I can start the full stack with one command

**Acceptance Criteria:**
- PostgreSQL 16 with initialization scripts
- Redis for sessions/caching
- MinIO for S3-compatible storage
- Volume mounts for source code (hot-reload)
- Port mappings matching current dev setup
- Environment variable templates

**Technical Notes:**
```yaml
services:
  postgres:
    image: postgres:16-alpine
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./backend/scripts/init.sql:/docker-entrypoint-initdb.d/init.sql
  redis:
    image: redis:7-alpine
  minio:
    image: minio/minio
  api-server:
    build: ./backend
    volumes:
      - ./backend:/app:cached
```

### Story 120.3: Frontend Dockerfiles

**As a** developer
**I want** Dockerfiles for ppt-web and reality-web
**So that** I can serve the frontend applications in containers

**Acceptance Criteria:**
- Multi-stage build (node:20-alpine + nginx:alpine)
- ppt-web: Vite SPA build
- reality-web: Next.js SSR/SSG
- Nginx configuration for SPA routing
- Environment variable injection at runtime
- Gzip compression enabled

### Story 120.4: Production Docker Compose

**As a** operations engineer
**I want** docker-compose.prod.yml for production deployment
**So that** I can deploy the full stack to any Docker host

**Acceptance Criteria:**
- Uses pre-built images from registry
- Resource limits defined (memory, CPU)
- Restart policies (unless-stopped)
- Health checks with proper intervals
- Logging configuration
- SSL/TLS termination via reverse proxy

### Story 120.5: Synology-Specific Configuration

**As a** home user deploying to Synology NAS
**I want** Synology-optimized configuration files
**So that** I can run PPT on my Synology NAS

**Acceptance Criteria:**
- docker-compose.synology.yml with NAS-specific settings
- Volume paths for Synology shared folders
- Memory limits suitable for NAS (2-4GB total)
- Synology Container Manager import instructions
- Backup/restore scripts for volumes

**Technical Notes:**
- Default volume path: /volume1/docker/ppt
- Use host networking option for NAS
- Support HTTPS via Synology reverse proxy

### Story 120.6: Multi-Architecture Build Pipeline

**As a** developer
**I want** scripts for building multi-arch Docker images
**So that** I can deploy to both x86_64 and ARM64 platforms

**Acceptance Criteria:**
- buildx configuration for cross-compilation
- Build script: `./scripts/docker-build.sh`
- Push to container registry (optional)
- Manifest list for multi-arch images
- GitHub Actions workflow for automated builds

### Story 120.7: Development Environment Documentation

**As a** developer
**I want** comprehensive Docker documentation
**So that** I can quickly set up and troubleshoot the development environment

**Acceptance Criteria:**
- README.md in docker/ directory
- Quick start guide
- Troubleshooting section
- Performance tuning tips
- Synology deployment guide

## Dependencies

- Rust 1.75+ (workspace version)
- PostgreSQL 16 (matches current requirement)
- Redis 7 (for sessions)
- MinIO (S3-compatible, for file storage)
- Node.js 20 LTS (frontend builds)

## Risks

| Risk | Mitigation |
|------|------------|
| ARM64 cross-compilation issues | Use cargo-zigbuild or native ARM runners |
| Image size too large for NAS | Use alpine-based images, strip binaries |
| Hot-reload not working | Use proper volume mount strategies |
| Database migrations failing | Add init container or healthcheck dependency |

## File Structure

```
property-management/
├── docker/
│   ├── README.md                    # Docker documentation
│   ├── .env.example                 # Environment template
│   ├── backend/
│   │   ├── Dockerfile              # Rust services Dockerfile
│   │   └── Dockerfile.dev          # Development with hot-reload
│   ├── frontend/
│   │   ├── ppt-web.Dockerfile      # Property Management SPA
│   │   └── reality-web.Dockerfile  # Reality Portal SSR
│   ├── nginx/
│   │   └── nginx.conf              # Nginx configuration
│   └── scripts/
│       ├── docker-build.sh         # Build script
│       └── docker-push.sh          # Push to registry
├── docker-compose.yml              # Production compose
├── docker-compose.dev.yml          # Local development
├── docker-compose.synology.yml     # Synology NAS optimized
└── .dockerignore                   # Docker ignore rules
```

## Definition of Done

- [ ] All Dockerfiles build successfully on amd64 and arm64
- [ ] `docker compose -f docker-compose.dev.yml up` works first try
- [ ] All services pass health checks
- [ ] Documentation reviewed and tested
- [ ] CI workflow for building images
- [ ] Synology deployment tested on DS920+ or equivalent

---

## STATUS Signal

```
STATUS: PENDING
SUMMARY: Epic created, ready for implementation
FILES: _bmad-output/epics/epic-120-docker-infrastructure.md
NEXT: Create feature branch and implement Story 120.1
```
