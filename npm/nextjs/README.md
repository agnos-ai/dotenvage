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

## Files

### `config.mjs` (Recommended)

A configuration wrapper that automatically loads encrypted environment
variables. This is the simplest way to integrate dotenvage with
Next.js.

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

**Note:** For full Edge Runtime (middleware) support where
`NEXT_PUBLIC_*` variables need to be inlined, you still need to use
`NODE_OPTIONS` (see below). The config wrapper handles server-side
code automatically.

### `nextjs-loader.mjs`

A standard loader for use in `next.config.mjs` or other Node.js
runtime contexts. Use this when you don't need to pre-load before
Next.js starts.

**Usage in `next.config.mjs`:**

```javascript
import { loadEnv } from "@dotenvage/node/nextjs";

// Load environment variables early
loadEnv();

export default {
  // Your Next.js config
};
```

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

**Create a local wrapper in your project:**

Create `scripts/next-with-env.mjs` in your project:

```javascript
#!/usr/bin/env node
// Load encrypted env vars before Next.js starts
await import("@dotenvage/node/nextjs/preinit");

// Import and start Next.js
const { spawn } = await import("child_process");
const { resolve } = await import("path");

const nextBin = resolve(process.cwd(), "node_modules/.bin/next");
const args = process.argv.slice(2);

const child = spawn("node", [nextBin, ...args], {
  stdio: "inherit",
  cwd: process.cwd(),
  env: process.env,
});

child.on("exit", (code) => process.exit(code || 0));
child.on("error", (error) => {
  console.error("Failed to start Next.js:", error);
  process.exit(1);
});
```

Then update your `package.json`:

```json
{
  "scripts": {
    "dev": "node scripts/next-with-env.mjs dev",
    "build": "node scripts/next-with-env.mjs build"
  }
}
```

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
