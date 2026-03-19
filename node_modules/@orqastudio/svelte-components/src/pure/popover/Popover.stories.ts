import type { Meta, StoryObj } from "@storybook/svelte";
import Popover from "./SimplePopover.svelte";

const meta = {
	title: "Pure/Popover",
	component: Popover,
	tags: ["autodocs"],
	argTypes: {
		align: { control: "select", options: ["start", "center", "end"] },
		side: { control: "select", options: ["top", "bottom", "left", "right"] },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Bottom: Story = { args: { side: "bottom", align: "center" } };
export const StartAligned: Story = { args: { side: "bottom", align: "start" } };
