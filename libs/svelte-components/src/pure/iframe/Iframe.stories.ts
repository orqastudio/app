import type { Meta, StoryObj } from "@storybook/svelte";
import Iframe from "./Iframe.svelte";

const meta = {
	title: "Pure/Iframe",
	component: Iframe,
	tags: ["autodocs"],
	argTypes: {
		src: { control: "text" },
		title: { control: "text" },
		fill: { control: "boolean" },
		sandbox: { control: "text" },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: {
		src: "https://example.com",
		title: "Example page",
		fill: true,
		sandbox: "allow-scripts allow-same-origin",
	},
};
