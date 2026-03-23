import type { Preview } from "@storybook/svelte";
import "../src/tokens/index.css";
import { createMockStores } from "../src/testing/mock-stores.js";

// Register mock stores so connected component stories work automatically
createMockStores();

const preview: Preview = {
	parameters: {
		backgrounds: {
			default: "dark",
			values: [
				{ name: "dark", value: "oklch(0.105 0.018 268)" },
				{ name: "light", value: "oklch(0.970 0.007 227)" },
			],
		},
	},
	decorators: [
		(_, { parameters }) => {
			// Apply dark class to document for dark mode tokens
			const isDark = parameters?.backgrounds?.default !== "light";
			document.documentElement.classList.toggle("dark", isDark);
			return {};
		},
	],
};

export default preview;
