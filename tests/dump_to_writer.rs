//! Integration tests for the dump-to-writer functionality.

use std::fs;

use dotenvage::SecretManager;
use dotenvage::loader::EnvLoader;
use tempfile::TempDir;

#[test]
fn test_dump_to_writer_basic() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Create test .env files
    fs::write(temp_path.join(".env"), "VAR1=value1\nVAR2=value2").unwrap();
    fs::write(temp_path.join(".env.local"), "VAR3=value3\nVAR1=overridden").unwrap();

    let manager = SecretManager::generate().unwrap();
    let loader = EnvLoader::with_manager(manager);

    let mut buffer = Vec::new();
    loader
        .dump_to_writer_from_dir(temp_path, &mut buffer)
        .unwrap();
    let output = String::from_utf8(buffer).unwrap();

    // VAR1 should be overridden by .env.local
    assert!(output.contains("VAR1=overridden"));
    assert!(output.contains("VAR2=value2"));
    assert!(output.contains("VAR3=value3"));

    // Variables should be sorted
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "VAR1=overridden");
    assert_eq!(lines[1], "VAR2=value2");
    assert_eq!(lines[2], "VAR3=value3");
}

#[test]
fn test_dump_to_writer_filters_age_keys() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Create test .env file with AGE keys
    fs::write(
        temp_path.join(".env"),
        "VAR1=value1\nDOTENVAGE_AGE_KEY=secret_key\nAGE_KEY=another_key\nVAR2=value2\nEKG_AGE_KEY=ekg_key\nAGE_KEY_NAME=mykey\nPROJECT_AGE_KEY_NAME=project_key",
    )
    .unwrap();

    let manager = SecretManager::generate().unwrap();
    let loader = EnvLoader::with_manager(manager);

    let mut buffer = Vec::new();
    loader
        .dump_to_writer_from_dir(temp_path, &mut buffer)
        .unwrap();
    let output = String::from_utf8(buffer).unwrap();

    // Regular variables should be present
    assert!(
        output.contains("VAR1=value1"),
        "Output should contain VAR1=value1"
    );
    assert!(
        output.contains("VAR2=value2"),
        "Output should contain VAR2=value2"
    );

    // AGE key variables should be filtered out
    assert!(
        !output.contains("DOTENVAGE_AGE_KEY"),
        "Should not contain DOTENVAGE_AGE_KEY"
    );
    assert!(!output.contains("AGE_KEY="), "Should not contain AGE_KEY=");
    assert!(
        !output.contains("EKG_AGE_KEY"),
        "Should not contain EKG_AGE_KEY"
    );
    assert!(
        !output.contains("AGE_KEY_NAME"),
        "Should not contain AGE_KEY_NAME"
    );
    assert!(
        !output.contains("PROJECT_AGE_KEY_NAME"),
        "Should not contain PROJECT_AGE_KEY_NAME"
    );
}

#[test]
fn test_dump_to_writer_handles_encrypted_values() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let manager = SecretManager::generate().unwrap();
    let encrypted = manager.encrypt_value("secret_value").unwrap();

    // Create test .env file with encrypted value
    fs::write(
        temp_path.join(".env"),
        format!("VAR1=plain\nVAR2={}", encrypted),
    )
    .unwrap();

    let loader = EnvLoader::with_manager(manager);

    let mut buffer = Vec::new();
    loader
        .dump_to_writer_from_dir(temp_path, &mut buffer)
        .unwrap();
    let output = String::from_utf8(buffer).unwrap();

    // Should contain decrypted value, not encrypted
    assert!(output.contains("VAR1=plain"));
    assert!(output.contains("VAR2=secret_value"));
    assert!(!output.contains("age:"));
}

#[test]
fn test_dump_to_writer_quotes_special_values() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Create test .env file with values that need quoting
    fs::write(
        temp_path.join(".env"),
        "VAR1=simple\nVAR2=has spaces\nVAR3=has\"quotes\nVAR4=\nVAR5=has=equals",
    )
    .unwrap();

    let manager = SecretManager::generate().unwrap();
    let loader = EnvLoader::with_manager(manager);

    let mut buffer = Vec::new();
    loader
        .dump_to_writer_from_dir(temp_path, &mut buffer)
        .unwrap();
    let output = String::from_utf8(buffer).unwrap();

    // Simple value should not be quoted
    assert!(output.contains("VAR1=simple"));

    // Values with special characters should be quoted
    assert!(output.contains("VAR2=\"has spaces\""));
    assert!(output.contains("VAR3=\"has\\\"quotes\""));
    assert!(output.contains("VAR4=\"\""));
    assert!(output.contains("VAR5=\"has=equals\""));
}
