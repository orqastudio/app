import type { Meta, StoryObj } from "@storybook/svelte";
import ArtifactListItem from "./ArtifactListItem.svelte";

const meta = {
	title: "Connected/ArtifactListItem",
	component: ArtifactListItem,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const WithStatus: Story = {
	args: {
		label: "EPIC-001: Core Architecture",
		description: "Define and implement the foundational architecture.",
		status: "active",
		onclick: () => console.log("clicked"),
	},
};

export const WithBadge: Story = {
	args: {
		label: "AD-005: Use Tauri v2",
		badge: "AD",
		onclick: () => console.log("clicked"),
	},
};

export const Active: Story = {
	args: {
		label: "TASK-042: Build graph query engine",
		status: "exploring",
		active: true,
		onclick: () => console.log("clicked"),
	},
};
