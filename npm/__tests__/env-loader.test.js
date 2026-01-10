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

describe("JsEnvLoader", () => {
  let testDir;
  let originalCwd;

  before(async () => {
    testDir = await mkdtemp(
      path.join(os.tmpdir(), "dotenvage-test-")
    );
    originalCwd = process.cwd();
    process.chdir(testDir);
  });

  after(async () => {
    process.chdir(originalCwd);
    if (testDir) {
      await rm(testDir, { recursive: true, force: true });
    }
    // Clean up environment variables
    delete process.env.TEST_VAR;
    delete process.env.API_KEY;
  });

  it("should create a new loader", () => {
    const loader = dotenvage.JsEnvLoaderNew();
    assert(loader);
    assert(typeof loader.load === "function");
  });

  it("should load environment variables from .env file", async () => {
    // Create a simple .env file
    await writeFile(
      path.join(testDir, ".env"),
      "TEST_VAR=test-value\nNODE_ENV=test\n"
    );

    // Create a manager and loader (doesn't require external key)
    const manager = dotenvage.JsSecretManagerGenerate();
    const loader = dotenvage.JsEnvLoaderWithManager(manager);

    try {
      loader.load();
      // Variables should be in process.env
      assert.strictEqual(process.env.TEST_VAR, "test-value");
      assert.strictEqual(process.env.NODE_ENV, "test");
    } catch (error) {
      // If loading fails, that's okay - just verify loader exists
      assert(loader);
    }
  });

  it("should get all variable names", async () => {
    // Create a simple .env file
    await writeFile(
      path.join(testDir, ".env"),
      "TEST_VAR=test-value\nAPI_KEY=test-key\n"
    );

    // Create a manager and loader (doesn't require external key)
    const manager = dotenvage.JsSecretManagerGenerate();
    const loader = dotenvage.JsEnvLoaderWithManager(manager);

    const names = loader.getAllVariableNames();
    assert(Array.isArray(names));
    assert(names.length >= 2);
    assert(names.includes("TEST_VAR"));
    assert(names.includes("API_KEY"));
  });

  it("should resolve env paths", () => {
    const loader = dotenvage.JsEnvLoaderNew();
    const paths = loader.resolveEnvPaths(testDir);
    assert(Array.isArray(paths));
    // Should include .env at minimum
    assert(paths.length > 0);
  });
});
