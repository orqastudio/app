import type { Meta, StoryObj } from "@storybook/svelte";
import CountBadge from "./CountBadge.svelte";

const meta = {
	title: "Pure/CountBadge",
	component: CountBadge,
	tags: ["autodocs"],
	argTypes: {
		count: { control: "number" },
		variant: {
			control: "select",
			options: ["default", "success", "muted", "warning"],
		},
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: { count: 5, variant: "default" } };
export const Success: Story = { args: { count: 3, variant: "success" } };
export const Muted: Story = { args: { count: 12, variant: "muted" } };
export const Warning: Story = { args: { count: 1, variant: "warning" } };
export const LargeCount: Story = { args: { count: 99, variant: "default" } };
