#!/bin/bash
# Sync all package versions with the main crate version
# Usage: sync-npm-version.sh <version>

VERSION="$1"

if [ -z "$VERSION" ]; then
  echo "Error: Version required"
  exit 1
fi

# Update [workspace.package] version in root Cargo.toml
# This is separate from [package] version which cargo set-version handles
if [ -f "Cargo.toml" ] && grep -q '^\[workspace\.package\]' Cargo.toml; then
  if [ "$(uname)" == "Darwin" ]; then
    # macOS: Update version line after [workspace.package] section
    sed -i '' '/^\[workspace\.package\]/,/^\[/{s/^version = ".*"/version = "'"$VERSION"'"/;}' Cargo.toml
  else
    # Linux
    sed -i '/^\[workspace\.package\]/,/^\[/{s/^version = ".*"/version = "'"$VERSION"'"/;}' Cargo.toml
  fi
  echo "✅ Updated [workspace.package] version to $VERSION"
fi

# Update npm/package.json
if [ "$(uname)" == "Darwin" ]; then
  # macOS
  sed -i '' "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" npm/package.json
else
  # Linux
  sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" npm/package.json
fi

# Update npm/dotenvage-napi/Cargo.toml dependency version
if [ -f "npm/dotenvage-napi/Cargo.toml" ]; then
  if [ "$(uname)" == "Darwin" ]; then
    sed -i '' "s/dotenvage = { path = \"..\/..\/\", version = \".*\" }/dotenvage = { path = \"..\/..\/\", version = \"$VERSION\" }/" npm/dotenvage-napi/Cargo.toml
  else
    sed -i "s/dotenvage = { path = \"..\/..\/\", version = \".*\" }/dotenvage = { path = \"..\/..\/\", version = \"$VERSION\" }/" npm/dotenvage-napi/Cargo.toml
  fi
  echo "✅ Updated npm/dotenvage-napi/Cargo.toml dependency to $VERSION"
fi

# Update python/pyproject.toml
if [ -f "python/pyproject.toml" ]; then
  if [ "$(uname)" == "Darwin" ]; then
    sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" python/pyproject.toml
  else
    sed -i "s/^version = \".*\"/version = \"$VERSION\"/" python/pyproject.toml
  fi
  echo "✅ Updated python/pyproject.toml version to $VERSION"
fi

# Update python/dotenvage-pyo3/Cargo.toml dependency version
if [ -f "python/dotenvage-pyo3/Cargo.toml" ]; then
  if [ "$(uname)" == "Darwin" ]; then
    sed -i '' "s/dotenvage = { path = \"..\/..\/\", version = \".*\" }/dotenvage = { path = \"..\/..\/\", version = \"$VERSION\" }/" python/dotenvage-pyo3/Cargo.toml
  else
    sed -i "s/dotenvage = { path = \"..\/..\/\", version = \".*\" }/dotenvage = { path = \"..\/..\/\", version = \"$VERSION\" }/" python/dotenvage-pyo3/Cargo.toml
  fi
  echo "✅ Updated python/dotenvage-pyo3/Cargo.toml dependency to $VERSION"
fi

# Update root package.json if it exists and has a version field
if [ -f "package.json" ] && grep -q '"version":' package.json; then
  if [ "$(uname)" == "Darwin" ]; then
    sed -i '' "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json
  else
    sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json
  fi
fi

echo "✅ Synced all package versions to $VERSION"
