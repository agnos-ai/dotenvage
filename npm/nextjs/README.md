# Next.js Integration with dotenvage

This directory contains utilities for integrating dotenvage with
Next.js applications, ensuring that encrypted environment variables
are properly loaded before Next.js processes them.

## The Problem

Next.js loads environment variables from `.env*` files using
`@next/env` **before** your `next.config.mjs` runs. If your `.env`
files contain encrypted values (like `ENC[AGE:...]`), Next.js will try
to use those encrypted values directly, which won't work.

Additionally, for `NEXT_PUBLIC_*` variables that need to be available
in Edge Runtime (middleware), Next.js must inline them at build time.
If the values are encrypted when Next.js reads them, they'll be
inlined as encrypted strings, which is useless.

## The Solution

We leverage the fact that `@next/env` **does not overwrite** existing
`process.env` values. By loading and decrypting environment variables
**before** Next.js starts, we ensure:

1. Encrypted values are decrypted and set in `process.env` first
2. When `@next/env` runs, it sees existing values and preserves them
3. Decrypted values are available for Next.js to inline into the build

## Quick Start

**For Edge Runtime (middleware) support (recommended):**

Use the `dotenvage-next` bin script to start Next.js:

```json
{
  "scripts": {
    "dev": "dotenvage-next dev",
    "build": "dotenvage-next build"
  }
}
```

And wrap your config:

```javascript
import { withDotenvage } from "@dotenvage/node/nextjs/config";

export default withDotenvage({
  // Your Next.js config
});
```

**For server-side only (no Edge Runtime):**

Use `loadEnv()` in your config file:

```javascript
import { loadEnv } from "@dotenvage/node/nextjs";

loadEnv();

export default {
  // Your Next.js config
};
```

## Files

### `config.mjs`

A configuration wrapper for API consistency. This is mainly for
convenience when using the `dotenvage-next` wrapper script.

**Note:** For Edge Runtime (middleware) support, you MUST use the
`dotenvage-next` wrapper script. For server-side only (no Edge
Runtime), you can use `loadEnv()` directly (see `loader.mjs` below).

**Usage in `next.config.mjs`:**

```javascript
import { withDotenvage } from "@dotenvage/node/nextjs/config";
import nextMDX from "@next/mdx";

const nextConfig = {
  // Your Next.js config
};

const withMDX = nextMDX({
  // MDX options
});

export default withDotenvage(withMDX(nextConfig));
```

**Note:** When using `dotenvage-next` wrapper script, env vars are
already loaded, so this wrapper is just a pass-through for
consistency.

### `loader.mjs` (For server-side only, no Edge Runtime)

A standard loader for use in `next.config.mjs` when you don't use Edge
Runtime (middleware). Use this when you only need server-side code to
access encrypted environment variables.

**Important:** This works for server-side code (API routes, server
components) because `loadEnv()` runs in `next.config.mjs` and can
decrypt values before server-side code accesses them. However, it does
NOT work for Edge Runtime (middleware) because Next.js inlines
`NEXT_PUBLIC_*` variables at build time, BEFORE `next.config.mjs`
runs.

For Edge Runtime support, you MUST use the `dotenvage-next` wrapper
script.

**Usage in `next.config.mjs` (server-side only):**

```javascript
import { loadEnv } from "@dotenvage/node/nextjs";

// Load and decrypt environment variables
loadEnv();

export default {
  // Your Next.js config
};
```

**Note:** When using `loadEnv()` in the config file, `@next/env` will
have already loaded encrypted values, but `loadEnv()` overwrites them
with decrypted values, so server-side code sees the decrypted values.

### `nextjs-preinit.mjs`

A pre-initialization loader that must run **before** Next.js starts.
This is critical for ensuring `NEXT_PUBLIC_*` variables are available
in Edge Runtime.

**Usage options:**

1. **Via Node.js `-r` flag:**

```json
{
  "scripts": {
    "dev": "node -r @dotenvage/node/nextjs/preinit node_modules/.bin/next dev",
    "build": "node -r @dotenvage/node/nextjs/preinit node_modules/.bin/next build"
  }
}
```

1. **Via wrapper script (recommended):**

See `wrapper.mjs` below.

### `nextjs-wrapper.mjs`

A wrapper script that loads environment variables and then starts
Next.js. This is the easiest way to ensure proper loading order.

**The wrapper automatically:**

- Detects your package manager (pnpm/npm/yarn) by checking for lock
  files
- Uses the correct package manager's `exec` command to run Next.js
- Handles path resolution automatically for all package manager setups
- Works reliably in monorepos and pnpm workspaces

**Usage - Update your `package.json`:**

```json
{
  "scripts": {
    "dev": "dotenvage-next dev",
    "build": "dotenvage-next build"
  }
}
```

That's it! No need to create local wrapper scripts - the
`dotenvage-next` bin script handles everything automatically. The
`dotenvage-next` command is installed when you install
`@dotenvage/node`.

## Complete Integration Example

1. **Install dotenvage:**

```bash
npm install @dotenvage/node
```

2. **Encrypt your `.env` files:**

```bash
npx dotenvage encrypt .env.local
```

3. **Set your encryption key as an environment variable:**

```bash
export EKG_AGE_KEY=your-key-name
# or
export DOTENVAGE_AGE_KEY="your-age-key-string"
# or
export AGE_KEY="your-age-key-string"
```

4. **Create a wrapper script** (see `wrapper.mjs` example above, or
   use the one in this directory)

5. **Update your `next.config.mjs`** (optional, for additional
   safety):

```javascript
import { loadEnv } from "@dotenvage/node/nextjs";

// This is a backup - the preinit loader should have already loaded
// but this ensures variables are available if preinit wasn't used
loadEnv();

export default {
  // Your Next.js config
  env: {
    // Explicitly expose critical variables to ensure they're inlined
    NEXT_PUBLIC_CLERK_PUBLISHABLE_KEY:
      process.env.NEXT_PUBLIC_CLERK_PUBLISHABLE_KEY || "",
  },
};
```

## How It Works

1. **Pre-init loader runs first** → Decrypts `.env` files → Sets
   values in `process.env`
2. **Next.js starts** → `@next/env` runs → Sees existing values →
   Doesn't overwrite
3. **Next.js builds** → Inlines `NEXT_PUBLIC_*` variables → Uses
   decrypted values ✅
4. **Edge Runtime** → Has access to decrypted `NEXT_PUBLIC_*`
   variables ✅

## Troubleshooting

### Variables still encrypted in Edge Runtime

- Make sure you're using the pre-init loader (not just the standard
  loader)
- Verify the wrapper script is being used in your `package.json`
  scripts
- Check that the encryption key is set correctly

### Variables not loading

- Check that your encryption key environment variable is set
- Verify your `.env` files contain encrypted values (starting with
  `ENC[AGE:`)
- Check console output for error messages from the loader

### Build fails

- In production/Vercel, ensure the encryption key is set as an
  environment variable
- The pre-init loader will fail hard in production if it can't decrypt
  files
- Check Vercel environment variables section in your project settings

## See Also

- [dotenvage main documentation](../../README.md)
- [Next.js environment variables documentation](https://nextjs.org/docs/app/building-your-application/configuring/environment-variables)
