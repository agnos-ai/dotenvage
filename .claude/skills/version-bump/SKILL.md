---
name: version-bump
description: Bump the version of the dotenvage project using cargo version-info
---

# Version Bump

Use this skill when the user asks to bump, increase, or release a new
version of the project.

## Prerequisites

- `cargo-version-info` must be installed (v0.0.15 or later)
- Working directory must be clean (no uncommitted changes)

## Commands

```bash
# Patch version bump (0.2.4 -> 0.2.5)
cargo version-info bump --patch

# Minor version bump (0.2.4 -> 0.3.0)
cargo version-info bump --minor

# Major version bump (0.2.4 -> 1.0.0)
cargo version-info bump --major
```

## What It Does

The `cargo version-info bump` command:

1. Runs `pre_bump_hooks` defined in `Cargo.toml`:
   - Executes `./scripts/sync-npm-version.sh {{version}}`
   - Updates `[workspace.package]` version in Cargo.toml
   - Updates `npm/package.json` version
   - Updates `npm/dotenvage-napi/Cargo.toml` dependency version
   - Updates `package.json` version
2. Bumps the version in the main `[package]` section
3. Updates `Cargo.lock`
4. Commits all changed files listed in `additional_files`

## Files Updated

- `Cargo.toml` - workspace and package versions
- `Cargo.lock` - lockfile
- `npm/package.json` - npm package version
- `npm/dotenvage-napi/Cargo.toml` - NAPI binding dependency
- `package.json` - root workspace version

## After Bumping

After the version bump commit is created:

1. **Do NOT push** - let the user push manually
2. The CI will detect the version change and:
   - Create a git tag
   - Generate changelog
   - Create GitHub release
   - Publish to crates.io
   - Build binaries for all platforms
   - Publish to npmjs.org

## Troubleshooting

If the version bump fails or files are missing from the commit:

1. Check that `additional_files` in `Cargo.toml` includes all npm
   files
2. Verify pre_bump_hooks ran successfully (look for the sync
   messages)
3. If needed, manually stage files and amend the commit

## Configuration

The version bump is configured in `Cargo.toml`:

```toml
[package.metadata.version-info]
pre_bump_hooks = [
    "./scripts/sync-npm-version.sh {{version}}"
]
additional_files = [
    "npm/package.json",
    "npm/dotenvage-napi/Cargo.toml",
    "package.json"
]
```
