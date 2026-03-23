import type { Meta, StoryObj } from "@storybook/svelte";
import DashboardCard from "./DashboardCard.svelte";

const meta = {
	title: "Pure/DashboardCard",
	component: DashboardCard,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const WithTitle: Story = {
	args: {
		title: "Health Trends",
		description: "Based on 20 scans",
	},
};

export const TitleOnly: Story = {
	args: {
		title: "Pipeline Overview",
	},
};

export const ContentOnly: Story = {
	args: {},
};
