import { mergeConfig, defineConfig } from "vitest/config";
import { baseVitestConfig } from "@orqastudio/test-config/config";

export default mergeConfig(
	baseVitestConfig,
	defineConfig({
		test: {
			include: ["__tests__/**/*.test.ts"],
		},
	}),
);
