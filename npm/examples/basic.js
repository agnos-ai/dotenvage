/**
 * Basic example of using dotenvage in Node.js
 *
 * This is the most common pattern: load encrypted .env files into process.env
 * (similar to how dotenv works with require('dotenv').config())
 */

const dotenvage = require("../index.js");

// Example 1: Load environment files (MOST COMMON - like dotenv.config())
// This loads and decrypts .env files, setting variables into process.env
// Yes, this is how dotenv packages work - they mutate process.env
console.log(
  "=== Example 1: Load Environment Files (Most Common) ==="
);
try {
  const loader = dotenvage.JsEnvLoaderNew();
  loader.load(); // Loads into process.env (mutates environment, like dotenv)

  // Now variables are available in process.env
  console.log("Loaded variables into process.env");
  const variableNames = loader.getAllVariableNames();
  console.log("Total variables:", variableNames.length);

  if (variableNames.length > 0) {
    console.log(
      "Sample variables:",
      variableNames.slice(0, 5).join(", ")
    );
    // Example: access a variable (if it exists)
    const sampleKey = variableNames[0];
    if (process.env[sampleKey]) {
      console.log(
        `Example: process.env.${sampleKey} = ${process.env[
          sampleKey
        ].substring(0, 20)}...`
      );
    }
  }
} catch (error) {
  console.log(
    "Note: Environment loading requires a valid encryption key."
  );
  console.log(
    "Set DOTENVAGE_AGE_KEY, AGE_KEY, or EKG_AGE_KEY environment variable."
  );
  console.log("Error:", error.message);
}

// Example 1b: Load without mutating process.env (alternative pattern)
// Get variables as an object instead of setting process.env
console.log(
  "\n=== Example 1b: Get Variables as Object (Non-mutating) ==="
);
try {
  const loader = dotenvage.JsEnvLoaderNew();
  const variables = loader.getAllVariables(); // Returns object without modifying process.env
  const keys = Object.keys(variables);
  console.log(
    "Got",
    keys.length,
    "variables as object (not in process.env)"
  );
  if (keys.length > 0) {
    const sampleKey = keys[0];
    console.log(
      `Example: variables.${sampleKey} = ${variables[
        sampleKey
      ].substring(0, 20)}...`
    );
  }
} catch (error) {
  console.log(
    "Note: Requires valid encryption key. Error:",
    error.message
  );
}

// Example 2: Generate a new encryption key
console.log("\n=== Example 2: Generate Key ===");
try {
  const manager = dotenvage.JsSecretManagerGenerate();
  const publicKey = manager.publicKeyString();
  console.log("Generated public key:", publicKey);
  console.log("Save this key - you'll need it to decrypt values!");
  console.log(
    'Set it as: export DOTENVAGE_AGE_KEY="<private-key-string>"'
  );
} catch (error) {
  console.error("Error generating key:", error.message);
  console.log(
    'Note: Make sure you have built the native bindings with "npm run build"'
  );
}

// Example 3: Encrypt and decrypt a value
console.log("\n=== Example 3: Encrypt/Decrypt ===");
try {
  const manager = dotenvage.JsSecretManagerGenerate();
  const secret = "my-super-secret-api-key-12345";

  const encrypted = manager.encryptValue(secret);
  console.log("Original:", secret);
  console.log("Encrypted:", encrypted);

  const decrypted = manager.decryptValue(encrypted);
  console.log("Decrypted:", decrypted);
  console.log("Match:", secret === decrypted ? "✅" : "❌");
} catch (error) {
  console.error("Error:", error.message);
}

// Example 4: Check if key should be encrypted (auto-detection patterns)
console.log("\n=== Example 4: Auto-detection Patterns ===");
const keys = [
  "API_KEY",
  "FLY_API_TOKEN",
  "NODE_ENV",
  "PORT",
  "DATABASE_URL",
];
for (const key of keys) {
  const shouldEncrypt = dotenvage.shouldEncrypt(key);
  console.log(
    `${key}: ${shouldEncrypt ? "🔒 Should encrypt" : "✅ Plain text"}`
  );
}
