import type { Meta, StoryObj } from "@storybook/svelte";
import LogLevelBadge from "./LogLevelBadge.svelte";

const meta: Meta<typeof LogLevelBadge> = {
	title: "Pure/LogLevelBadge",
	component: LogLevelBadge,
	tags: ["autodocs"],
	argTypes: {
		level: {
			control: "select",
			options: ["Debug", "Info", "Warn", "Error", "Perf"],
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Debug: Story = { args: { level: "Debug" } };
export const Info: Story = { args: { level: "Info" } };
export const Warn: Story = { args: { level: "Warn" } };
export const Error: Story = { args: { level: "Error" } };
export const Perf: Story = { args: { level: "Perf" } };
