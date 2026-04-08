import type { Meta, StoryObj } from "@storybook/svelte";
import ColorSwatch from "./ColorSwatch.svelte";

const meta = {
	title: "Pure/ColorSwatch",
	component: ColorSwatch,
	tags: ["autodocs"],
} satisfies Meta<typeof ColorSwatch>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: { color: "#6366f1", label: "Pick colour for TASK" },
};

export const Red: Story = {
	args: { color: "#ef4444", label: "Pick colour for BUG" },
};

export const Green: Story = {
	args: { color: "#22c55e", label: "Pick colour for DONE" },
};
