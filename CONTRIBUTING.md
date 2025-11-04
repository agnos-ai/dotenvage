# Contributing to dotenvage

Thank you for your interest in contributing to dotenvage! This document provides guidelines and instructions for setting up your development environment.

## Security-First Project

⚠️ **Important**: dotenvage is a **security and cryptography tool** that handles sensitive data (encrypted secrets, API keys, passwords). This requires higher standards than typical projects:

- **All commits must be signed** - No exceptions for authenticity
- **Zero tolerance for warnings** - Clippy must pass with `-D warnings`
- **Detailed commit history** - For audit trails and AI agent analysis
- **Linear history only** - No merge commits, always rebase
- **Comprehensive testing** - Security-critical code needs thorough tests
- **Code review required** - All changes must be reviewed before merge

These strict requirements protect users who trust us with their secrets.

## Prerequisites

- [Rust](https://rustup.rs) (stable and nightly toolchains)
- Git with commit signing configured (GPG or SSH)
- Basic familiarity with Rust and cryptography concepts
- Understanding of secure coding practices

## Development Setup

### 1. Clone the Repository

```bash
git clone https://github.com/agnos-ai/dotenvage.git
cd dotenvage
```

### 2. Run the Setup Script

**⚠️ IMPORTANT**: Run the setup script to install git hooks and verify your development environment:

```bash
./scripts/setup-dev.sh
```

This script will:

- Check for required tools (Rust, nightly toolchain, rustfmt, clippy)
- Install git pre-commit hooks from `.cargo-husky/hooks/`
- Build the project
- Run tests to verify everything works

### 3. Manual Hook Installation (if needed)

If the setup script doesn't work, manually install the git hooks:

```bash
cp .cargo-husky/hooks/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

## Git Hooks

The pre-commit hook enforces code quality by running:

1. **Formatting check**: `cargo +nightly fmt --check`

   - Uses nightly for advanced rustfmt features
   - If this fails, run `cargo +nightly fmt` to fix

2. **Clippy lints**: `cargo clippy --all-targets --all-features -- -D warnings`
   - Treats all warnings as errors
   - Enforces documentation on public items
   - Checks for common mistakes and code smells

**The hook will block commits that don't pass these checks.**

## Development Workflow

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Documentation

```bash
# Build and open docs
cargo doc --no-deps --open

# Test documentation examples
cargo test --doc
```

### Linting and Formatting

```bash
# Check formatting (what the hook runs)
cargo +nightly fmt --check

# Fix formatting
cargo +nightly fmt

# Run clippy (what the hook runs)
cargo clippy --all-targets --all-features -- -D warnings

# Run clippy with auto-fixes
cargo clippy --fix --all-targets --all-features
```

## Code Style

- Follow Rust standard style guidelines
- Use `cargo +nightly fmt` for formatting
- All public items must have documentation
- Include `# Errors`, `# Panics`, and `# Safety` sections where applicable
- Write tests for new functionality
- Update README and examples when adding features

## Commit Guidelines

### Commit Signing

**All commits must be signed.** This ensures authenticity and integrity of the code.

To set up commit signing:

```bash
# Configure git to sign commits with GPG
git config --global user.signingkey YOUR_GPG_KEY_ID
git config --global commit.gpgsign true

# Or use SSH signing (GitHub supports this)
git config --global gpg.format ssh
git config --global user.signingkey ~/.ssh/id_ed25519.pub
```

See GitHub's guide: [Signing commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits)

### Conventional Commits

We follow [Conventional Commits](https://www.conventionalcommits.org/) with **detailed multi-line commit messages**.

Commit types:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

### Detailed Commit Messages

**Use multiple `-m` flags** to create detailed commit messages with bullet points:

```bash
git commit \
  -m "feat: add --json output format to list command" \
  -m "- Add --json flag to list command arguments" \
  -m "- Serialize variable data as JSON with encryption status" \
  -m "- Include decrypted values when --verbose is combined with --json" \
  -m "- Add serde_json dependency to Cargo.toml" \
  -m "- Update README with JSON output examples"
```

**Why detailed commits matter:**

1. **Better changelogs**: CI automatically generates release notes from commits
2. **AI agent context**: Subsequent AI agents can understand changes by reading `git log`
3. **Code archaeology**: Makes it easier to understand why changes were made
4. **Review clarity**: Reviewers can see exactly what changed and why

The **first `-m`** is the conventional commit title (max 72 chars).  
**Additional `-m` flags** are bullet points explaining what was done.

Examples:

```bash
# Good: Detailed commit
git commit \
  -m "refactor: extract helper function to reduce nesting" \
  -m "- Extract print_list_entry() function from list command" \
  -m "- Use pattern matching on (verbose, plain) tuple" \
  -m "- Reduces nesting from 6 to 4 levels" \
  -m "- Fixes clippy::excessive_nesting warnings"

# Also good: Simple change needs less detail
git commit -m "docs: fix typo in README"

# Bad: Vague, no detail
git commit -m "fix stuff"
```

## Testing

- Write unit tests for new functions
- Write integration tests for CLI commands
- Ensure all tests pass before submitting PR
- Test both encrypted and plaintext scenarios

## Documentation

When adding new public APIs:

1. Add rustdoc comments with examples
2. Include in `README.md` if it's a major feature
3. Add usage examples in `examples/` directory if helpful

### Changelog

**Never commit CHANGELOG.md** - it's generated automatically:

- ✅ **Generated by cocogitto**: Uses conventional commits
- ✅ **In .gitignore**: CHANGELOG.md is not tracked in git
- ✅ **Auto-generated in CI**: Created during release process
- ✅ **Shows all versions**: Full changelog history

To preview the changelog locally:

```bash
cog changelog > CHANGELOG.md
cat CHANGELOG.md
# Don't commit it!
```

## Pull Request Process

### Branching and History

We maintain a **strictly linear commit history** on `main`:

- ✅ **No merge commits** - PRs are rebased, not merged
- ✅ **Each commit should be atomic and well-documented**
- ✅ **All commits must pass tests and checks individually**

### Workflow

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes with detailed commits
4. Keep your branch up to date by rebasing on `main`:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```
5. Ensure all tests pass and hooks succeed
6. Push to your fork (use `--force-with-lease` after rebasing)
7. Submit a pull request

### Merge Strategy

PRs will be merged using **rebase and merge** (not squash, not merge commit):

- Your individual commits will be preserved in `main`
- This maintains detailed history for changelog generation and AI agent context
- Make sure each commit is clean and well-documented before submitting

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] All tests pass (`cargo test`)
- [ ] Clippy is clean (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo +nightly fmt`)
- [ ] Documentation is updated
- [ ] Commit messages follow conventional commits
- [ ] **All commits are signed** (GPG or SSH)
- [ ] Git hooks are passing

## Getting Help

- Check existing issues and PRs
- Ask questions in issue comments
- Review the documentation: https://docs.rs/dotenvage

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
