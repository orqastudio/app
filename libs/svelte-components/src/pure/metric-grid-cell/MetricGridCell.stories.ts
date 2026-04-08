import type { Meta, StoryObj } from "@storybook/svelte";
import MetricGridCell from "./MetricGridCell.svelte";

const meta = {
	title: "Pure/MetricGridCell",
	component: MetricGridCell,
	tags: ["autodocs"],
	argTypes: {
		borderRight: { control: "boolean" },
		borderTop: { control: "boolean" },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: { borderRight: false, borderTop: false },
};

export const WithBorders: Story = {
	args: { borderRight: true, borderTop: true },
};

export const RightOnly: Story = {
	args: { borderRight: true, borderTop: false },
};
