use std::collections::HashMap;
use std::io::Write;
use std::path::{
    Path,
    PathBuf,
};

use anyhow::{
    Context,
    Result,
};
use clap::{
    Parser,
    Subcommand,
};
use dotenvage::{
    AutoDetectPatterns,
    SecretManager,
};

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
    Edit {
        #[arg(default_value = ".env.local")]
        file: PathBuf,
    },
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
    Dump {
        /// Specific file to dump (if not provided, scans .env* files in order)
        #[arg(short, long)]
        file: Option<PathBuf>,
        /// Use bash-compliant escaping rules (strict quoting and escaping)
        #[arg(short, long)]
        bash: bool,
        /// Output in GNU Make format (VAR := value) with Make-safe escaping
        #[arg(short, long)]
        make: bool,
        /// Prefix each line with 'export ' for bash sourcing
        #[arg(short, long)]
        export: bool,
    },
}

#[derive(Parser, Debug, Clone)]
#[command(name = "dotenvage", version, about = "Dotenv with age encryption")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

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
        Commands::Dump { file, bash, make, export } => dump(file, bash, make, export),
    }
}

fn keygen(output: Option<PathBuf>, force: bool) -> Result<()> {
    let manager = SecretManager::generate().context("Failed to generate key")?;
    let out = output.unwrap_or_else(SecretManager::default_key_path);
    if out.exists() && !force {
        anyhow::bail!(
            "Key file already exists at {}. Use --force to overwrite.",
            out.display()
        );
    }
    manager.save_key(&out).context("Failed to save key")?;
    println!("✓ Private key saved to: {}", out.display());
    println!("Public recipient: {}", manager.public_key_string());
    Ok(())
}

fn encrypt(file: PathBuf, keys: Option<Vec<String>>, auto: bool) -> Result<()> {
    let manager = SecretManager::new().context("Failed to load encryption key")?;
    if !file.exists() {
        anyhow::bail!("File not found: {}", file.display());
    }
    let content = std::fs::read_to_string(&file)
        .with_context(|| format!("Failed to read {}", file.display()))?;
    let mut vars = parse_env_file(&content)?;
    let mut encrypted_count = 0;
    let keys_to_encrypt: Vec<String> = if let Some(specific) = keys {
        specific
    } else if auto {
        vars.keys()
            .filter(|k| AutoDetectPatterns::should_encrypt(k))
            .cloned()
            .collect()
    } else {
        anyhow::bail!("Either --keys or --auto must be specified");
    };
    for key in &keys_to_encrypt {
        if let Some(value) = vars.get(key)
            && !SecretManager::is_encrypted(value)
        {
            let encrypted = manager
                .encrypt_value(value)
                .with_context(|| format!("Failed to encrypt {}", key))?;
            vars.insert(key.clone(), encrypted);
            encrypted_count += 1;
        }
    }
    write_env_file(&file, &vars)?;
    println!(
        "✓ Encrypted {} value(s) in {}",
        encrypted_count,
        file.display()
    );
    if encrypted_count > 0 {
        println!("  Encrypted keys:");
        for key in &keys_to_encrypt {
            if vars
                .get(key)
                .is_some_and(|v| SecretManager::is_encrypted(v))
            {
                println!("    - {}", key);
            }
        }
    }
    Ok(())
}

fn edit(file: PathBuf) -> Result<()> {
    let manager = SecretManager::new().context("Failed to load encryption key")?;
    if !file.exists() {
        anyhow::bail!("File not found: {}", file.display());
    }
    let content = std::fs::read_to_string(&file)
        .with_context(|| format!("Failed to read {}", file.display()))?;
    let mut vars = parse_env_file(&content)?;
    let mut keys_to_encrypt = Vec::new();
    for (key, value) in &mut vars {
        if SecretManager::is_encrypted(value) {
            keys_to_encrypt.push(key.clone());
            *value = manager
                .decrypt_value(value)
                .with_context(|| format!("Failed to decrypt {}", key))?;
        }
    }
    let temp = tempfile::Builder::new()
        .suffix(".env")
        .tempfile()
        .context("Failed to create temp file")?;
    write_env_file(temp.path(), &vars)?;
    let original = std::fs::read_to_string(temp.path()).context("Failed to read temp file")?;
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let status = std::process::Command::new(&editor)
        .arg(temp.path())
        .status()
        .with_context(|| format!("Failed to launch editor: {}", editor))?;
    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status");
    }
    let edited = std::fs::read_to_string(temp.path()).context("Failed to read edited file")?;
    if edited == original {
        println!("No changes made.");
        return Ok(());
    }
    let mut edited_vars = parse_env_file(&edited)?;
    for key in &keys_to_encrypt {
        if let Some(value) = edited_vars.get_mut(key)
            && !SecretManager::is_encrypted(value)
        {
            *value = manager
                .encrypt_value(value)
                .with_context(|| format!("Failed to encrypt {}", key))?;
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
        let content = std::fs::read_to_string(&file)
            .with_context(|| format!("Failed to read {}", file.display()))?;
        parse_env_file(&content)?
    } else {
        HashMap::new()
    };
    let final_value = if AutoDetectPatterns::should_encrypt(key) {
        manager
            .encrypt_value(value)
            .context("Failed to encrypt value")?
    } else {
        value.to_string()
    };
    vars.insert(key.to_string(), final_value.clone());
    write_env_file(&file, &vars)?;
    let status = if SecretManager::is_encrypted(&final_value) {
        "encrypted"
    } else {
        "plain"
    };
    println!("✓ Set {} ({}) in {}", key, status, file.display());
    Ok(())
}

fn get(key: String, file: Option<PathBuf>) -> Result<()> {
    let manager = SecretManager::new().context("Failed to load encryption key")?;
    let value = if let Some(file_path) = file {
        let content = std::fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read {}", file_path.display()))?;
        let vars = parse_env_file(&content)?;
        vars.get(&key)
            .with_context(|| format!("Key '{}' not found in {}", key, file_path.display()))?
            .clone()
    } else {
        // Scan ordered files similar to EnvLoader
        let loader = dotenvage::EnvLoader::with_manager(manager.clone());
        let paths = loader.resolve_env_paths(Path::new("."));
        let mut found: Option<String> = None;
        for p in paths {
            if p.exists() {
                let content = std::fs::read_to_string(&p)?;
                let vars = parse_env_file(&content)?;
                if let Some(v) = vars.get(&key) {
                    found = Some(v.clone());
                }
            }
        }
        found.with_context(|| format!("Key '{}' not found in any .env* file", key))?
    };
    let decrypted = manager
        .decrypt_value(&value)
        .context("Failed to decrypt value")?;
    println!("{}", decrypted);
    Ok(())
}

fn list(file: PathBuf, verbose: bool) -> Result<()> {
    let manager = SecretManager::new().context("Failed to load encryption key")?;
    if !file.exists() {
        anyhow::bail!("File not found: {}", file.display());
    }
    let content = std::fs::read_to_string(&file)
        .with_context(|| format!("Failed to read {}", file.display()))?;
    let vars = parse_env_file(&content)?;
    println!("Environment variables in {}:\n", file.display());
    let mut keys: Vec<_> = vars.keys().collect();
    keys.sort();
    for key in keys {
        let value = vars.get(key).unwrap();
        let is_encrypted = SecretManager::is_encrypted(value);
        let lock_icon = if is_encrypted { "🔒" } else { "  " };
        if verbose {
            let display_value = if is_encrypted {
                manager
                    .decrypt_value(value)
                    .unwrap_or_else(|_| "<decryption failed>".to_string())
            } else {
                value.clone()
            };
            println!("{} {} = {}", lock_icon, key, display_value);
        } else {
            println!("{} {}", lock_icon, key);
        }
    }
    Ok(())
}

fn dump(file: Option<PathBuf>, bash: bool, make: bool, export: bool) -> Result<()> {
    let manager = SecretManager::new().context("Failed to load encryption key")?;
    
    if let Some(file_path) = file {
        // Dump specific file only (no comments, just vars)
        if !file_path.exists() {
            anyhow::bail!("File not found: {}", file_path.display());
        }
        let content = std::fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read {}", file_path.display()))?;
        let all_vars = parse_env_file(&content)?;
        dump_vars(&manager, &all_vars, bash, make, export)?;
    } else {
        // Scan ordered files and show sections for each file
        let loader = dotenvage::EnvLoader::with_manager(manager.clone());
        let paths = loader.resolve_env_paths(Path::new("."));
        let mut is_first = true;
        
        for p in paths {
            if p.exists() {
                let content = std::fs::read_to_string(&p)?;
                let vars = parse_env_file(&content)?;
                
                // Only show section if the file has variables
                if !vars.is_empty() {
                    if !is_first && !make {
                        println!(); // Blank line between sections (not for make mode)
                    }
                    if !make {
                        println!("# {}", p.display());
                    }
                    dump_vars(&manager, &vars, bash, make, export)?;
                    is_first = false;
                }
            }
        }
    }
    
    Ok(())
}

fn dump_vars(manager: &SecretManager, vars: &HashMap<String, String>, bash: bool, make: bool, export: bool) -> Result<()> {
    let mut keys: Vec<_> = vars.keys().cloned().collect();
    keys.sort();
    
    for key in keys {
        if let Some(value) = vars.get(&key) {
            let decrypted_value = manager
                .decrypt_value(value)
                .with_context(|| format!("Failed to decrypt {}", key))?;
            
            if make {
                // GNU Make format: VAR := value (no quotes - they'd be literal)
                // Values with spaces/special chars are escaped but not quoted
                // This is intended for use with 'export' and accessing as $$VAR in recipes
                let prefix = if export { "export " } else { "" };
                let escaped_value = escape_for_make(&decrypted_value);
                println!("{}{} := {}", prefix, key, escaped_value);
            } else {
                // --export implies --bash (bash-compliant escaping)
                let use_bash_mode = bash || export;
                let prefix = if export { "export " } else { "" };
                
                if use_bash_mode {
                    // Bash-compliant mode: strict escaping
                    if needs_bash_quoting(&decrypted_value) {
                        println!("{}{}=\"{}\"", prefix, key, escape_for_bash_double_quotes(&decrypted_value));
                    } else {
                        println!("{}{}={}", prefix, key, decrypted_value);
                    }
                } else {
                    // Simple .env format: minimal escaping
                    if needs_simple_quoting(&decrypted_value) {
                        println!("{}{}=\"{}\"", prefix, key, escape_for_simple_quotes(&decrypted_value));
                    } else {
                        println!("{}{}={}", prefix, key, decrypted_value);
                    }
                }
            }
        }
    }
    Ok(())
}

/// Checks if a value needs simple quoting (for .env format)
fn needs_simple_quoting(value: &str) -> bool {
    if value.is_empty() {
        return true;
    }
    
    // Simple check for basic .env format
    value.contains(char::is_whitespace)
        || value.contains('=')
        || value.contains('"')
        || value.contains('\'')
}

/// Escapes a string for use inside simple double quotes (.env format)
fn escape_for_simple_quotes(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Checks if a value needs to be quoted for bash safety
fn needs_bash_quoting(value: &str) -> bool {
    if value.is_empty() {
        return true;
    }
    
    // Bash special characters that require quoting
    const SPECIAL_CHARS: &[char] = &[
        ' ', '\t', '\n', '\r',  // Whitespace
        '$', '`', '\\',          // Expansion/escaping
        '"', '\'',               // Quotes
        '&', '|', ';',           // Command separators
        '<', '>',                // Redirection
        '(', ')', '{', '}',      // Grouping
        '[', ']',                // Globbing
        '*', '?',                // Wildcards
        '!',                     // History expansion (in interactive shells)
        '~',                     // Tilde expansion
        '#',                     // Comments
        '=',                     // Assignment (problematic in some contexts)
    ];
    
    value.chars().any(|c| SPECIAL_CHARS.contains(&c))
}

/// Escapes a string for use inside bash double quotes
fn escape_for_bash_double_quotes(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            // Characters that need escaping inside double quotes
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '$' => result.push_str("\\$"),
            '`' => result.push_str("\\`"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            // Exclamation mark can trigger history expansion in interactive bash
            // but it's generally safe in scripts and with 'set +H'
            // We'll escape it to be extra safe
            '!' => result.push_str("\\!"),
            _ => result.push(c),
        }
    }
    result
}

/// Escapes a string for use in GNU Make variable assignment (without quotes)
/// 
/// The value will be stored in a Make variable, exported to the environment,
/// and accessed as $$VAR in shell recipes. We need to escape for Make's
/// processing during the include and variable expansion.
/// 
/// Key insight: When a Make variable is exported and accessed as $$VAR in a recipe,
/// the value passes through:
/// 1. include/assignment: $$ becomes $ in the variable value
/// 2. export: the variable value is set in the environment  
/// 3. recipe: $$VAR expands to the environment variable value
/// 
/// So we use $$ to get a literal $ in the final environment variable.
fn escape_for_make(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            // Use $$ to get literal $ in the environment variable
            '$' => result.push_str("$$"),
            // Hash starts a comment in Make - escape it
            '#' => result.push_str("\\#"),
            // Backslash needs escaping
            '\\' => result.push_str("\\\\"),
            // Spaces and other chars are fine in Make variable values
            _ => result.push(c),
        }
    }
    result
}
