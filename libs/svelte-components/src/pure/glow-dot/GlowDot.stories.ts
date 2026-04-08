import type { Meta, StoryObj } from "@storybook/svelte";
import GlowDot from "./GlowDot.svelte";

const meta = {
	title: "Pure/GlowDot",
	component: GlowDot,
	tags: ["autodocs"],
	argTypes: {
		tone: {
			control: "select",
			options: ["green", "amber", "red", "empty"],
		},
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {};
export const Green: Story = { args: { tone: "green" } };
export const Amber: Story = { args: { tone: "amber" } };
export const Red: Story = { args: { tone: "red" } };
export const Empty: Story = { args: { tone: "empty" } };
