import type { Meta, StoryObj } from "@storybook/svelte";
import SparklineYAxis from "./SparklineYAxis.svelte";

const meta = {
	title: "Pure/SparklineYAxis",
	component: SparklineYAxis,
	tags: ["autodocs"],
	argTypes: {
		height: { control: { type: "range", min: 40, max: 200, step: 10 } },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: { maxLabel: 42, minLabel: "0", height: 80 },
};

export const WithDecimal: Story = {
	args: { maxLabel: "2.5", minLabel: "0", height: 80 },
};
