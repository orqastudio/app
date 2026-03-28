/**
 * Base vitest configuration for OrqaStudio packages.
 *
 * Provides shared coverage thresholds, reporters, and include/exclude patterns.
 * Consumers merge this into their own vitest config.
 *
 * ```ts
 * // vitest.config.ts in a consumer package
 * import { mergeConfig, defineConfig } from "vitest/config";
 * import { baseVitestConfig } from "@orqastudio/test-config/config";
 *
 * export default mergeConfig(baseVitestConfig, defineConfig({ ... }));
 * ```
 */
import { defineConfig } from "vitest/config";

export const baseVitestConfig = defineConfig({
	test: {
		coverage: {
			provider: "v8",
			reporter: ["text", "text-summary"],
			include: ["src/**/*.{ts,svelte}"],
			exclude: ["**/*.test.ts", "**/node_modules/**"],
			thresholds: {
				lines: 80,
				functions: 80,
				branches: 80,
				statements: 80,
			},
		},
	},
});
