import type { Meta, StoryObj } from "@storybook/svelte";
import Sparkline from "./Sparkline.svelte";

const meta = {
	title: "Pure/Sparkline",
	component: Sparkline,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Upward: Story = {
	args: {
		values: [2, 4, 3, 8, 5, 12, 9, 15],
		strokeColor: "#22c55e",
	},
};

export const Downward: Story = {
	args: {
		values: [15, 12, 9, 11, 6, 4, 3, 2],
		strokeColor: "#ef4444",
	},
};

export const Flat: Story = {
	args: {
		values: [5, 5, 6, 5, 4, 5, 5, 6],
		strokeColor: "#6b7280",
	},
};

export const SmallSize: Story = {
	args: {
		values: [3, 7, 2, 9, 4, 6],
		width: 60,
		height: 30,
		strokeColor: "#3b82f6",
	},
};

export const NoFill: Story = {
	args: {
		values: [1, 4, 2, 8, 5, 3, 7],
		fillOpacity: 0,
		strokeColor: "#f59e0b",
	},
};
