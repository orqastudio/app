import { base } from "@orqastudio/plugin-typescript/eslint";
import tseslint from "typescript-eslint";
import type { ConfigArray, ConfigWithExtends } from "typescript-eslint";

/**
 * Re-export the TypeScript base config for convenience.
 */
export { base } from "@orqastudio/plugin-typescript/eslint";

/**
 * Svelte 5 ESLint config for OrqaStudio projects.
 *
 * Extends the TypeScript base config with Svelte-specific rules.
 * Requires `eslint-plugin-svelte` as a peer dependency in the consuming project.
 *
 * Includes:
 * - All base TypeScript rules (from @orqastudio/plugin-typescript)
 * - Svelte flat/recommended rules
 * - TypeScript parser for .svelte files
 * - no-explicit-any enforced in .svelte and .svelte.ts files
 *
 * Usage:
 * ```js
 * import { svelte } from "@orqastudio/plugin-svelte/eslint";
 * import sveltePlugin from "eslint-plugin-svelte";
 * export default [...svelte(sveltePlugin)];
 * ```
 */
export function svelte(
  sveltePlugin: {
    configs: Record<string, ConfigWithExtends[]>;
  },
): ConfigArray {
  return tseslint.config(
    ...base,
    ...sveltePlugin.configs["flat/recommended"],
    {
      files: ["**/*.svelte", "**/*.svelte.ts"],
      languageOptions: {
        parserOptions: {
          parser: tseslint.parser,
        },
      },
      rules: {
        "@typescript-eslint/no-explicit-any": "error",
      },
    },
    {
      files: ["**/*.test.ts", "**/*.test.js", "**/__tests__/**"],
      rules: {
        "@typescript-eslint/no-unused-vars": "off",
      },
    },
  );
}
