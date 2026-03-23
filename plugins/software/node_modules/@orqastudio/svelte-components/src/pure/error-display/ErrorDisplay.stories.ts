import type { Meta, StoryObj } from "@storybook/svelte";
import ErrorDisplay from "./ErrorDisplay.svelte";

const meta = {
	title: "Pure/ErrorDisplay",
	component: ErrorDisplay,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: { message: "Failed to load artifact data." },
};

export const WithRetry: Story = {
	args: {
		message: "Connection timed out.",
		onRetry: () => console.log("retry"),
	},
};
