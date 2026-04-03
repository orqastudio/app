// Vitest configuration for @orqastudio/sdk.
// Uses jsdom so browser DOM APIs are available without a real browser.
// Svelte 5 runes compilation is handled by vite-plugin-svelte.
import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";

export default defineConfig({
	plugins: [svelte({ hot: false })],
	test: {
		environment: "jsdom",
		include: ["__tests__/**/*.test.ts"],
	},
});
