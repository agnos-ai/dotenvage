#!/bin/bash
# Sync all package versions with the main crate version
# Usage: sync-npm-version.sh <version>
#
# Note: Cargo.toml versions are handled by cargo version-info bump.
# This script only syncs non-Cargo package files (npm, python).

VERSION="$1"

if [ -z "$VERSION" ]; then
  echo "Error: Version required"
  exit 1
fi

# Update npm/package.json
if [ -f "npm/package.json" ]; then
  if [ "$(uname)" == "Darwin" ]; then
    sed -i '' "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" npm/package.json
  else
    sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" npm/package.json
  fi
  echo "✅ Updated npm/package.json to $VERSION"
fi

# Update python/pyproject.toml
if [ -f "python/pyproject.toml" ]; then
  if [ "$(uname)" == "Darwin" ]; then
    sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" python/pyproject.toml
  else
    sed -i "s/^version = \".*\"/version = \"$VERSION\"/" python/pyproject.toml
  fi
  echo "✅ Updated python/pyproject.toml to $VERSION"
fi

# Update root package.json if it exists and has a version field
if [ -f "package.json" ] && grep -q '"version":' package.json; then
  if [ "$(uname)" == "Darwin" ]; then
    sed -i '' "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json
  else
    sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json
  fi
  echo "✅ Updated package.json to $VERSION"
fi

echo "✅ Synced all package versions to $VERSION"
