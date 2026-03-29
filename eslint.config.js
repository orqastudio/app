import js from "@eslint/js";
import tseslint from "typescript-eslint";
import svelte from "eslint-plugin-svelte";
import globals from "globals";
import jsdocPlugin from "eslint-plugin-jsdoc";

// Shared restriction objects for no-restricted-syntax rule.
// Defined here so they can be composed into multiple config blocks
// without duplication (flat config merges by last-match, so overlapping
// file globs need combined rule arrays).

/** RULE-006 / Component purity: No invoke() in $lib/components/ */
const noInvokeInComponents = {
	selector: "CallExpression[callee.name='invoke']",
	message:
		"Component purity violation (RULE-006): invoke() must not be called in components. Move the invoke() call to a store or page, and pass data via props.",
};

/** RULE-033: No HTML title attribute — use shadcn Tooltip instead */
const noHtmlTitleAttribute = {
	selector:
		"SvelteElement[kind='html'] > SvelteStartTag > SvelteAttribute[key.name='title']",
	message:
		"Tooltip violation (RULE-033): Use shadcn <Tooltip.Root> instead of HTML title attribute. Exempt: alt on images, title on <svg>.",
};

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
		rules: {
			"@typescript-eslint/no-unused-vars": [
				"error",
				{ argsIgnorePattern: "^_", varsIgnorePattern: "^_" },
			],
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
	// Documentation enforcement: every exported function and class must have a
	// JSDoc description. File-level purpose is enforced by requiring JSDoc on
	// module declarations. Applies to .ts and .svelte.ts files only (not .svelte
	// templates where JSDoc is not idiomatic).
	jsdocPlugin.configs["flat/recommended-typescript"],
	{
		files: ["**/*.ts", "**/*.svelte.ts"],
		rules: {
			"jsdoc/require-jsdoc": ["warn", {
				require: {
					FunctionDeclaration: true,
					MethodDefinition: true,
					ClassDeclaration: true,
				},
				publicOnly: true,
			}],
			"jsdoc/require-description": "warn",
		},
	},
	// RULE-006 / Component purity + RULE-033 / Tooltip usage for component files.
	// Component .svelte files get BOTH restrictions in one block because flat
	// config merges no-restricted-syntax by last-match — separate blocks would
	// cause the later one to override the earlier one.
	{
		files: ["src/lib/components/**/*.svelte"],
		ignores: ["src/lib/components/ui/**"],
		rules: {
			"no-restricted-syntax": [
				"error",
				noInvokeInComponents,
				noHtmlTitleAttribute,
			],
		},
	},
	// RULE-006 / Component purity for non-Svelte component files (.ts helpers).
	// Only the invoke() restriction applies here (no HTML templates in .ts).
	{
		files: ["src/lib/components/**/*.ts"],
		ignores: ["src/lib/components/ui/**"],
		rules: {
			"no-restricted-syntax": ["error", noInvokeInComponents],
		},
	},
	// RULE-033: No HTML title attribute in Svelte files outside components.
	// (Component files get this via the combined block above.)
	{
		files: ["**/*.svelte"],
		ignores: ["src/lib/components/**/*.svelte"],
		rules: {
			"no-restricted-syntax": ["warn", noHtmlTitleAttribute],
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
