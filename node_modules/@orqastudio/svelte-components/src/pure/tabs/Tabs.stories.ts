import type { Meta, StoryObj } from "@storybook/svelte";
import Tabs from "./SimpleTabs.svelte";

const meta = {
	title: "Pure/Tabs",
	component: Tabs,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const SimpleTabs: Story = {
	args: {
		tabs: [
			{ value: "overview", label: "Overview" },
			{ value: "details", label: "Details" },
			{ value: "history", label: "History" },
		],
	},
};

export const WithDisabledTab: Story = {
	args: {
		tabs: [
			{ value: "active", label: "Active" },
			{ value: "archived", label: "Archived" },
			{ value: "deleted", label: "Deleted", disabled: true },
		],
	},
};
