//! Environment loader with automatic secret decryption.
//!
//! This module provides [`EnvLoader`] for loading and decrypting environment
//! files, and [`AutoDetectPatterns`] for automatically identifying sensitive
//! variables.

use std::collections::HashMap;
use std::path::{
    Path,
    PathBuf,
};

use crate::error::{
    SecretsError,
    SecretsResult,
};
use crate::manager::SecretManager;

/// Loads environment files with automatic decryption of encrypted values.
///
/// `EnvLoader` reads `.env` files in a specific order and automatically
/// decrypts any encrypted values it encounters. It supports multiple
/// environment variants and user-specific configuration files.
///
/// # Examples
///
/// ```rust,no_run
/// use dotenvage::EnvLoader;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Load from current directory
/// EnvLoader::new()?.load()?;
///
/// // Now encrypted values are available via std::env::var
/// let api_key = std::env::var("API_KEY")?;
/// # Ok(())
/// # }
/// ```
///
/// # File Loading Order
///
/// Files are loaded in the following order (later files override earlier ones):
///
/// 1. `.env` - Base configuration
/// 2. `.env.<ENV>` - Environment-specific
/// 3. `.env.<ENV>-<ARCH>` - Architecture-specific (if `<ARCH>` is set)
/// 4. `.env.<ENV>.<USER>` or `.env.<ENV>-<ARCH>.<USER>` - User-specific
///    overrides
/// 5. `.env.pr-<PR_NUMBER>` - PR-specific (GitHub Actions only)
///
/// **Note**: Separators can be either `.` or `-` (e.g., `.env.local` or
/// `.env-local`)
///
/// # Placeholders
///
/// The following placeholders are resolved from environment variables:
///
/// - **`<ENV>`**: Environment name (see [`resolve_env()`](Self::resolve_env))
/// - **`<ARCH>`**: Architecture name (see
///   [`resolve_arch()`](Self::resolve_arch))
/// - **`<USER>`**: Username (see [`resolve_user()`](Self::resolve_user))
/// - **`<PR_NUMBER>`**: Pull request number (see
///   [`resolve_pr_number()`](Self::resolve_pr_number))
pub struct EnvLoader {
    manager: SecretManager,
}

impl EnvLoader {
    fn find_file_case_insensitive(dir: &Path, filename: &str) -> Option<PathBuf> {
        let target = filename.to_lowercase();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                if name.to_string_lossy().to_lowercase() == target {
                    return Some(entry.path());
                }
            }
        }
        None
    }

    fn gen_names(parts: &[&str]) -> Vec<String> {
        let num_parts = parts.len();
        let mut names = Vec::new();
        for mask in 0..(1u32 << num_parts) {
            let mut name = String::from(".env");
            for (idx, part) in parts.iter().enumerate() {
                let sep = if (mask >> idx) & 1 == 1 { '-' } else { '.' };
                name.push(sep);
                name.push_str(part);
            }
            names.push(name);
        }
        names
    }

    fn add_names_if_exist(dir: &Path, paths: &mut Vec<PathBuf>, parts: &[&str]) {
        for name in Self::gen_names(parts) {
            if let Some(p) = Self::find_file_case_insensitive(dir, &name)
                && !paths.iter().any(|x| x == &p)
            {
                paths.push(p);
            }
        }
    }

    fn add_exact_if_exist(dir: &Path, paths: &mut Vec<PathBuf>, filename: &str) {
        if let Some(p) = Self::find_file_case_insensitive(dir, filename)
            && !paths.iter().any(|x| x == &p)
        {
            paths.push(p);
        }
    }

    /// Creates a new `EnvLoader` with a default `SecretManager`.
    ///
    /// This will load the encryption key from standard locations:
    /// 0. **Auto-discover** `AGE_KEY_NAME` from `.env` or `.env.local` files
    /// 1. `DOTENVAGE_AGE_KEY` environment variable (full identity string)
    /// 2. `AGE_KEY` environment variable
    /// 3. Key file at path determined by discovered `AGE_KEY_NAME`
    /// 4. Default key file at XDG path (e.g.,
    ///    `~/.local/state/dotenvage/dotenvage.key`)
    ///
    /// # Errors
    ///
    /// Returns an error if no encryption key can be found or loaded.
    pub fn new() -> SecretsResult<Self> {
        Ok(Self {
            manager: SecretManager::new()?,
        })
    }

    /// Creates an `EnvLoader` with a specific `SecretManager`.
    ///
    /// Use this when you want to provide your own encryption key.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dotenvage::{
    ///     EnvLoader,
    ///     SecretManager,
    /// };
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SecretManager::generate()?;
    /// let loader = EnvLoader::with_manager(manager);
    /// loader.load()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_manager(manager: SecretManager) -> Self {
        Self { manager }
    }

    /// Loads `.env` files from the current directory in standard order.
    ///
    /// Decrypted values are loaded into the process environment and can be
    /// accessed via `std::env::var()`.
    ///
    /// # Errors
    ///
    /// Returns an error if any file cannot be read or parsed, or if
    /// decryption fails for any encrypted value.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dotenvage::EnvLoader;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// EnvLoader::new()?.load()?;
    /// let secret = std::env::var("API_TOKEN")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn load(&self) -> SecretsResult<()> {
        self.load_from_dir(".")
    }

    /// Loads `.env` files from a specific directory using the same order as
    /// [`load`](Self::load).
    ///
    /// # Errors
    ///
    /// Returns an error if any file cannot be read or parsed, or if
    /// decryption fails for any encrypted value.
    pub fn load_from_dir(&self, dir: impl AsRef<Path>) -> SecretsResult<()> {
        let dir = dir.as_ref();
        let mut env_vars = HashMap::new();

        for path in self.resolve_env_paths(dir) {
            if path.exists() {
                let vars = self.load_env_file(&path)?;
                env_vars.extend(vars);
            }
        }

        for (k, v) in env_vars {
            unsafe {
                std::env::set_var(k, v);
            }
        }
        Ok(())
    }

    /// Computes the ordered list of env file paths to load.
    ///
    /// This method determines which `.env` files exist and should be loaded,
    /// in the correct precedence order.
    ///
    /// # Returns
    ///
    /// A vector of paths in load order (later paths override earlier ones).
    pub fn resolve_env_paths(&self, dir: &Path) -> Vec<PathBuf> {
        let mut paths: Vec<PathBuf> = Vec::new();

        // 1) Always read .env
        Self::add_exact_if_exist(dir, &mut paths, ".env");

        // 2) Resolve ENV and load environment-specific files
        let env = Self::resolve_env();
        Self::add_names_if_exist(dir, &mut paths, &[&env]);

        // 3) Arch-specific: .env.<ENV>-<ARCH>
        let arch = Self::resolve_arch();
        if let Some(ref a) = arch {
            Self::add_names_if_exist(dir, &mut paths, &[&env, a]);
        }

        // 4) User-specific layers
        let user = Self::resolve_user();
        if let Some(ref u) = user {
            Self::add_names_if_exist(dir, &mut paths, &[&env, u]);
            if let Some(ref a) = arch {
                Self::add_names_if_exist(dir, &mut paths, &[&env, a, u]);
            }
        }

        // 5) PR-specific only on GitHub Actions PRs
        if let Some(pr_number) = Self::resolve_pr_number() {
            Self::add_exact_if_exist(dir, &mut paths, &format!(".env.pr-{}", pr_number));
        }

        paths
    }

    /// Loads and decrypts a single `.env` file, returning key/value pairs.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or if decryption fails.
    pub fn load_env_file(&self, path: &Path) -> SecretsResult<HashMap<String, String>> {
        let content =
            std::fs::read_to_string(path).map_err(|e| SecretsError::EnvFileReadFailed {
                path: path.display().to_string(),
                reason: e.to_string(),
            })?;
        self.parse_and_decrypt(&content, path)
    }

    /// Parses env file content and decrypts encrypted values.
    ///
    /// # Errors
    ///
    /// Returns an error if the content cannot be parsed or if decryption fails.
    pub fn parse_and_decrypt(
        &self,
        content: &str,
        path: &Path,
    ) -> SecretsResult<HashMap<String, String>> {
        let mut vars = HashMap::new();
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let mut value = value.trim().to_string();
                if (value.starts_with('"') && value.ends_with('"'))
                    || (value.starts_with('\'') && value.ends_with('\''))
                {
                    value = value[1..value.len() - 1].to_string();
                }
                let decrypted = self.manager.decrypt_value(&value).map_err(|e| {
                    SecretsError::EnvFileParseFailed {
                        path: path.display().to_string(),
                        reason: format!("line {} for '{}': {}", line_num + 1, key, e),
                    }
                })?;
                vars.insert(key, decrypted);
            }
        }
        Ok(vars)
    }

    /// Gets a decrypted environment variable value from the process
    /// environment.
    ///
    /// If the value is encrypted, it will be automatically decrypted.
    ///
    /// # Errors
    ///
    /// Returns an error if the variable is not set or if decryption fails.
    pub fn get_var(&self, key: &str) -> SecretsResult<String> {
        let value = std::env::var(key).map_err(|_| SecretsError::EnvVarNotFound {
            key: key.to_string(),
        })?;
        self.manager.decrypt_value(&value)
    }

    /// Gets a decrypted environment variable, or returns a default if not set.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dotenvage::EnvLoader;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let loader = EnvLoader::new()?;
    /// let port = loader.get_var_or("PORT", "8080");
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_var_or(&self, key: &str, default: &str) -> String {
        self.get_var(key).unwrap_or_else(|_| default.to_string())
    }

    /// Resolves the `<ENV>` placeholder for environment-specific file names.
    ///
    /// The environment name is resolved in the following order:
    ///
    /// 1. `DOTENVAGE_ENV` environment variable (preferred)
    /// 2. `EKG_ENV` environment variable (alternative)
    /// 3. `VERCEL_ENV` environment variable
    /// 4. `NODE_ENV` environment variable
    /// 5. Defaults to `"local"` if none of the above are set
    ///
    /// The value is always converted to lowercase.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dotenvage::EnvLoader;
    ///
    /// // With DOTENVAGE_ENV=production, returns "production"
    /// // Without any env vars set, returns "local"
    /// let env = EnvLoader::resolve_env();
    /// println!("Environment: {}", env);
    /// ```
    pub fn resolve_env() -> String {
        std::env::var("DOTENVAGE_ENV")
            .ok()
            .filter(|s| !s.is_empty())
            .or_else(|| std::env::var("EKG_ENV").ok().filter(|s| !s.is_empty()))
            .or_else(|| std::env::var("VERCEL_ENV").ok().filter(|s| !s.is_empty()))
            .or_else(|| std::env::var("NODE_ENV").ok().filter(|s| !s.is_empty()))
            .map(|e| e.to_lowercase())
            .unwrap_or_else(|| "local".to_string())
    }

    /// Resolves the `<ARCH>` placeholder for architecture-specific file names.
    ///
    /// The architecture name is resolved from the first available source:
    ///
    /// 1. `DOTENVAGE_ARCH` environment variable (preferred)
    /// 2. `EKG_ARCH` environment variable (alternative)
    /// 3. `TARGETARCH` environment variable (Docker multi-platform builds,
    ///    e.g., "amd64", "arm64")
    /// 4. `TARGETPLATFORM` environment variable (Docker, parsed for arch, e.g.,
    ///    "linux/arm64" → "arm64")
    /// 5. `RUNNER_ARCH` environment variable (GitHub Actions, e.g., "X64",
    ///    "ARM64")
    /// 6. Returns `None` if none are set
    ///
    /// The value is always converted to lowercase and normalized:
    /// - `"x64"`, `"amd64"`, `"x86_64"` → `"amd64"`
    /// - `"aarch64"` → `"arm64"`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dotenvage::EnvLoader;
    ///
    /// // With TARGETARCH=arm64 (Docker build), resolves to Some("arm64")
    /// // With RUNNER_ARCH=X64 (GitHub Actions), resolves to Some("amd64")
    /// if let Some(arch) = EnvLoader::resolve_arch() {
    ///     println!("Architecture: {}", arch);
    /// }
    /// ```
    pub fn resolve_arch() -> Option<String> {
        let arch = std::env::var("DOTENVAGE_ARCH")
            .ok()
            .filter(|s| !s.is_empty())
            .or_else(|| std::env::var("EKG_ARCH").ok().filter(|s| !s.is_empty()))
            .or_else(|| std::env::var("TARGETARCH").ok().filter(|s| !s.is_empty()))
            .or_else(|| {
                // Parse TARGETPLATFORM (e.g., "linux/arm64" → "arm64")
                std::env::var("TARGETPLATFORM")
                    .ok()
                    .filter(|s| !s.is_empty())
                    .and_then(|p| p.split('/').nth(1).map(String::from))
            })
            .or_else(|| std::env::var("RUNNER_ARCH").ok().filter(|s| !s.is_empty()))?;

        // Normalize and convert to lowercase
        let arch_lower = arch.to_lowercase();
        let normalized = match arch_lower.as_str() {
            "x64" | "x86_64" => "amd64",
            "aarch64" => "arm64",
            other => other,
        };

        Some(normalized.to_string())
    }

    /// Resolves the `<USER>` placeholder for user-specific file names.
    ///
    /// The username is resolved from the first available environment variable:
    ///
    /// 1. `DOTENVAGE_USER` (preferred)
    /// 2. `EKG_USER`
    /// 3. `GITHUB_ACTOR` (GitHub Actions)
    /// 4. `GITHUB_TRIGGERING_ACTOR` (GitHub Actions)
    /// 5. `GITHUB_REPOSITORY_OWNER` (GitHub Actions)
    /// 6. `USER` (Unix standard)
    /// 7. `USERNAME` (Windows standard)
    /// 8. Returns `None` if none are set
    ///
    /// The value is always converted to lowercase.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dotenvage::EnvLoader;
    ///
    /// // Typically resolves from $USER on Unix or %USERNAME% on Windows
    /// if let Some(user) = EnvLoader::resolve_user() {
    ///     println!("User: {}", user);
    /// }
    /// ```
    pub fn resolve_user() -> Option<String> {
        std::env::var("DOTENVAGE_USER")
            .ok()
            .filter(|s| !s.is_empty())
            .or_else(|| std::env::var("EKG_USER").ok().filter(|s| !s.is_empty()))
            .or_else(|| std::env::var("GITHUB_ACTOR").ok().filter(|s| !s.is_empty()))
            .or_else(|| {
                std::env::var("GITHUB_TRIGGERING_ACTOR")
                    .ok()
                    .filter(|s| !s.is_empty())
            })
            .or_else(|| {
                std::env::var("GITHUB_REPOSITORY_OWNER")
                    .ok()
                    .filter(|s| !s.is_empty())
            })
            .or_else(|| std::env::var("USER").ok().filter(|s| !s.is_empty()))
            .or_else(|| std::env::var("USERNAME").ok().filter(|s| !s.is_empty()))
            .map(|u| u.to_lowercase())
    }

    /// Resolves the `<PR_NUMBER>` placeholder for PR-specific file names.
    ///
    /// The PR number is only resolved in GitHub Actions pull request contexts:
    ///
    /// 1. Checks that `GITHUB_EVENT_NAME` starts with `"pull_request"`
    /// 2. Reads from `PR_NUMBER` environment variable
    /// 3. Falls back to parsing `GITHUB_REF` (e.g., `refs/pull/123/merge`)
    /// 4. Returns `None` if not in a PR context
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dotenvage::EnvLoader;
    ///
    /// // In GitHub Actions PR, resolves to Some("123")
    /// // Outside of PR context, returns None
    /// if let Some(pr_number) = EnvLoader::resolve_pr_number() {
    ///     println!("PR Number: {}", pr_number);
    /// }
    /// ```
    pub fn resolve_pr_number() -> Option<String> {
        // Only resolve in GitHub Actions pull request context
        if let Ok(event) = std::env::var("GITHUB_EVENT_NAME")
            && event.starts_with("pull_request")
            && let Some(pr) = std::env::var("PR_NUMBER").ok().filter(|s| !s.is_empty())
        {
            return Some(pr);
        }

        // Try parsing from GITHUB_REF
        if let Ok(gref) = std::env::var("GITHUB_REF")
            && let Some(idx) = gref.find("/pull/")
        {
            let mut pr_number = String::new();
            for c in gref[idx + 6..].chars() {
                if c.is_ascii_digit() {
                    pr_number.push(c);
                } else {
                    break;
                }
            }
            if !pr_number.is_empty() {
                return Some(pr_number);
            }
        }

        None
    }
}

/// Auto-detection patterns for identifying sensitive environment variables.
///
/// This utility helps determine which environment variables should be encrypted
/// based on their names. It uses common patterns to identify secrets like
/// tokens, passwords, and API keys.
///
/// # Examples
///
/// ```rust
/// use dotenvage::AutoDetectPatterns;
///
/// assert!(AutoDetectPatterns::should_encrypt("API_TOKEN"));
/// assert!(AutoDetectPatterns::should_encrypt("DATABASE_PASSWORD"));
/// assert!(!AutoDetectPatterns::should_encrypt("PORT"));
/// assert!(!AutoDetectPatterns::should_encrypt("AWS_REGION"));
/// ```
pub struct AutoDetectPatterns;

impl AutoDetectPatterns {
    /// Patterns indicating a variable should be encrypted.
    ///
    /// Variables containing any of these substrings (case-insensitive) will be
    /// automatically encrypted unless they match a pattern in
    /// [`NEVER_ENCRYPT`](Self::NEVER_ENCRYPT).
    pub const ENCRYPT_PATTERNS: &'static [&'static str] = &[
        "TOKEN",
        "SECRET",
        "PASSWORD",
        "CREDENTIAL",
        "_KEY",
        "API_KEY",
        "PRIVATE_KEY",
    ];

    /// Variables that should never be encrypted.
    ///
    /// These are typically configuration values that need to be plaintext for
    /// readability or compatibility reasons.
    pub const NEVER_ENCRYPT: &'static [&'static str] = &[
        "AWS_REGION",
        "FLY_PRIMARY_REGION",
        "PORT",
        "RUST_LOG",
        "DATABASE_NAME",
        "APP_NAME",
        "ENDPOINT_URL",
        "ORG",
        "PUBLIC_KEY",
        "PUB_KEY",
    ];

    /// Returns `true` if an environment variable name should be encrypted.
    ///
    /// This checks the variable name against
    /// [`ENCRYPT_PATTERNS`](Self::ENCRYPT_PATTERNS)
    /// and [`NEVER_ENCRYPT`](Self::NEVER_ENCRYPT) lists.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dotenvage::AutoDetectPatterns;
    ///
    /// assert!(AutoDetectPatterns::should_encrypt("STRIPE_API_KEY"));
    /// assert!(AutoDetectPatterns::should_encrypt("github_token"));
    /// assert!(!AutoDetectPatterns::should_encrypt("DATABASE_NAME"));
    /// ```
    pub fn should_encrypt(key: &str) -> bool {
        let key_upper = key.to_uppercase();
        if Self::NEVER_ENCRYPT.iter().any(|p| key_upper.contains(p)) {
            return false;
        }
        Self::ENCRYPT_PATTERNS.iter().any(|p| key_upper.contains(p))
    }
}
