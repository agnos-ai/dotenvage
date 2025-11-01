# dotenvage

[![Crates.io](https://img.shields.io/crates/v/dotenvage.svg)](https://crates.io/crates/dotenvage)
[![Documentation](https://docs.rs/dotenvage/badge.svg)](https://docs.rs/dotenvage)
[![CI](https://github.com/agnos-ai/dotenvage/workflows/CI%2FCD/badge.svg)](https://github.com/agnos-ai/dotenvage/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/dotenvage.svg)](https://crates.io/crates/dotenvage)

Dotenv with age encryption: encrypt/decrypt secrets in
`.env` files.

- Selective encryption of sensitive keys
- Uses age (X25519) for modern encryption
- Library + CLI
- CI-friendly (supports key via env var)

## Install

```bash
cargo install dotenvage
```

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

# List keys (show lock icon; verbose shows decrypted values)
dotenvage list --file .env.local --verbose

# Dump all decrypted env vars (merges all .env* files with layering)
dotenvage dump

# Dump a specific file
dotenvage dump .env.local
```

## Library

```rust
use dotenvage::{SecretManager, EnvLoader};

// Load env files with auto-decryption
EnvLoader::new()?.load()?;

// Encrypt and decrypt values
let manager = SecretManager::generate()?;
let enc = manager.encrypt_value("secret")?;
let dec = manager.decrypt_value(&enc)?;
```

## File Layering

One of dotenvage's key features is **automatic file layering** - multiple `.env*` files are loaded and merged with a clear precedence order. Later files override values from earlier files.

### Loading Order

Files are loaded in this order (later overrides earlier):

1. **`.env`** - Base configuration (committed to repo)
2. **`.env.<ENV>`** - Environment-specific (e.g., `.env.local`, `.env.production`)
3. **`.env.<ENV>-<ARCH>`** - Architecture-specific (e.g., `.env.local-arm64`)
4. **`.env.<ENV>.<USER>`** - User-specific overrides (e.g., `.env.local.alice`)
5. **`.env.pr-<NUMBER>`** - PR-specific (GitHub Actions only)

**Note**: Separators can be `.` or `-` (e.g., `.env.local` or `.env-local` both work)

### Placeholders

- **`<ENV>`**: From `DOTENVAGE_ENV`, `EKG_ENV`, `VERCEL_ENV`, `NODE_ENV`, or defaults to `local`
- **`<ARCH>`**: From `DOTENVAGE_ARCH` or `EKG_ARCH` (e.g., `arm64`, `x86_64`)
- **`<USER>`**: From `DOTENVAGE_USER`, `EKG_USER`, or system username
- **`<PR_NUMBER>`**: Auto-detected from GitHub Actions `GITHUB_REF`

### Example

Given these files:

```bash
# .env (base)
DATABASE_URL=postgres://localhost/dev
API_KEY=public_key

# .env.local (overrides)
DATABASE_URL=postgres://localhost/mydb
SECRET_TOKEN=age[...]  # encrypted
```

Running `dotenvage dump` produces:
```bash
DATABASE_URL=postgres://localhost/mydb  # from .env.local
API_KEY=public_key                       # from .env
SECRET_TOKEN=decrypted_value             # decrypted from .env.local
```

This layering system allows you to:
- Commit base config (`.env`) to version control
- Keep local overrides (`.env.local`) in `.gitignore`
- Share environment-specific configs (`.env.production`)
- Support multiple developers with user-specific overrides

## Key Management

Keys are discovered in this priority order:

1. **`DOTENVAGE_AGE_KEY`** env var (full identity string)
2. **`AGE_KEY`** env var (full identity string)  
3. **`AGE_KEY_NAME`** from .env ? key file at `$XDG_STATE_HOME/{AGE_KEY_NAME}.key`
4. **Default**: `~/.local/state/{CARGO_PKG_NAME}/dotenvage.key`

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

Set `DOTENVAGE_AGE_KEY` or `AGE_KEY` in CI secrets:

```yaml
env:
  DOTENVAGE_AGE_KEY: ${{ secrets.AGE_KEY }}
```

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for
details.
