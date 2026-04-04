import type { Meta, StoryObj } from "@storybook/svelte";
import Grid from "./Grid.svelte";

const meta = {
	title: "Pure/Layout/Grid",
	component: Grid,
	tags: ["autodocs"],
	argTypes: {
		cols: {
			control: "select",
			options: [1, 2, 3, 4, 6, 12],
		},
		gap: {
			control: "select",
			options: [0, 1, 2, 3, 4, 6, 8],
		},
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const TwoColumns: Story = { args: { cols: 2, gap: 4 } };
export const ThreeColumns: Story = { args: { cols: 3, gap: 4 } };
export const FourColumns: Story = { args: { cols: 4, gap: 2 } };
