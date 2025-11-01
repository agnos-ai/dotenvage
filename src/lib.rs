#![warn(missing_docs)]

//! # dotenvage
//!
//! A library and CLI tool for managing encrypted secrets in `.env` files using [age](https://age-encryption.org/) encryption.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use dotenvage::{
//!     EnvLoader,
//!     SecretManager,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Load environment files with automatic decryption
//! EnvLoader::new()?.load()?;
//!
//! // Or manage secrets manually
//! let manager = SecretManager::generate()?;
//! let encrypted = manager.encrypt_value("my-secret")?;
//! let decrypted = manager.decrypt_value(&encrypted)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Features
//!
//! - **Selective Encryption**: Only encrypt sensitive values (tokens,
//!   passwords, API keys)
//! - **Modern Cryptography**: Uses age (X25519) for encryption
//! - **CI/CD Friendly**: Supports key loading via environment variables
//! - **Smart Detection**: Auto-detects sensitive keys based on naming patterns
//! - **Multiple Environments**: Support for .env, .env.local, .env.production,
//!   etc.

pub mod error;
pub mod loader;
pub mod manager;
pub mod patterns;

pub use crate::error::{
    SecretsError,
    SecretsResult,
};
pub use crate::loader::{
    Arch,
    AutoDetectPatterns,
    EnvLoader,
};
pub use crate::manager::SecretManager;
