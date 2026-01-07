# Reusable GitHub Actions

This directory contains reusable composite actions for common CI/CD
tasks. These actions provide caching and consistent behavior across
workflows.

## Available Actions

### setup-cargo-binstall

Installs `cargo-binstall` for fast binary installations with caching.

**Usage:**

```yaml
- name: Setup cargo-binstall
  uses: ./.github/actions/setup-cargo-binstall
```

**Features:**

- Caches the binary between runs
- Cross-platform support (Linux, macOS, Windows)
- Automatic PATH configuration

**Dependencies:** None

---

### setup-cargo-edit

Installs `cargo-edit` (provides `cargo set-version`, `cargo add`,
etc.) with caching.

**Usage:**

```yaml
- name: Setup cargo-edit
  uses: ./.github/actions/setup-cargo-edit
```

**Features:**

- Uses cargo-binstall for fast installation
- Caches the binary between runs
- Pins to version 0.13.0 for consistency
- Cross-platform support
- Required for Cocogitto version bumping

**Dependencies:**

- `setup-cargo-binstall` (called automatically)

**After this step:**

```bash
cargo set-version 0.1.0    # Set version in Cargo.toml
cargo add serde            # Add dependencies
cargo rm old-dep           # Remove dependencies
```

---

### setup-cocogitto

Installs Cocogitto (`cog`) for version management and changelog
generation, with caching.

**Usage:**

```yaml
- name: Setup Cocogitto
  uses: ./.github/actions/setup-cocogitto
```

**Features:**

- Uses cargo-binstall for fast installation
- Caches the binary between runs
- Pins to version 6.5.0 for consistency
- Cross-platform support
- Automatically installs cargo-edit dependency

**Dependencies:**

- `setup-cargo-binstall` (called automatically)
- `setup-cargo-edit` (called automatically, required for version
  bumping)

**After this step:**

```bash
cog --version           # Check cocogitto version
cog bump --patch        # Bump patch version (uses cargo set-version)
cog changelog           # Generate changelog
```

**Important:** Cocogitto requires `cargo set-version` (from
cargo-edit) to be available for version bumping. This action
automatically installs both tools.

---

### generate-changelog

Generates a changelog from conventional commits using Cocogitto.

**Usage:**

```yaml
- name: Generate changelog
  uses: ./.github/actions/generate-changelog
  with:
    release-tag: v0.1.3
    output-file: CHANGELOG.md # optional, defaults to CHANGELOG.md
```

**Inputs:**

- `release-tag` (required): Release tag (e.g., `v0.1.3`)
- `output-file` (optional): File to write changelog to (default:
  `CHANGELOG.md`)

**Features:**

- Automatically handles tags that don't exist yet
- Falls back to generating changelog from previous tag to HEAD
- Smart handling of first release (no previous tag)

**Dependencies:**

- `setup-cocogitto` (called automatically, which installs cargo-edit
  too)
- `.bash/gha-generate-changelog.sh` (bash script)

**Output:**

Creates a markdown file with the changelog for the specified release.

---

## Implementation Details

### Caching Strategy

All actions use GitHub Actions cache to speed up builds:

- **cargo-binstall**: Cached by OS and architecture
- **cargo-edit**: Cached by version, OS, and architecture
- **cocogitto**: Cached by version, OS, and architecture

Cache keys include platform and architecture to ensure cross-platform
builds work correctly.

### Dependency Chain

Actions have the following dependency chain:

```
setup-cocogitto
    ├── setup-cargo-binstall
    └── setup-cargo-edit
            └── setup-cargo-binstall
```

This ensures all required tools are available with proper caching.

### PATH Management

All actions automatically add `~/.cargo/bin` to PATH, making installed
tools immediately available in subsequent steps.

### Cross-Platform Support

All actions work on:

- Linux (ubuntu-latest, ubuntu-24.04, etc.)
- macOS (macos-latest, macos-15-xlarge, etc.)
- Windows (windows-latest)

Windows binaries are handled with `.exe` extension automatically.

---

## Example Workflow

Here's a complete example using all actions:

```yaml
name: Release

on:
  push:
    branches: [main]

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Configure Git
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"

      - name: Setup Cocogitto
        uses: ./.github/actions/setup-cocogitto
        # This automatically installs cargo-binstall and cargo-edit

      - name: Bump version
        run: cog bump --patch
        # This uses cargo set-version internally via pre_bump_hooks

      - name: Get new version
        id: version
        run: |
          VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
          echo "version=$VERSION" >> $GITHUB_OUTPUT

      - name: Generate changelog
        uses: ./.github/actions/generate-changelog
        with:
          release-tag: v${{ steps.version.outputs.version }}
          output-file: CHANGELOG.md

      - name: Create GitHub Release
        run: |
          gh release create "v${{ steps.version.outputs.version }}" \
            --title "v${{ steps.version.outputs.version }}" \
            --notes-file CHANGELOG.md
        env:
          GH_TOKEN: ${{ github.token }}
```

---

## Maintenance

### Updating cargo-edit Version

To update the cargo-edit version, edit
`.github/actions/setup-cargo-edit/action.yml` and change the version
in the cache key and install command:

```yaml
key: cargo-edit-0.13.0-${{ runner.os }}-${{ runner.arch }}
# and
cargo binstall --no-confirm --force --version 0.13.0 cargo-edit
```

### Updating Cocogitto Version

To update the Cocogitto version, edit
`.github/actions/setup-cocogitto/action.yml` and change the version in
the cache key and install command:

```yaml
key: cocogitto-6.5.0-${{ runner.os }}-${{ runner.arch }}
# and
cargo binstall --no-confirm --force --version 6.5.0 cocogitto
```

### Testing Locally

The bash scripts in `.bash/` can be tested locally:

```bash
# Test changelog generation
./.bash/gha-generate-changelog.sh CHANGELOG.md v0.1.3
```

---

## Origin

These actions are shared across multiple repositories:

- cargo-fmt-toml
- cargo-version-info
- dotenvage
- cargo-propagate-features
- cargo-nightly

Updates should be synchronized across repositories when possible.

---

## Further Reading

- [cargo-edit Documentation](https://github.com/killercup/cargo-edit)
- [Cocogitto Documentation](https://docs.cocogitto.io/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [GitHub Actions Composite Actions](https://docs.github.com/actions/creating-actions/creating-a-composite-action)
