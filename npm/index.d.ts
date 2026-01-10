/* tslint:disable */
/* eslint-disable */

export interface JsSecretManager {
  /**
   * Gets the public key as a string in age format (starts with `age1`)
   */
  publicKeyString(): string;

  /**
   * Encrypts a plaintext value and wraps it in the format `ENC[AGE:b64:...]`
   */
  encryptValue(plaintext: string): string;

  /**
   * Decrypts a value if it's encrypted; otherwise returns it unchanged
   */
  decryptValue(value: string): string;

  /**
   * Checks if a value is in a recognized encrypted format
   */
  isEncrypted(value: string): boolean;
}

export interface JsEnvLoader {
  /**
   * Loads `.env` files from the current directory in standard order
   * Decrypted values are loaded into the process environment
   */
  load(): void;

  /**
   * Loads `.env` files from a specific directory using the same order
   */
  loadFromDir(dir: string): void;

  /**
   * Gets all variable names from all loaded `.env` files
   */
  getAllVariableNames(): string[];

  /**
   * Gets all variable names from `.env` files in a specific directory
   */
  getAllVariableNamesFromDir(dir: string): string[];

  /**
   * Computes the ordered list of env file paths to load
   */
  resolveEnvPaths(dir: string): string[];

  /**
   * Gets all environment variables as a map (decrypted)
   * Note: This loads variables into the process environment first
   */
  getAllVariables(): Record<string, string>;

  /**
   * Gets all environment variables from a specific directory as a map (decrypted)
   * Note: This loads variables into the process environment first
   */
  getAllVariablesFromDir(dir: string): Record<string, string>;
}

/**
 * Creates a new SecretManager by loading the key from standard locations
 */
export function JsSecretManagerNew(): JsSecretManager;

/**
 * Generates a new random identity
 */
export function JsSecretManagerGenerate(): JsSecretManager;

/**
 * Creates a SecretManager from an existing identity string
 */
export function JsSecretManagerFromIdentityString(identity: string): JsSecretManager;

/**
 * Creates a new EnvLoader with a default SecretManager
 */
export function JsEnvLoaderNew(): JsEnvLoader;

/**
 * Creates an EnvLoader with a specific SecretManager
 */
export function JsEnvLoaderWithManager(manager: JsSecretManager): JsEnvLoader;

/**
 * Checks if a key name should be encrypted based on auto-detection patterns
 */
export function shouldEncrypt(key: string): boolean;

/**
 * Module initialization - exported but does nothing
 */
export function init(): void;
