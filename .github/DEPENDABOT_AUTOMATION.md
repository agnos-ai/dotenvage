# Dependabot Automation Setup

This repository uses automated dependency management with Dependabot,
including automatic merging and publishing.

## How It Works

### 1. Weekly Dependency Updates (Mondays 2 AM UTC)

Dependabot checks for dependency updates every Monday at 2 AM UTC and
creates a **single grouped pull request** containing all patch and
minor updates.

**Configuration:** `.github/dependabot.yml`

### 2. Automatic Approval & Merge

When Dependabot creates a PR:

1. The `dependabot-auto-merge.yml` workflow triggers
2. For **patch** and **minor** updates:
   - PR is automatically approved
   - Auto-merge is enabled (merges after CI passes)
3. For **major** updates:
   - A comment is added requesting manual review
   - PR is NOT auto-merged

**Configuration:** `.github/workflows/dependabot-auto-merge.yml`

### 3. Automatic Version Bump & Publish

When dependency updates are merged to `main`:

1. The `auto-version-bump.yml` workflow triggers
2. Detects if the commit is a dependency update
3. Automatically bumps the **patch version** in `Cargo.toml`
4. Creates a git tag (e.g., `v0.1.3`)
5. Generates a GitHub Release with changelog
6. Publishes the new version to crates.io

**Configuration:** `.github/workflows/auto-version-bump.yml`

## Setup Requirements

### 1. Enable Auto-Merge in Repository Settings

Navigate to your repository settings and enable auto-merge:

```text
Settings → General → Pull Requests
☑ Allow auto-merge
```

### 2. Configure GitHub Actions Permissions

Allow GitHub Actions to create and approve PRs:

```text
Settings → Actions → General → Workflow permissions
● Read and write permissions
☑ Allow GitHub Actions to create and approve pull requests
```

### 3. crates.io Token Configuration

✅ **Already Configured**: This repository already has
`CRATES_IO_TOKEN` secret configured.

The auto-publishing workflow uses this existing token. No additional
configuration needed.

**Note:** Dependabot itself doesn't need a separate token - it uses
the `GITHUB_TOKEN` to create PRs. Only the publishing workflow needs
the crates.io token.

**Security Note:** GitHub now supports
[Trusted Publishing](https://crates.io/docs/trusted-publishing) for
enhanced security (available for crates.io since July 2025), which is
recommended for future implementations.

### 4. Set Up Branch Protection Rules (Optional but Recommended)

To ensure quality:

```text
Settings → Branches → Add rule for 'main'
☑ Require status checks to pass before merging
  Required checks:
  - Format
  - Clippy
  - Test (ubuntu-latest, stable)
  - Test (macos-latest, stable)
  - Test (windows-latest, stable)
```

This ensures that auto-merged PRs only merge after all tests pass.

## Manual Overrides

### Disable Auto-Merge for a Specific PR

If you want to review a specific Dependabot PR manually:

```bash
gh pr ready --undo <PR-NUMBER>
```

### Manually Trigger Version Bump

The version bump workflow only triggers on dependency updates. For
other changes, manually bump the version:

```bash
cargo set-version --bump patch
git add Cargo.toml Cargo.lock
git commit -m "chore(release): bump version to $(grep '^version' Cargo.toml | head -1 | sed 's/.*\"\(.*\)\".*/\1/')"
git push
```

## Monitoring

### View Dependabot Activity

- **Dashboard:** `Insights → Dependency graph → Dependabot`
- **PRs:** Filter by label: `dependencies` or `automated`

### Check Automation Status

```bash
# View recent workflow runs
gh run list --workflow=dependabot-auto-merge.yml --limit 5

# View recent releases
gh release list --limit 5

# Check latest crates.io version
cargo search cargo-fmt-toml --limit 1
```

## Customization

### Change Update Schedule

Edit `.github/dependabot.yml`:

```yaml
schedule:
  interval: "daily" # Options: daily, weekly, monthly
  day: "monday" # For weekly: monday-sunday
  time: "02:00" # Time in UTC
```

### Exclude Specific Dependencies

Add to `.github/dependabot.yml`:

```yaml
ignore:
  - dependency-name: "some-crate"
    versions: [">=2.0.0"]
```

### Disable Auto-Publishing

Comment out or remove the "Publish to crates.io" step in
`.github/workflows/auto-version-bump.yml`.

## Troubleshooting

### PR Not Auto-Merging

1. Check that auto-merge is enabled in repository settings
2. Verify all required CI checks are passing
3. Check workflow permissions in Actions settings
4. Review logs: `gh run view <RUN-ID>`

### Version Not Publishing to crates.io

1. Verify `CARGO_REGISTRY_TOKEN` secret is set correctly
2. Check that you have ownership/publishing rights for the crate
3. Review publish workflow logs for errors
4. Ensure `Cargo.toml` has all required fields (license, description,
   etc.)

### Too Many Dependabot PRs

This shouldn't happen with grouping enabled, but if it does:

1. Check that `groups` configuration is present in `dependabot.yml`
2. Reduce `open-pull-requests-limit`
3. Consider changing to monthly schedule

## Security Considerations

- ✅ Auto-merge only applies to **patch** and **minor** updates
- ✅ **Major** updates require manual review
- ✅ All updates must pass CI checks before merging
- ✅ Commit messages follow conventional commits standard
- ✅ GitHub Actions runs with minimal necessary permissions
- ⚠️ Consider enabling branch protection for added security

## Further Reading

- [Dependabot Documentation](https://docs.github.com/code-security/dependabot)
- [Automating Dependabot with GitHub Actions](https://docs.github.com/code-security/supply-chain-security/keeping-your-dependencies-updated-automatically/automating-dependabot-with-github-actions)
- [crates.io Trusted Publishing](https://crates.io/docs/trusted-publishing)
- [Semantic Versioning](https://semver.org/)
