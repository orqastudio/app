// Vitest configuration for @orqastudio/graph-visualiser.
// Node environment is sufficient — no browser DOM needed for pure element builders.
import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		environment: "node",
		include: ["__tests__/**/*.test.ts"],
	},
});
