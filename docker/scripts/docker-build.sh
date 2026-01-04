#!/usr/bin/env bash
# =============================================================================
# Multi-Architecture Docker Build Script
# =============================================================================
# Builds Docker images for linux/amd64 and linux/arm64 platforms
# Usage: ./docker/scripts/docker-build.sh [options]
#
# Options:
#   --push          Push images to registry
#   --platform      Specific platform (default: linux/amd64,linux/arm64)
#   --registry      Container registry (default: ghcr.io/hanibalsk)
#   --version       Image version tag (default: latest)
#   --service       Build specific service (api-server, reality-server, ppt-web, reality-web)
#   --help          Show this help message

set -euo pipefail

# =============================================================================
# Configuration
# =============================================================================
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Defaults
REGISTRY="${REGISTRY:-ghcr.io/hanibalsk}"
VERSION="${VERSION:-$(cat "$PROJECT_ROOT/VERSION" 2>/dev/null || echo "latest")}"
PLATFORMS="${PLATFORMS:-linux/amd64,linux/arm64}"
PUSH=false
SERVICE=""

# =============================================================================
# Parse Arguments
# =============================================================================
while [[ $# -gt 0 ]]; do
    case $1 in
        --push)
            PUSH=true
            shift
            ;;
        --platform)
            PLATFORMS="$2"
            shift 2
            ;;
        --registry)
            REGISTRY="$2"
            shift 2
            ;;
        --version)
            VERSION="$2"
            shift 2
            ;;
        --service)
            SERVICE="$2"
            shift 2
            ;;
        --help)
            head -20 "$0" | tail -n +2 | sed 's/^# //'
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# =============================================================================
# Helper Functions
# =============================================================================
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $*"
}

error() {
    log "ERROR: $*" >&2
    exit 1
}

build_image() {
    local name="$1"
    local dockerfile="$2"
    local context="$3"
    local target="${4:-}"

    local image_name="$REGISTRY/ppt-$name:$VERSION"
    local build_args=(
        --file "$dockerfile"
        --platform "$PLATFORMS"
        --tag "$image_name"
        --tag "$REGISTRY/ppt-$name:latest"
    )

    if [[ -n "$target" ]]; then
        build_args+=(--target "$target")
    fi

    if [[ "$PUSH" == "true" ]]; then
        build_args+=(--push)
    else
        build_args+=(--load)
    fi

    log "Building $name..."
    log "  Image: $image_name"
    log "  Platforms: $PLATFORMS"
    log "  Push: $PUSH"

    docker buildx build "${build_args[@]}" "$context"

    log "Successfully built $name"
}

# =============================================================================
# Setup Buildx
# =============================================================================
setup_buildx() {
    log "Setting up Docker Buildx..."

    # Create/use buildx builder
    if ! docker buildx inspect ppt-builder &>/dev/null; then
        docker buildx create --name ppt-builder --driver docker-container --bootstrap
    fi
    docker buildx use ppt-builder

    log "Buildx ready"
}

# =============================================================================
# Build Services
# =============================================================================
build_api_server() {
    build_image "api-server" \
        "$PROJECT_ROOT/docker/backend/Dockerfile" \
        "$PROJECT_ROOT" \
        "api-server"
}

build_reality_server() {
    build_image "reality-server" \
        "$PROJECT_ROOT/docker/backend/Dockerfile" \
        "$PROJECT_ROOT" \
        "reality-server"
}

build_ppt_web() {
    build_image "ppt-web" \
        "$PROJECT_ROOT/docker/frontend/ppt-web.Dockerfile" \
        "$PROJECT_ROOT"
}

build_reality_web() {
    build_image "reality-web" \
        "$PROJECT_ROOT/docker/frontend/reality-web.Dockerfile" \
        "$PROJECT_ROOT"
}

# =============================================================================
# Main
# =============================================================================
main() {
    cd "$PROJECT_ROOT"

    setup_buildx

    if [[ -n "$SERVICE" ]]; then
        case "$SERVICE" in
            api-server)     build_api_server ;;
            reality-server) build_reality_server ;;
            ppt-web)        build_ppt_web ;;
            reality-web)    build_reality_web ;;
            *)              error "Unknown service: $SERVICE" ;;
        esac
    else
        log "Building all services..."
        build_api_server
        build_reality_server
        build_ppt_web
        build_reality_web
    fi

    log "All builds completed successfully!"

    if [[ "$PUSH" == "true" ]]; then
        log "Images pushed to $REGISTRY"
    else
        log "Images built locally. Use --push to push to registry."
    fi
}

main "$@"
