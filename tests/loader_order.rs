use std::env;

use serial_test::serial;

fn names(paths: Vec<std::path::PathBuf>) -> Vec<String> {
    paths
        .into_iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect()
}

fn clear_env(keys: &[&str]) {
    for k in keys {
        unsafe { env::remove_var(k) };
    }
}

#[test]
#[serial]
fn order_no_env_uses_local() {
    clear_env(&[
        "DOTENVAGE_ENV",
        "EKG_ENV",
        "DOTENVAGE_ARCH",
        "EKG_ARCH",
        "DOTENVAGE_USER",
        "EKG_USER",
        "GITHUB_EVENT_NAME",
        "GITHUB_REF",
        "PR_NUMBER",
        "USER",
        "USERNAME",
    ]);
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join(".env"), "").unwrap();
    std::fs::write(tmp.path().join(".env.local"), "").unwrap();
    let loader = dotenvage::EnvLoader::with_manager(dotenvage::SecretManager::generate().unwrap());
    let got = loader.resolve_env_paths(tmp.path());
    let got = names(got);
    assert_eq!(got, vec![".env", ".env.local"]);
}

#[test]
#[serial]
fn order_env_prod() {
    clear_env(&[
        "DOTENVAGE_ENV",
        "EKG_ENV",
        "DOTENVAGE_ARCH",
        "EKG_ARCH",
        "DOTENVAGE_USER",
        "EKG_USER",
        "GITHUB_EVENT_NAME",
        "GITHUB_REF",
        "PR_NUMBER",
        "USER",
        "USERNAME",
    ]);
    unsafe {
        env::set_var("DOTENVAGE_ENV", "prod");
    }
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join(".env"), "").unwrap();
    std::fs::write(tmp.path().join(".env.prod"), "").unwrap();
    let loader = dotenvage::EnvLoader::with_manager(dotenvage::SecretManager::generate().unwrap());
    let got = names(loader.resolve_env_paths(tmp.path()));
    assert_eq!(got, vec![".env", ".env.prod"]);
}

#[test]
#[serial]
fn order_env_arch() {
    clear_env(&[
        "DOTENVAGE_ENV",
        "EKG_ENV",
        "DOTENVAGE_ARCH",
        "EKG_ARCH",
        "DOTENVAGE_USER",
        "EKG_USER",
        "GITHUB_EVENT_NAME",
        "GITHUB_REF",
        "PR_NUMBER",
        "USER",
        "USERNAME",
    ]);
    unsafe {
        env::set_var("EKG_ENV", "prod");
    }
    unsafe {
        env::set_var("DOTENVAGE_ARCH", "docker-s3");
    }
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join(".env"), "").unwrap();
    std::fs::write(tmp.path().join(".env.prod"), "").unwrap();
    std::fs::write(tmp.path().join(".env.prod-docker-s3"), "").unwrap();
    let loader = dotenvage::EnvLoader::with_manager(dotenvage::SecretManager::generate().unwrap());
    let got = names(loader.resolve_env_paths(tmp.path()));
    assert_eq!(got, vec![".env", ".env.prod", ".env.prod-docker-s3"]);
}

#[test]
#[serial]
fn order_user_and_arch() {
    clear_env(&[
        "DOTENVAGE_ENV",
        "EKG_ENV",
        "DOTENVAGE_ARCH",
        "EKG_ARCH",
        "DOTENVAGE_USER",
        "EKG_USER",
        "GITHUB_EVENT_NAME",
        "GITHUB_REF",
        "PR_NUMBER",
        "USER",
        "USERNAME",
    ]);
    unsafe {
        env::set_var("DOTENVAGE_ENV", "prod");
    }
    unsafe {
        env::set_var("DOTENVAGE_ARCH", "docker-s3");
    }
    unsafe {
        env::set_var("DOTENVAGE_USER", "alice");
    }
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join(".env"), "").unwrap();
    std::fs::write(tmp.path().join(".env.prod"), "").unwrap();
    std::fs::write(tmp.path().join(".env.prod-alice"), "").unwrap();
    std::fs::write(tmp.path().join(".env.prod-docker-s3-alice"), "").unwrap();
    let loader = dotenvage::EnvLoader::with_manager(dotenvage::SecretManager::generate().unwrap());
    let got = names(loader.resolve_env_paths(tmp.path()));
    assert_eq!(
        got,
        vec![
            ".env",
            ".env.prod",
            ".env.prod-alice",
            ".env.prod-docker-s3-alice"
        ]
    );
}

#[test]
#[serial]
fn order_github_pr_number() {
    clear_env(&[
        "DOTENVAGE_ENV",
        "EKG_ENV",
        "DOTENVAGE_ARCH",
        "EKG_ARCH",
        "DOTENVAGE_USER",
        "EKG_USER",
        "GITHUB_EVENT_NAME",
        "GITHUB_REF",
        "PR_NUMBER",
        "USER",
        "USERNAME",
    ]);
    unsafe {
        env::set_var("GITHUB_EVENT_NAME", "pull_request");
    }
    unsafe {
        env::set_var("PR_NUMBER", "123");
    }
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join(".env"), "").unwrap();
    std::fs::write(tmp.path().join(".env.pr-123"), "").unwrap();
    let loader = dotenvage::EnvLoader::with_manager(dotenvage::SecretManager::generate().unwrap());
    let got = names(loader.resolve_env_paths(tmp.path()));
    assert_eq!(got, vec![".env", ".env.pr-123"]);
}

#[test]
#[serial]
fn order_github_ref_parsing() {
    clear_env(&[
        "DOTENVAGE_ENV",
        "EKG_ENV",
        "DOTENVAGE_ARCH",
        "EKG_ARCH",
        "DOTENVAGE_USER",
        "EKG_USER",
        "GITHUB_EVENT_NAME",
        "GITHUB_REF",
        "PR_NUMBER",
        "USER",
        "USERNAME",
    ]);
    unsafe {
        env::set_var("GITHUB_REF", "refs/pull/456/merge");
    }
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join(".env"), "").unwrap();
    std::fs::write(tmp.path().join(".env.pr-456"), "").unwrap();
    let loader = dotenvage::EnvLoader::with_manager(dotenvage::SecretManager::generate().unwrap());
    let got = names(loader.resolve_env_paths(tmp.path()));
    assert_eq!(got, vec![".env", ".env.pr-456"]);
}

#[test]
#[serial]
fn case_insensitive_and_separators() {
    clear_env(&[
        "DOTENVAGE_ENV",
        "EKG_ENV",
        "DOTENVAGE_ARCH",
        "EKG_ARCH",
        "DOTENVAGE_USER",
        "EKG_USER",
        "GITHUB_EVENT_NAME",
        "GITHUB_REF",
        "PR_NUMBER",
        "USER",
        "USERNAME",
    ]);
    unsafe {
        env::set_var("DOTENVAGE_ENV", "PrOd");
    }
    unsafe {
        env::set_var("DOTENVAGE_ARCH", "DoCkEr-S3");
    }
    unsafe {
        env::set_var("DOTENVAGE_USER", "AlIcE");
    }
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join(".ENV"), "").unwrap();
    std::fs::write(tmp.path().join(".ENV.PROD"), "").unwrap();
    std::fs::write(tmp.path().join(".ENV.PROD-DOCKER-S3-ALICE"), "").unwrap();
    let loader = dotenvage::EnvLoader::with_manager(dotenvage::SecretManager::generate().unwrap());
    let got = names(loader.resolve_env_paths(tmp.path()));
    assert_eq!(got, vec![".ENV", ".ENV.PROD", ".ENV.PROD-DOCKER-S3-ALICE"]);
}
