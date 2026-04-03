// Vitest configuration for @orqastudio/svelte-components.
// Uses jsdom so browser DOM APIs are available without a real browser.
// The svelteTesting plugin from @testing-library/svelte/vite ensures
// Svelte 5 resolves to its browser bundle rather than the SSR build.
import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { svelteTesting } from "@testing-library/svelte/vite";

export default defineConfig({
	plugins: [svelte({ hot: false }), svelteTesting({ autoCleanup: true })],
	test: {
		environment: "jsdom",
		include: ["__tests__/**/*.test.ts"],
		globals: false,
		setupFiles: ["__tests__/setup.ts"],
	},
	ssr: {
		// Ensure jest-dom is processed by Vite rather than treated as external,
		// so the vitest.js entrypoint can import 'vitest' correctly.
		noExternal: ["@testing-library/jest-dom"],
	},
});
