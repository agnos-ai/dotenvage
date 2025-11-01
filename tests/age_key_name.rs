//! Tests for AGE_KEY_NAME feature - project-specific key discovery

use std::env;
use std::fs;

use serial_test::serial;

fn clear_key_env() {
    unsafe {
        env::remove_var("AGE_KEY");
        env::remove_var("DOTENVAGE_AGE_KEY");
        env::remove_var("AGE_KEY_NAME");
        env::remove_var("XDG_STATE_HOME");
    }
}

#[test]
#[serial]
fn age_key_name_from_env() {
    clear_key_env();

    let tmpdir = tempfile::tempdir().unwrap();
    let state_dir = tmpdir.path().join("state");
    let key_dir = state_dir.join("myproject").join("myapp");
    fs::create_dir_all(&key_dir).unwrap();

    // Generate a key and save it
    let manager = dotenvage::SecretManager::generate().unwrap();
    let key_path = key_dir.join("test.key");
    manager.save_key(&key_path).unwrap();

    // Set up environment
    unsafe {
        env::set_var("XDG_STATE_HOME", state_dir.to_str().unwrap());
        env::set_var("AGE_KEY_NAME", "myproject/myapp/test");
    }

    // Load key should find it via AGE_KEY_NAME
    let loaded_manager = dotenvage::SecretManager::load_key().unwrap();

    // Verify it's the same key by encrypting/decrypting
    let plaintext = "test-secret-123";
    let encrypted = manager.encrypt_value(plaintext).unwrap();
    let decrypted = loaded_manager.decrypt_value(&encrypted).unwrap();
    assert_eq!(plaintext, decrypted);

    // Verify the key path is correct
    let expected_path = state_dir.join("myproject/myapp/test.key");
    assert_eq!(
        dotenvage::SecretManager::key_path_from_env_or_default(),
        expected_path
    );
}

#[test]
#[serial]
fn age_key_name_default_to_cargo_pkg_name() {
    clear_key_env();

    let tmpdir = tempfile::tempdir().unwrap();
    let state_dir = tmpdir.path().join("state");

    unsafe {
        env::set_var("XDG_STATE_HOME", state_dir.to_str().unwrap());
    }

    // Without AGE_KEY_NAME, should default to {CARGO_PKG_NAME}/dotenvage
    let path = dotenvage::SecretManager::key_path_from_env_or_default();
    let expected = state_dir.join("dotenvage").join("dotenvage.key");
    assert_eq!(path, expected);
}

#[test]
#[serial]
fn age_key_name_with_env_loader() {
    clear_key_env();

    let tmpdir = tempfile::tempdir().unwrap();
    let state_dir = tmpdir.path().join("state");
    let key_dir = state_dir.join("testapp");
    fs::create_dir_all(&key_dir).unwrap();

    // Generate a key and save it
    let manager = dotenvage::SecretManager::generate().unwrap();
    let key_path = key_dir.join("dotenvage.key");
    manager.save_key(&key_path).unwrap();

    // Create .env with AGE_KEY_NAME and encrypted secret
    let env_file = tmpdir.path().join(".env");
    let encrypted_value = manager.encrypt_value("supersecret").unwrap();
    fs::write(
        &env_file,
        format!("AGE_KEY_NAME=testapp\nAPI_KEY={}", encrypted_value),
    )
    .unwrap();

    // Set up environment
    unsafe {
        env::set_var("XDG_STATE_HOME", state_dir.to_str().unwrap());
    }

    // Load environment - should auto-discover key via AGE_KEY_NAME from .env
    let loader = dotenvage::EnvLoader::with_manager(manager);
    let vars = loader.load_env_file(&env_file).unwrap();

    assert_eq!(vars.get("AGE_KEY_NAME"), Some(&"testapp".to_string()));
    assert_eq!(vars.get("API_KEY"), Some(&"supersecret".to_string()));
}

#[test]
#[serial]
fn age_key_name_priority_order() {
    clear_key_env();

    let tmpdir = tempfile::tempdir().unwrap();
    let state_dir = tmpdir.path().join("state");

    // Generate two different keys
    let manager1 = dotenvage::SecretManager::generate().unwrap();
    let manager2 = dotenvage::SecretManager::generate().unwrap();

    let key1_dir = state_dir.join("project1");
    fs::create_dir_all(&key1_dir).unwrap();
    let key1_path = key1_dir.join("dotenvage.key");
    manager1.save_key(&key1_path).unwrap();

    // Priority 1: AGE_KEY env var should win
    let key1_content = fs::read_to_string(&key1_path).unwrap();
    unsafe {
        env::set_var("AGE_KEY", key1_content.trim());
        env::set_var("AGE_KEY_NAME", "project2"); // Should be ignored
        env::set_var("XDG_STATE_HOME", state_dir.to_str().unwrap());
    }

    let loaded = dotenvage::SecretManager::load_key().unwrap();
    let plaintext = "test-123";
    let encrypted = manager1.encrypt_value(plaintext).unwrap();
    let decrypted = loaded.decrypt_value(&encrypted).unwrap();
    assert_eq!(plaintext, decrypted);

    // Should NOT be able to decrypt with manager2
    assert!(
        loaded
            .decrypt_value(&manager2.encrypt_value(plaintext).unwrap())
            .is_err()
            || loaded
                .decrypt_value(&manager2.encrypt_value(plaintext).unwrap())
                .unwrap()
                != plaintext
    );
}

#[test]
#[serial]
fn age_key_name_empty_string_uses_default() {
    clear_key_env();

    let tmpdir = tempfile::tempdir().unwrap();
    let state_dir = tmpdir.path().join("state");

    unsafe {
        env::set_var("XDG_STATE_HOME", state_dir.to_str().unwrap());
        env::set_var("AGE_KEY_NAME", "   "); // Whitespace only
    }

    // Empty/whitespace AGE_KEY_NAME should fall back to default
    let path = dotenvage::SecretManager::key_path_from_env_or_default();
    let expected = state_dir.join("dotenvage").join("dotenvage.key");
    assert_eq!(path, expected);
}
