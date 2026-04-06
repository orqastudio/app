import type { Meta, StoryObj } from "@storybook/svelte";
import ProgressBar from "./ProgressBar.svelte";

const meta = {
	title: "Pure/ProgressBar",
	component: ProgressBar,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const HalfComplete: Story = {
	args: { label: "Tasks", current: 12, total: 24 },
};

export const Complete: Story = {
	args: { label: "Migration", current: 58, total: 58, colorClass: "bg-success" },
};

export const Empty: Story = {
	args: { label: "Reviews", current: 0, total: 10 },
};

export const Custom: Story = {
	args: { label: "Errors Fixed", current: 3, total: 7, colorClass: "bg-destructive" },
};

export const Mini: Story = {
	args: { mini: true, ratio: 0.65, colorClass: "bg-success" },
};

export const MiniEmpty: Story = {
	args: { mini: true, ratio: 0 },
};

export const MiniComplete: Story = {
	args: { mini: true, ratio: 1, colorClass: "bg-success" },
};
