/**
 * TypeScript example: Type-safe application configuration
 *
 * This shows how to use dotenvage with TypeScript for type-safe
 * configuration management.
 */

import * as dotenvage from "../index.js";

// Define your application configuration schema
interface AppConfig {
  readonly apiKey: string;
  readonly databaseUrl: string;
  readonly port: number;
  readonly nodeEnv: "development" | "production" | "test";
  readonly logLevel: "debug" | "info" | "warn" | "error";
}

// Type-safe config loader
class ConfigLoader {
  private loader: dotenvage.JsEnvLoader;

  constructor() {
    this.loader = dotenvage.JsEnvLoaderNew();
    this.loader.load(); // Load into process.env
  }

  /**
   * Load and validate configuration
   */
  loadConfig(): AppConfig {
    const raw = this.loader.getAllVariables();

    // Validate and transform
    const apiKey = this.getRequired("API_KEY", raw);
    const databaseUrl = this.getRequired("DATABASE_URL", raw);
    const port = this.getNumber("PORT", raw, 8080);
    const nodeEnv = this.getEnum(
      "NODE_ENV",
      raw,
      ["development", "production", "test"],
      "development"
    ) as AppConfig["nodeEnv"];
    const logLevel = this.getEnum(
      "LOG_LEVEL",
      raw,
      ["debug", "info", "warn", "error"],
      "info"
    ) as AppConfig["logLevel"];

    return {
      apiKey,
      databaseUrl,
      port,
      nodeEnv,
      logLevel,
    };
  }

  private getRequired(key: string, env: Record<string, string>): string {
    const value = env[key] || process.env[key];
    if (!value) {
      throw new Error(`Missing required environment variable: ${key}`);
    }
    return value;
  }

  private getNumber(
    key: string,
    env: Record<string, string>,
    defaultValue: number
  ): number {
    const value = env[key] || process.env[key];
    if (!value) {
      return defaultValue;
    }
    const parsed = parseInt(value, 10);
    if (isNaN(parsed)) {
      console.warn(`Invalid number for ${key}, using default: ${defaultValue}`);
      return defaultValue;
    }
    return parsed;
  }

  private getEnum<T extends string>(
    key: string,
    env: Record<string, string>,
    validValues: readonly T[],
    defaultValue: T
  ): T {
    const value = (env[key] || process.env[key] || defaultValue).toLowerCase();
    if (validValues.includes(value as T)) {
      return value as T;
    }
    console.warn(
      `Invalid value for ${key}: ${value}, using default: ${defaultValue}`
    );
    return defaultValue;
  }
}

// Usage in your application
function main() {
  try {
    const loader = new ConfigLoader();
    const config: AppConfig = loader.loadConfig();

    console.log("Application Configuration:");
    console.log(`  API Key: ${config.apiKey.substring(0, 10)}...`);
    console.log(`  Database: ${config.databaseUrl}`);
    console.log(`  Port: ${config.port}`);
    console.log(`  Environment: ${config.nodeEnv}`);
    console.log(`  Log Level: ${config.logLevel}`);
  } catch (error) {
    if (error instanceof Error) {
      console.error(`Configuration error: ${error.message}`);
    } else {
      console.error("Unknown configuration error");
    }
    process.exit(1);
  }
}

// Run if this is the main module
if (import.meta.url === `file://${process.argv[1]}`) {
  main();
}

export { ConfigLoader, type AppConfig };
