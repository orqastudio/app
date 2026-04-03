// Vitest configuration for OrqaDev tests.
// All test files are plain TypeScript — no Svelte compilation needed.
// jsdom provides localStorage and other browser APIs for store logic tests.
import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		environment: "jsdom",
		include: ["__tests__/**/*.test.ts"],
	},
});
