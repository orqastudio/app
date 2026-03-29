// eslint.config.js
//
// GENERATED target state — self-contained, no @orqastudio/plugin-* imports.
// This file is the live config for the app. It will be replaced by the
// output of generate-eslint-config.ts once the generation pipeline is
// validated. Until then it stays as a hand-authored self-contained config.

import tseslint from "typescript-eslint";
import sveltePlugin from "eslint-plugin-svelte";
import jsdocPlugin from "eslint-plugin-jsdoc";
import globals from "globals";

// --- Project-Specific AST Restrictions ---
// Defined as objects so they can be composed into overlapping file-glob
// blocks without flat-config last-match override issues.

/** RULE-006 / Component purity: No invoke() in $lib/components/ */
const noInvokeInComponents = {
	selector: "CallExpression[callee.name='invoke']",
	message:
		"Component purity violation (RULE-006): invoke() must not be called " +
		"in components. Move the invoke() call to a store or page, and pass " +
		"data via props.",
};

/** RULE-033: No HTML title attribute -- use shadcn Tooltip instead */
const noHtmlTitleAttribute = {
	selector:
		"SvelteElement[kind='html'] > SvelteStartTag > SvelteAttribute[key.name='title']",
	message:
		"Tooltip violation (RULE-033): Use shadcn <Tooltip.Root> instead of " +
		"HTML title attribute. Exempt: alt on images, title on <svg>.",
};

export default tseslint.config(
	// ── Layer 1: TypeScript base ──────────────────────────────────────────
	...tseslint.configs.recommended,
	{
		files: ["**/*.ts"],
		rules: {
			"@typescript-eslint/no-explicit-any": "error",
			"@typescript-eslint/ban-ts-comment": [
				"error",
				{
					"ts-ignore": true,
					"ts-expect-error": "allow-with-description",
					"ts-nocheck": true,
					"ts-check": false,
				},
			],
			"@typescript-eslint/no-unused-vars": [
				"error",
				{ argsIgnorePattern: "^_", varsIgnorePattern: "^_" },
			],
			"no-console": "error",
		},
	},
	{
		files: ["**/*.test.ts", "**/*.test.js", "**/__tests__/**"],
		rules: {
			"@typescript-eslint/no-unused-vars": "off",
		},
	},
	{
		files: ["**/*.worker.ts", "**/logger.ts", "**/dev-console.ts"],
		rules: {
			"no-console": "off",
		},
	},

	// ── Layer 2: Svelte ───────────────────────────────────────────────────
	...sveltePlugin.configs["flat/recommended"],
	{
		files: ["**/*.svelte", "**/*.svelte.ts"],
		languageOptions: {
			parserOptions: {
				parser: tseslint.parser,
			},
		},
		rules: {
			"@typescript-eslint/no-explicit-any": "error",
		},
	},

	// ── Layer 3: Environment globals ──────────────────────────────────────
	// Tauri apps run in both browser (webview) and node (build tooling).
	{
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.node,
			},
		},
	},

	// ── Layer 4: Documentation enforcement ───────────────────────────────
	// Every exported function and class must have a JSDoc description.
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

	// ── Layer 5: Tauri-specific overrides ─────────────────────────────────
	{
		files: ["**/*.svelte", "**/*.svelte.ts"],
		rules: {
			// Tauri desktop app -- anchor navigation is handled by Tauri shell
			// or intercepted via onclick. SvelteKit's resolve() does not apply.
			"svelte/no-navigation-without-resolve": "off",
		},
	},

	// ── Layer 6: Project architecture rules ───────────────────────────────

	// RULE-006 + RULE-033: Component .svelte files get BOTH restrictions.
	// Combined in one block because flat config merges no-restricted-syntax
	// by last-match -- separate blocks would cause override.
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
	// RULE-006: Component .ts helpers -- only invoke() restriction (no templates).
	{
		files: ["src/lib/components/**/*.ts"],
		ignores: ["src/lib/components/ui/**"],
		rules: {
			"no-restricted-syntax": ["error", noInvokeInComponents],
		},
	},
	// RULE-033: Non-component .svelte files -- only title attribute restriction.
	{
		files: ["**/*.svelte"],
		ignores: ["src/lib/components/**/*.svelte"],
		rules: {
			"no-restricted-syntax": ["warn", noHtmlTitleAttribute],
		},
	},

	// ── Layer 7: Ignores ──────────────────────────────────────────────────
	{
		ignores: [
			"build/",
			".svelte-kit/",
			"node_modules/",
			"dist/",
		],
	},
);
