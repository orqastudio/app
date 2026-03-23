import type { Meta, StoryObj } from "@storybook/svelte";
import LoadingSpinner from "./LoadingSpinner.svelte";

const meta = {
	title: "Pure/LoadingSpinner",
	component: LoadingSpinner,
	tags: ["autodocs"],
	argTypes: {
		size: { control: "select", options: ["sm", "md", "lg"] },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Small: Story = { args: { size: "sm" } };
export const Medium: Story = { args: { size: "md" } };
export const Large: Story = { args: { size: "lg" } };
