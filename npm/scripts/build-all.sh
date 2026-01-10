#!/bin/bash
# Build NAPI bindings for all configured targets
# Note: Cross-compilation requires proper toolchains. This script attempts
# to build for all targets but gracefully handles failures.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NPM_DIR="$(dirname "$SCRIPT_DIR")"
CARGO_TOML="$NPM_DIR/dotenvage-napi/Cargo.toml"

# Targets from package.json napi.targets
# Only build targets that are possible from the current platform
# Cross-compilation requires additional setup
TARGETS=(
  "x86_64-unknown-linux-gnu"
  "x86_64-unknown-linux-musl"
  "aarch64-unknown-linux-gnu"
  "aarch64-unknown-linux-musl"
  "x86_64-apple-darwin"
  "aarch64-apple-darwin"
  "x86_64-pc-windows-msvc"
  "aarch64-pc-windows-msvc"
  "i686-pc-windows-msvc"
)

cd "$NPM_DIR"

BUILT=0
FAILED=0

for target in "${TARGETS[@]}"; do
  echo "Building for target: $target"
  if napi build \
    --manifest-path "$CARGO_TOML" \
    --output-dir . \
    --platform \
    --release \
    --target "$target" 2>&1; then
    ((BUILT++))
    echo "✅ Successfully built for $target"
  else
    ((FAILED++))
    echo "⚠️  Failed to build for $target (cross-compilation toolchain may be required)"
  fi
done

echo ""
echo "Build summary: $BUILT succeeded, $FAILED failed"
if [ $BUILT -eq 0 ]; then
  echo "❌ No targets built successfully"
  exit 1
fi
