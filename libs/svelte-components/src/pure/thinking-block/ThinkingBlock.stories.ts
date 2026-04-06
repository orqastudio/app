import type { Meta, StoryObj } from "@storybook/svelte";
import ThinkingBlock from "./ThinkingBlock.svelte";

const meta = {
	title: "Pure/ThinkingBlock",
	component: ThinkingBlock,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Completed: Story = {
	args: {
		content:
			"Let me analyze the artifact relationships...\nChecking for circular dependencies.\nAll clear — no cycles detected.",
		isStreaming: false,
	},
};

export const Streaming: Story = {
	args: {
		content: "Analyzing the graph structure...",
		isStreaming: true,
	},
};

export const Empty: Story = {
	args: { content: "", isStreaming: false },
};
