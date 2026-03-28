/**
 * Svelte-specific vitest configuration for OrqaStudio packages.
 *
 * Extends the base config with jsdom environment for component testing.
 * Consumers must install @sveltejs/vite-plugin-svelte and add the svelte
 * plugin in their own config merge.
 *
 * ```ts
 * // vitest.config.ts in a Svelte consumer package
 * import { mergeConfig, defineConfig } from "vitest/config";
 * import { svelte } from "@sveltejs/vite-plugin-svelte";
 * import { svelteVitestConfig } from "@orqastudio/test-config/config";
 *
 * export default mergeConfig(svelteVitestConfig, defineConfig({
 *   plugins: [svelte()],
 * }));
 * ```
 */
import { mergeConfig, defineConfig } from "vitest/config";
import { baseVitestConfig } from "./vitest.base.js";

export const svelteVitestConfig = mergeConfig(
	baseVitestConfig,
	defineConfig({
		test: {
			environment: "jsdom",
		},
	}),
);
