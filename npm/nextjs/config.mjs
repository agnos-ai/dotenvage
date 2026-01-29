/**
 * Next.js configuration wrapper for dotenvage integration.
 *
 * Wrapping your Next.js config with `withDotenvage()` ensures that
 * encrypted .env files are decrypted and loaded into `process.env`
 * when `next.config.mjs` is evaluated (at build time and dev startup).
 *
 * Usage in next.config.mjs:
 *   import { withDotenvage } from '@dotenvage/node/nextjs/config'
 *
 *   const nextConfig = { ... }
 *   export default withDotenvage(nextConfig)
 *
 * If the `dotenvage-next` wrapper script already loaded env vars,
 * `loadEnv()` is a no-op (idempotent).
 */
import { loadEnv } from "./loader.mjs";

/**
 * Wraps a Next.js configuration object.
 * Loads encrypted .env files via dotenvage before returning the config.
 *
 * @param {import('next').NextConfig | ((phase: string) => import('next').NextConfig)} config - Next.js config object or function
 * @returns {import('next').NextConfig | ((phase: string) => import('next').NextConfig)} The same config, after loading env vars
 */
export function withDotenvage(config) {
  loadEnv();
  return config;
}

/**
 * Wraps a Next.js configuration function.
 * Loads encrypted .env files via dotenvage before returning the function.
 *
 * @param {(phase: string, defaultConfig: import('next').NextConfig) => import('next').NextConfig} configFn - Next.js config function
 * @returns {(phase: string, defaultConfig: import('next').NextConfig) => import('next').NextConfig} The same function, after loading env vars
 */
export function withDotenvageConfig(configFn) {
  loadEnv();
  return configFn;
}
