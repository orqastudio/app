import type { Meta, StoryObj } from "@storybook/svelte";
import MetadataRow from "./MetadataRow.svelte";

const meta = {
	title: "Pure/MetadataRow",
	component: MetadataRow,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const WithItems: Story = {
	args: {
		label: "Tags",
		items: ["governance", "architecture", "critical"],
	},
};

export const SingleItem: Story = {
	args: {
		label: "Priority",
		items: ["P1"],
		badgeVariant: "destructive",
	},
};

export const Empty: Story = {
	args: {
		label: "Labels",
		items: [],
	},
};
