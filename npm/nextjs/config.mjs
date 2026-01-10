/**
 * Next.js configuration wrapper for dotenvage integration.
 *
 * This wrapper can be used in two ways:
 *
 * 1. **With wrapper script (recommended for Edge Runtime):**
 *    Use `dotenvage-next` bin script to start Next.js:
 *      "dev": "dotenvage-next dev"
 *      "build": "dotenvage-next build"
 *    The wrapper script loads env vars BEFORE Next.js starts, ensuring
 *    they're available everywhere including Edge Runtime.
 *
 * 2. **Without wrapper script (server-side only, no Edge Runtime):**
 *    Use `loadEnv()` from `@dotenvage/node/nextjs` in your config file:
 *      import { loadEnv } from '@dotenvage/node/nextjs'
 *      loadEnv()
 *    Note: This only works for server-side code, not Edge Runtime/middleware.
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
 * IMPORTANT: When using `dotenvage-next` wrapper script, env vars are
 * already loaded, so this wrapper is just a pass-through for consistency.
 * When NOT using the wrapper script, use `loadEnv()` directly in your config.
 */

/**
 * Wraps a Next.js configuration object.
 * This is mainly for convenience and API consistency.
 * When using `dotenvage-next` wrapper script, env vars are already loaded.
 *
 * @param {import('next').NextConfig | ((phase: string) => import('next').NextConfig)} config - Next.js config object or function
 * @returns {import('next').NextConfig | ((phase: string) => import('next').NextConfig)} The same config (pass-through)
 */
export function withDotenvage(config) {
  // Pass-through wrapper for API consistency
  // When using `dotenvage-next` wrapper script, env vars are already loaded
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
  // Pass-through wrapper for API consistency
  // When using `dotenvage-next` wrapper script, env vars are already loaded
  return configFn;
}
