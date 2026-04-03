/**
 * Vitest configuration for @orqastudio/logger.
 *
 * Uses jsdom so that globals like `fetch`, `navigator`, and `Blob` behave
 * as they do in the browser context where the logger primarily runs.
 */

import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		environment: "jsdom",
		include: ["__tests__/**/*.test.ts"],
	},
});
