use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dotenvage::{AutoDetectPatterns, SecretManager};

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Generate a new encryption key pair
    #[command(alias = "gen")]
    Keygen {
        /// Output file path (default: XDG path)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Force overwrite if key already exists
        #[arg(short, long)]
        force: bool,
    },
    /// Encrypt sensitive values in an environment file
    Encrypt {
        /// Path to environment file (e.g., .env.local)
        #[arg(default_value = ".env.local")]
        file: PathBuf,
        /// Specific keys to encrypt (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        keys: Option<Vec<String>>,
        /// Use auto-detection based on key name patterns
        #[arg(short, long, default_value = "true")]
        auto: bool,
    },
    /// Edit an environment file (decrypts, opens editor, re-encrypts)
    Edit { #[arg(default_value = ".env.local")] file: PathBuf },
    /// Set a secret value
    Set {
        /// KEY=VALUE pair to set
        pair: String,
        /// Environment file to update
        #[arg(short, long, default_value = ".env.local")]
        file: PathBuf,
    },
    /// Get a decrypted secret value (scans .env files in order)
    Get {
        /// Environment variable name
        key: String,
        /// Specific file to read from (if not provided, scans .env* files)
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    /// List environment variables and their encryption status
    List {
        /// Environment file to list from
        #[arg(short, long, default_value = ".env.local")]
        file: PathBuf,
        /// Show values (decrypted)
        #[arg(short, long)]
        verbose: bool,
    },
    /// Dump environment file to stdout with all values decrypted
    Dump { #[arg(default_value = ".env.local")] file: PathBuf },
}

#[derive(Parser, Debug, Clone)]
#[command(name = "dotenvage", version, about = "Dotenv with age encryption")] 
struct Cli { #[command(subcommand)] command: Commands }

fn parse_env_file(content: &str) -> Result<HashMap<String, String>> {
    dotenvy::from_read_iter(content.as_bytes())
        .collect::<Result<HashMap<String, String>, _>>()
        .context("Failed to parse .env file")
}

fn write_env_file(path: &Path, vars: &HashMap<String, String>) -> Result<()> {
    let mut file = std::fs::File::create(path)
        .with_context(|| format!("Failed to create {}", path.display()))?;
    let mut keys: Vec<_> = vars.keys().collect();
    keys.sort();
    for key in keys {
        let value = vars.get(key).unwrap();
        if value.contains(' ') || value.contains('$') || value.contains('\n') {
            writeln!(file, "{}=\"{}\"", key, value.replace('"', "\\\""))?;
        } else {
            writeln!(file, "{}={}", key, value)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match cli.command {
        Commands::Keygen { output, force } => keygen(output, force),
        Commands::Encrypt { file, keys, auto } => encrypt(file, keys, auto),
        Commands::Edit { file } => edit(file),
        Commands::Set { pair, file } => set(pair, file),
        Commands::Get { key, file } => get(key, file),
        Commands::List { file, verbose } => list(file, verbose),
        Commands::Dump { file } => dump(file),
    }
}

fn keygen(output: Option<PathBuf>, force: bool) -> Result<()> {
    let manager = SecretManager::generate().context("Failed to generate key")?;
    let out = output.unwrap_or_else(|| SecretManager::default_key_path());
    if out.exists() && !force {
        anyhow::bail!("Key file already exists at {}. Use --force to overwrite.", out.display());
    }
    manager.save_key(&out).context("Failed to save key")?;
    println!("✓ Private key saved to: {}", out.display());
    println!("Public recipient: {}", manager.public_key_string());
    Ok(())
}

fn encrypt(file: PathBuf, keys: Option<Vec<String>>, auto: bool) -> Result<()> {
    let manager = SecretManager::new().context("Failed to load encryption key")?;
    if !file.exists() { anyhow::bail!("File not found: {}", file.display()); }
    let content = std::fs::read_to_string(&file)
        .with_context(|| format!("Failed to read {}", file.display()))?;
    let mut vars = parse_env_file(&content)?;
    let mut encrypted_count = 0;
    let keys_to_encrypt: Vec<String> = if let Some(specific) = keys { specific } else if auto {
        vars.keys().filter(|k| AutoDetectPatterns::should_encrypt(k)).cloned().collect()
    } else { anyhow::bail!("Either --keys or --auto must be specified"); };
    for key in &keys_to_encrypt {
        if let Some(value) = vars.get(key) {
            if !SecretManager::is_encrypted(value) {
                let encrypted = manager.encrypt_value(value)
                    .with_context(|| format!("Failed to encrypt {}", key))?;
                vars.insert(key.clone(), encrypted);
                encrypted_count += 1;
            }
        }
    }
    write_env_file(&file, &vars)?;
    println!("✓ Encrypted {} value(s) in {}", encrypted_count, file.display());
    if encrypted_count > 0 {
        println!("  Encrypted keys:");
        for key in &keys_to_encrypt {
            if vars.get(key).map_or(false, |v| SecretManager::is_encrypted(v)) { println!("    - {}", key); }
        }
    }
    Ok(())
}

fn edit(file: PathBuf) -> Result<()> {
    let manager = SecretManager::new().context("Failed to load encryption key")?;
    if !file.exists() { anyhow::bail!("File not found: {}", file.display()); }
    let content = std::fs::read_to_string(&file)
        .with_context(|| format!("Failed to read {}", file.display()))?;
    let mut vars = parse_env_file(&content)?;
    let mut keys_to_encrypt = Vec::new();
    for (key, value) in &mut vars {
        if SecretManager::is_encrypted(value) {
            keys_to_encrypt.push(key.clone());
            *value = manager.decrypt_value(value).with_context(|| format!("Failed to decrypt {}", key))?;
        }
    }
    let temp = tempfile::Builder::new().suffix(".env").tempfile().context("Failed to create temp file")?;
    write_env_file(temp.path(), &vars)?;
    let original = std::fs::read_to_string(temp.path()).context("Failed to read temp file")?;
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let status = std::process::Command::new(&editor).arg(temp.path()).status()
        .with_context(|| format!("Failed to launch editor: {}", editor))?;
    if !status.success() { anyhow::bail!("Editor exited with non-zero status"); }
    let edited = std::fs::read_to_string(temp.path()).context("Failed to read edited file")?;
    if edited == original { println!("No changes made."); return Ok(()); }
    let mut edited_vars = parse_env_file(&edited)?;
    for key in &keys_to_encrypt {
        if let Some(value) = edited_vars.get_mut(key) {
            if !SecretManager::is_encrypted(value) {
                *value = manager.encrypt_value(value).with_context(|| format!("Failed to encrypt {}", key))?;
            }
        }
    }
    write_env_file(&file, &edited_vars)?;
    println!("✓ Saved encrypted changes to {}", file.display());
    Ok(())
}

fn set(pair: String, file: PathBuf) -> Result<()> {
    let manager = SecretManager::new().context("Failed to load encryption key")?;
    let (key, value) = pair.split_once('=').context("Invalid KEY=VALUE format")?;
    let mut vars = if file.exists() {
        let content = std::fs::read_to_string(&file).with_context(|| format!("Failed to read {}", file.display()))?;
        parse_env_file(&content)?
    } else { HashMap::new() };
    let final_value = if AutoDetectPatterns::should_encrypt(key) { manager.encrypt_value(value).context("Failed to encrypt value")? } else { value.to_string() };
    vars.insert(key.to_string(), final_value.clone());
    write_env_file(&file, &vars)?;
    let status = if SecretManager::is_encrypted(&final_value) { "encrypted" } else { "plain" };
    println!("✓ Set {} ({}) in {}", key, status, file.display());
    Ok(())
}

fn get(key: String, file: Option<PathBuf>) -> Result<()> {
    let manager = SecretManager::new().context("Failed to load encryption key")?;
    let value = if let Some(file_path) = file {
        let content = std::fs::read_to_string(&file_path).with_context(|| format!("Failed to read {}", file_path.display()))?;
        let vars = parse_env_file(&content)?;
        vars.get(&key).with_context(|| format!("Key '{}' not found in {}", key, file_path.display()))?.clone()
    } else {
        // Scan ordered files similar to EnvLoader
        let loader = dotenvage::EnvLoader::with_manager(manager.clone());
        let paths = loader.resolve_env_paths(Path::new("."));
        let mut found: Option<String> = None;
        for p in paths {
            if p.exists() {
                let content = std::fs::read_to_string(&p)?;
                let vars = parse_env_file(&content)?;
                if let Some(v) = vars.get(&key) { found = Some(v.clone()); }
            }
        }
        found.with_context(|| format!("Key '{}' not found in any .env* file", key))?
    };
    let decrypted = manager.decrypt_value(&value).context("Failed to decrypt value")?;
    println!("{}", decrypted);
    Ok(())
}

fn list(file: PathBuf, verbose: bool) -> Result<()> {
    let manager = SecretManager::new().context("Failed to load encryption key")?;
    if !file.exists() { anyhow::bail!("File not found: {}", file.display()); }
    let content = std::fs::read_to_string(&file).with_context(|| format!("Failed to read {}", file.display()))?;
    let vars = parse_env_file(&content)?;
    println!("Environment variables in {}:\n", file.display());
    let mut keys: Vec<_> = vars.keys().collect();
    keys.sort();
    for key in keys {
        let value = vars.get(key).unwrap();
        let is_encrypted = SecretManager::is_encrypted(value);
        let lock_icon = if is_encrypted { "🔒" } else { "  " };
        if verbose {
            let display_value = if is_encrypted { manager.decrypt_value(value).unwrap_or_else(|_| "<decryption failed>".to_string()) } else { value.clone() };
            println!("{} {} = {}", lock_icon, key, display_value);
        } else {
            println!("{} {}", lock_icon, key);
        }
    }
    Ok(())
}

fn dump(file: PathBuf) -> Result<()> {
    let manager = SecretManager::new().context("Failed to load encryption key")?;
    if !file.exists() { anyhow::bail!("File not found: {}", file.display()); }
    let content = std::fs::read_to_string(&file).with_context(|| format!("Failed to read {}", file.display()))?;
    let vars = parse_env_file(&content)?;
    let mut keys: Vec<_> = vars.keys().cloned().collect();
    keys.sort();
    for key in keys {
        if let Some(value) = vars.get(&key) {
            let decrypted_value = manager.decrypt_value(value).with_context(|| format!("Failed to decrypt {}", key))?;
            if decrypted_value.contains(char::is_whitespace) || decrypted_value.contains('=') || decrypted_value.contains('"') || decrypted_value.contains('\'') {
                println!("{}=\"{}\"", key, decrypted_value.replace('"', "\\\""));
            } else {
                println!("{}={}", key, decrypted_value);
            }
        }
    }
    Ok(())
}
