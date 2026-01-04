#!/bin/bash
#
# clean.sh - Clean all build artifacts across all platforms
#
# Usage:
#   ./scripts/clean.sh           # Clean all
#   ./scripts/clean.sh backend   # Clean only backend
#   ./scripts/clean.sh frontend  # Clean only frontend
#   ./scripts/clean.sh mobile    # Clean only mobile-native
#   ./scripts/clean.sh --dry-run # Show what would be deleted
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

# Options
DRY_RUN=false
CLEAN_BACKEND=false
CLEAN_FRONTEND=false
CLEAN_MOBILE=false
CLEAN_ALL=true

# Parse arguments
for arg in "$@"; do
    case $arg in
        --dry-run|-n)
            DRY_RUN=true
            ;;
        backend|rust)
            CLEAN_BACKEND=true
            CLEAN_ALL=false
            ;;
        frontend|node|typescript)
            CLEAN_FRONTEND=true
            CLEAN_ALL=false
            ;;
        mobile|kotlin|mobile-native)
            CLEAN_MOBILE=true
            CLEAN_ALL=false
            ;;
        --help|-h)
            echo "Usage: $0 [options] [targets]"
            echo ""
            echo "Targets:"
            echo "  backend     Clean Rust build artifacts (target/)"
            echo "  frontend    Clean Node.js artifacts (node_modules/, dist/, .next/)"
            echo "  mobile      Clean Kotlin/Gradle artifacts (build/, .gradle/)"
            echo "  (default)   Clean all targets"
            echo ""
            echo "Options:"
            echo "  --dry-run, -n    Show what would be deleted without deleting"
            echo "  --help, -h       Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown argument: $arg${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# If specific targets were specified, use them; otherwise clean all
if [ "$CLEAN_ALL" = true ]; then
    CLEAN_BACKEND=true
    CLEAN_FRONTEND=true
    CLEAN_MOBILE=true
fi

# Helper functions
clean_dir() {
    local dir="$1"
    local desc="$2"

    if [ -d "$dir" ]; then
        local size
        size=$(du -sh "$dir" 2>/dev/null | cut -f1)
        if [ "$DRY_RUN" = true ]; then
            echo -e "  ${YELLOW}Would delete:${NC} $dir ($size)"
        else
            echo -e "  ${GREEN}Deleting:${NC} $dir ($size)"
            rm -rf "$dir"
        fi
    fi
}

clean_pattern() {
    local base_dir="$1"
    local pattern="$2"
    local desc="$3"

    # Find directories matching pattern
    while IFS= read -r -d '' dir; do
        clean_dir "$dir" "$desc"
    done < <(find "$base_dir" -type d -name "$pattern" -print0 2>/dev/null)
}

# =============================================================================
# Clean Backend (Rust)
# =============================================================================
if [ "$CLEAN_BACKEND" = true ]; then
    echo ""
    echo -e "${CYAN}${BOLD}Cleaning Backend (Rust)...${NC}"

    clean_dir "$ROOT_DIR/backend/target" "Rust build artifacts"

    # Optional: clean Cargo cache (usually not needed)
    # clean_dir "$HOME/.cargo/registry/cache" "Cargo cache"
fi

# =============================================================================
# Clean Frontend (TypeScript/Node.js)
# =============================================================================
if [ "$CLEAN_FRONTEND" = true ]; then
    echo ""
    echo -e "${CYAN}${BOLD}Cleaning Frontend (TypeScript)...${NC}"

    # Root node_modules
    clean_dir "$ROOT_DIR/frontend/node_modules" "Root node_modules"

    # App-specific artifacts
    clean_pattern "$ROOT_DIR/frontend/apps" "node_modules" "App node_modules"
    clean_pattern "$ROOT_DIR/frontend/apps" "dist" "Vite build output"
    clean_pattern "$ROOT_DIR/frontend/apps" ".next" "Next.js build output"
    clean_pattern "$ROOT_DIR/frontend/apps" ".turbo" "Turbo cache"
    clean_pattern "$ROOT_DIR/frontend/apps" "coverage" "Test coverage"

    # Package-specific artifacts
    clean_pattern "$ROOT_DIR/frontend/packages" "node_modules" "Package node_modules"
    clean_pattern "$ROOT_DIR/frontend/packages" "dist" "Package build output"

    # pnpm store (optional - can be large)
    # Uncomment to also clean pnpm store
    # PNPM_STORE=$(pnpm store path 2>/dev/null || true)
    # if [ -n "$PNPM_STORE" ] && [ -d "$PNPM_STORE" ]; then
    #     clean_dir "$PNPM_STORE" "pnpm store"
    # fi
fi

# =============================================================================
# Clean Mobile Native (Kotlin/Gradle)
# =============================================================================
if [ "$CLEAN_MOBILE" = true ]; then
    echo ""
    echo -e "${CYAN}${BOLD}Cleaning Mobile Native (Kotlin)...${NC}"

    clean_dir "$ROOT_DIR/mobile-native/.gradle" "Gradle cache"
    clean_dir "$ROOT_DIR/mobile-native/build" "Root build"
    clean_pattern "$ROOT_DIR/mobile-native" "build" "Module build directories"
    clean_dir "$ROOT_DIR/mobile-native/.kotlin" "Kotlin cache"

    # Clean Kotlin Multiplatform artifacts
    clean_dir "$ROOT_DIR/mobile-native/shared/build" "Shared module build"
    clean_dir "$ROOT_DIR/mobile-native/androidApp/build" "Android app build"
fi

# =============================================================================
# Clean Worktrees (if exists)
# =============================================================================
if [ -d "$ROOT_DIR/.worktrees" ]; then
    echo ""
    echo -e "${CYAN}${BOLD}Note: .worktrees/ directory exists${NC}"
    echo -e "  ${YELLOW}Worktrees contain their own build artifacts.${NC}"
    echo -e "  ${YELLOW}To clean them, run this script in each worktree.${NC}"
fi

# =============================================================================
# Summary
# =============================================================================
echo ""

if [ "$DRY_RUN" = true ]; then
    echo -e "${YELLOW}${BOLD}Dry run complete.${NC} No files were deleted."
    echo "Run without --dry-run to actually clean."
else
    echo -e "${GREEN}${BOLD}Clean complete!${NC}"
    echo ""
    echo "Freed disk space by removing build artifacts."
    echo ""
    echo "To rebuild:"
    echo "  Backend:  cd backend && cargo build"
    echo "  Frontend: cd frontend && pnpm install && pnpm build"
    echo "  Mobile:   cd mobile-native && ./gradlew build"
fi
