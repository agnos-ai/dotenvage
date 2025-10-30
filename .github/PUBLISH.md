# Publishing Guide (Cocogitto-based)

This project uses **Cocogitto** for automated changelog
generation and a **version-change detection** workflow for
publishing.

## How It Works

The CI workflow automatically detects when the version in
`Cargo.toml` changes and:

1. ✅ Runs all checks (format, clippy, build, test)
2. 📝 Generates changelog from conventional commits using
   Cocogitto
3. 📌 Creates and pushes a git tag (e.g., `v0.0.2`)
4. 🎉 Creates a GitHub Release with generated changelog
5. 📦 Publishes to crates.io

**No manual tagging required!** Just bump the version and
push.

## Setup (One-time)

### 1. Add crates.io Token to GitHub Secrets

1. Get your crates.io API token:

   ```bash
   # Visit https://crates.io/settings/tokens
   # Create a new token with "publish-new" and
   # "publish-update" scopes
   ```

2. Add to GitHub repository secrets:
   - Go to:
     https://github.com/agnos-ai/dotenvage/settings/secrets/actions
   - Click "New repository secret"
   - Name: `CRATES_IO_TOKEN`
   - Value: `<your-token-here>`

### 2. Install Cocogitto Locally (Optional but Recommended)

```bash
cargo install cocogitto

# Validate your commits
cog check

# Verify changelog generation
cog changelog --at v0.0.1
```

## Publishing a New Version

### Step-by-Step Process

```bash
# 1. Ensure you're on main and up to date
git checkout main
git pull origin main

# 2. Update version in Cargo.toml
vim Cargo.toml  # Change version = "0.0.1" to "0.0.2"

# 3. Commit the version bump (using conventional
#    commit format)
git add Cargo.toml
git commit -m "chore: bump version to 0.0.2"

# 4. Push to main
git push origin main

# 5. Watch the magic happen! ✨
# GitHub Actions will automatically:
#   - Run all checks
#   - Generate changelog from commits
#   - Create git tag v0.0.2
#   - Create GitHub Release
#   - Publish to crates.io
```

### What Gets Published

The changelog will include all commits since the last
version with types:
- ✅ **feat**: New features
- ✅ **fix**: Bug fixes
- ✅ **docs**: Documentation changes
- ✅ **refactor**: Code refactoring
- ✅ **perf**: Performance improvements
- ✅ **build**: Build system changes
- ✅ **revert**: Reverted commits

Excluded from changelog:
- ❌ **style**: Code style changes
- ❌ **test**: Test updates
- ❌ **ci**: CI/CD changes
- ❌ **chore**: Maintenance tasks

## Conventional Commits

All commits should follow the
[Conventional Commits](https://www.conventionalcommits.org/)
format:

```bash
<type>(<scope>): <subject>
```

### Examples

```bash
# Features
feat(loader): add Docker architecture detection
feat: support VERCEL_ENV variable

# Bug Fixes
fix(manager): correct Windows path handling
fix: handle empty environment values

# Documentation
docs: update README with new examples
docs(api): improve rustdoc comments

# Refactoring
refactor(loader): simplify env resolution
refactor: extract common logic

# Chores (won't appear in changelog)
chore: bump version to 0.0.2
chore: update dependencies
test: add integration tests
ci: improve workflow caching
```

### Breaking Changes

Use `!` after type and include `BREAKING CHANGE:` footer:

```bash
feat!: change default key location

BREAKING CHANGE: Default key path changed from
~/.config to ~/.local/state
```

## Monitoring the Release

After pushing your version bump:

1. Go to:
   https://github.com/agnos-ai/dotenvage/actions
2. Watch the "CI/CD" workflow
3. The "Release" job will show:
   - Changelog generation
   - Tag creation
   - GitHub Release creation
   - crates.io publication

## Verification

After the workflow completes:

```bash
# Check the new release
open https://github.com/agnos-ai/dotenvage/releases

# Check crates.io
open https://crates.io/crates/dotenvage

# Check documentation
open https://docs.rs/dotenvage

# Pull the new tag locally
git pull --tags
```

## Version Bump Types

- **Patch** (0.0.1 → 0.0.2): Bug fixes, minor improvements
- **Minor** (0.0.1 → 0.1.0): New features, backwards
  compatible
- **Major** (0.0.1 → 1.0.0): Breaking changes

## Troubleshooting

### "Version hasn't changed"

The workflow only runs when the version in `Cargo.toml`
differs from the latest git tag.

**Solution**: Make sure you actually bumped the version
number.

### "Changelog is empty"

If no conventional commits exist since the last tag, the
changelog will be minimal.

**Solution**: Use conventional commit format for meaningful
commits.

### "Tag already exists"

You can't publish the same version twice.

**Solution**: Bump to a new version number.

### "Authentication failed"

The crates.io token is missing or invalid.

**Solution**: Check that `CRATES_IO_TOKEN` secret is set
correctly in GitHub settings.

### "CI checks failed"

The release won't happen if any check fails.

**Solution**: Fix the failing checks and push again.

## First Release Checklist

- [x] Version in `Cargo.toml` is `0.0.1`
- [x] `LICENSE` file exists
- [x] `README.md` is complete
- [x] All tests pass locally (`cargo test`)
- [x] Code is formatted (`cargo fmt`)
- [x] Clippy passes (`cargo clippy`)
- [ ] `CRATES_IO_TOKEN` added to GitHub secrets
- [ ] Commits follow conventional format
- [ ] Ready to push and release!

## Example Workflow

```bash
# Day 1: Work on features
git checkout -b feat/docker-support
# ... make changes ...
git commit -m "feat(loader): add Docker TARGETARCH detection"
git commit -m "docs: update README with Docker examples"
git push origin feat/docker-support
# Create PR, get review, merge to main

# Day 2: Work on bug fixes
git checkout -b fix/windows-paths
# ... fix bugs ...
git commit -m "fix(manager): handle Windows path separators"
git push origin fix/windows-paths
# Create PR, get review, merge to main

# Day 3: Ready to release!
git checkout main
git pull
vim Cargo.toml  # 0.0.1 → 0.0.2
git commit -m "chore: bump version to 0.0.2"
git push

# ✨ Automatic release happens!
# Generated changelog will include:
# - feat(loader): add Docker TARGETARCH detection
# - docs: update README with Docker examples
# - fix(manager): handle Windows path separators
```

## Resources

- [Conventional Commits Specification](https://www.conventionalcommits.org/)
- [Cocogitto Documentation](https://github.com/cocogitto/cocogitto)
- [Contributing Guide](CONTRIBUTING.md)
