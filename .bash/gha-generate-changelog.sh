#!/usr/bin/env bash
# shellcheck shell=bash
#
# gha-generate-changelog.sh
#
# Generate changelog from conventional commits using cocogitto
# Used by .github/actions/generate-changelog
#
set -euo pipefail

OUTPUT_FILE="${1:-CHANGELOG.md}"
TAG="${2:-}"

if [ -z "$TAG" ]; then
  echo "Error: Release tag required"
  exit 1
fi

# Ensure cargo-installed binaries (like `cog`) are discoverable in composite actions.
export PATH="$HOME/.cargo/bin:$PATH"

echo "Generating changelog for ${TAG}..."

# Ensure cocogitto is available
if ! command -v cog &> /dev/null; then
  echo "Error: cocogitto (cog) not found in PATH"
  exit 1
fi

# Check if tag exists (it might not exist yet when called from build jobs)
if git rev-parse "$TAG" >/dev/null 2>&1; then
  # Tag exists, generate changelog for this tag
  echo "Tag ${TAG} exists, generating changelog..."
  cog changelog --at "$TAG" > "$OUTPUT_FILE"
else
  # Tag doesn't exist yet, generate changelog from previous tag to HEAD
  echo "Tag ${TAG} doesn't exist yet, generating changelog from previous tag..."

  # Get the previous tag (or empty if none)
  PREVIOUS_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")

  if [ -n "$PREVIOUS_TAG" ] && [ "$PREVIOUS_TAG" != "$TAG" ]; then
    echo "Generating changelog from ${PREVIOUS_TAG} to HEAD..."
    cog changelog "${PREVIOUS_TAG}..HEAD" > "$OUTPUT_FILE"
  else
    # No previous tag, generate changelog for all commits
    echo "No previous tag found, generating changelog for all commits..."
    cog changelog > "$OUTPUT_FILE"
  fi
fi

echo "✓ Changelog generated"

