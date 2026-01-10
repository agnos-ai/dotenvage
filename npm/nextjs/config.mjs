/**
 * Next.js configuration wrapper that automatically loads encrypted environment variables.
 *
 * This wrapper loads env vars when imported (before the config is exported), ensuring
 * encrypted .env files are decrypted and available.
 *
 * IMPORTANT: Next.js loads env vars via @next/env BEFORE next.config.mjs runs. For
 * full Edge Runtime (middleware) support where NEXT_PUBLIC_* variables need to be
 * inlined, you still need NODE_OPTIONS. However, this wrapper handles server-side
 * code automatically.
 *
 * Usage in next.config.mjs:
 *   import { withDotenvage } from '@dotenvage/node/nextjs/config'
 *   import nextMDX from '@next/mdx'
 *
 *   const nextConfig = {
 *     // your config
 *   }
 *
 *   export default withDotenvage(nextConfig)
 *
 * For Edge Runtime, also set NODE_OPTIONS in package.json:
 *   "dev": "NODE_OPTIONS='-r @dotenvage/node/nextjs/preinit' pnpm exec next dev"
 */

// Load environment variables synchronously at module evaluation time
// This runs when this module is imported, before the config is exported
import "./preinit.mjs";

/**
 * Wraps a Next.js configuration object to ensure dotenvage is loaded.
 * Since we're already loading env vars at module evaluation time (via the import above),
 * this is mainly for convenience and API consistency.
 *
 * @param {import('next').NextConfig | ((phase: string) => import('next').NextConfig)} config - Next.js config object or function
 * @returns {import('next').NextConfig | ((phase: string) => import('next').NextConfig)} The same config (pass-through)
 */
export function withDotenvage(config) {
  // Environment variables are already loaded via the import above
  // Just return the config as-is (pass-through wrapper)
  return config;
}

/**
 * Wraps a Next.js configuration function to ensure dotenvage is loaded.
 * Useful when you need to access the build phase.
 *
 * @param {(phase: string, defaultConfig: import('next').NextConfig) => import('next').NextConfig} configFn - Next.js config function
 * @returns {(phase: string, defaultConfig: import('next').NextConfig) => import('next').NextConfig} The same function (pass-through)
 */
export function withDotenvageConfig(configFn) {
  // Environment variables are already loaded via the import above
  // Just return the function as-is (pass-through wrapper)
  return configFn;
}
