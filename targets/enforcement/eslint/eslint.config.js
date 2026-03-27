// eslint.config.js
//
// TARGET STATE — composes from OrqaStudio plugin configs. Project-specific rules only.
// Plugin base rules: no-explicit-any, ban-ts-comment, no-unused-vars,
// no-console, svelte/recommended. See plugin sources for details.
//
// This file is a target state artifact. Do not overwrite with generated
// output until the generation pipeline is validated against this target.

import { svelte } from "@orqastudio/plugin-svelte/eslint";
import sveltePlugin from "eslint-plugin-svelte";
import jsdocPlugin from "eslint-plugin-jsdoc";
import globals from "globals";
import tseslint from "typescript-eslint";

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
	// ── Layer 1: Plugin base ──────────────────────────────────────────
	// Imports ALL base rules from the Svelte plugin (which itself extends
	// the TypeScript plugin). This gives us: js/recommended,
	// typescript-eslint/recommended, svelte/flat/recommended, no-explicit-any,
	// ban-ts-comment, no-unused-vars (underscore OK), no-console,
	// TS parser for .svelte files, and test file overrides.
	...svelte(sveltePlugin),

	// ── Layer 2: Environment globals ──────────────────────────────────
	// Tauri apps run in both browser (webview) and node (build tooling).
	{
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.node,
			},
		},
	},

	// ── Layer 3: Documentation enforcement ───────────────────────────
	// Every exported function and class must have a JSDoc description.
	// File-level purpose is enforced by requiring JSDoc on module declarations.
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

	// ── Layer 4: Tauri-specific overrides ─────────────────────────────
	{
		files: ["**/*.svelte", "**/*.svelte.ts"],
		rules: {
			// Tauri desktop app -- anchor navigation is handled by Tauri shell
			// or intercepted via onclick. SvelteKit's resolve() does not apply.
			"svelte/no-navigation-without-resolve": "off",
		},
	},

	// ── Layer 4: Project architecture rules ───────────────────────────

	// RULE-006 + RULE-033: Component .svelte files get BOTH restrictions.
	// Combined in one block because flat config merges no-restricted-syntax
	// by last-match -- separate blocks would cause override.
	{
		files: ["src/lib/components/**/*.svelte"],
		ignores: ["src/lib/components/ui/**"],
		rules: {
			"no-restricted-syntax": [
				"warn",
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
			"no-restricted-syntax": ["warn", noInvokeInComponents],
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

	// ── Layer 5: Ignores ──────────────────────────────────────────────
	{
		ignores: [
			"build/",
			".svelte-kit/",
			"node_modules/",
			"dist/",
		],
	},
);
