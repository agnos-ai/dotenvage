---
name: version-bump
description:
  Bump version in Cargo.toml using cargo-version-info bump command
---

# Version Bump Skill

Use this skill when bumping the version.

## Important: Use cargo version-info bump

**Always use `cargo version-info bump`** for version management.

**Never use `cog bump`** - it creates local tags which conflict with
the CI workflow that creates tags after tests pass.

## Bump Commands

```bash
# Patch bump: 0.2.4 -> 0.2.5
cargo version-info bump --patch

# Minor bump: 0.2.4 -> 0.3.0
cargo version-info bump --minor

# Major bump: 0.2.4 -> 1.0.0
cargo version-info bump --major
```

## What the Bump Command Does

1. Runs `pre_bump_hooks` defined in `Cargo.toml`:
   - Executes `./scripts/sync-npm-version.sh {{version}}`
   - Updates `[workspace.package]` version in Cargo.toml
   - Updates `npm/package.json` version
   - Updates `npm/dotenvage-napi/Cargo.toml` dependency version
   - Updates `package.json` version
2. Bumps the version in the main `[package]` section
3. Updates `Cargo.lock`
4. Commits all changed files listed in `additional_files`
5. Creates a git commit with message:
   `chore(version): bump X.Y.Z -> A.B.C`

The bump command uses hunk-level selective staging, so it only commits
version-related changes. Any other uncommitted work remains unstaged.

## Files Updated

- `Cargo.toml` - workspace and package versions
- `Cargo.lock` - lockfile
- `npm/package.json` - npm package version
- `npm/dotenvage-napi/Cargo.toml` - NAPI binding dependency
- `package.json` - root workspace version

## What the Bump Command Does NOT Do

- Does NOT create git tags (CI creates tags after merge)
- Does NOT push to remote (you must push manually)

## Workflow

1. Run `cargo version-info bump --patch` (or --minor/--major)
2. Push the branch or create a PR
3. Merge to main
4. CI detects version change, creates tag, publishes release

## Checking Current Version

```bash
# Get version from Cargo.toml
cargo version-info current

# Get computed build version (includes git SHA in dev)
cargo version-info build-version

# Check if version changed since last tag
cargo version-info changed
```

## Dry Run

To see what would change without making changes:

```bash
# Check current version
cargo version-info current

# Calculate what next patch would be
cargo version-info next
```

## After Bumping

After running bump, verify the commit includes all version-related files:

```bash
git log -1 --oneline
git diff HEAD~1 --stat
git status
```

**Important**: Check that all files modified by pre-bump hooks are
included in the commit. If `git status` shows uncommitted version
changes (from hooks), amend the commit:

```bash
git add <missing-files>
git commit --amend --no-edit
```

Then add those files to `additional_files` in Cargo.toml to prevent
this in future bumps.

Then push when ready:

```bash
git push origin <branch>
```

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

## Troubleshooting

If the version bump fails or files are missing from the commit:

1. Check that `additional_files` in `Cargo.toml` includes all npm
   files
2. Verify pre_bump_hooks ran successfully (look for the sync
   messages)
3. If needed, manually stage files and amend the commit
