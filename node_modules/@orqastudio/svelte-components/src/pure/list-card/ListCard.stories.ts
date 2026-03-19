import type { Meta, StoryObj } from "@storybook/svelte";
import ListCard from "./ListCard.svelte";

const meta = {
	title: "Pure/ListCard",
	component: ListCard,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: {
		title: "EPIC-001: Core Architecture",
		description: "Define and implement the foundational architecture.",
		onclick: () => console.log("clicked"),
	},
};

export const WithBadge: Story = {
	args: {
		title: "TASK-042: Implement graph query",
		badge: "TASK",
		description: "Build traversal capabilities.",
		onclick: () => console.log("clicked"),
	},
};

export const Active: Story = {
	args: {
		title: "AD-005: Use Tauri v2",
		active: true,
		onclick: () => console.log("clicked"),
	},
};
