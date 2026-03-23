import type { Meta, StoryObj } from "@storybook/svelte";
import MetricCell from "./MetricCell.svelte";

const meta = {
	title: "Pure/MetricCell",
	component: MetricCell,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const WithPositiveTrend: Story = {
	args: {
		label: "Errors",
		value: 12,
		trend: 25,
		valueColor: "destructive",
	},
};

export const WithNegativeTrend: Story = {
	args: {
		label: "Warnings",
		value: 3,
		trend: -40,
		valueColor: "warning",
	},
};

export const NoTrend: Story = {
	args: {
		label: "Orphans",
		value: 8,
	},
};

export const HigherIsBetter: Story = {
	args: {
		label: "Coverage",
		value: "92%",
		trend: 5,
		lowerIsBetter: false,
	},
};

export const ZeroTrend: Story = {
	args: {
		label: "Broken Refs",
		value: 0,
		trend: 0,
	},
};
