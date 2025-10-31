//! Secret manager implementation for encryption and decryption using age.
//!
//! This module provides the core [`SecretManager`] type for encrypting and
//! decrypting sensitive values using the [age encryption tool](https://age-encryption.org/).

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use age::secrecy::ExposeSecret;
use age::x25519;
use base64::Engine as _;

use crate::error::{SecretsError, SecretsResult};

/// Manages encryption and decryption of secrets using age/X25519.
///
/// `SecretManager` provides a simple interface for encrypting and decrypting
/// sensitive values. It uses the age encryption format with X25519 keys.
///
/// Encrypted values are stored in the compact format: `ENC[AGE:b64:...]`
///
/// # Examples
///
/// ```rust
/// use dotenvage::SecretManager;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Generate a new key
/// let manager = SecretManager::generate()?;
///
/// // Encrypt a value
/// let encrypted = manager.encrypt_value("my-secret-token")?;
/// assert!(SecretManager::is_encrypted(&encrypted));
///
/// // Decrypt it back
/// let decrypted = manager.decrypt_value(&encrypted)?;
/// assert_eq!(decrypted, "my-secret-token");
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct SecretManager {
    identity: x25519::Identity,
}

impl SecretManager {
    /// Creates a new `SecretManager` by loading the key from standard locations.
    ///
    /// # Key Loading Order
    ///
    /// 1. `DOTENVAGE_AGE_KEY` environment variable (full identity string)
    /// 2. `AGE_KEY` environment variable (for compatibility)
    /// 3. `EKG_AGE_KEY` environment variable (for EKG project compatibility)
    /// 4. Default key file at XDG path (e.g., `~/.local/state/dotenvage/dotenvage.key`)
    ///
    /// # Errors
    ///
    /// Returns an error if no key can be found or if the key is invalid.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dotenvage::SecretManager;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SecretManager::new()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> SecretsResult<Self> {
        Self::load_key()
    }

    /// Generates a new random identity.
    ///
    /// Use this when creating a new encryption key. You'll typically want to
    /// save this key using [`save_key`](Self::save_key) or [`save_key_to_default`](Self::save_key_to_default).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dotenvage::SecretManager;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SecretManager::generate()?;
    /// println!("Public key: {}", manager.public_key_string());
    /// # Ok(())
    /// # }
    /// ```
    pub fn generate() -> SecretsResult<Self> {
        Ok(Self {
            identity: x25519::Identity::generate(),
        })
    }

    /// Creates a `SecretManager` from an existing identity.
    ///
    /// Use this when you have an age X25519 identity that you want to use directly.
    pub fn from_identity(identity: x25519::Identity) -> Self {
        Self { identity }
    }

    /// Gets the public key (recipient) corresponding to this identity.
    ///
    /// The public key can be shared with others who want to encrypt values
    /// that only you can decrypt.
    pub fn public_key(&self) -> x25519::Recipient {
        self.identity.to_public()
    }

    /// Gets the public key as a string in age format (starts with `age1`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dotenvage::SecretManager;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SecretManager::generate()?;
    /// let public_key = manager.public_key_string();
    /// assert!(public_key.starts_with("age1"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn public_key_string(&self) -> String {
        self.public_key().to_string()
    }

    /// Encrypts a plaintext value and wraps it in the format `ENC[AGE:b64:...]`.
    ///
    /// The encrypted value can be safely stored in `.env` files and version control.
    ///
    /// # Errors
    ///
    /// Returns an error if encryption fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dotenvage::SecretManager;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SecretManager::generate()?;
    /// let encrypted = manager.encrypt_value("sk_live_abc123")?;
    /// assert!(encrypted.starts_with("ENC[AGE:b64:"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn encrypt_value(&self, plaintext: &str) -> SecretsResult<String> {
        let recipient = self.public_key();
        let recipients: Vec<&dyn age::Recipient> = vec![&recipient];
        let encryptor = age::Encryptor::with_recipients(recipients.into_iter())
            .map_err(|e: age::EncryptError| SecretsError::EncryptionFailed(e.to_string()))?;

        let mut encrypted = Vec::new();
        let mut writer = encryptor
            .wrap_output(&mut encrypted)
            .map_err(|e: std::io::Error| SecretsError::EncryptionFailed(e.to_string()))?;
        writer
            .write_all(plaintext.as_bytes())
            .map_err(|e: std::io::Error| SecretsError::EncryptionFailed(e.to_string()))?;
        writer
            .finish()
            .map_err(|e: std::io::Error| SecretsError::EncryptionFailed(e.to_string()))?;

        let b64 = base64::engine::general_purpose::STANDARD.encode(&encrypted);
        Ok(format!("ENC[AGE:b64:{}]", b64))
    }

    /// Decrypts a value if it's encrypted; otherwise returns it unchanged.
    ///
    /// This method automatically detects whether a value is encrypted by checking
    /// for the `ENC[AGE:b64:...]` prefix or the legacy armor format. If the value
    /// is not encrypted, it's returned as-is.
    ///
    /// # Supported Formats
    ///
    /// - Compact: `ENC[AGE:b64:...]` (recommended)
    /// - Legacy: `-----BEGIN AGE ENCRYPTED FILE-----`
    ///
    /// # Errors
    ///
    /// Returns an error if the value is encrypted but decryption fails
    /// (e.g., wrong key, corrupted data).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dotenvage::SecretManager;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SecretManager::generate()?;
    ///
    /// // Decrypt an encrypted value
    /// let encrypted = manager.encrypt_value("secret")?;
    /// let decrypted = manager.decrypt_value(&encrypted)?;
    /// assert_eq!(decrypted, "secret");
    ///
    /// // Pass through unencrypted values
    /// let plain = manager.decrypt_value("not-encrypted")?;
    /// assert_eq!(plain, "not-encrypted");
    /// # Ok(())
    /// # }
    /// ```
    pub fn decrypt_value(&self, value: &str) -> SecretsResult<String> {
        let trimmed = value.trim();

        // Compact format: ENC[AGE:b64:...]
        if let Some(inner) = trimmed
            .strip_prefix("ENC[AGE:b64:")
            .and_then(|s| s.strip_suffix(']'))
        {
            let encrypted = base64::engine::general_purpose::STANDARD
                .decode(inner)
                .map_err(|e| SecretsError::DecryptionFailed(format!("invalid base64: {}", e)))?;

            let decryptor = age::Decryptor::new(&encrypted[..])
                .map_err(|e: age::DecryptError| SecretsError::DecryptionFailed(e.to_string()))?;
            let identities: Vec<&dyn age::Identity> = vec![&self.identity];
            let mut reader = decryptor
                .decrypt(identities.into_iter())
                .map_err(|e: age::DecryptError| SecretsError::DecryptionFailed(e.to_string()))?;

            let mut decrypted = Vec::new();
            reader
                .read_to_end(&mut decrypted)
                .map_err(|e: std::io::Error| SecretsError::DecryptionFailed(e.to_string()))?;
            return String::from_utf8(decrypted)
                .map_err(|e| SecretsError::DecryptionFailed(e.to_string()));
        }

        // Legacy armor format
        if trimmed.starts_with("-----BEGIN AGE ENCRYPTED FILE-----") {
            let armor_reader = age::armor::ArmoredReader::new(trimmed.as_bytes());
            let decryptor = age::Decryptor::new(armor_reader)
                .map_err(|e: age::DecryptError| SecretsError::DecryptionFailed(e.to_string()))?;
            let identities: Vec<&dyn age::Identity> = vec![&self.identity];
            let mut reader = decryptor
                .decrypt(identities.into_iter())
                .map_err(|e: age::DecryptError| SecretsError::DecryptionFailed(e.to_string()))?;

            let mut decrypted = Vec::new();
            reader
                .read_to_end(&mut decrypted)
                .map_err(|e: std::io::Error| SecretsError::DecryptionFailed(e.to_string()))?;
            return String::from_utf8(decrypted)
                .map_err(|e| SecretsError::DecryptionFailed(e.to_string()));
        }

        Ok(value.to_string())
    }

    /// Checks if a value is in a recognized encrypted format.
    ///
    /// Returns `true` if the value starts with `ENC[AGE:b64:` or the legacy
    /// age armor format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dotenvage::SecretManager;
    ///
    /// assert!(SecretManager::is_encrypted("ENC[AGE:b64:YWdlLWVuY3J5cHRpb24ub3JnL3YxCi0+...]"));
    /// assert!(!SecretManager::is_encrypted("plaintext"));
    /// ```
    pub fn is_encrypted(value: &str) -> bool {
        let t = value.trim();
        t.starts_with("ENC[AGE:b64:") || t.starts_with("-----BEGIN AGE ENCRYPTED FILE-----")
    }

    /// Saves the private identity to a file with restricted permissions.
    ///
    /// On Unix systems, the file permissions are set to `0o600` (readable and
    /// writable only by the owner).
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be created or written.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dotenvage::SecretManager;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SecretManager::generate()?;
    /// manager.save_key("my-key.txt")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn save_key(&self, path: impl AsRef<Path>) -> SecretsResult<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                SecretsError::KeySaveFailed(format!("create dir {}: {}", parent.display(), e))
            })?;
        }
        let identity_string = self.identity.to_string().expose_secret().to_string();
        std::fs::write(path, identity_string.as_bytes())
            .map_err(|e| SecretsError::KeySaveFailed(format!("write {}: {}", path.display(), e)))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(path)
                .map_err(|e| {
                    SecretsError::KeySaveFailed(format!("metadata {}: {}", path.display(), e))
                })?
                .permissions();
            perms.set_mode(0o600);
            std::fs::set_permissions(path, perms).map_err(|e| {
                SecretsError::KeySaveFailed(format!("chmod {}: {}", path.display(), e))
            })?;
        }
        Ok(())
    }

    /// Saves the key to the default path and returns that path.
    ///
    /// The default path is typically `~/.local/state/dotenvage/dotenvage.key`
    /// on Unix systems.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be created or written.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dotenvage::SecretManager;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SecretManager::generate()?;
    /// let path = manager.save_key_to_default()?;
    /// println!("Key saved to: {}", path.display());
    /// # Ok(())
    /// # }
    /// ```
    pub fn save_key_to_default(&self) -> SecretsResult<PathBuf> {
        let p = Self::default_key_path();
        self.save_key(&p)?;
        Ok(p)
    }

    /// Loads the identity from standard locations.
    ///
    /// This is called internally by [`new`](Self::new).
    pub fn load_key() -> SecretsResult<Self> {
        if let Ok(data) = std::env::var("DOTENVAGE_AGE_KEY") {
            return Self::load_from_string(&data);
        }
        if let Ok(data) = std::env::var("AGE_KEY") {
            return Self::load_from_string(&data);
        }
        if let Ok(data) = std::env::var("EKG_AGE_KEY") {
            return Self::load_from_string(&data);
        }
        let key_path = Self::default_key_path();
        if key_path.exists() {
            return Self::load_from_file(&key_path);
        }
        Err(SecretsError::KeyLoadFailed(
            "no key found (DOTENVAGE_AGE_KEY, AGE_KEY, EKG_AGE_KEY, or default key file)".to_string(),
        ))
    }

    fn load_from_file(path: &Path) -> SecretsResult<Self> {
        let key_data = std::fs::read_to_string(path)
            .map_err(|e| SecretsError::KeyLoadFailed(format!("read {}: {}", path.display(), e)))?;
        Self::load_from_string(&key_data)
    }

    fn load_from_string(data: &str) -> SecretsResult<Self> {
        let identity = data
            .parse::<x25519::Identity>()
            .map_err(|e| SecretsError::KeyLoadFailed(format!("parse key: {}", e)))?;
        Ok(Self { identity })
    }

    /// Returns the default key path under XDG directories.
    ///
    /// The path is determined in this order:
    /// 1. `$XDG_STATE_HOME/dotenvage/dotenvage.key`
    /// 2. `$XDG_CONFIG_HOME/dotenvage/dotenvage.key`
    /// 3. `~/.local/state/dotenvage/dotenvage.key`
    /// 4. `~/.config/dotenvage/dotenvage.key` (if it already exists)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dotenvage::SecretManager;
    ///
    /// let path = SecretManager::default_key_path();
    /// println!("Default key path: {}", path.display());
    /// ```
    pub fn default_key_path() -> PathBuf {
        Self::xdg_base_dir()
            .unwrap_or_else(|| PathBuf::from(".").join("dotenvage"))
            .join("dotenvage.key")
    }

    fn xdg_base_dir() -> Option<PathBuf> {
        if let Ok(p) = std::env::var("XDG_STATE_HOME")
            && !p.is_empty()
        {
            return Some(PathBuf::from(p).join("dotenvage"));
        }
        if let Ok(p) = std::env::var("XDG_CONFIG_HOME")
            && !p.is_empty()
        {
            return Some(PathBuf::from(p).join("dotenvage"));
        }
        let home = std::env::var("HOME").ok()?;
        let home_path = PathBuf::from(home);
        let state_dir = home_path.join(".local/state/dotenvage");
        if state_dir.exists() || !home_path.join(".config/dotenvage").exists() {
            return Some(state_dir);
        }
        Some(home_path.join(".config/dotenvage"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let manager = SecretManager::generate().expect("failed to generate manager");
        let plaintext = "sk_live_abc123";
        let encrypted = manager.encrypt_value(plaintext).expect("encryption failed");
        assert!(SecretManager::is_encrypted(&encrypted));
        let decrypted = manager
            .decrypt_value(&encrypted)
            .expect("decryption failed");
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_decrypt_unencrypted_value() {
        let manager = SecretManager::generate().expect("failed to generate manager");
        let plaintext = "not_encrypted";
        let result = manager
            .decrypt_value(plaintext)
            .expect("decrypt should pass through");
        assert_eq!(plaintext, result);
    }
}
