import type { Meta, StoryObj } from "@storybook/svelte";
import TimingChartSvg from "./TimingChartSvg.svelte";

const meta = {
	title: "Pure/TimingChartSvg",
	component: TimingChartSvg,
	tags: ["autodocs"],
	argTypes: {
		width: { control: { type: "range", min: 160, max: 640, step: 16 } },
		height: { control: { type: "range", min: 40, max: 160, step: 8 } },
		sampleCount: { control: { type: "number" } },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: {
		values: [12, 18, 14, 22, 16, 25, 19, 13, 21, 17, 23, 15, 20, 18, 24],
		width: 320,
		height: 80,
		sampleCount: 15,
	},
};

export const Wide: Story = {
	args: {
		values: [5, 10, 8, 15, 12, 20, 18, 14, 22, 19, 25, 16, 21, 17, 23, 11, 13, 24, 9, 7],
		width: 480,
		height: 100,
		sampleCount: 20,
	},
};
