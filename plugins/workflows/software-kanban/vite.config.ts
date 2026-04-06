/**
 * Vite config for building plugin views.
 *
 * Plugin views are pre-bundled as IIFE modules that the app loads at runtime.
 * Shared dependencies (SDK, components, Svelte) are resolved from
 * window.__orqa globals — not bundled into the plugin.
 *
 * IIFE format is required because Rollup's `output.globals` option is
 * silently ignored for ESM bundles. With IIFE, bare imports are replaced
 * with references to the globals map, so the bundle has zero bare `import`
 * statements and can be loaded via blob URL without import resolution.
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
			formats: ["iife"],
			name: "OrqaPluginView",
			fileName: (_, entryName) => `views/${entryName}.js`,
		},
		outDir: "dist",
		rollupOptions: {
			// Use a function to externalize all svelte/* subpaths (the compiler
			// emits imports from svelte/internal/client, svelte/internal/disclose-version, etc.)
			external: (id) => {
				if (id === "svelte" || id.startsWith("svelte/")) return true;
				if (id.startsWith("@orqastudio/")) return true;
				return false;
			},
			output: {
				globals: (id: string) => {
					if (id === "svelte" || id.startsWith("svelte/")) return "window.__orqa.svelteInternal";
					if (id.startsWith("@orqastudio/svelte-components/connected"))
						return "window.__orqa.componentsConnected";
					if (id.startsWith("@orqastudio/svelte-components")) return "window.__orqa.components";
					if (id.startsWith("@orqastudio/")) {
						const map: Record<string, string> = {
							"@orqastudio/sdk": "window.__orqa.sdk",
							"@orqastudio/graph-visualiser": "window.__orqa.graphVisualiser",
							"@orqastudio/types": "window.__orqa.types",
						};
						return map[id] ?? "window.__orqa.types";
					}
					return id;
				},
			},
		},
	},
});
