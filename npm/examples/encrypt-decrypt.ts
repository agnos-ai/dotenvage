/**
 * TypeScript example: Encryption and decryption
 */

import * as dotenvage from "../index.js";

// Generate a new key pair
const manager = dotenvage.JsSecretManagerGenerate();

// Get the public key (can be shared)
const publicKey: string = manager.publicKeyString();
console.log(`Public key: ${publicKey}`);

// Encrypt a secret value
const secret = "sk_live_abc123xyz";
const encrypted: string = manager.encryptValue(secret);
console.log(`\nEncrypted: ${encrypted}`);

// Decrypt it back
const decrypted: string = manager.decryptValue(encrypted);
console.log(`Decrypted: ${decrypted}`);
console.log(`Match: ${secret === decrypted ? "✅" : "❌"}`);

// Check if a value is encrypted
const isEncrypted: boolean = manager.isEncrypted(encrypted);
console.log(`\nIs encrypted: ${isEncrypted}`);

// Pass through unencrypted values
const plaintext = "not-encrypted";
const result: string = manager.decryptValue(plaintext);
console.log(`Plaintext passthrough: ${result}`);

// Type-safe wrapper function
function encryptSecret(
  manager: dotenvage.JsSecretManager,
  value: string
): string {
  return manager.encryptValue(value);
}

function decryptSecret(
  manager: dotenvage.JsSecretManager,
  value: string
): string {
  return manager.decryptValue(value);
}

// Usage with type safety
const apiToken = "my-api-token-123";
const encryptedToken = encryptSecret(manager, apiToken);
const decryptedToken = decryptSecret(manager, encryptedToken);

console.log(`\nType-safe encryption:`);
console.log(`Original: ${apiToken}`);
console.log(`Encrypted: ${encryptedToken.substring(0, 30)}...`);
console.log(`Decrypted: ${decryptedToken}`);
