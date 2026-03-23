import type { Meta, StoryObj } from "@storybook/svelte";
import DropdownMenu from "./SimpleDropdownMenu.svelte";

const meta = {
	title: "Pure/DropdownMenu",
	component: DropdownMenu,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const SimpleItems: Story = {
	args: {
		items: [
			{ label: "Edit", onclick: () => console.log("edit") },
			{ label: "Duplicate", onclick: () => console.log("duplicate") },
			{ separator: true },
			{ label: "Delete", onclick: () => console.log("delete"), destructive: true },
		],
	},
};

export const DisabledItems: Story = {
	args: {
		items: [
			{ label: "Save", onclick: () => {} },
			{ label: "Export (Pro)", onclick: () => {}, disabled: true },
		],
	},
};
