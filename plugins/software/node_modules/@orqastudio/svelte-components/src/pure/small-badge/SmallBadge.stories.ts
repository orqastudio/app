import type { Meta, StoryObj } from "@storybook/svelte";
import SmallBadge from "./SmallBadge.svelte";

const meta = {
	title: "Pure/SmallBadge",
	component: SmallBadge,
	tags: ["autodocs"],
	argTypes: {
		variant: {
			control: "select",
			options: ["default", "secondary", "destructive", "outline", "warning"],
		},
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: { variant: "default" } };
export const Secondary: Story = { args: { variant: "secondary" } };
export const Destructive: Story = { args: { variant: "destructive" } };
