use std::fs;

use dotenvage::{
    EnvLoader,
    SecretManager,
};
use tempfile::TempDir;

#[test]
fn test_get_all_variable_names() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Create a test key
    let manager = SecretManager::generate().unwrap();

    // Create test .env files
    fs::write(
        temp_path.join(".env"),
        "DATABASE_URL=postgres://localhost\nPORT=3000\n",
    )
    .unwrap();

    fs::write(
        temp_path.join(".env.local"),
        "API_KEY=secret123\nDEBUG=true\n",
    )
    .unwrap();

    // Create loader and get all variable names
    let loader = EnvLoader::with_manager(manager);
    let variable_names = loader.get_all_variable_names_from_dir(temp_path).unwrap();

    // Verify all variables are collected
    assert_eq!(variable_names.len(), 4);
    assert!(variable_names.contains(&"DATABASE_URL".to_string()));
    assert!(variable_names.contains(&"PORT".to_string()));
    assert!(variable_names.contains(&"API_KEY".to_string()));
    assert!(variable_names.contains(&"DEBUG".to_string()));
}

#[test]
fn test_get_all_variable_names_with_duplicates() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let manager = SecretManager::generate().unwrap();

    // Create test .env files with duplicate variable names
    fs::write(temp_path.join(".env"), "DATABASE_URL=base\nPORT=3000\n").unwrap();

    fs::write(
        temp_path.join(".env.local"),
        "DATABASE_URL=override\nAPI_KEY=secret\n",
    )
    .unwrap();

    let loader = EnvLoader::with_manager(manager);
    let variable_names = loader.get_all_variable_names_from_dir(temp_path).unwrap();

    // Verify duplicates are removed
    assert_eq!(variable_names.len(), 3);
    assert!(variable_names.contains(&"DATABASE_URL".to_string()));
    assert!(variable_names.contains(&"PORT".to_string()));
    assert!(variable_names.contains(&"API_KEY".to_string()));

    // DATABASE_URL should only appear once despite being in both files
    assert_eq!(
        variable_names
            .iter()
            .filter(|&name| name == "DATABASE_URL")
            .count(),
        1
    );
}

#[test]
fn test_get_all_variable_names_with_encrypted_values() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let manager = SecretManager::generate().unwrap();

    // Create an encrypted value
    let encrypted_secret = manager.encrypt_value("my-secret-token").unwrap();

    // Create test .env file with encrypted value
    fs::write(
        temp_path.join(".env"),
        format!("PUBLIC_VAR=not-secret\nSECRET_TOKEN={}\n", encrypted_secret),
    )
    .unwrap();

    let loader = EnvLoader::with_manager(manager);
    let variable_names = loader.get_all_variable_names_from_dir(temp_path).unwrap();

    // Verify encrypted variables are included in the names
    assert_eq!(variable_names.len(), 2);
    assert!(variable_names.contains(&"PUBLIC_VAR".to_string()));
    assert!(variable_names.contains(&"SECRET_TOKEN".to_string()));
}

#[test]
fn test_get_all_variable_names_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let manager = SecretManager::generate().unwrap();
    let loader = EnvLoader::with_manager(manager);
    let variable_names = loader.get_all_variable_names_from_dir(temp_path).unwrap();

    // Should return empty vector for directory with no .env files
    assert_eq!(variable_names.len(), 0);
}
