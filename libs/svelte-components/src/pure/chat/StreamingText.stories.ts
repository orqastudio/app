import type { Meta, StoryObj } from "@storybook/svelte";
import StreamingText from "./StreamingText.svelte";

const meta = {
	title: "Pure/Chat/StreamingText",
	component: StreamingText,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Short: Story = {
	args: { content: "Hello, world!" },
};

export const MultiLine: Story = {
	args: {
		content:
			"Line one of streaming output.\nLine two appears next.\nLine three is the last one so far.",
	},
};

export const LongContent: Story = {
	args: {
		content:
			"This is a longer piece of streaming text that demonstrates how the component handles content that spans multiple lines and continues to grow as the assistant generates its response.",
	},
};
