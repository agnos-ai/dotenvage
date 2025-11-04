# Maintainer Guide

This document contains information for repository maintainers.

## GitHub Repository Settings

### Enforce Commit Signing

To ensure all commits are signed, configure branch protection rules:

1. Go to repository **Settings** → **Branches**
2. Edit (or create) branch protection rule for `main`
3. Enable: **"Require signed commits"**
4. Save changes

**To verify current settings:**

```bash
# Using GitHub CLI
gh api repos/agnos-ai/dotenvage/branches/main/protection

# Or check in the web UI:
# https://github.com/agnos-ai/dotenvage/settings/branch_protection_rules
```

### Other Recommended Branch Protection Rules

For the `main` branch, also enable:

- [ ] **Require a pull request before merging**
  - Require approvals: 1
  - Dismiss stale pull request approvals when new commits are pushed
- [ ] **Require status checks to pass before merging**
  - Require branches to be up to date before merging
  - Status checks: `fmt`, `clippy`, `build`, `test`
- [ ] **Require conversation resolution before merging**
- [ ] **Require signed commits** ✓
- [ ] **Require linear history**
- [ ] **Do not allow bypassing the above settings**

### Secrets Configuration

Ensure these secrets are configured:

- `CRATES_IO_TOKEN`: For automated publishing to crates.io
  - Generate at: https://crates.io/me
  - Set at: Repository Settings → Secrets → Actions

## Release Process

Releases are automated via GitHub Actions when the version in `Cargo.toml` changes:

1. Update version in `Cargo.toml`
2. Commit and push to `main`
3. CI will automatically:
   - Create git tag
   - Generate changelog
   - Create GitHub release
   - Publish to crates.io

## Security

- All maintainer commits must be signed
- Review all PRs for security implications
- Never commit secrets or keys
- Use age encryption for sensitive values in .env files
