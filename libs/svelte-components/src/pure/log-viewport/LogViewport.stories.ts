import type { Meta, StoryObj } from "@storybook/svelte";
import LogViewport from "./LogViewport.svelte";

const meta: Meta<typeof LogViewport> = {
	title: "Pure/LogViewport",
	component: LogViewport,
	tags: ["autodocs"],
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		ariaLabel: "Log events",
		ariaRowCount: 100,
	},
};
