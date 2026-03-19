import type { Meta, StoryObj } from "@storybook/svelte";
import Tooltip from "./SimpleTooltip.svelte";

const meta = {
	title: "Pure/Tooltip",
	component: Tooltip,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const TopSide: Story = {
	args: {
		side: "top",
	},
};

export const RightSide: Story = {
	args: {
		side: "right",
	},
};

export const BottomSide: Story = {
	args: {
		side: "bottom",
	},
};
