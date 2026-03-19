import type { Meta, StoryObj } from "@storybook/svelte";
import Card from "./SimpleCard.svelte";

const meta = {
	title: "Pure/Card",
	component: Card,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const SimpleWithTitle: Story = {
	args: {
		title: "Graph Health",
		description: "Connection metrics for the artifact graph",
	},
};

export const TitleOnly: Story = {
	args: {
		title: "Settings",
	},
};

export const ContentOnly: Story = {
	args: {},
};
