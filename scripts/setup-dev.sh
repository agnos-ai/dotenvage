#!/bin/bash
#
# Development environment setup script
# Run this after cloning the repository
#

set -e

echo "🔧 Setting up dotenvage development environment..."
echo ""

# Check for required tools
echo "Checking required tools..."

# Check for Rust
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install from https://rustup.rs"
    exit 1
fi
echo "✓ Rust found: $(rustc --version)"

# Check for nightly toolchain (needed for rustfmt)
if ! rustup toolchain list | grep -q nightly; then
    echo "❌ Rust nightly toolchain not found"
    echo "Installing nightly toolchain..."
    rustup toolchain install nightly
fi
echo "✓ Nightly toolchain available"

# Check for rustfmt (nightly)
if ! command -v rustfmt &> /dev/null; then
    echo "Installing rustfmt..."
    rustup component add rustfmt
fi
echo "✓ rustfmt available"

# Check for clippy
if ! command -v cargo-clippy &> /dev/null; then
    echo "Installing clippy..."
    rustup component add clippy
fi
echo "✓ clippy available"

echo ""
echo "Installing git hooks..."

# Install cargo-husky hooks
if [ -d ".cargo-husky/hooks" ]; then
    for hook in .cargo-husky/hooks/*; do
        hook_name=$(basename "$hook")
        target=".git/hooks/$hook_name"
        
        echo "Installing $hook_name hook..."
        cp "$hook" "$target"
        chmod +x "$target"
        echo "✓ $hook_name hook installed"
    done
else
    echo "⚠️  .cargo-husky/hooks directory not found"
fi

echo ""
echo "Building project..."
cargo build

echo ""
echo "Running tests..."
cargo test

echo ""
echo "✅ Development environment setup complete!"
echo ""
echo "Git hooks installed:"
echo "  - pre-commit: Runs formatting and clippy checks"
echo ""
echo "Next steps:"
echo "  cargo build          # Build the project"
echo "  cargo test           # Run tests"
echo "  cargo run -- --help  # Run the CLI"
echo ""

