/**
 * Next.js environment variable loader using dotenvage
 *
 * @module @dotenvage/node/nextjs
 */

/**
 * Loads environment variables from encrypted .env files
 * This runs automatically when this module is imported
 * Always loads from dotenvage in all environments (local, Vercel CI, production)
 */
export function loadEnv(): void

/**
 * Get all environment variables (for testing/debugging)
 */
export function getAllEnvVars(): Record<string, string>

/**
 * Get variable names (for testing/debugging)
 */
export function getEnvVarNames(): string[]
