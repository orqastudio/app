import type { Meta, StoryObj } from "@storybook/svelte";
import VisuallyHidden from "./VisuallyHidden.svelte";

const meta = {
	title: "Pure/VisuallyHidden",
	component: VisuallyHidden,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {};
