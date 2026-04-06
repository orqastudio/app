import type { Meta, StoryObj } from "@storybook/svelte";
import { Badge } from "./index.js";

const meta = {
	title: "Pure/Badge",
	component: Badge,
	tags: ["autodocs"],
	argTypes: {
		variant: {
			control: "select",
			options: ["default", "secondary", "destructive", "outline", "warning", "success"],
		},
		capitalize: { control: "boolean" },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: { variant: "default" } };
export const Secondary: Story = { args: { variant: "secondary" } };
export const Destructive: Story = { args: { variant: "destructive" } };
export const Outline: Story = { args: { variant: "outline" } };
export const Warning: Story = { args: { variant: "warning" } };
export const Success: Story = { args: { variant: "success" } };
// Combined: success + xs size + capitalize for live status labels.
export const SuccessXsCapitalize: Story = {
	args: { variant: "success", size: "xs", capitalize: true },
};
