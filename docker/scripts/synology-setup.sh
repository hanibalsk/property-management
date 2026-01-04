#!/usr/bin/env bash
# =============================================================================
# Synology NAS Setup Script
# =============================================================================
# Prepares the Synology NAS for PPT deployment
# Run this script via SSH on your Synology NAS
#
# Usage: ./synology-setup.sh [volume_path]
# Default volume path: /volume1/docker/ppt

set -euo pipefail

# =============================================================================
# Configuration
# =============================================================================
VOLUME_PATH="${1:-/volume1/docker/ppt}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# =============================================================================
# Helper Functions
# =============================================================================
log() {
    echo -e "${GREEN}[$(date '+%Y-%m-%d %H:%M:%S')]${NC} $*"
}

warn() {
    echo -e "${YELLOW}[$(date '+%Y-%m-%d %H:%M:%S')] WARNING:${NC} $*"
}

error() {
    echo -e "${RED}[$(date '+%Y-%m-%d %H:%M:%S')] ERROR:${NC} $*" >&2
    exit 1
}

# =============================================================================
# Pre-flight Checks
# =============================================================================
check_prerequisites() {
    log "Checking prerequisites..."

    # Check if running as root or sudo
    if [[ $EUID -ne 0 ]]; then
        error "This script must be run as root or with sudo"
    fi

    # Check if Docker is installed
    if ! command -v docker &>/dev/null; then
        error "Docker is not installed. Please install Container Manager from Package Center."
    fi

    # Check if docker compose is available
    if ! docker compose version &>/dev/null; then
        error "Docker Compose is not available. Please update Container Manager."
    fi

    # Check if volume exists
    local volume_dir
    volume_dir=$(dirname "$VOLUME_PATH")
    if [[ ! -d "$volume_dir" ]]; then
        error "Volume directory $volume_dir does not exist"
    fi

    log "Prerequisites check passed"
}

# =============================================================================
# Directory Setup
# =============================================================================
setup_directories() {
    log "Setting up directories at $VOLUME_PATH..."

    # Create main directory structure
    mkdir -p "$VOLUME_PATH"/{postgres,redis,logs/{api-server,reality-server}}

    # Create backup directory
    mkdir -p "$VOLUME_PATH/backups"

    # Set permissions for logs and backups (UID 1000 for app containers)
    chown -R 1000:1000 "$VOLUME_PATH/logs" "$VOLUME_PATH/backups"

    # Set proper permissions for PostgreSQL (UID 70 is postgres user in alpine)
    chown -R 70:70 "$VOLUME_PATH/postgres"
    chmod 700 "$VOLUME_PATH/postgres"

    # Set permissions for Redis (UID 999 is redis user in alpine)
    chown -R 999:999 "$VOLUME_PATH/redis"

    log "Directories created successfully"
}

# =============================================================================
# Environment Configuration
# =============================================================================
setup_environment() {
    local env_file="$VOLUME_PATH/.env"

    if [[ -f "$env_file" ]]; then
        warn ".env file already exists at $env_file"
        warn "Skipping environment setup. Please verify your configuration."
        return
    fi

    log "Creating environment configuration..."

    # Generate secure passwords (using stronger entropy)
    local db_password
    local redis_password
    local jwt_secret
    db_password=$(openssl rand -base64 32)
    redis_password=$(openssl rand -base64 32)
    jwt_secret=$(openssl rand -base64 64)

    cat > "$env_file" << EOF
# =============================================================================
# PPT Synology Configuration (Auto-generated)
# =============================================================================
# Generated on: $(date)
# IMPORTANT: Keep this file secure and do not commit to version control

# Synology Settings
SYNOLOGY_VOLUME_PATH=$VOLUME_PATH
TZ=$(cat /etc/timezone 2>/dev/null || echo "Europe/Bratislava")

# Database
POSTGRES_USER=ppt
POSTGRES_PASSWORD=$db_password
POSTGRES_DB=ppt

# Redis
REDIS_PASSWORD=$redis_password

# Security
JWT_SECRET=$jwt_secret

# S3 Storage (configure your S3-compatible storage)
S3_ENDPOINT=https://your-s3-endpoint.com
S3_ACCESS_KEY=your-access-key
S3_SECRET_KEY=your-secret-key
S3_BUCKET=ppt-documents
S3_REGION=eu-central-1

# URLs (configure after setting up reverse proxy)
CORS_ALLOWED_ORIGINS=http://localhost:3000,http://localhost:3001
NEXT_PUBLIC_API_URL=http://localhost:8081
NEXT_PUBLIC_SITE_URL=http://localhost:3001

# Logging
RUST_LOG=warn

# Container Registry
REGISTRY=ghcr.io/hanibalsk
VERSION=latest
EOF

    chmod 600 "$env_file"
    log "Environment configuration created at $env_file"
    log "Please edit the file to configure S3 storage and URLs"
}

# =============================================================================
# Docker Compose Setup
# =============================================================================
setup_compose() {
    local compose_file="$VOLUME_PATH/docker-compose.yml"

    if [[ -f "$compose_file" ]]; then
        warn "docker-compose.yml already exists at $compose_file"
        return
    fi

    # Check if source compose file exists
    local source_compose="$SCRIPT_DIR/../../docker-compose.synology.yml"
    if [[ -f "$source_compose" ]]; then
        cp "$source_compose" "$compose_file"
        log "Copied docker-compose.synology.yml to $compose_file"
    else
        warn "Source compose file not found at $source_compose"
        warn "Please manually copy docker-compose.synology.yml to $VOLUME_PATH"
    fi
}

# =============================================================================
# Backup Script
# =============================================================================
setup_backup_script() {
    local backup_script="$VOLUME_PATH/backup.sh"

    cat > "$backup_script" << 'EOF'
#!/usr/bin/env bash
# PPT Backup Script for Synology
# Run via Task Scheduler for automated backups

set -euo pipefail

VOLUME_PATH="$(dirname "$0")"
BACKUP_DIR="$VOLUME_PATH/backups"
DATE=$(date '+%Y%m%d_%H%M%S')

# PostgreSQL backup
docker exec ppt-postgres pg_dump -U ppt ppt | gzip > "$BACKUP_DIR/postgres_$DATE.sql.gz"

# Keep only last 7 days of backups
find "$BACKUP_DIR" -name "postgres_*.sql.gz" -mtime +7 -delete

echo "Backup completed: postgres_$DATE.sql.gz"
EOF

    chmod +x "$backup_script"
    log "Backup script created at $backup_script"
    log "Add to Task Scheduler for automated backups"
}

# =============================================================================
# Summary
# =============================================================================
print_summary() {
    echo ""
    echo "============================================================================="
    echo " PPT Synology Setup Complete"
    echo "============================================================================="
    echo ""
    echo " Volume Path: $VOLUME_PATH"
    echo ""
    echo " Next Steps:"
    echo " 1. Edit $VOLUME_PATH/.env to configure:"
    echo "    - S3 storage settings"
    echo "    - Domain/URL settings"
    echo ""
    echo " 2. Copy docker-compose.synology.yml if not already present:"
    echo "    cp docker-compose.synology.yml $VOLUME_PATH/docker-compose.yml"
    echo ""
    echo " 3. Start the containers:"
    echo "    cd $VOLUME_PATH"
    echo "    docker compose up -d"
    echo ""
    echo " 4. Configure Synology Reverse Proxy (optional):"
    echo "    Control Panel > Login Portal > Advanced > Reverse Proxy"
    echo ""
    echo " 5. Set up automated backups:"
    echo "    Control Panel > Task Scheduler > Create > Scheduled Task"
    echo "    Script: $VOLUME_PATH/backup.sh"
    echo ""
    echo "============================================================================="
}

# =============================================================================
# Main
# =============================================================================
main() {
    log "Starting PPT Synology setup..."

    check_prerequisites
    setup_directories
    setup_environment
    setup_compose
    setup_backup_script

    print_summary
}

main "$@"
