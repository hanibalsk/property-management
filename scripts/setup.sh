#!/bin/bash
#
# setup.sh - One-command development environment setup
#
# Usage: ./scripts/setup.sh
#
# This script will:
#   1. Verify required tools are installed
#   2. Install git hooks
#   3. Install all dependencies
#   4. Set up environment files
#   5. Run initial build checks
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# Track any warnings
WARNINGS=""

# Helper functions
print_step() {
    echo ""
    echo -e "${CYAN}${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}${BOLD}  $1${NC}"
    echo -e "${CYAN}${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
    WARNINGS="${WARNINGS}\n  - $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

check_command() {
    if command -v "$1" &> /dev/null; then
        print_success "$1 found: $(command -v "$1")"
        return 0
    else
        print_error "$1 not found"
        return 1
    fi
}

# =============================================================================
print_step "1/6 - Checking Required Tools"
# =============================================================================

MISSING_TOOLS=0

# Rust
if check_command rustc; then
    echo "    Version: $(rustc --version)"
else
    print_warning "Rust not installed. Install from https://rustup.rs"
    MISSING_TOOLS=1
fi

# Cargo
if check_command cargo; then
    echo "    Version: $(cargo --version)"
else
    MISSING_TOOLS=1
fi

# Node.js
if check_command node; then
    echo "    Version: $(node --version)"
    NODE_VERSION=$(node --version | sed 's/v//' | cut -d. -f1)
    if [ "$NODE_VERSION" -lt 20 ]; then
        print_warning "Node.js version 20+ recommended, found v$NODE_VERSION"
    fi
else
    print_warning "Node.js not installed. Install from https://nodejs.org (v20+)"
    MISSING_TOOLS=1
fi

# pnpm
if check_command pnpm; then
    echo "    Version: $(pnpm --version)"
else
    print_warning "pnpm not installed. Install with: npm install -g pnpm"
    MISSING_TOOLS=1
fi

# PostgreSQL client (optional but recommended)
if check_command psql; then
    echo "    Version: $(psql --version)"
else
    print_warning "PostgreSQL client (psql) not found - needed for database operations"
fi

# SQLx CLI (optional)
if check_command sqlx; then
    print_success "sqlx-cli found"
else
    print_warning "sqlx-cli not installed. Install with: cargo install sqlx-cli --no-default-features --features postgres"
fi

# Docker (optional)
if check_command docker; then
    print_success "docker found"
else
    print_warning "Docker not found - useful for running PostgreSQL locally"
fi

# Just (optional task runner)
if check_command just; then
    print_success "just found: $(just --version)"
else
    print_warning "just not installed - recommended for task running. Install from https://github.com/casey/just"
fi

if [ $MISSING_TOOLS -eq 1 ]; then
    echo ""
    print_error "Some required tools are missing. Install them and re-run setup."
    exit 1
fi

# =============================================================================
print_step "2/6 - Installing Git Hooks"
# =============================================================================

if [ -f "$SCRIPT_DIR/install-hooks.sh" ]; then
    "$SCRIPT_DIR/install-hooks.sh"
    print_success "Git hooks installed"
else
    print_warning "install-hooks.sh not found, skipping"
fi

# =============================================================================
print_step "3/6 - Installing Backend Dependencies"
# =============================================================================

cd "$ROOT_DIR/backend"

# Check if Cargo.lock exists, if not create it
if [ ! -f "Cargo.lock" ]; then
    echo "Creating Cargo.lock..."
    cargo generate-lockfile
fi

echo "Checking Rust dependencies..."
if cargo check --workspace 2>/dev/null; then
    print_success "Backend dependencies OK"
else
    print_warning "Some backend dependencies may need attention"
fi

# Install useful cargo tools
echo ""
echo "Installing recommended Cargo tools..."
if cargo install cargo-watch 2>/dev/null; then
    print_success "cargo-watch installed"
else
    echo "    (cargo-watch already installed or failed)"
fi

# =============================================================================
print_step "4/6 - Installing Frontend Dependencies"
# =============================================================================

cd "$ROOT_DIR/frontend"

echo "Installing pnpm dependencies..."
if pnpm install; then
    print_success "Frontend dependencies installed"
else
    print_error "Failed to install frontend dependencies"
fi

# =============================================================================
print_step "5/6 - Setting Up Environment"
# =============================================================================

cd "$ROOT_DIR"

# Create .env.local template if it doesn't exist
if [ ! -f "$ROOT_DIR/backend/.env" ] && [ ! -f "$ROOT_DIR/backend/.env.local" ]; then
    cat > "$ROOT_DIR/backend/.env.example" << 'EOF'
# Backend Environment Variables
#
# Copy this file to .env or .env.local and fill in the values
#

# Database (required)
DATABASE_URL=postgres://user:password@localhost:5432/ppt

# JWT Secret (required - minimum 32 characters)
JWT_SECRET=your-secure-random-secret-key-minimum-32-characters

# Optional
RUST_LOG=info
CORS_ALLOWED_ORIGINS=http://localhost:3000,http://localhost:3001

# Redis (optional, for sessions)
REDIS_URL=redis://localhost:6379
EOF
    print_success "Created backend/.env.example template"
    print_warning "Copy backend/.env.example to backend/.env and configure your database"
else
    print_success "Backend environment file exists"
fi

# Create frontend env template if needed
if [ ! -f "$ROOT_DIR/frontend/.env" ] && [ ! -f "$ROOT_DIR/frontend/.env.local" ]; then
    cat > "$ROOT_DIR/frontend/.env.example" << 'EOF'
# Frontend Environment Variables
#
# Copy this file to .env.local and fill in the values
#

# API Server URL
VITE_API_URL=http://localhost:8080
NEXT_PUBLIC_API_URL=http://localhost:8081
EOF
    print_success "Created frontend/.env.example template"
fi

# =============================================================================
print_step "6/6 - Running Initial Checks"
# =============================================================================

cd "$ROOT_DIR"

echo "Checking code formatting..."
cd "$ROOT_DIR/backend"
if cargo fmt --all -- --check 2>/dev/null; then
    print_success "Backend formatting OK"
else
    print_warning "Backend has formatting issues (run: cd backend && cargo fmt --all)"
fi

cd "$ROOT_DIR/frontend"
if pnpm check 2>/dev/null; then
    print_success "Frontend formatting OK"
else
    print_warning "Frontend has formatting issues (run: cd frontend && pnpm check:fix)"
fi

# =============================================================================
# Summary
# =============================================================================

echo ""
echo -e "${GREEN}${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}${BOLD}  Setup Complete!${NC}"
echo -e "${GREEN}${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

if [ -n "$WARNINGS" ]; then
    echo -e "${YELLOW}Warnings:${NC}"
    echo -e "$WARNINGS"
    echo ""
fi

echo -e "${CYAN}Quick Start:${NC}"
echo ""
echo "  # Start backend API server"
echo "  cd backend && cargo run -p api-server"
echo ""
echo "  # Start frontend (Property Management)"
echo "  cd frontend && pnpm dev:ppt"
echo ""
echo "  # Or use just (if installed):"
echo "  just api     # Run API server"
echo "  just web     # Run frontend"
echo ""
echo -e "${CYAN}Available Commands:${NC}"
echo ""
echo "  just --list         # Show all available commands"
echo "  npm run             # Show npm scripts (from root)"
echo "  ./scripts/health-check.sh  # Verify environment"
echo ""
echo -e "${CYAN}Documentation:${NC}"
echo ""
echo "  CLAUDE.md           # Project conventions"
echo "  docs/               # Full documentation"
echo ""
