import type { Meta, StoryObj } from "@storybook/svelte";
import LogRowShell from "./LogRowShell.svelte";

const meta: Meta<typeof LogRowShell> = {
	title: "Pure/LogRowShell",
	component: LogRowShell,
	tags: ["autodocs"],
	argTypes: {
		level: {
			control: "select",
			options: ["debug", "info", "warn", "error", "perf", undefined],
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		level: "info",
		topPx: 0,
	},
};

export const Warning: Story = {
	args: {
		level: "warn",
		topPx: 24,
	},
};

export const Error: Story = {
	args: {
		level: "error",
		topPx: 48,
	},
};
