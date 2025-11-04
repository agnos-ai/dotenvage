# Contributing to dotenvage

Thank you for your interest in contributing to dotenvage! This document provides guidelines and instructions for setting up your development environment.

## Prerequisites

- [Rust](https://rustup.rs) (stable and nightly toolchains)
- Git
- Basic familiarity with Rust and cryptography concepts

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

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

Examples:

```text
feat: add --json output format to list command
fix: handle empty .env files correctly
docs: update README with new examples
refactor: extract helper function to reduce nesting
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
4. Update CHANGELOG.md (done automatically via conventional commits)

## Pull Request Process

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes
4. Ensure all tests pass and hooks succeed
5. Push to your fork
6. Submit a pull request

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
