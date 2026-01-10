#!/usr/bin/env node
/**
 * Wrapper script for Next.js that loads encrypted environment variables
 * before Next.js starts.
 *
 * IMPORTANT: This works because @next/env does NOT overwrite existing process.env values.
 * Flow:
 * 1. This script loads encrypted .env files and decrypts them into process.env
 * 2. Next.js starts and @next/env runs to load .env files
 * 3. @next/env sees existing values in process.env and preserves them (doesn't overwrite)
 * 4. Result: Decrypted values from dotenvage are used, not encrypted values from .env files
 *
 * Usage:
 *   node @dotenvage/node/nextjs/wrapper.mjs dev
 *   node @dotenvage/node/nextjs/wrapper.mjs build
 *
 * Or via package.json scripts:
 *   "dev": "node @dotenvage/node/nextjs/wrapper.mjs dev"
 *   "build": "node @dotenvage/node/nextjs/wrapper.mjs build"
 */

// Load encrypted environment variables BEFORE Next.js starts
try {
  // Load env vars first (synchronously via side-effect)
  await import("./preinit.mjs");

  // Now import Node.js modules
  const childProcess = await import("child_process");
  const fsModule = await import("fs/promises");
  const pathModule = await import("path");

  // Detect package manager by checking for lock files
  const cwd = process.cwd();
  const packageManager = await (async () => {
    try {
      const pnpmLock = await fsModule
        .access(pathModule.join(cwd, "pnpm-lock.yaml"))
        .then(() => true)
        .catch(() => false);
      if (pnpmLock) return "pnpm";

      const yarnLock = await fsModule
        .access(pathModule.join(cwd, "yarn.lock"))
        .then(() => true)
        .catch(() => false);
      if (yarnLock) return "yarn";

      const packageLock = await fsModule
        .access(pathModule.join(cwd, "package-lock.json"))
        .then(() => true)
        .catch(() => false);
      if (packageLock) return "npm";

      // Default to npm if no lock file found
      return "npm";
    } catch {
      return "npm";
    }
  })();

  // Forward all command-line arguments to Next.js
  const args = process.argv.slice(2);

  // Use package manager exec to ensure we use the correct Next.js binary
  // This works with all package managers and handles path resolution automatically
  const child = childProcess.spawn(
    packageManager,
    ["exec", "next", ...args],
    {
      stdio: "inherit",
      cwd: cwd,
      env: process.env, // Pass through all environment variables (including decrypted ones)
    }
  );

  child.on("exit", (code) => {
    process.exit(code || 0);
  });

  child.on("error", (error) => {
    console.error("Failed to start Next.js:", error);
    process.exit(1);
  });
} catch (error) {
  console.error("Failed to load environment variables:", error);
  process.exit(1);
}
