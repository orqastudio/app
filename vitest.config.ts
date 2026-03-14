import { mergeConfig, defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { svelteVitestConfig } from "@orqastudio/test-config/config";

export default mergeConfig(
	svelteVitestConfig,
	defineConfig({
		plugins: [svelte()],
		test: {
			include: ["__tests__/**/*.test.ts"],
		},
	}),
);
