import type { Meta, StoryObj } from "@storybook/svelte";
import NavItem from "./NavItem.svelte";

const meta = {
	title: "Pure/NavItem",
	component: NavItem,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: { label: "Epics", onclick: () => console.log("clicked") },
};

export const WithBadge: Story = {
	args: { label: "Tasks", badge: 42, onclick: () => console.log("clicked") },
};

export const Active: Story = {
	args: { label: "Rules", active: true, badge: 8 },
};

export const CollapsibleClosed: Story = {
	args: { label: "Delivery", collapsible: true, expanded: false, badge: 3 },
};

export const CollapsibleOpen: Story = {
	args: { label: "Delivery", collapsible: true, expanded: true, badge: 3 },
};
