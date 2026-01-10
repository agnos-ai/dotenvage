/**
 * TypeScript example: Auto-detection patterns
 *
 * Shows how to use pattern matching to determine which
 * environment variables should be encrypted.
 */

import * as dotenvage from "../index.js";

// Test various key names
const testKeys = [
  "API_KEY",
  "FLY_API_TOKEN",
  "SECRET_TOKEN",
  "DATABASE_PASSWORD",
  "AWS_SECRET_ACCESS_KEY",
  "STRIPE_SECRET_KEY",
  "NODE_ENV",
  "PORT",
  "DATABASE_URL",
  "LOG_LEVEL",
  "PUBLIC_API_URL",
] as const;

// Type-safe function to check if a key should be encrypted
function shouldEncryptKey(key: string): boolean {
  return dotenvage.shouldEncrypt(key);
}

// Group keys by whether they should be encrypted
const encryptedKeys: string[] = [];
const plainKeys: string[] = [];

testKeys.forEach((key) => {
  if (shouldEncryptKey(key)) {
    encryptedKeys.push(key);
  } else {
    plainKeys.push(key);
  }
});

console.log("Auto-detection Results:");
console.log(`\n🔒 Should encrypt (${encryptedKeys.length}):`);
encryptedKeys.forEach((key) => {
  console.log(`  - ${key}`);
});

console.log(`\n✅ Plain text (${plainKeys.length}):`);
plainKeys.forEach((key) => {
  console.log(`  - ${key}`);
});

// Example: Type-safe helper to encrypt secrets automatically
class SecretManager {
  private manager: dotenvage.JsSecretManager;

  constructor() {
    this.manager = dotenvage.JsSecretManagerNew();
  }

  /**
   * Set an environment variable, auto-encrypting if the key matches patterns
   */
  setEnvVar(key: string, value: string): string {
    if (dotenvage.shouldEncrypt(key)) {
      // Auto-encrypt sensitive keys
      const encrypted = this.manager.encryptValue(value);
      console.log(`🔒 Auto-encrypted ${key}`);
      return encrypted;
    } else {
      // Keep plain text for non-sensitive keys
      console.log(`✅ Kept ${key} as plain text`);
      return value;
    }
  }
}

// Usage example
const secretMgr = new SecretManager();
const apiKey = secretMgr.setEnvVar("API_KEY", "sk_live_abc123");
const port = secretMgr.setEnvVar("PORT", "8080");

console.log(`\nAPI_KEY (encrypted): ${apiKey.substring(0, 30)}...`);
console.log(`PORT (plain): ${port}`);

// Type-safe configuration with auto-encryption
type ConfigEntry<T> = {
  key: string;
  value: T;
  encrypt: boolean;
};

function createConfig<T>(
  entries: Array<{ key: string; value: T; encrypt?: boolean }>
): ConfigEntry<T>[] {
  const manager = dotenvage.JsSecretManagerGenerate();

  return entries.map((entry) => {
    const shouldEncrypt =
      entry.encrypt !== undefined
        ? entry.encrypt
        : dotenvage.shouldEncrypt(entry.key);

    let processedValue: T = entry.value;

    if (shouldEncrypt && typeof entry.value === "string") {
      processedValue = manager.encryptValue(entry.value) as T;
    }

    return {
      key: entry.key,
      value: processedValue,
      encrypt: shouldEncrypt,
    };
  });
}

const configEntries = createConfig([
  { key: "API_KEY", value: "sk_live_123" },
  { key: "PORT", value: "8080" },
  { key: "DATABASE_URL", value: "postgres://localhost/db" },
  { key: "SECRET_TOKEN", value: "secret-value" },
]);

console.log("\nConfig entries:");
configEntries.forEach((entry) => {
  const displayValue =
    entry.encrypt && typeof entry.value === "string"
      ? `${entry.value.substring(0, 30)}...`
      : entry.value;
  console.log(`  ${entry.key}: ${displayValue} ${entry.encrypt ? "🔒" : "✅"}`);
});
