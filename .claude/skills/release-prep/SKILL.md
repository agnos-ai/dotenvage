---
name: release-prep
description:
  Prepare a release including version bump, testing, and PR creation
---

# Release Preparation Skill

Use this skill when preparing a new release.

## Release Workflow Overview

1. Ensure all changes are committed and tests pass
2. Bump the version
3. Push and create PR
4. After merge, CI automatically creates tag and publishes

## Step 1: Pre-release Checks

Run all quality checks before releasing:

```bash
# Format check
cargo +nightly fmt --all -- --check

# Lint check
cargo +nightly clippy --all-targets --all-features -- -D warnings -W missing-docs

# Run tests
cargo test
```

## Step 2: Check Current State

```bash
# Current version
cargo version-info current

# Latest published release
cargo version-info latest

# Check for uncommitted changes
git status

# Check if version already changed
cargo version-info changed
```

## Step 3: Bump Version

Choose the appropriate bump type:

- **patch**: Bug fixes, minor improvements (0.0.8 -> 0.0.9)
- **minor**: New features, backward compatible (0.0.9 -> 0.1.0)
- **major**: Breaking changes (0.1.0 -> 1.0.0)

```bash
cargo version-info bump --patch
# or --minor / --major
```

This will also sync npm/package.json via the pre-bump hook.

## Step 4: Verify the Bump

```bash
# Check the commit
git log -1

# Verify version in Cargo.toml
cargo version-info current

# Check what files changed (should include npm/package.json)
git diff HEAD~1 --stat
```

## Step 5: Push and Create PR

```bash
# Push branch
git push origin HEAD

# Create PR (if on feature branch)
gh pr create --title "chore(version): bump to X.Y.Z" --body "Release X.Y.Z"
```

## Step 6: After Merge

Once the PR is merged to main:

1. CI runs tests (fmt, clippy, test on Linux/macOS/Windows)
2. CI detects version change (compares Cargo.toml with latest git tag)
3. CI creates git tag `vX.Y.Z`
4. CI generates changelog from conventional commits using Cocogitto
5. CI creates GitHub Release with changelog as release body
6. CI publishes to crates.io
7. CI builds and uploads binaries for all platforms
8. CI publishes npm packages

This is why all commits must follow Angular Conventional Commit style
(`<type>(<scope>): <subject>`) - Cocogitto parses these to generate
the changelog automatically.

## Checking Release Status

```bash
# Check latest release on GitHub
cargo version-info latest

# Check crates.io
cargo search dotenvage

# Check npm
npm view @anthropic/dotenvage version
```

## Troubleshooting

### Version already bumped

If `cargo version-info changed` returns true but you haven't released:

```bash
# Check what the current version is
cargo version-info current

# Check what the latest release is
cargo version-info latest
```

### CI didn't create a release

Check that:

1. The version in Cargo.toml differs from the latest git tag
2. The merge was to the main branch
3. CI workflow completed successfully

### Need to re-release same version

You cannot republish the same version to crates.io. Bump to a new
patch version instead.

### NPM version out of sync

If npm/package.json version doesn't match Cargo.toml, run:

```bash
./scripts/sync-npm-version.sh $(cargo version-info current)
```
