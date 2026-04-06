import type { Meta, StoryObj } from "@storybook/svelte";
import StatusIndicator from "./StatusIndicator.svelte";

const meta = {
	title: "Connected/StatusIndicator",
	component: StatusIndicator,
	tags: ["autodocs"],
	argTypes: {
		status: {
			control: "select",
			options: [
				"captured",
				"exploring",
				"ready",
				"prioritised",
				"active",
				"hold",
				"blocked",
				"review",
				"completed",
				"surpassed",
				"recurring",
				"archived",
			],
		},
		mode: { control: "select", options: ["badge", "dot", "inline"] },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Badge: Story = { args: { status: "active", mode: "badge" } };
export const Dot: Story = { args: { status: "completed", mode: "dot" } };
export const Inline: Story = { args: { status: "blocked", mode: "inline" } };
