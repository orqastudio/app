import type { Meta, StoryObj } from "@storybook/svelte";
import LogColumn from "./LogColumn.svelte";

const meta: Meta<typeof LogColumn> = {
	title: "Pure/LogColumn",
	component: LogColumn,
	tags: ["autodocs"],
	argTypes: {
		variant: {
			control: "select",
			options: ["timestamp", "badge", "source", "category", "fill"],
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Timestamp: Story = {
	args: {
		variant: "timestamp",
	},
};

export const Source: Story = {
	args: {
		variant: "source",
	},
};

export const Fill: Story = {
	args: {
		variant: "fill",
	},
};
