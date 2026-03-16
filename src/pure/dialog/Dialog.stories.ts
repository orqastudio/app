import type { Meta, StoryObj } from "@storybook/svelte";
import Dialog from "./SimpleDialog.svelte";

const meta = {
	title: "Pure/Dialog",
	component: Dialog,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const SimpleWithTitle: Story = {
	args: {
		open: true,
		title: "Edit Project",
		description: "Update your project settings below.",
	},
};

export const OpenByDefault: Story = {
	args: {
		open: true,
		title: "Confirm Action",
	},
};
