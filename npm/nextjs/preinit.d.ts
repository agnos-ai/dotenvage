/**
 * Pre-initialization environment variable loader for Next.js
 *
 * This module loads encrypted .env files BEFORE Next.js loads its own .env files.
 * It must be loaded before Next.js starts (via -r flag, require hook, or wrapper script).
 *
 * @module @dotenvage/node/nextjs/preinit
 */

// This module has side effects (loads environment variables)
// TypeScript declaration for importing the module
declare const _default: void
export default _default
