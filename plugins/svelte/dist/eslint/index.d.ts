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
export declare function svelte(sveltePlugin: {
    configs: Record<string, ConfigWithExtends[]>;
}): ConfigArray;
//# sourceMappingURL=index.d.ts.map