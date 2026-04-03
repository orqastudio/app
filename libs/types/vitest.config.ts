/**
 * Vitest configuration for @orqastudio/types.
 *
 * Uses the Node environment. The type library has no browser dependencies —
 * tests cover pure utility functions and constants derived from core.json.
 */

import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		environment: "node",
		include: ["__tests__/**/*.test.ts"],
	},
});
