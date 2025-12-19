#!/bin/bash
#
# update-version.sh - Synchronize version across all projects
#
# Reads VERSION file and updates:
# - frontend/package.json
# - frontend/apps/*/package.json
# - frontend/packages/*/package.json
# - mobile-native/gradle.properties (for Android versionName)
#
# Usage: ./scripts/update-version.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
VERSION_FILE="$ROOT_DIR/VERSION"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check VERSION file exists
if [[ ! -f "$VERSION_FILE" ]]; then
    echo -e "${RED}ERROR: VERSION file not found at $VERSION_FILE${NC}"
    exit 1
fi

# Read and validate version
VERSION=$(cat "$VERSION_FILE" | tr -d '[:space:]')

if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}ERROR: Invalid version format '$VERSION'. Expected X.Y.Z (semantic versioning)${NC}"
    exit 1
fi

# Parse version components
IFS='.' read -r MAJOR MINOR PATCH <<< "$VERSION"

# Calculate Android versionCode: MAJOR * 10000 + MINOR * 100 + PATCH
VERSION_CODE=$((MAJOR * 10000 + MINOR * 100 + PATCH))

echo -e "${GREEN}Version: $VERSION${NC}"
echo -e "${GREEN}Version Code: $VERSION_CODE${NC}"
echo ""

# Function to update package.json version
update_package_json() {
    local file="$1"
    if [[ -f "$file" ]]; then
        # Only update if file has a version field
        if grep -q '"version"' "$file"; then
            # Use temporary file to avoid sed -i portability issues
            local tmp_file="${file}.tmp"
            sed "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" "$file" > "$tmp_file"
            mv "$tmp_file" "$file"
            echo -e "  ${GREEN}✓${NC} Updated $file"
        fi
    fi
}

# Update frontend package.json files
echo "Updating frontend packages..."
update_package_json "$ROOT_DIR/frontend/package.json"

for app_dir in "$ROOT_DIR/frontend/apps"/*; do
    if [[ -d "$app_dir" ]]; then
        update_package_json "$app_dir/package.json"
    fi
done

for pkg_dir in "$ROOT_DIR/frontend/packages"/*; do
    if [[ -d "$pkg_dir" ]]; then
        update_package_json "$pkg_dir/package.json"
    fi
done

# Update mobile-native gradle.properties with version info
GRADLE_PROPS="$ROOT_DIR/mobile-native/gradle.properties"
if [[ -f "$GRADLE_PROPS" ]]; then
    # Remove existing version properties if present
    grep -v "^app.version" "$GRADLE_PROPS" > "$GRADLE_PROPS.tmp" || true
    mv "$GRADLE_PROPS.tmp" "$GRADLE_PROPS"

    # Append version properties
    echo "" >> "$GRADLE_PROPS"
    echo "# App version (synced from VERSION file)" >> "$GRADLE_PROPS"
    echo "app.versionName=$VERSION" >> "$GRADLE_PROPS"
    echo "app.versionCode=$VERSION_CODE" >> "$GRADLE_PROPS"

    echo -e "  ${GREEN}✓${NC} Updated $GRADLE_PROPS"
fi

echo ""
echo -e "${GREEN}Version synchronization complete!${NC}"
echo ""
echo "Summary:"
echo "  Version:      $VERSION"
echo "  Version Code: $VERSION_CODE"
echo "  Major:        $MAJOR"
echo "  Minor:        $MINOR"
echo "  Patch:        $PATCH"
