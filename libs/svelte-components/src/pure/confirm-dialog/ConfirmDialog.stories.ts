import type { Meta, StoryObj } from "@storybook/svelte";
import ConfirmDialog from "./ConfirmDialog.svelte";

const meta = {
	title: "Pure/ConfirmDialog",
	component: ConfirmDialog,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const DeleteConfirm: Story = {
	args: {
		open: true,
		title: "Delete permanently?",
		description: "This action cannot be undone.",
		confirmLabel: "Delete",
		onConfirm: () => console.log("deleted"),
	},
};

export const CustomLabels: Story = {
	args: {
		open: true,
		title: "Discard changes?",
		description: "You have unsaved changes that will be lost.",
		confirmLabel: "Discard",
		confirmVariant: "destructive",
		onConfirm: () => console.log("discarded"),
	},
};
