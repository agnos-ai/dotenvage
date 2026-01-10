/**
 * TypeScript example: Loading environment variables
 *
 * This is the most common pattern - loading encrypted .env files
 * into process.env (like dotenv.config())
 */

import * as dotenvage from "../index.js";

// Pattern 1: Load into process.env (most common, mutates environment)
// This matches how dotenv works - variables become available in process.env
const loader = dotenvage.JsEnvLoaderNew();
loader.load(); // Sets variables in process.env

// Now access via process.env
const apiKey = process.env.API_KEY;
const databaseUrl = process.env.DATABASE_URL;

console.log("Loaded variables into process.env");
console.log(`API_KEY exists: ${!!apiKey}`);
console.log(`DATABASE_URL exists: ${!!databaseUrl}`);

// Pattern 2: Get as object (non-mutating, functional style)
// Use this when you don't want to modify process.env
const loader2 = dotenvage.JsEnvLoaderNew();
const env = loader2.getAllVariables(); // Returns Record<string, string>

// Type-safe access
type EnvConfig = {
  API_KEY?: string;
  DATABASE_URL?: string;
  PORT?: string;
};

const config: EnvConfig = env as EnvConfig;
console.log("\nGot variables as object:");
console.log(`API_KEY: ${config.API_KEY ? "✅" : "❌"}`);
console.log(`DATABASE_URL: ${config.DATABASE_URL ? "✅" : "❌"}`);
console.log(`PORT: ${config.PORT || "default (8080)"}`);

// Pattern 3: Load from specific directory
const loader3 = dotenvage.JsEnvLoaderNew();
loader3.loadFromDir("./config"); // Load .env files from ./config directory

// Pattern 4: Get variable names only (without loading into environment)
const loader4 = dotenvage.JsEnvLoaderNew();
const variableNames: string[] = loader4.getAllVariableNames();
console.log(`\nFound ${variableNames.length} variables in .env files:`);
variableNames.slice(0, 5).forEach((name) => {
  console.log(`  - ${name}`);
});
