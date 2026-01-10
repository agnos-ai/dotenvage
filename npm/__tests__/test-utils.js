/**
 * Test utilities for dotenvage tests
 */

const { mkdtemp, writeFile, rm } = require("fs/promises");
const path = require("path");
const os = require("os");
const fs = require("fs");

/**
 * Creates a temporary test directory
 */
async function createTestDir(prefix = "dotenvage-test-") {
  return await mkdtemp(path.join(os.tmpdir(), prefix));
}

/**
 * Cleans up a test directory
 */
async function cleanupTestDir(testDir) {
  if (testDir && fs.existsSync(testDir)) {
    await rm(testDir, { recursive: true, force: true });
  }
}

/**
 * Creates a .env file with the given content
 */
async function createEnvFile(testDir, filename, content) {
  const filePath = path.join(testDir, filename);
  await writeFile(filePath, content);
  return filePath;
}

/**
 * Creates a test encryption key manager
 */
function createTestManager(dotenvage) {
  return dotenvage.JsSecretManager.generate();
}

/**
 * Creates a test loader with a manager
 */
function createTestLoader(dotenvage, manager) {
  return dotenvage.JsEnvLoader.withManager(manager);
}

/**
 * Sets up a test environment with .env files and encryption key
 */
async function setupTestEnvironment(
  dotenvage,
  testDir,
  envFiles = {}
) {
  const manager = createTestManager(dotenvage);

  // Create .env files
  for (const [filename, content] of Object.entries(envFiles)) {
    await createEnvFile(testDir, filename, content);
  }

  return { manager, loader: createTestLoader(dotenvage, manager) };
}

/**
 * Cleans up process.env by removing specified keys
 */
function cleanupEnv(keys) {
  keys.forEach((key) => {
    delete process.env[key];
  });
}

module.exports = {
  createTestDir,
  cleanupTestDir,
  createEnvFile,
  createTestManager,
  createTestLoader,
  setupTestEnvironment,
  cleanupEnv,
};
