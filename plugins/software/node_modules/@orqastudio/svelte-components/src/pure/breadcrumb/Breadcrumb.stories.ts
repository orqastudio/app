import type { Meta, StoryObj } from "@storybook/svelte";
import Breadcrumb from "./Breadcrumb.svelte";

const meta = {
	title: "Pure/Breadcrumb",
	component: Breadcrumb,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const StringItems: Story = {
	args: {
		items: ["Delivery", "Epics", "EPIC-001"],
		showHome: true,
		onHome: () => console.log("home"),
	},
};

export const ObjectItems: Story = {
	args: {
		items: [
			{ label: "Milestones", onClick: () => console.log("milestones") },
			{ label: "MVP Foundation", onClick: () => console.log("ms-001") },
			{ label: "EPIC-003" },
		],
		showHome: true,
		onHome: () => console.log("home"),
	},
};

export const NoHome: Story = {
	args: {
		items: ["Settings", "General"],
		showHome: false,
	},
};

export const WithTruncation: Story = {
	args: {
		items: [
			{ label: "A very long category name", onClick: () => {} },
			{ label: "Another deeply nested subcategory", onClick: () => {} },
			{ label: "The final leaf item" },
		],
		showHome: true,
		onHome: () => {},
		maxWidth: "150px",
	},
};
