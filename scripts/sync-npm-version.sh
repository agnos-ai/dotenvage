#!/bin/bash
# Sync npm package versions with Rust crate version
# Usage: sync-npm-version.sh <version>

VERSION="$1"

if [ -z "$VERSION" ]; then
  echo "Error: Version required"
  exit 1
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

# Update root package.json if it exists and has a version field
if [ -f "package.json" ] && grep -q '"version":' package.json; then
  if [ "$(uname)" == "Darwin" ]; then
    sed -i '' "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json
  else
    sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json
  fi
fi

echo "✅ Synced all package versions to $VERSION"
