#!/bin/bash
#
# Install git hooks for version management
#
# This script installs the pre-commit hook that auto-bumps
# patch version on every commit.
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
HOOKS_DIR="$ROOT_DIR/.git/hooks"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${YELLOW}Installing git hooks...${NC}"

# Check if .git directory exists
if [[ ! -d "$ROOT_DIR/.git" ]]; then
    echo -e "${RED}ERROR: .git directory not found. Are you in a git repository?${NC}"
    exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p "$HOOKS_DIR"

# Install pre-commit hook
if [[ -f "$HOOKS_DIR/pre-commit" ]]; then
    echo -e "${YELLOW}Backing up existing pre-commit hook...${NC}"
    mv "$HOOKS_DIR/pre-commit" "$HOOKS_DIR/pre-commit.backup.$(date +%Y%m%d%H%M%S)"
fi

cp "$SCRIPT_DIR/pre-commit" "$HOOKS_DIR/pre-commit"
chmod +x "$HOOKS_DIR/pre-commit"

echo -e "${GREEN}✓ Pre-commit hook installed${NC}"
echo ""
echo "The following hooks are now active:"
echo "  - pre-commit: Auto-bumps patch version on every commit"
echo ""
echo "Manual version bumping:"
echo "  ./scripts/bump-version.sh minor   # Bump minor version (resets patch)"
echo "  ./scripts/bump-version.sh major   # Bump major version (resets minor, patch)"
echo ""
