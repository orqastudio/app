import type { Meta, StoryObj } from "@storybook/svelte";
import LogColLabel from "./LogColLabel.svelte";

const meta: Meta<typeof LogColLabel> = {
	title: "Pure/LogColLabel",
	component: LogColLabel,
	tags: ["autodocs"],
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Fixed: Story = { args: { width: 90 } };
export const Fill: Story = { args: { fill: true } };
