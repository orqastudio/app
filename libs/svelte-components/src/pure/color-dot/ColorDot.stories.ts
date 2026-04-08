import type { Meta, StoryObj } from "@storybook/svelte";
import ColorDot from "./ColorDot.svelte";

const meta = {
	title: "Pure/ColorDot",
	component: ColorDot,
	tags: ["autodocs"],
	argTypes: {
		color: { control: "color" },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {};
export const Blue: Story = { args: { color: "#3b82f6" } };
export const Green: Story = { args: { color: "#22c55e" } };
export const Red: Story = { args: { color: "#ef4444" } };
export const Amber: Story = { args: { color: "#f59e0b" } };
