import type { Meta, StoryObj } from "@storybook/svelte";
import Prose from "./Prose.svelte";

const meta = {
	title: "Pure/Prose",
	component: Prose,
	tags: ["autodocs"],
	argTypes: {
		size: {
			control: "select",
			options: ["sm", "base", "lg"],
		},
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: { size: "base" } };
export const Small: Story = { args: { size: "sm" } };
export const Large: Story = { args: { size: "lg" } };
