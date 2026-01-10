/**
 * TypeScript example: Next.js integration
 *
 * Shows how to integrate dotenvage with Next.js for
 * server-side configuration.
 */

import * as dotenvage from "../index.js";

// Load environment variables (do this early in your app)
// In Next.js, you'd typically do this in next.config.js or in your API routes
const loader = dotenvage.JsEnvLoaderNew();
loader.load();

// Type-safe Next.js environment configuration
interface NextjsEnv {
  readonly NEXT_PUBLIC_APP_URL: string;
  readonly DATABASE_URL: string;
  readonly API_KEY: string;
  readonly NEXT_PUBLIC_API_URL?: string;
  readonly NODE_ENV: "development" | "production" | "test";
}

// Get environment variables with type safety
function getNextjsEnv(): NextjsEnv {
  const required = (key: keyof NextjsEnv): string => {
    const value = process.env[key];
    if (!value) {
      throw new Error(`Missing required environment variable: ${key}`);
    }
    return value;
  };

  const optional = (key: string): string | undefined => {
    return process.env[key];
  };

  return {
    NEXT_PUBLIC_APP_URL: required("NEXT_PUBLIC_APP_URL"),
    DATABASE_URL: required("DATABASE_URL"),
    API_KEY: required("API_KEY"),
    NEXT_PUBLIC_API_URL: optional("NEXT_PUBLIC_API_URL"),
    NODE_ENV: (process.env.NODE_ENV || "development") as NextjsEnv["NODE_ENV"],
  };
}

// Example: Next.js API route handler
export function createApiHandler<T extends Record<string, unknown>>(
  handler: (env: NextjsEnv, req: T) => Promise<Response>
) {
  return async (req: T): Promise<Response> => {
    try {
      const env = getNextjsEnv();
      return await handler(env, req);
    } catch (error) {
      console.error("API error:", error);
      return new Response(
        JSON.stringify({ error: "Internal server error" }),
        {
          status: 500,
          headers: { "Content-Type": "application/json" },
        }
      );
    }
  };
}

// Example: Next.js API route
async function exampleApiRoute(
  env: NextjsEnv,
  req: { method: string }
): Promise<Response> {
  if (req.method !== "GET") {
    return new Response("Method not allowed", { status: 405 });
  }

  return new Response(
    JSON.stringify({
      message: "API is working",
      environment: env.NODE_ENV,
      // Don't expose secrets in API responses!
      hasApiKey: !!env.API_KEY,
    }),
    {
      status: 200,
      headers: { "Content-Type": "application/json" },
    }
  );
}

// Export the handler (Next.js pattern)
export const handler = createApiHandler(exampleApiRoute);

// Example: Server-side component data fetching
export async function getServerSideData() {
  const env = getNextjsEnv();

  // Use the decrypted environment variables
  const response = await fetch(`${env.NEXT_PUBLIC_API_URL}/api/data`, {
    headers: {
      Authorization: `Bearer ${env.API_KEY}`,
    },
  });

  if (!response.ok) {
    throw new Error("Failed to fetch data");
  }

  return response.json();
}

// Example: Environment validation at startup
export function validateNextjsEnv(): void {
  try {
    const env = getNextjsEnv();
    console.log("✅ All required environment variables are set");
    console.log(`   Environment: ${env.NODE_ENV}`);
    console.log(`   App URL: ${env.NEXT_PUBLIC_APP_URL}`);
  } catch (error) {
    if (error instanceof Error) {
      console.error(`❌ Environment validation failed: ${error.message}`);
    }
    process.exit(1);
  }
}

// Run validation if this is the main module
if (import.meta.url === `file://${process.argv[1]}`) {
  validateNextjsEnv();
}

export { getNextjsEnv, type NextjsEnv };
