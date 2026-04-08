import type { Meta, StoryObj } from "@storybook/svelte";
import PreText from "./PreText.svelte";

const meta = {
	title: "Pure/PreText",
	component: PreText,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: {} };
export const MultiLine: Story = { args: {} };
