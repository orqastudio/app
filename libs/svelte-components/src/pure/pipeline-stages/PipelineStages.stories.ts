import type { Meta, StoryObj } from "@storybook/svelte";
import PipelineStages from "./PipelineStages.svelte";

const meta = {
	title: "Pure/PipelineStages",
	component: PipelineStages,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const BasicStages: Story = {
	args: {
		stages: [
			{ key: "captured", label: "Captured", count: 12, dotColorClass: "bg-muted-foreground" },
			{ key: "active", label: "Active", count: 8, dotColorClass: "bg-primary" },
			{ key: "completed", label: "Completed", count: 24, dotColorClass: "bg-emerald-500" },
		],
	},
};

export const WithEdges: Story = {
	args: {
		stages: [
			{ key: "ideas", label: "Ideas", count: 15, dotColorClass: "bg-blue-400" },
			{ key: "research", label: "Research", count: 6, dotColorClass: "bg-amber-400" },
			{ key: "ready", label: "Ready", count: 3, dotColorClass: "bg-emerald-400" },
		],
		edges: [{ count: 4 }, { count: 2 }],
	},
};

export const WithTooltips: Story = {
	args: {
		stages: [
			{
				key: "captured",
				label: "Captured",
				count: 5,
				dotColorClass: "bg-muted-foreground",
				tooltipTitle: "Newly captured",
				tooltipBody: "Ideas that haven't been explored yet.",
			},
			{
				key: "active",
				label: "Active",
				count: 3,
				dotColorClass: "bg-primary",
				tooltipTitle: "In progress",
				tooltipBody: "Currently being worked on.",
			},
		],
	},
};
