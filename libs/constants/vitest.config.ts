/**
 * Vitest configuration for @orqastudio/constants.
 *
 * Uses the Node environment since constants are consumed server-side and in Node
 * processes (daemon, CLI). No browser globals needed.
 */

import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		environment: "node",
		include: ["__tests__/**/*.test.ts"],
	},
});
