import type { Meta, StoryObj } from "@storybook/svelte";
import DynamicGrid from "./DynamicGrid.svelte";

const meta = {
	title: "Pure/DynamicGrid",
	component: DynamicGrid,
	tags: ["autodocs"],
	argTypes: {
		columns: { control: { type: "range", min: 1, max: 8, step: 1 } },
		minWidth: { control: "text" },
		gap: {
			control: "select",
			options: [1, 2, 3, 4, 6, 8],
		},
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const TwoColumns: Story = {
	args: { columns: 2, minWidth: "200px", gap: 4 },
};

export const FourColumns: Story = {
	args: { columns: 4, minWidth: "200px", gap: 4 },
};

export const NarrowMin: Story = {
	args: { columns: 3, minWidth: "120px", gap: 2 },
};
