/**
 * Next.js configuration wrapper that automatically loads encrypted environment variables
 *
 * @module @dotenvage/node/nextjs/config
 */

import type { NextConfig } from 'next'

/**
 * Wraps a Next.js configuration object to ensure dotenvage is loaded.
 * Environment variables are automatically loaded when this module is imported.
 *
 * @param config - Next.js config object or function
 * @returns The same config (pass-through wrapper)
 */
export function withDotenvage(
  config: NextConfig | ((phase: string) => NextConfig),
): NextConfig | ((phase: string) => NextConfig)

/**
 * Wraps a Next.js configuration function to ensure dotenvage is loaded.
 * Useful when you need to access the build phase.
 *
 * @param configFn - Next.js config function
 * @returns The same function (pass-through wrapper)
 */
export function withDotenvageConfig(
  configFn: (
    phase: string,
    defaultConfig: NextConfig,
  ) => NextConfig | Promise<NextConfig>,
): (phase: string, defaultConfig: NextConfig) => NextConfig | Promise<NextConfig>
