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
	// RULE-006: No any types — make explicit (already error from recommended,
	// but stated here for visibility and to prevent accidental override)
	{
		files: ["**/*.ts", "**/*.svelte", "**/*.svelte.ts"],
		rules: {
			"@typescript-eslint/no-explicit-any": "error",
		},
	},
	// RULE-006 / Component purity: No invoke() calls in $lib/components/.
	// Components receive data via props; only stores and pages call invoke().
	// Exceptions: ui/components (shadcn primitives) are excluded.
	{
		files: ["src/lib/components/**/*.svelte", "src/lib/components/**/*.ts"],
		ignores: ["src/lib/components/ui/**"],
		rules: {
			"no-restricted-syntax": [
				"warn",
				{
					selector: "CallExpression[callee.name='invoke']",
					message:
						"Component purity violation (RULE-006): invoke() must not be called in components. Move the invoke() call to a store or page, and pass data via props.",
				},
			],
		},
	},
	// RULE-033: No HTML title attribute on interactive elements.
	// Use shadcn Tooltip component instead of native browser tooltips.
	// This targets <button title="...">, <a title="...">, etc.
	// Component props named "title" (e.g. <EmptyState title="...">) are NOT
	// flagged because they use SvelteComponent AST nodes, not SvelteHTMLElement.
	{
		files: ["**/*.svelte"],
		rules: {
			"no-restricted-syntax": [
				"warn",
				{
					selector:
						"SvelteElement[kind='html'] > SvelteStartTag > SvelteAttribute[key.name='title']",
					message:
						"Tooltip violation (RULE-033): Use shadcn <Tooltip.Root> instead of HTML title attribute. Exempt: alt on images, title on <svg>.",
				},
			],
		},
	},
	{
		files: ["**/*.test.ts", "**/*.test.js", "**/__tests__/**"],
		rules: {
			"@typescript-eslint/no-unused-vars": "off",
		},
	},
	{
		ignores: [
			"build/",
			".svelte-kit/",
			"node_modules/",
			"dist/",
		],
	},
);
