/**
 * Basic tests for dotenvage Node.js bindings
 * Run with: npm test or node --test
 */

const { test } = require("node:test");
const assert = require("node:assert");

// Try to load the module (will fail if not built)
let dotenvage;
try {
  dotenvage = require("../index.js");
} catch (error) {
  console.warn(
    "Warning: Native bindings not available. Run 'npm run build' first."
  );
  console.warn("Skipping tests.");
  process.exit(0);
}

test("SecretManager.generate() creates a new manager", () => {
  const manager = dotenvage.JsSecretManagerGenerate();
  assert(manager, "Manager should be created");
  const publicKey = manager.publicKeyString();
  assert(
    publicKey.startsWith("age1"),
    "Public key should start with 'age1'"
  );
});

test("SecretManager encrypt/decrypt round-trip", () => {
  const manager = dotenvage.JsSecretManagerGenerate();
  const plaintext = "my-secret-value-12345";

  const encrypted = manager.encryptValue(plaintext);
  assert(encrypted.startsWith("ENC[AGE:b64:"), "Should be encrypted");

  const decrypted = manager.decryptValue(encrypted);
  assert.strictEqual(
    decrypted,
    plaintext,
    "Decrypted should match original"
  );
});

test("SecretManager.isEncrypted() detects encrypted values", () => {
  const manager = dotenvage.JsSecretManagerGenerate();
  const plaintext = "my-secret-value";
  const encrypted = manager.encryptValue(plaintext);

  assert.strictEqual(manager.isEncrypted(encrypted), true);
  assert.strictEqual(manager.isEncrypted(plaintext), false);
});

test("shouldEncrypt() detects keys that should be encrypted", () => {
  assert.strictEqual(dotenvage.shouldEncrypt("API_KEY"), true);
  assert.strictEqual(dotenvage.shouldEncrypt("FLY_API_TOKEN"), true);
  assert.strictEqual(dotenvage.shouldEncrypt("SECRET"), true);
  assert.strictEqual(dotenvage.shouldEncrypt("NODE_ENV"), false);
  assert.strictEqual(dotenvage.shouldEncrypt("PORT"), false);
  assert.strictEqual(dotenvage.shouldEncrypt("DATABASE_URL"), true); // URL can contain secrets
});

test("EnvLoader.new() creates a loader", () => {
  try {
    const loader = dotenvage.JsEnvLoaderNew();
    assert(loader, "Loader should be created");
  } catch (error) {
    // This will fail if no key is available, which is expected in CI
    assert(
      error.message.includes("key") || error.message.includes("AGE"),
      "Should fail with key-related error if no key available"
    );
  }
});

test("EnvLoader.resolveEnvPaths() returns array of paths", () => {
  try {
    const loader = dotenvage.JsEnvLoaderNew();
    const paths = loader.resolveEnvPaths(".");
    assert(Array.isArray(paths), "Should return an array");
    assert(paths.length > 0, "Should have at least .env path");
    assert(
      paths.includes(".env") || paths.some((p) => p.includes(".env")),
      "Should include .env"
    );
  } catch (error) {
    // This will fail if no key is available
    assert(
      error.message.includes("key") || error.message.includes("AGE"),
      "Should fail with key-related error if no key available"
    );
  }
});

// Integration test (requires valid key and .env files)
test(
  "EnvLoader.load() loads variables into process.env",
  { skip: true },
  () => {
    // This test is skipped by default as it requires:
    // 1. A valid encryption key
    // 2. .env files in the test directory
    // To run: Set DOTENVAGE_AGE_KEY and create test .env files
    const loader = dotenvage.JsEnvLoaderNew();
    loader.load();
    const names = loader.getAllVariableNames();
    assert(
      Array.isArray(names),
      "Should return array of variable names"
    );
  }
);
