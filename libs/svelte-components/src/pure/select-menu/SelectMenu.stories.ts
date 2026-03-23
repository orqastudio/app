import type { Meta, StoryObj } from "@storybook/svelte";
import SelectMenu from "./SelectMenu.svelte";

const meta = {
	title: "Pure/SelectMenu",
	component: SelectMenu,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: {
		items: [
			{ value: "all", label: "All" },
			{ value: "active", label: "Active" },
			{ value: "completed", label: "Completed" },
		],
		selected: "all",
		triggerLabel: "Status",
		onSelect: (v: string) => console.log("selected", v),
	},
};

export const SmallTrigger: Story = {
	args: {
		items: [
			{ value: "name", label: "Name" },
			{ value: "date", label: "Date" },
			{ value: "status", label: "Status" },
		],
		selected: "name",
		triggerLabel: "Sort by",
		triggerSize: "sm",
		onSelect: (v: string) => console.log("sort", v),
	},
};
