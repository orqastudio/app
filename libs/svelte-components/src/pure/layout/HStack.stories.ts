import type { Meta, StoryObj } from "@storybook/svelte";
import HStack from "./HStack.svelte";

const meta = {
	title: "Pure/Layout/HStack",
	component: HStack,
	tags: ["autodocs"],
	argTypes: {
		gap: {
			control: "select",
			options: [0, 0.5, 1, 1.5, 2, 3, 4, 6, 8],
		},
		align: {
			control: "select",
			options: ["start", "center", "end", "baseline", "stretch"],
		},
		justify: {
			control: "select",
			options: ["start", "center", "end", "between", "around"],
		},
		wrap: { control: "boolean" },
		full: { control: "boolean" },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: { gap: 2 } };
export const SpaceBetween: Story = { args: { gap: 2, justify: "between" } };
export const Wrapped: Story = { args: { gap: 2, wrap: true } };
