import type { Meta, StoryObj } from "@storybook/svelte";
import Status from "./Status.svelte";

const meta = {
	title: "Pure/Status",
	component: Status,
	tags: ["autodocs"],
	argTypes: {
		status: {
			control: "select",
			options: ["captured", "exploring", "ready", "prioritised", "active", "hold", "blocked", "review", "completed", "surpassed", "recurring", "archived"],
		},
		mode: {
			control: "select",
			options: ["badge", "dot", "inline"],
		},
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const BadgeMode: Story = {
	args: { status: "active", mode: "badge" },
};

export const DotMode: Story = {
	args: { status: "completed", mode: "dot" },
};

export const InlineMode: Story = {
	args: { status: "blocked", mode: "inline" },
};

export const Captured: Story = {
	args: { status: "captured", mode: "badge" },
};

export const Completed: Story = {
	args: { status: "completed", mode: "badge" },
};

export const Blocked: Story = {
	args: { status: "blocked", mode: "badge" },
};

export const CustomStatuses: Story = {
	args: {
		status: "draft",
		mode: "badge",
		statuses: [
			{ key: "draft", label: "Draft", icon: "circle", color: "muted" },
			{ key: "published", label: "Published", icon: "circle-check-big", color: "success" },
		],
	},
};
