import type { Meta, StoryObj } from "@storybook/svelte";
import MarkdownRenderer from "./MarkdownRenderer.svelte";

const meta = {
	title: "Connected/MarkdownRenderer",
	component: MarkdownRenderer,
	tags: ["autodocs"],
	argTypes: {
		content: { control: "text" },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: {
		content:
			"# Hello World\n\nThis is **bold** and *italic* text.\n\n- List item 1\n- List item 2\n\n```typescript\nconst x = 42;\n```",
	},
};

export const Empty: Story = {
	args: { content: "" },
};

export const CodeBlock: Story = {
	args: { content: '```rust\nfn main() {\n    println!("Hello, world!");\n}\n```' },
};
