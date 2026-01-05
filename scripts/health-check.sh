#!/bin/bash
#
# health-check.sh - Verify development environment health
#
# Usage: ./scripts/health-check.sh
#
# This script checks:
#   - Required tools and their versions
#   - Project dependencies status
#   - Build artifacts
#   - Database connectivity (if configured)
#   - Environment configuration
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

# Counters
CHECKS_PASSED=0
CHECKS_FAILED=0
CHECKS_WARNED=0

# Helper functions
print_header() {
    echo ""
    echo -e "${CYAN}${BOLD}━━━ $1 ━━━${NC}"
}

check_pass() {
    echo -e "  ${GREEN}✓${NC} $1"
    ((CHECKS_PASSED++))
}

check_fail() {
    echo -e "  ${RED}✗${NC} $1"
    ((CHECKS_FAILED++))
}

check_warn() {
    echo -e "  ${YELLOW}⚠${NC} $1"
    ((CHECKS_WARNED++))
}

check_info() {
    echo -e "  ${CYAN}ℹ${NC} $1"
}

# =============================================================================
print_header "System Tools"
# =============================================================================

# Rust
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    RUST_MAJOR=$(echo "$RUST_VERSION" | cut -d. -f1)
    RUST_MINOR=$(echo "$RUST_VERSION" | cut -d. -f2)
    if [ "$RUST_MAJOR" -ge 1 ] && [ "$RUST_MINOR" -ge 75 ]; then
        check_pass "Rust $RUST_VERSION (>= 1.75 required)"
    else
        check_warn "Rust $RUST_VERSION (1.75+ recommended)"
    fi
else
    check_fail "Rust not installed"
fi

# Cargo
if command -v cargo &> /dev/null; then
    check_pass "Cargo $(cargo --version | cut -d' ' -f2)"
else
    check_fail "Cargo not installed"
fi

# Node.js
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version | sed 's/v//')
    NODE_MAJOR=$(echo "$NODE_VERSION" | cut -d. -f1)
    if [ "$NODE_MAJOR" -ge 20 ]; then
        check_pass "Node.js $NODE_VERSION (>= 20 required)"
    else
        check_warn "Node.js $NODE_VERSION (20+ recommended)"
    fi
else
    check_fail "Node.js not installed"
fi

# pnpm
if command -v pnpm &> /dev/null; then
    PNPM_VERSION=$(pnpm --version)
    PNPM_MAJOR=$(echo "$PNPM_VERSION" | cut -d. -f1)
    if [ "$PNPM_MAJOR" -ge 8 ]; then
        check_pass "pnpm $PNPM_VERSION (>= 8 required)"
    else
        check_warn "pnpm $PNPM_VERSION (8+ recommended)"
    fi
else
    check_fail "pnpm not installed"
fi

# Optional tools
if command -v sqlx &> /dev/null; then
    check_pass "sqlx-cli installed"
else
    check_warn "sqlx-cli not installed (needed for migrations)"
fi

if command -v docker &> /dev/null; then
    check_pass "Docker installed"
else
    check_info "Docker not installed (optional)"
fi

if command -v just &> /dev/null; then
    check_pass "just task runner installed"
else
    check_info "just not installed (optional, but recommended)"
fi

# =============================================================================
print_header "Project Structure"
# =============================================================================

# Check critical directories exist
for dir in "backend" "frontend" "mobile-native" "docs" "scripts"; do
    if [ -d "$ROOT_DIR/$dir" ]; then
        check_pass "$dir/ directory exists"
    else
        check_fail "$dir/ directory missing"
    fi
done

# Check VERSION file
if [ -f "$ROOT_DIR/VERSION" ]; then
    VERSION=$(cat "$ROOT_DIR/VERSION" | tr -d '[:space:]')
    if [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        check_pass "VERSION file valid: $VERSION"
    else
        check_fail "VERSION file has invalid format: $VERSION"
    fi
else
    check_fail "VERSION file missing"
fi

# =============================================================================
print_header "Backend (Rust)"
# =============================================================================

cd "$ROOT_DIR/backend"

# Check Cargo.toml
if [ -f "Cargo.toml" ]; then
    check_pass "Cargo.toml exists"
else
    check_fail "Cargo.toml missing"
fi

# Check Cargo.lock
if [ -f "Cargo.lock" ]; then
    check_pass "Cargo.lock exists"
else
    check_warn "Cargo.lock missing (run: cargo build)"
fi

# Check if target directory exists (has been built)
if [ -d "target" ]; then
    check_pass "target/ exists (has been built)"
else
    check_info "target/ missing (not built yet)"
fi

# Check SQLx offline data
if [ -f ".sqlx/query-*.json" ] 2>/dev/null || [ -d ".sqlx" ]; then
    check_pass "SQLx offline data present"
else
    check_warn "SQLx offline data missing (run: cargo sqlx prepare)"
fi

# Quick cargo check (only if fast)
echo -e "  ${CYAN}...${NC} Running cargo check (this may take a moment)"
if cargo check --workspace 2>/dev/null; then
    check_pass "cargo check passes"
else
    check_fail "cargo check failed"
fi

# =============================================================================
print_header "Frontend (TypeScript)"
# =============================================================================

cd "$ROOT_DIR/frontend"

# Check package.json
if [ -f "package.json" ]; then
    check_pass "package.json exists"
else
    check_fail "package.json missing"
fi

# Check node_modules
if [ -d "node_modules" ]; then
    check_pass "node_modules/ exists"
else
    check_fail "node_modules/ missing (run: pnpm install)"
fi

# Check pnpm-lock.yaml
if [ -f "pnpm-lock.yaml" ]; then
    check_pass "pnpm-lock.yaml exists"
else
    check_warn "pnpm-lock.yaml missing"
fi

# Check biome config
if [ -f "biome.json" ]; then
    check_pass "biome.json exists"
else
    check_warn "biome.json missing (linting config)"
fi

# Run typecheck
echo -e "  ${CYAN}...${NC} Running typecheck"
if pnpm typecheck 2>/dev/null; then
    check_pass "TypeScript typecheck passes"
else
    check_warn "TypeScript typecheck has issues"
fi

# =============================================================================
print_header "Mobile Native (Kotlin)"
# =============================================================================

cd "$ROOT_DIR/mobile-native"

# Check gradlew
if [ -f "gradlew" ]; then
    check_pass "gradlew exists"
    if [ -x "gradlew" ]; then
        check_pass "gradlew is executable"
    else
        check_warn "gradlew is not executable (run: chmod +x gradlew)"
    fi
else
    check_fail "gradlew missing"
fi

# Check gradle.properties
if [ -f "gradle.properties" ]; then
    if grep -q "app.versionName" gradle.properties; then
        check_pass "gradle.properties has version info"
    else
        check_warn "gradle.properties missing version info"
    fi
else
    check_fail "gradle.properties missing"
fi

# =============================================================================
print_header "Environment Configuration"
# =============================================================================

cd "$ROOT_DIR"

# Backend .env
if [ -f "backend/.env" ] || [ -f "backend/.env.local" ]; then
    check_pass "Backend environment file exists"

    # Check for required variables
    ENV_FILE="backend/.env"
    if [ -f "backend/.env.local" ]; then
        ENV_FILE="backend/.env.local"
    fi

    if grep -q "DATABASE_URL" "$ENV_FILE" 2>/dev/null; then
        check_pass "DATABASE_URL configured"
    else
        check_warn "DATABASE_URL not set"
    fi

    if grep -q "JWT_SECRET" "$ENV_FILE" 2>/dev/null; then
        check_pass "JWT_SECRET configured"
    else
        check_warn "JWT_SECRET not set"
    fi
else
    check_warn "Backend environment file missing (copy backend/.env.example)"
fi

# =============================================================================
print_header "Git Hooks"
# =============================================================================

if [ -f "$ROOT_DIR/.git/hooks/pre-commit" ]; then
    if [ -x "$ROOT_DIR/.git/hooks/pre-commit" ]; then
        check_pass "pre-commit hook installed and executable"
    else
        check_warn "pre-commit hook exists but not executable"
    fi
else
    check_warn "pre-commit hook not installed (run: ./scripts/install-hooks.sh)"
fi

# =============================================================================
# Summary
# =============================================================================

echo ""
echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}  Health Check Summary${NC}"
echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "  ${GREEN}Passed:${NC}  $CHECKS_PASSED"
echo -e "  ${YELLOW}Warnings:${NC} $CHECKS_WARNED"
echo -e "  ${RED}Failed:${NC}  $CHECKS_FAILED"
echo ""

if [ $CHECKS_FAILED -gt 0 ]; then
    echo -e "${RED}Some checks failed. Please resolve the issues above.${NC}"
    exit 1
elif [ $CHECKS_WARNED -gt 0 ]; then
    echo -e "${YELLOW}All required checks passed, but there are warnings to address.${NC}"
    exit 0
else
    echo -e "${GREEN}All checks passed! Environment is healthy.${NC}"
    exit 0
fi
