import type { Meta, StoryObj } from "@storybook/svelte";
import AlertDialog from "./SimpleAlertDialog.svelte";

const meta = {
	title: "Pure/AlertDialog",
	component: AlertDialog,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const SimpleConfirm: Story = {
	args: {
		open: true,
		title: "Delete permanently?",
		description: "This action cannot be undone. All related data will be lost.",
		confirmLabel: "Delete",
		cancelLabel: "Cancel",
		onConfirm: () => console.log("confirmed"),
	},
};

export const CustomLabels: Story = {
	args: {
		open: true,
		title: "Reset to defaults?",
		description: "All custom settings will be reverted.",
		confirmLabel: "Reset",
		cancelLabel: "Keep",
		onConfirm: () => console.log("reset"),
	},
};
