# Dependabot Automation Setup Checklist

Complete these steps to enable fully automated dependency management.

## ✅ Pre-flight Checklist

### 1. Repository Settings

- [x] **Auto-Merge Enabled** ✅

  - Configured via: `gh repo edit --enable-auto-merge=true`
  - Status: Active

- [x] **GitHub Actions Permissions Configured** ✅
  - Configured via: `gh api` with write permissions
  - Can approve pull requests: Enabled
  - Status: Active

### 2. Secrets Configuration

- [x] **crates.io Token Already Configured**
  - ✅ Secret `CRATES_IO_TOKEN` already exists
  - This will be used for automatic publishing
  - No additional token needed for Dependabot

### 3. Branch Protection (Recommended)

- [ ] **Set Up Main Branch Protection**
  - Go to: `Settings → Branches → Add rule`
  - Branch name pattern: `main`
  - Check: ☑ Require status checks to pass before merging
  - Required checks:
    - [ ] Format
    - [ ] Clippy
    - [ ] Test (ubuntu-latest, stable)
    - [ ] Test (macos-latest, stable)
    - [ ] Test (windows-latest, stable)
  - Check: ☑ Require branches to be up to date before merging
  - Click: Create

### 4. Verify Files

Ensure these files exist and are committed:

- [ ] `.github/dependabot.yml` (updated with grouping)
- [ ] `.github/workflows/dependabot-auto-merge.yml` (new)
- [ ] `.github/workflows/auto-version-bump.yml` (new)
- [ ] `.github/workflows/ci.yml` (existing)

## 🧪 Testing the Setup

### Test 1: Manual Dependabot Trigger

```bash
# Manually trigger Dependabot (requires GitHub CLI and permissions)
gh api repos/{owner}/{repo}/dependabot/version-updates \
  --method POST \
  --field package_ecosystem=cargo
```

Or wait until Monday 2 AM UTC for the scheduled run.

### Test 2: Check Workflow Runs

```bash
# List recent workflow runs
gh run list --limit 10

# Watch a specific run
gh run watch <RUN-ID>
```

### Test 3: Verify PR Creation

After Dependabot runs:

```bash
# List open PRs
gh pr list

# Check PR details
gh pr view <PR-NUMBER>

# Check CI status
gh pr checks <PR-NUMBER>
```

### Test 4: Monitor Auto-Merge

After CI passes on a Dependabot PR:

1. PR should be automatically approved
2. Auto-merge should be enabled
3. PR should merge automatically
4. Version bump workflow should trigger
5. New release should be created
6. Package should be published to crates.io

## 🔍 Verification Commands

### Check Current Setup

```bash
# Clone and navigate to repo
cd /path/to/cargo-fmt-toml

# Check Dependabot config
cat .github/dependabot.yml

# List workflows
ls -la .github/workflows/

# Check current version
grep '^version' Cargo.toml

# Check latest crates.io version
cargo search cargo-fmt-toml --limit 1

# View recent releases
gh release list --limit 5
```

### Monitor Automation

```bash
# Watch for Dependabot PRs
gh pr list --label dependencies --json number,title,state

# Check workflow status
gh run list --workflow=dependabot-auto-merge.yml
gh run list --workflow=auto-version-bump.yml

# View latest logs
gh run view --log
```

## 🚨 Common Issues

### Issue: PR Created but Not Auto-Approved

**Solution:**

1. Check workflow permissions: `Settings → Actions → General`
2. Ensure "Allow GitHub Actions to create and approve pull requests"
   is enabled
3. Check workflow logs:
   `gh run list --workflow=dependabot-auto-merge.yml`

### Issue: PR Not Auto-Merging

**Solution:**

1. Verify auto-merge is enabled: `Settings → General → Pull Requests`
2. Check CI status: All required checks must pass
3. Branch protection may require specific checks

### Issue: Version Not Bumping

**Solution:**

1. Check commit message starts with `build(deps):`
2. Verify workflow triggers: Should run on push to main with Cargo.\*
   changes
3. Check logs: `gh run list --workflow=auto-version-bump.yml`

### Issue: Publish to crates.io Fails

**Solution:**

1. Verify `CARGO_REGISTRY_TOKEN` secret exists and is valid
2. Check you have publish permissions for the crate
3. Ensure Cargo.toml has required metadata (license, description,
   etc.)
4. Check publish logs in workflow run

## 📊 Success Metrics

After setup, you should see:

- ✅ Weekly Dependabot PRs (grouped, every Monday)
- ✅ PRs automatically approved and merged (after CI passes)
- ✅ Patch version bumped automatically
- ✅ New releases created on GitHub
- ✅ New versions published to crates.io
- ✅ Dependencies always up-to-date

## 🎯 Next Steps

Once setup is complete:

1. Wait for next scheduled Dependabot run (Monday 2 AM UTC)
2. Or manually trigger: Close and reopen an existing Dependabot PR
3. Monitor first automated cycle
4. Adjust configuration as needed

## 📝 Notes

- First run may take time for Dependabot to analyze dependencies
- Auto-merge only works for **patch** and **minor** updates
- **Major** updates will need manual review
- You can always disable auto-merge for specific PRs

## ✅ Setup Complete!

Once all items are checked, your automated dependency management is
ready! 🎉
