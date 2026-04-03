import type { Meta, StoryObj } from "@storybook/svelte";
import ConnectionIndicator from "./ConnectionIndicator.svelte";

const meta = {
	title: "Pure/ConnectionIndicator",
	component: ConnectionIndicator,
	tags: ["autodocs"],
	argTypes: {
		state: {
			control: { type: "select" },
			options: ["connected", "reconnecting", "disconnected", "waiting"],
		},
	},
} satisfies Meta<ConnectionIndicator>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Connected: Story = { args: { state: "connected" } };
export const Reconnecting: Story = { args: { state: "reconnecting" } };
export const Disconnected: Story = { args: { state: "disconnected" } };
export const Waiting: Story = { args: { state: "waiting" } };
export const CustomLabel: Story = { args: { state: "connected", label: "Daemon online" } };
