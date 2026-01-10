#!/usr/bin/env node
/**
 * Next.js wrapper for dotenvage - loads encrypted environment variables before Next.js starts.
 *
 * This bin script allows users to call dotenvage-next directly without specifying the full path.
 *
 * Usage:
 *   pnpm exec dotenvage-next dev
 *   pnpm exec dotenvage-next build
 *
 * Or via package.json scripts:
 *   "dev": "dotenvage-next dev"
 *   "build": "dotenvage-next build"
 */

// Import and execute the wrapper script
// The wrapper.mjs file uses top-level await and has side effects, so importing it will execute it
import("../nextjs/wrapper.mjs");
