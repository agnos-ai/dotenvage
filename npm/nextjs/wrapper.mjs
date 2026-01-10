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
 * Or create a local wrapper that imports the preinit module:
 *   await import('@dotenvage/node/nextjs/preinit')
 */

// Load encrypted environment variables BEFORE Next.js starts
// Using dynamic import since this is an ES module project
try {
  // Load env vars first (synchronously via side-effect)
  await import("./preinit.mjs");

  // Now import Node.js modules
  const childProcess = await import("child_process");
  const pathModule = await import("path");
  const { fileURLToPath } = await import("url");
  const { dirname } = await import("path");

  const __filename = fileURLToPath(import.meta.url);
  const __dirname = dirname(__filename);

  // Get the next binary path (assuming node_modules is at project root)
  // This script should be in node_modules/@dotenvage/node/nextjs/
  // So we go up: ../../.. to get to project root, then node_modules/.bin/next
  const projectRoot = pathModule.resolve(__dirname, "../../../..");
  const nextBin = pathModule.resolve(
    projectRoot,
    "node_modules/.bin/next"
  );

  // Forward all command-line arguments to Next.js
  const args = process.argv.slice(2);
  const child = childProcess.spawn("node", [nextBin, ...args], {
    stdio: "inherit",
    cwd: process.cwd(),
    env: process.env, // Pass through all environment variables (including decrypted ones)
  });

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
