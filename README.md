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

# Dump decrypted .env to stdout (KEY=VALUE lines)
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

## Key Management

- Reads identity from `DOTENVAGE_AGE_KEY` (preferred),
  `AGE_KEY`, or `EKG_AGE_KEY` env vars
- Otherwise uses XDG path (e.g.,
  `~/.local/state/dotenvage/dotenvage.key`)

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for
details.
