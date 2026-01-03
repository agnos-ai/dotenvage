# dotenvage

[![Crates.io](https://img.shields.io/crates/v/dotenvage.svg)](https://crates.io/crates/dotenvage)
[![Documentation](https://docs.rs/dotenvage/badge.svg)](https://docs.rs/dotenvage)
[![CI](https://github.com/agnos-ai/dotenvage/workflows/CI%2FCD/badge.svg)](https://github.com/agnos-ai/dotenvage/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/agnos-ai/dotenvage/blob/main/LICENSE)
[![Downloads](https://img.shields.io/crates/d/dotenvage.svg)](https://crates.io/crates/dotenvage)

Dotenv with age encryption: encrypt/decrypt secrets in `.env` files.

**The key advantage**: With encrypted secrets, you can safely **commit
all your `.env*` files to version control** - including production
configs, user-specific settings, and files with sensitive data. No
more `.gitignore` juggling or secret management headaches.

- Selective encryption of sensitive keys
- Uses age (X25519) for modern encryption
- Library + CLI
- CI-friendly (supports key via env var)
- Automatic file layering with precedence rules

## Installation

### Using cargo-binstall (Recommended)

The fastest way to install pre-built binaries:

```bash
cargo install cargo-binstall
cargo binstall dotenvage
```

### Using cargo install

Build from source (slower, requires Rust toolchain):

```bash
cargo install dotenvage
```

### Manual Installation

Download pre-built binaries from
[GitHub Releases](https://github.com/agnos-ai/dotenvage/releases):

- Linux (x86_64): `dotenvage-x86_64-unknown-linux-gnu.zip`
- Linux (ARM64): `dotenvage-aarch64-unknown-linux-gnu.zip`
- macOS (Intel): `dotenvage-x86_64-apple-darwin.zip`
- macOS (Apple Silicon): `dotenvage-aarch64-apple-darwin.zip`
- Windows (x86_64): `dotenvage-x86_64-pc-windows-msvc.zip`
- Windows (ARM64): `dotenvage-aarch64-pc-windows-msvc.zip`

## Usage

```bash
# Generate a key
dotenvage keygen

# Encrypt sensitive values in .env.local
dotenvage encrypt .env.local

# Edit (decrypts in editor, re-encrypts on save)
dotenvage edit .env.local

# Set a value (auto-encrypts if key name matches patterns)
dotenvage set FLY_API_TOKEN=abc123 --file .env.local

# Get a decrypted value (searches .env then .env.local)
dotenvage get FLY_API_TOKEN

# List all variables from all .env* files (merged in standard order)
dotenvage list

# List with lock icons (🔒 = encrypted)
dotenvage list --verbose

# List in plain ASCII format (no icons, just variable names)
dotenvage list --plain

# List in JSON format
dotenvage list --json

# List in JSON with values
dotenvage list --json --verbose

# List from a specific file only
dotenvage list --file .env.local

# Dump all decrypted env vars (merges all .env* files with layering)
dotenvage dump

# Dump a specific file
dotenvage dump --file .env.local

# Dump with bash-compliant escaping (for values with $, `, etc.)
dotenvage dump --bash

# Dump in GNU Make format (VAR := value with Make-safe escaping)
dotenvage dump --make

# Dump with export prefix for bash sourcing (auto-enables --bash)
dotenvage dump --export

# Source in bash (loads all env vars into current shell)
eval "$(dotenvage dump --export)"
# or
source <(dotenvage dump --export)

# Use in Makefile (GNU Make) - secure, no temp file created
# $(eval $(shell dotenvage dump --make-eval))
# export
#
# In recipes, use: $$DATABASE_URL (shell env var)
# Not: $(DATABASE_URL) (Make variable expansion)
```

## Library

```rust,no_run
use dotenvage::{SecretManager, EnvLoader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load env files with auto-decryption
    EnvLoader::new()?.load()?;

    // Get all variable names (functional style)
    let vars = EnvLoader::new()?.get_all_variable_names()?.join(", ");

    // Encrypt and decrypt values
    let manager = SecretManager::generate()?;
    let enc = manager.encrypt_value("secret")?;
    let dec = manager.decrypt_value(&enc)?;
    Ok(())
}
```

## File Layering

One of dotenvage's key features is **automatic file layering** -
multiple `.env*` files are loaded and merged with a clear precedence
order. Later files override values from earlier files.

### Loading Order

Files are loaded using a **flexible power-set algorithm** that
generates all possible combinations of ENV, OS, ARCH, and USER. This
allows any combination you need without being constrained by a fixed
hierarchy.

**Key principle**: All multi-part file names use **dots as separators
only** (not dashes), ensuring unambiguous parsing.

Files are loaded in **specificity order** (later overrides earlier):

1. **`.env`** - Base configuration (always first)
2. **Single-part patterns**: `.env.<ENV>`, `.env.<OS>`, `.env.<ARCH>`,
   `.env.<USER>`
3. **Two-part combinations**: `.env.<ENV>.<OS>`, `.env.<ENV>.<ARCH>`,
   `.env.<ENV>.<USER>`, etc.
4. **Three-part combinations**: `.env.<ENV>.<OS>.<ARCH>`,
   `.env.<ENV>.<OS>.<USER>`, etc.
5. **Four-part combination**: `.env.<ENV>.<OS>.<ARCH>.<USER>` (most
   specific)
6. **`.env.pr-<NUMBER>`** - PR-specific (GitHub Actions only, always
   last)

**All files can be safely committed to git** since secrets are
encrypted.

#### Example Combinations

With `ENV=prod`, `OS=linux`, `ARCH=amd64`, `USER=alice`, these files
would be loaded (in order):

- `.env`
- `.env.prod`
- `.env.linux`
- `.env.amd64`
- `.env.alice`
- `.env.prod.linux`
- `.env.prod.amd64`
- `.env.prod.alice`
- `.env.linux.amd64`
- `.env.linux.alice`
- `.env.amd64.alice`
- `.env.prod.linux.amd64`
- `.env.prod.linux.alice`
- `.env.prod.amd64.alice`
- `.env.linux.amd64.alice`
- `.env.prod.linux.amd64.alice`

You only need to create the files you use - the loader checks which
exist.

### Placeholders

| Placeholder   | Environment Variables (priority order)                                                                                 | Default / Notes              |
| ------------- | ---------------------------------------------------------------------------------------------------------------------- | ---------------------------- |
| `<ENV>`       | `DOTENVAGE_ENV`, `EKG_ENV`, `VERCEL_ENV`, `NODE_ENV`                                                                   | Defaults to `local`          |
| `<OS>`        | `DOTENVAGE_OS`, `EKG_OS`, `CARGO_CFG_TARGET_OS`, `TARGET`, `RUNNER_OS`                                                 | Runtime detection if not set |
| `<ARCH>`      | `DOTENVAGE_ARCH`, `EKG_ARCH`, `CARGO_CFG_TARGET_ARCH`, `TARGET`, `TARGETARCH`, `TARGETPLATFORM`, `RUNNER_ARCH`         | None if not detected         |
| `<USER>`      | `DOTENVAGE_USER`, `EKG_USER`, `GITHUB_ACTOR`, `GITHUB_TRIGGERING_ACTOR`, `GITHUB_REPOSITORY_OWNER`, `USER`, `USERNAME` | System username              |
| `<PR_NUMBER>` | `PR_NUMBER`, `GITHUB_REF`                                                                                              | GitHub Actions only          |

### Supported Operating Systems

The `<OS>` placeholder supports these canonical values (with
normalization):

| Canonical | File Example        | Aliases (normalized to canonical) |
| --------- | ------------------- | --------------------------------- |
| `linux`   | `.env.prod.linux`   | -                                 |
| `macos`   | `.env.prod.macos`   | `darwin`, `osx`                   |
| `windows` | `.env.prod.windows` | `win32`, `win`                    |
| `freebsd` | `.env.prod.freebsd` | -                                 |
| `openbsd` | `.env.prod.openbsd` | -                                 |
| `netbsd`  | `.env.prod.netbsd`  | -                                 |
| `android` | `.env.prod.android` | -                                 |
| `ios`     | `.env.prod.ios`     | -                                 |

### Supported Architectures

The `<ARCH>` placeholder supports these canonical values (with
normalization):

| Canonical | File Example        | Aliases (normalized to canonical) |
| --------- | ------------------- | --------------------------------- |
| `amd64`   | `.env.prod.amd64`   | `x64`, `x86_64`                   |
| `arm64`   | `.env.prod.arm64`   | `aarch64`                         |
| `arm`     | `.env.prod.arm`     | `armv7`, `armv7l`, `armhf`        |
| `i386`    | `.env.prod.i386`    | `i686`, `x86`                     |
| `riscv64` | `.env.prod.riscv64` | `riscv64gc`                       |
| `ppc64le` | `.env.prod.ppc64le` | `powerpc64le`                     |
| `s390x`   | `.env.prod.s390x`   | -                                 |

**Note**: Custom architecture values (e.g., `docker-s3`) are passed
through as lowercase and can include dashes within the value itself
(e.g., `.env.prod.docker-s3`), but dots remain the separator between
file name parts.

### Example

Given these files:

```bash
# .env - Base config (safe to commit)
DATABASE_URL=postgres://localhost/dev
API_KEY=public_key

# .env.local - Local overrides (safe to commit with encryption)
DATABASE_URL=postgres://localhost/mydb
SECRET_TOKEN=age[...]  # encrypted, safe to commit!
```

Running `dotenvage dump` produces:

```bash
# .env
API_KEY=public_key
DATABASE_URL=postgres://localhost/dev

# .env.local
DATABASE_URL=postgres://localhost/mydb
SECRET_TOKEN=decrypted_value
```

Running `dotenvage dump --export` produces (note: `--export`
automatically enables bash-compliant escaping):

```bash
# .env
export API_KEY=public_key
export DATABASE_URL=postgres://localhost/dev

# .env.local
export DATABASE_URL=postgres://localhost/mydb
export SECRET_TOKEN=decrypted_value
```

### Bash-Compliant Escaping

When using `--bash` or `--export` (which auto-enables `--bash`),
special bash characters are properly escaped:

```bash
# Without --bash (simple .env format)
PASSWORD=my$ecret

# With --bash (bash-safe escaping)
PASSWORD="my\$ecret"
```

This ensures values with `$`, `` ` ``, `\`, `!`, and other bash
special characters are safely preserved when sourced.

### GNU Make Integration

Use `--make-eval` to securely load variables directly into Make
without creating temporary files:

```makefile
# Makefile example - secure, no temp file with secrets
$(eval $(shell dotenvage dump --make-eval))
export

.PHONY: deploy
deploy:
	@echo "Deploying to $$DATABASE_URL"
	@echo "Using API key: $$API_KEY"
```

**Security Note**: `--make-eval` outputs `$(eval ...)` statements that
are processed directly by Make, avoiding the security risk of writing
decrypted secrets to temporary files.

**Important**: Access variables as `$$VAR` (environment variables) in
recipes, not `$(VAR)` (Make variable expansion). The `export`
directive makes all variables available to recipe shells as
environment variables, where special characters like `$` are properly
preserved.

**Alternative**: If you need the Make format for other purposes,
`--make` outputs `VAR := value` format (but creates a file if
redirected).

This layering system allows you to:

- **Commit ALL `.env*` files to version control** - secrets are
  encrypted
- Share environment-specific configs across the team
  (`.env.production`, `.env.staging`)
- Provide user-specific overrides (`.env.local.alice`) without
  conflicts
- Configure architecture-specific settings (`.env.local.arm64`)

## Key Management

Keys are discovered in this priority order:

1. **`DOTENVAGE_AGE_KEY`** env var (full identity string)
2. **`AGE_KEY`** env var (full identity string)
3. **`EKG_AGE_KEY`** env var (for EKG project compatibility)
4. **`AGE_KEY_NAME`** from .env → key file at
   `$XDG_STATE_HOME/{AGE_KEY_NAME}.key`
5. **Default**: `~/.local/state/{CARGO_PKG_NAME}/dotenvage.key`

### Project-Specific Keys

For multi-project setups, configure in `.env`:

```bash
# .env (committed, not secret)
AGE_KEY_NAME=myproject/myapp
```

Key stored at: `~/.local/state/myproject/myapp.key`

### XDG Base Directories

- Prefers `$XDG_STATE_HOME`
- Falls back to `~/.local/state`
- Or `$XDG_CONFIG_HOME` / `~/.config` (legacy)

### CI/CD

Set `DOTENVAGE_AGE_KEY`, `AGE_KEY`, or `EKG_AGE_KEY` in CI secrets:

```yaml
env:
  DOTENVAGE_AGE_KEY: ${{ secrets.AGE_KEY }}
```

## Contributing

Contributions are welcome! Please see
[CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions and
guidelines.

## License

Licensed under the MIT License. See
[LICENSE](https://github.com/agnos-ai/dotenvage/blob/main/LICENSE) for
details.
