/**
 * Vite config for building plugin views.
 *
 * Plugin views are pre-bundled as ES modules that the app loads at runtime.
 * Shared dependencies (SDK, components, Svelte) are resolved from
 * window.__orqa — not bundled into the plugin.
 *
 * Output: dist/views/{viewKey}.js
 */

import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { resolve } from "path";

export default defineConfig({
	plugins: [svelte()],
	build: {
		lib: {
			entry: {
				roadmap: resolve(__dirname, "src/views/roadmap.ts"),
			},
			formats: ["es"],
			fileName: (_, entryName) => `views/${entryName}.js`,
		},
		outDir: "dist",
		rollupOptions: {
			external: [
				"@orqastudio/sdk",
				"@orqastudio/svelte-components/pure",
				"@orqastudio/svelte-components/connected",
				"@orqastudio/graph-visualiser",
				"@orqastudio/types",
				"svelte",
				"svelte/internal",
			],
			output: {
				globals: {
					"@orqastudio/sdk": "window.__orqa.sdk",
					"@orqastudio/svelte-components/pure": "window.__orqa.components",
					"@orqastudio/svelte-components/connected": "window.__orqa.componentsConnected",
					"@orqastudio/graph-visualiser": "window.__orqa.graphVisualiser",
					"svelte": "window.__orqa.svelte",
				},
			},
		},
	},
});
