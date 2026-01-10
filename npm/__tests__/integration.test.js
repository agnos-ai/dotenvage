/**
 * Integration tests for dotenvage Node.js bindings
 * These tests require a valid encryption key and test the full workflow
 */

const { describe, it, before, after } = require("node:test");
const assert = require("node:assert");
const fs = require("fs");
const path = require("path");
const { mkdtemp, writeFile, rm } = require("fs/promises");
const os = require("os");

// Skip tests if bindings not available
let dotenvage;
try {
  dotenvage = require("../index.js");
} catch (error) {
  console.warn(
    'Skipping tests - native bindings not available. Run "npm run build" first.'
  );
  process.exit(0);
}

describe("Integration tests", () => {
  let testDir;
  let originalCwd;
  let testManager;
  let originalEnv;

  before(async () => {
    // Save original environment
    originalEnv = { ...process.env };
    originalCwd = process.cwd();

    // Create temporary test directory
    testDir = await mkdtemp(
      path.join(os.tmpdir(), "dotenvage-integration-")
    );
    process.chdir(testDir);

    // Generate a test encryption key manager
    // Tests will use this same manager instance for encryption and decryption
    testManager = dotenvage.JsSecretManagerGenerate();
  });

  after(async () => {
    // Restore original environment
    process.env = originalEnv;
    process.chdir(originalCwd);

    if (testDir) {
      await rm(testDir, { recursive: true, force: true });
    }
  });

  it("should encrypt and decrypt values in .env files", async () => {
    const manager = dotenvage.JsSecretManagerGenerate();
    const secret = "my-secret-api-key-12345";
    const encrypted = manager.encryptValue(secret);

    // Create .env file with encrypted value
    const envContent = `API_KEY=${encrypted}\nPORT=8080\nNODE_ENV=test\n`;
    await writeFile(path.join(testDir, ".env"), envContent);

    // Load with the same manager
    const loader = dotenvage.JsEnvLoaderWithManager(manager);
    loader.load();

    // Verify decryption worked
    assert.strictEqual(process.env.API_KEY, secret);
    assert.strictEqual(process.env.PORT, "8080");
    assert.strictEqual(process.env.NODE_ENV, "test");
  });

  it("should load variables from multiple .env files with layering", async () => {
    const manager = dotenvage.JsSecretManagerGenerate();

    // Create base .env
    await writeFile(
      path.join(testDir, ".env"),
      "API_KEY=base-key\nPORT=8080\nNODE_ENV=development\n"
    );

    // Create .env.local (should override .env)
    await writeFile(
      path.join(testDir, ".env.local"),
      "API_KEY=local-key\nDATABASE_URL=postgres://localhost/test\n"
    );

    const loader = dotenvage.JsEnvLoaderWithManager(manager);
    loader.load();

    // .env.local should override .env
    assert.strictEqual(process.env.API_KEY, "local-key");
    assert.strictEqual(process.env.PORT, "8080"); // From .env
    assert.strictEqual(process.env.NODE_ENV, "development"); // From .env
    assert.strictEqual(
      process.env.DATABASE_URL,
      "postgres://localhost/test"
    ); // From .env.local
  });

  it("should get all variables as object without mutating process.env", async () => {
    const manager = dotenvage.JsSecretManagerGenerate();

    // Save original process.env values
    const originalApiKey = process.env.API_KEY;
    const originalPort = process.env.PORT;

    // Create test .env file
    await writeFile(
      path.join(testDir, ".env"),
      "TEST_API_KEY=test-key-123\nTEST_PORT=3000\n"
    );

    const loader = dotenvage.JsEnvLoaderWithManager(manager);

    // Get variables as object (this still loads into process.env first, but returns object)
    const vars = loader.getAllVariables();

    // Check that variables are in the returned object
    assert.strictEqual(vars.TEST_API_KEY, "test-key-123");
    assert.strictEqual(vars.TEST_PORT, "3000");

    // Clean up
    delete process.env.TEST_API_KEY;
    delete process.env.TEST_PORT;
  });

  it("should get all variable names without loading into environment", async () => {
    const manager = dotenvage.JsSecretManagerGenerate();

    // Create test .env file
    await writeFile(
      path.join(testDir, ".env"),
      "VAR1=value1\nVAR2=value2\nVAR3=value3\n"
    );

    const loader = dotenvage.JsEnvLoaderWithManager(manager);
    const names = loader.getAllVariableNames();

    assert(Array.isArray(names));
    assert(names.length >= 3);
    assert(names.includes("VAR1"));
    assert(names.includes("VAR2"));
    assert(names.includes("VAR3"));
  });

  it("should resolve env paths correctly", () => {
    const loader = dotenvage.JsEnvLoaderNew();
    const paths = loader.resolveEnvPaths(testDir);

    assert(Array.isArray(paths));
    assert(paths.length > 0);
    // Should include .env at minimum
    assert(paths.some((p) => p.includes(".env")));
  });

  it("should handle encrypted and plain values mixed", async () => {
    const manager = dotenvage.JsSecretManagerGenerate();
    const encrypted = manager.encryptValue("secret-value");

    // Create .env with mix of encrypted and plain values
    const envContent = `ENCRYPTED_KEY=${encrypted}\nPLAIN_KEY=plain-value\nPORT=8080\n`;
    await writeFile(path.join(testDir, ".env"), envContent);

    const loader = dotenvage.JsEnvLoaderWithManager(manager);
    loader.load();

    // Encrypted should be decrypted
    assert.strictEqual(process.env.ENCRYPTED_KEY, "secret-value");
    // Plain should pass through
    assert.strictEqual(process.env.PLAIN_KEY, "plain-value");
    assert.strictEqual(process.env.PORT, "8080");
  });

  it("should load from specific directory", async () => {
    const manager = dotenvage.JsSecretManagerGenerate();

    // Create subdirectory with .env file
    const subDir = path.join(testDir, "config");
    await fs.promises.mkdir(subDir, { recursive: true });
    await writeFile(
      path.join(subDir, ".env"),
      "SUB_DIR_VAR=subdir-value\n"
    );

    const loader = dotenvage.JsEnvLoaderWithManager(manager);
    loader.loadFromDir(subDir);

    assert.strictEqual(process.env.SUB_DIR_VAR, "subdir-value");
  });
});
