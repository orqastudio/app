import type { Meta, StoryObj } from "@storybook/svelte";
import ViewContainer from "./ViewContainer.svelte";

const meta = {
	title: "Pure/ViewContainer",
	component: ViewContainer,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Loading: Story = {
	args: { loading: true },
};

export const Error: Story = {
	args: {
		error: "Failed to load artifacts from the graph index.",
		onRetry: () => console.log("retry"),
	},
};

export const Empty: Story = {
	args: {
		empty: true,
		emptyTitle: "No artifacts yet",
		emptyDescription: "Create your first artifact to get started.",
	},
};

export const Content: Story = {
	args: { loading: false, empty: false },
};
