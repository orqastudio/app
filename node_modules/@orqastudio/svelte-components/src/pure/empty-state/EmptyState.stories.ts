import type { Meta, StoryObj } from "@storybook/svelte";
import EmptyState from "./EmptyState.svelte";

const meta = {
	title: "Pure/EmptyState",
	component: EmptyState,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const TitleOnly: Story = {
	args: { title: "Nothing here yet" },
};

export const WithDescription: Story = {
	args: {
		title: "No artifacts found",
		description: "Try adjusting your filters or creating a new artifact.",
	},
};

export const WithAction: Story = {
	args: {
		title: "No projects",
		description: "Get started by creating your first project.",
		action: { label: "Create Project", onclick: () => console.log("create") },
	},
};
