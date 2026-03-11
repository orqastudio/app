import js from "@eslint/js";
import tseslint from "typescript-eslint";
import svelte from "eslint-plugin-svelte";
import globals from "globals";

export default tseslint.config(
	js.configs.recommended,
	...tseslint.configs.recommended,
	...svelte.configs["flat/recommended"],
	{
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.node,
			},
		},
	},
	{
		files: ["**/*.svelte", "**/*.svelte.ts"],
		languageOptions: {
			parserOptions: {
				parser: tseslint.parser,
			},
		},
		rules: {
			// This is a Tauri desktop app, not a SvelteKit web app.
			// Anchor href navigation is handled by Tauri shell or intercepted
			// via onclick handlers — SvelteKit's resolve() does not apply.
			"svelte/no-navigation-without-resolve": "off",
		},
	},
	{
		ignores: [
			"build/",
			".svelte-kit/",
			"node_modules/",
			"src-tauri/",
			"sidecar/",
			"dist/",
			"tmp/",
			".githooks/",
			".orqa/plugins/",
			"scripts/",
		],
	},
);
